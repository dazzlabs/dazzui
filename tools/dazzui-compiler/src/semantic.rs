use std::collections::HashMap;

use crate::ir::{
    ActionPayload, ActionRef, ChildrenSpec, Component, ComponentId, ComponentSpec, Expr, Page,
    PropType, SourceLocation, UiEvent, UiNode, UiProp, Value,
};
use crate::syntax::{ArgAst, ComponentAst, ModifierAst, NodeAst, PageAst, PropAst, ValueAst, spanned_path};
use proc_macro2::Span;
use syn::Lit;

use crate::error::{SemanticError, closest, locate, suggest};
use crate::registry::Registry;
use crate::rust_types::{EnumDef, Fields, RustTypes};

/// Lowers a parsed page to IR.
///
/// `file` is the path locations are recorded against. With `types`, every
/// action is also checked against the page's action enum; without them —
/// in the macro — that check is rustc's.
pub fn lower(
    page: &PageAst,
    registry: &Registry,
    types: Option<&RustTypes>,
    file: &str,
) -> Result<Page, Vec<SemanticError>> {
    let action_type = last_ident(&page.action_ty);
    let mut cx = Cx { registry, file, errors: Vec::new(), action_enum: None };

    if let Some(types) = types {
        match types.enums.get(&action_type) {
            Some(def) => cx.action_enum = Some(def),
            None => cx.errors.push(
                SemanticError::new(
                    spanned_path(&page.action_ty),
                    format!("cannot find enum `{action_type}` in this crate"),
                )
                .help("the page's action type must be an enum the CLI can read from `--src`"),
            ),
        }
    }

    let root = cx.node(&page.root);
    if !cx.errors.is_empty() {
        return Err(cx.errors);
    }
    Ok(Page {
        name: page.name.to_string(),
        state_type: last_ident(&page.state_ty),
        action_type,
        root: root.expect("a node with no errors lowers"),
        source: cx.loc(page.name.span()),
    })
}

fn last_ident(path: &syn::Path) -> String {
    path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
}

struct Cx<'a> {
    registry: &'a Registry,
    file: &'a str,
    errors: Vec<SemanticError>,
    action_enum: Option<&'a EnumDef>,
}

impl Cx<'_> {
    fn loc(&self, span: Span) -> SourceLocation {
        locate(self.file, span).0
    }

    fn node(&mut self, node: &NodeAst) -> Option<UiNode> {
        match node {
            NodeAst::Component(c) => self.component(c).map(UiNode::Component),
        }
    }

    fn component(&mut self, ast: &ComponentAst) -> Option<Component> {
        let name = ast.name.to_string();
        let Some(spec) = self.registry.get(&name) else {
            self.errors.push(
                SemanticError::new(ast.name.span(), format!("unknown component `{name}`"))
                    .label("not in the component registry")
                    .help_opt(suggest(&name, self.registry.names())),
            );
            // Its children are still worth checking.
            for child in &ast.children {
                self.node(child);
            }
            return None;
        };
        let errors_before = self.errors.len();

        // Every way a prop can be given, in source order: positionally,
        // named in the parentheses, named in the body.
        let mut given: Vec<&PropAst> = Vec::new();
        let mut positional_props: Vec<(String, &ValueAst)> = Vec::new();
        let mut positional_index = 0;
        for arg in &ast.args {
            match arg {
                ArgAst::Named(prop) => given.push(prop),
                ArgAst::Positional(value) => {
                    match spec.positional.get(positional_index) {
                        Some(prop) => positional_props.push((prop.clone(), value)),
                        None => self.errors.push(
                            SemanticError::new(
                                value.span(),
                                format!(
                                    "`{name}` takes {} positional argument{}",
                                    spec.positional.len(),
                                    if spec.positional.len() == 1 { "" } else { "s" },
                                ),
                            )
                            .label("unexpected argument")
                            .help("name it: `prop: value`"),
                        ),
                    }
                    positional_index += 1;
                }
            }
        }
        given.extend(&ast.props);

        let mut seen: HashMap<String, Span> = HashMap::new();
        // Props a misspelling was taken for: already reported, so not
        // reported again as missing.
        let mut meant: Vec<String> = Vec::new();
        let mut props = Vec::new();
        let named = given.iter().map(|p| (p.name.to_string(), p.name.span(), &p.value));
        let positional = positional_props.iter().map(|(n, v)| (n.clone(), v.span(), *v));
        for (prop_name, span, value) in positional.chain(named) {
            let Some(prop_spec) = spec.prop(&prop_name) else {
                let candidates = spec.props.iter().map(|p| p.name.as_str());
                let closest = closest(&prop_name, candidates);
                meant.extend(closest.map(str::to_owned));
                self.errors.push(
                    SemanticError::new(
                        span,
                        format!("property `{prop_name}` does not exist on component `{name}`"),
                    )
                    .help_opt(closest.map(|c| format!("did you mean `{c}`?"))),
                );
                continue;
            };
            if seen.insert(prop_name.clone(), span).is_some() {
                self.errors.push(
                    SemanticError::new(span, format!("property `{prop_name}` is given twice"))
                        .label("given again here"),
                );
                continue;
            }
            if let Some(expr) = self.value(value, prop_spec.ty, &prop_name, &name) {
                props.push(UiProp { name: prop_name, value: expr, source: self.loc(span) });
            }
        }

        // A default is resolved here rather than by each backend, so the IR
        // says what is drawn and no platform's own default leaks in.
        for prop_spec in &spec.props {
            if let (false, Some(default)) = (seen.contains_key(&prop_spec.name), &prop_spec.default) {
                props.push(UiProp {
                    name: prop_spec.name.clone(),
                    value: Expr::Literal(default.clone()),
                    source: self.loc(ast.name.span()),
                });
            }
        }

        for prop_spec in spec.props.iter().filter(|p| p.required) {
            if !seen.contains_key(&prop_spec.name) && !meant.contains(&prop_spec.name) {
                let how = if spec.positional.first() == Some(&prop_spec.name) {
                    format!("pass it first: `{name}(…)`, or name it: `{}: …`", prop_spec.name)
                } else {
                    format!("add `{}: …`", prop_spec.name)
                };
                self.errors.push(
                    SemanticError::new(
                        ast.name.span(),
                        format!(
                            "missing required property `{}` on component `{name}`",
                            prop_spec.name
                        ),
                    )
                    .help(how),
                );
            }
        }

        let events = self.events(spec, &ast.modifiers);

        let mut children = Vec::new();
        if spec.children == ChildrenSpec::None && !ast.children.is_empty() {
            let NodeAst::Component(first) = &ast.children[0];
            self.errors.push(
                SemanticError::new(first.name.span(), format!("`{name}` takes no children"))
                    .label("child given here"),
            );
        } else {
            for child in &ast.children {
                if let Some(node) = self.node(child) {
                    children.push(node);
                }
            }
        }

        if self.errors.len() > errors_before {
            return None;
        }
        Some(Component {
            id: ComponentId(name),
            props,
            events,
            children,
            source: self.loc(ast.name.span()),
        })
    }

    fn value(&mut self, value: &ValueAst, ty: PropType, prop: &str, component: &str) -> Option<Expr> {
        let found = match value {
            ValueAst::Lit(Lit::Str(s)) if ty == PropType::String => {
                return Some(Expr::Literal(Value::String(s.value())));
            }
            ValueAst::Lit(Lit::Bool(b)) if ty == PropType::Bool => {
                return Some(Expr::Literal(Value::Bool(b.value)));
            }
            ValueAst::Lit(Lit::Int(i)) if ty == PropType::Int => match i.base10_parse::<i64>() {
                Ok(n) => return Some(Expr::Literal(Value::Int(n))),
                Err(e) => {
                    self.errors.push(SemanticError::new(i.span(), e.to_string()));
                    return None;
                }
            },
            ValueAst::NegInt(Lit::Int(i)) if ty == PropType::Int => {
                match format!("-{}", i.base10_digits()).parse::<i64>() {
                    Ok(n) => return Some(Expr::Literal(Value::Int(n))),
                    Err(e) => {
                        self.errors.push(SemanticError::new(i.span(), e.to_string()));
                        return None;
                    }
                }
            }
            ValueAst::NegInt(_) => "an integer",
            ValueAst::Lit(Lit::Str(_)) => "a string",
            ValueAst::Lit(Lit::Bool(_)) => "a bool",
            ValueAst::Lit(Lit::Int(_)) => "an integer",
            ValueAst::Lit(_) => "an unsupported literal",
            ValueAst::Path(p) if is_action(p) => "an action",
            ValueAst::Path(_) => "an enum case",
        };
        let mut error = SemanticError::new(
            value.span(),
            format!("property `{prop}` on `{component}` expects {}, found {found}", ty.name()),
        )
        .label(format!("expected {}", ty.name()));
        if found == "an action" {
            error = error.help("actions are wired with an event modifier, such as `.on_click(…)`");
        }
        self.errors.push(error);
        None
    }

    fn events(&mut self, spec: &ComponentSpec, modifiers: &[ModifierAst]) -> Vec<UiEvent> {
        let mut events: Vec<UiEvent> = Vec::new();
        for modifier in modifiers {
            let modifier_name = modifier.name.to_string();
            let Some(event) = spec.event_by_modifier(&modifier_name) else {
                let candidates = spec.events.iter().map(|e| e.modifier.as_str());
                self.errors.push(
                    SemanticError::new(
                        modifier.name.span(),
                        format!("unknown modifier `{modifier_name}` on component `{}`", spec.name),
                    )
                    .help_opt(suggest(&modifier_name, candidates)),
                );
                continue;
            };
            if events.iter().any(|e| e.name == event.name) {
                self.errors.push(
                    SemanticError::new(
                        modifier.name.span(),
                        format!("`{modifier_name}` is given twice"),
                    )
                    .label("given again here"),
                );
                continue;
            }
            let action = match modifier.args.as_slice() {
                [ArgAst::Positional(ValueAst::Path(path))] if is_action(path) => self.action(path, &event.name),
                _ => {
                    self.errors.push(
                        SemanticError::new(
                            modifier.name.span(),
                            format!("`{modifier_name}` takes one action: `{modifier_name}(Action::Variant)`"),
                        ),
                    );
                    None
                }
            };
            if let Some(action) = action {
                events.push(UiEvent {
                    name: event.name.clone(),
                    action,
                    source: self.loc(modifier.name.span()),
                });
            }
        }
        events
    }

    fn action(&mut self, path: &syn::Path, event: &str) -> Option<ActionRef> {
        let segments: Vec<_> = path.segments.iter().collect();
        if segments.len() != 2 {
            self.errors.push(SemanticError::new(
                spanned_path(path),
                "an action is written `Action::Variant`",
            ));
            return None;
        }
        let variant_ident = &segments[1].ident;
        let variant = variant_ident.to_string();
        if let Some(def) = self.action_enum {
            match def.variant(&variant) {
                None => {
                    let candidates = def.variants.iter().map(|v| v.name.as_str());
                    self.errors.push(
                        SemanticError::new(
                            variant_ident.span(),
                            format!("no variant `{variant}` on action enum `{}`", def.name),
                        )
                        .help_opt(suggest(&variant, candidates)),
                    );
                    return None;
                }
                Some(v) if v.fields != Fields::Unit => {
                    self.errors.push(
                        SemanticError::new(
                            variant_ident.span(),
                            format!("`{}::{variant}` carries a value, but `{event}` has none to give it", def.name),
                        )
                        .label("expects a payload"),
                    );
                    return None;
                }
                Some(_) => {}
            }
        }
        Some(ActionRef { variant, payload: ActionPayload::None })
    }
}

fn is_action(path: &syn::Path) -> bool {
    path.segments.first().is_some_and(|s| s.ident == "Action")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_types::RustTypes;

    fn types() -> RustTypes {
        let file: syn::File = syn::parse_str("enum A { Save, SetOn(bool) }").unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items);
        types
    }

    fn lower_str(src: &str) -> Result<Page, Vec<String>> {
        let ast: PageAst = syn::parse_str(src).unwrap();
        lower(&ast, &Registry::builtin(), Some(&types()), "page.rs")
            .map_err(|errors| errors.into_iter().map(|e| e.message).collect())
    }

    #[test]
    fn lowers_a_page_and_fills_defaults() {
        let page = lower_str(r#"page P(state: S, action: A) { VStack { Button("Save").on_click(Action::Save) } }"#).unwrap();
        let UiNode::Component(stack) = &page.root;
        assert_eq!(stack.prop("spacing"), Some(&Expr::Literal(Value::Int(0))));
        let UiNode::Component(button) = &stack.children[0];
        assert_eq!(button.event("click").unwrap().variant, "Save");
    }

    #[test]
    fn reports_unknown_component_prop_and_modifier() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack { Buton("x") Text("a", "b") Button { titel: "x" }.on_tap(Action::Save) } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "unknown component `Buton`",
                "`Text` takes 1 positional argument",
                "property `titel` does not exist on component `Button`",
                "unknown modifier `on_tap` on component `Button`",
            ]
        );
    }

    #[test]
    fn checks_actions_against_the_enum() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack { Button("a").on_click(Action::Sav) Button("b").on_click(Action::SetOn) } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no variant `Sav` on action enum `A`",
                "`A::SetOn` carries a value, but `click` has none to give it",
            ]
        );
    }

    #[test]
    fn checks_types_and_duplicates() {
        let errors = lower_str(r#"page P(state: S, action: A) { VStack(spacing: "x") { Button("a", title: "b") Text("t") { Text("u") } } }"#)
            .unwrap_err();
        assert_eq!(
            errors,
            [
                "property `spacing` on `VStack` expects Integer, found a string",
                "property `title` is given twice",
                "`Text` takes no children",
            ]
        );
    }
}
