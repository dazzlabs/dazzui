use std::collections::HashMap;

use crate::ir::{
    ActionPayload, ActionRef, ChildrenSpec, Component, ComponentId, ComponentSpec, Condition, EventSpec, Expr,
    LocalPath, Page, PropSpec, PropType, SourceLocation, StatePath, UiBinding, UiEvent, UiNode, UiProp, UiSlot, Value,
};
use crate::syntax::{ArgAst, ComponentAst, DottedAst, ForAst, IfAst, NodeAst, PageAst, ValueAst, spanned_path};
use proc_macro2::Span;
use syn::Lit;

use crate::error::{SemanticError, closest, locate, suggest};
use crate::registry::Registry;
use crate::rust_types::{EnumDef, Fields, RustType, RustTypes, StructDef};

/// Lowers a parsed page to IR.
///
/// `file` is the path locations are recorded against. With `types`, every
/// state path and action is also checked against the page's state struct
/// and action enum; without them — in the macro — that check is rustc's.
pub fn lower(
    page: &PageAst,
    registry: &Registry,
    types: Option<&RustTypes>,
    file: &str,
) -> Result<Page, Vec<SemanticError>> {
    let action_type = last_ident(&page.action_ty);
    let state_type = last_ident(&page.state_ty);
    let mut cx = Cx { registry, scopes: Vec::new(), file, errors: Vec::new(), types, action_enum: None, state_struct: None };

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
        match types.structs.get(&state_type) {
            Some(def) => cx.state_struct = Some(def),
            None => cx.errors.push(
                SemanticError::new(
                    spanned_path(&page.state_ty),
                    format!("cannot find struct `{state_type}` in this crate"),
                )
                .help("the page's state type must be a struct with named fields the CLI can read from `--src`"),
            ),
        }
    }

    let root = cx.node(&page.root);
    if !cx.errors.is_empty() {
        return Err(cx.errors);
    }
    Ok(Page {
        name: page.name.to_string(),
        state_type,
        action_type,
        root: root.expect("a node with no errors lowers"),
        source: cx.loc(page.name.span()),
    })
}

fn last_ident(path: &syn::Path) -> String {
    path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
}

/// A `for` item in scope, and the struct its elements are.
struct Scope<'a> {
    name: String,
    element: Option<&'a StructDef>,
}

struct Cx<'a> {
    registry: &'a Registry,
    scopes: Vec<Scope<'a>>,
    file: &'a str,
    errors: Vec<SemanticError>,
    types: Option<&'a RustTypes>,
    action_enum: Option<&'a EnumDef>,
    state_struct: Option<&'a StructDef>,
}

/// One way a component was given something: `name: value` in parentheses
/// or the body, a positional argument, or `.name(value)`.
struct Entry<'v> {
    name: String,
    span: Span,
    value: Option<&'v ValueAst>,
    modifier: bool,
}

impl Cx<'_> {
    fn loc(&self, span: Span) -> SourceLocation {
        locate(self.file, span).0
    }

    fn node(&mut self, node: &NodeAst) -> Option<UiNode> {
        match node {
            NodeAst::Component(c) => self.component(c).map(UiNode::Component),
            NodeAst::If(i) => self.if_node(i),
            NodeAst::For(f) => self.for_node(f),
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
            self.nodes(&ast.children);
            return None;
        };
        let errors_before = self.errors.len();
        let entries = self.entries(ast, spec);

        let mut seen: HashMap<String, Span> = HashMap::new();
        // Props a misspelling was taken for: already reported, so not
        // reported again as missing.
        let mut meant: Vec<String> = Vec::new();
        let mut props = Vec::new();
        let mut bound: Vec<(&PropSpec, StatePath, Span)> = Vec::new();
        let mut events: Vec<UiEvent> = Vec::new();
        let mut slots: Vec<UiSlot> = Vec::new();
        // Slots written, whether or not their node lowered: one that did
        // not is already reported, so not reported again as missing.
        let mut given_slots: Vec<String> = Vec::new();
        for entry in entries {
            let kind = if entry.modifier { "modifier" } else { "property" };
            if let Some(event) = spec.event_by_modifier(&entry.name) {
                if events.iter().any(|e| e.name == event.name) {
                    self.errors.push(
                        SemanticError::new(entry.span, format!("`{}` is given twice", entry.name))
                            .label("given again here"),
                    );
                } else if let Some(action) = self.event_action(&entry, event) {
                    events.push(UiEvent { name: event.name.clone(), action, source: self.loc(entry.span) });
                }
                continue;
            }
            if let (false, Some(slot)) = (entry.modifier, spec.slot(&entry.name)) {
                if given_slots.contains(&slot.name) {
                    self.errors.push(
                        SemanticError::new(entry.span, format!("slot `{}` is given twice", slot.name))
                            .label("given again here"),
                    );
                } else if let Some(ValueAst::Component(node)) = entry.value {
                    given_slots.push(slot.name.clone());
                    if let Some(node) = self.component(node) {
                        slots.push(UiSlot { name: slot.name.clone(), node: UiNode::Component(node), source: self.loc(entry.span) });
                    }
                } else {
                    self.errors.push(
                        SemanticError::new(entry.span, format!("slot `{}` on `{name}` takes a component", slot.name))
                            .help(format!("`{}: Component {{ … }}`", slot.name)),
                    );
                }
                continue;
            }
            let prop_spec = if entry.modifier { spec.modifier_prop(&entry.name) } else { spec.prop(&entry.name) };
            let Some(prop_spec) = prop_spec else {
                let candidates: Vec<&str> = if entry.modifier {
                    spec.props.iter().filter(|p| p.modifier).map(|p| p.name.as_str()).collect()
                } else {
                    spec.props.iter().map(|p| p.name.as_str()).collect()
                };
                let events = spec.events.iter().map(|e| e.modifier.as_str());
                let slot_names = spec.slots.iter().filter(|_| !entry.modifier).map(|s| s.name.as_str());
                let closest = closest(&entry.name, candidates.iter().copied().chain(events).chain(slot_names));
                meant.extend(closest.map(str::to_owned));
                self.errors.push(
                    SemanticError::new(
                        entry.span,
                        format!("unknown {kind} `{}` on component `{name}`", entry.name),
                    )
                    .help_opt(closest.map(|c| format!("did you mean `{c}`?"))),
                );
                continue;
            };
            if seen.insert(prop_spec.name.clone(), entry.span).is_some() {
                self.errors.push(
                    SemanticError::new(entry.span, format!("property `{}` is given twice", prop_spec.name))
                        .label("given again here"),
                );
                continue;
            }
            let Some(value) = entry.value else {
                self.errors.push(SemanticError::new(
                    entry.span,
                    format!("`.{}(…)` takes one value", entry.name),
                ));
                continue;
            };
            if let ValueAst::Bind(path) = value {
                if !prop_spec.bindable {
                    self.errors.push(
                        SemanticError::new(path.span(), format!("property `{}` on `{name}` cannot be bound", prop_spec.name))
                            .help(format!("pass the value itself: `{}: state.…`", prop_spec.name)),
                    );
                } else if let Some(path) = self.state_path(path, &prop_spec.ty, &prop_spec.name, &name) {
                    bound.push((prop_spec, path, entry.span));
                }
                continue;
            }
            if let Some(expr) = self.expr(value, &prop_spec.ty, &prop_spec.name, &name) {
                props.push(UiProp { name: prop_spec.name.clone(), value: expr, source: self.loc(entry.span) });
            }
        }

        // A binding writes back through the event that carries its new
        // value, which then belongs to the binding rather than standing on
        // its own.
        let mut bindings = Vec::new();
        for (prop_spec, path, span) in bound {
            let event = spec.events.iter().find(|e| e.binds.as_deref() == Some(&prop_spec.name));
            let position = event.and_then(|event| events.iter().position(|e| e.name == event.name));
            match (event, position) {
                (_, Some(i)) => {
                    let on_change = events.remove(i).action;
                    bindings.push(UiBinding { name: prop_spec.name.clone(), path, on_change, source: self.loc(span) });
                }
                (Some(event), None) => self.errors.push(
                    SemanticError::new(span, format!("`bind` on `{}` needs an action to write the change back", prop_spec.name))
                        .label("nothing sends the new value to Rust")
                        .help(format!("add `{}: Action::…`", event.modifier)),
                ),
                (None, None) => unreachable!("bindable prop `{}` has no event to write back", prop_spec.name),
            }
        }

        // A default is resolved here rather than by each backend, so the IR
        // says what is drawn and no platform's own default leaks in.
        for prop_spec in &spec.props {
            if let (false, Some(default)) = (seen.contains_key(&prop_spec.name), &prop_spec.default) {
                let value = match (&prop_spec.ty, default) {
                    (PropType::Enum(e), Value::String(case)) => Expr::EnumCase { ty: e.name.clone(), case: case.clone() },
                    _ => Expr::Literal(default.clone()),
                };
                props.push(UiProp { name: prop_spec.name.clone(), value, source: self.loc(ast.name.span()) });
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
                        format!("missing required property `{}` on component `{name}`", prop_spec.name),
                    )
                    .help(how),
                );
            }
        }

        for slot in spec.slots.iter().filter(|s| s.required) {
            if !given_slots.contains(&slot.name) && !meant.contains(&slot.name) {
                self.errors.push(
                    SemanticError::new(ast.name.span(), format!("missing required slot `{}` on component `{name}`", slot.name))
                        .help(format!("add `{}: Component {{ … }}`", slot.name)),
                );
            }
        }

        let mut children = Vec::new();
        match &spec.children {
            ChildrenSpec::None if !ast.children.is_empty() => self.errors.push(
                SemanticError::new(ast.children[0].span(), format!("`{name}` takes no children"))
                    .label("child given here"),
            ),
            ChildrenSpec::None => {}
            ChildrenSpec::Many { allow } => {
                if let Some(allow) = allow {
                    self.check_allowed(&ast.children, allow, &name);
                }
                children = self.nodes(&ast.children);
            }
        }

        if self.errors.len() > errors_before {
            return None;
        }
        Some(Component { id: ComponentId(name), props, bindings, events, slots, children, source: self.loc(ast.name.span()) })
    }

    /// Only `allow`ed components as children, directly or in an `if`; a
    /// `for` cannot promise what it repeats is one.
    fn check_allowed(&mut self, children: &[NodeAst], allow: &[String], parent: &str) {
        let list = allow.iter().map(|a| format!("`{a}`")).collect::<Vec<_>>().join(", ");
        for child in children {
            match child {
                NodeAst::Component(c) if allow.iter().any(|a| c.name == a) => {}
                // Reported as unknown already.
                NodeAst::Component(c) if self.registry.get(&c.name.to_string()).is_none() => {}
                NodeAst::If(i) => {
                    self.check_allowed(&i.then_branch, allow, parent);
                    self.check_allowed(&i.else_branch, allow, parent);
                }
                other => self.errors.push(
                    SemanticError::new(other.span(), format!("`{parent}` takes only {list} as children"))
                        .label("not allowed here"),
                ),
            }
        }
    }

    /// Every way something was given, in source order: positionally, named
    /// in the parentheses, named in the body, then modifiers.
    fn entries<'v>(&mut self, ast: &'v ComponentAst, spec: &ComponentSpec) -> Vec<Entry<'v>> {
        let mut positional = Vec::new();
        let mut named = Vec::new();
        let mut positional_index = 0;
        for arg in &ast.args {
            match arg {
                ArgAst::Named(prop) => named.push(Entry {
                    name: prop.name.to_string(),
                    span: prop.name.span(),
                    value: Some(&prop.value),
                    modifier: false,
                }),
                ArgAst::Positional(value) => {
                    match spec.positional.get(positional_index) {
                        Some(prop) => positional.push(Entry {
                            name: prop.clone(),
                            span: value.span(),
                            value: Some(value),
                            modifier: false,
                        }),
                        None => self.errors.push(
                            SemanticError::new(
                                value.span(),
                                format!(
                                    "`{}` takes {} positional argument{}",
                                    spec.name,
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
        named.extend(ast.props.iter().map(|prop| Entry {
            name: prop.name.to_string(),
            span: prop.name.span(),
            value: Some(&prop.value),
            modifier: false,
        }));
        let modifiers = ast.modifiers.iter().map(|m| Entry {
            name: m.name.to_string(),
            span: m.name.span(),
            value: match m.args.as_slice() {
                [ArgAst::Positional(value)] => Some(value),
                _ => None,
            },
            modifier: true,
        });
        positional.into_iter().chain(named).chain(modifiers).collect()
    }

    /// A read-only value for a prop of type `ty`.
    fn expr(&mut self, value: &ValueAst, ty: &PropType, prop: &str, component: &str) -> Option<Expr> {
        let found = match value {
            ValueAst::Lit(Lit::Str(s)) if *ty == PropType::String => {
                return Some(Expr::Literal(Value::String(s.value())));
            }
            ValueAst::Lit(Lit::Bool(b)) if *ty == PropType::Bool => {
                return Some(Expr::Literal(Value::Bool(b.value)));
            }
            ValueAst::Lit(Lit::Int(_)) | ValueAst::NegInt(Lit::Int(_)) if *ty == PropType::Int => {
                return self.int(value).map(|n| Expr::Literal(Value::Int(n)));
            }
            ValueAst::Path(path) if !is_action(path) => {
                if let PropType::Enum(spec) = ty {
                    return self.enum_case(path, spec);
                }
                "an enum case"
            }
            ValueAst::Dotted(path) => {
                if matches!(ty, PropType::Int | PropType::Enum(_)) {
                    self.errors.push(SemanticError::new(
                        path.span(),
                        format!("property `{prop}` on `{component}` takes a literal for now, not a state value"),
                    ));
                    return None;
                }
                return self.path_value(path, ty, prop, component);
            }
            ValueAst::NegInt(_) => "an integer",
            ValueAst::Lit(Lit::Str(_)) => "a string",
            ValueAst::Lit(Lit::Bool(_)) => "a bool",
            ValueAst::Lit(Lit::Int(_)) => "an integer",
            ValueAst::Lit(_) => "an unsupported literal",
            ValueAst::Path(_) | ValueAst::Call(..) => "an action",
            ValueAst::Bind(_) => "a binding",
            ValueAst::Component(_) => "a component",
        };
        let mut error = SemanticError::new(
            value.span(),
            format!("property `{prop}` on `{component}` expects {}, found {found}", ty.name()),
        )
        .label(format!("expected {}", ty.name()));
        if found == "an action" {
            error = error.help("actions are wired with an event, such as `.on_click(…)`");
        }
        self.errors.push(error);
        None
    }

    fn int(&mut self, value: &ValueAst) -> Option<i64> {
        let parsed = match value {
            ValueAst::Lit(Lit::Int(i)) => i.base10_parse::<i64>().map_err(|e| e.to_string()),
            ValueAst::NegInt(Lit::Int(i)) => format!("-{}", i.base10_digits()).parse::<i64>().map_err(|e| e.to_string()),
            _ => unreachable!("not an integer literal"),
        };
        parsed.map_err(|e| self.errors.push(SemanticError::new(value.span(), e))).ok()
    }

    /// `ButtonStyle::Primary` for a prop of that enum.
    fn enum_case(&mut self, path: &syn::Path, spec: &crate::ir::EnumSpec) -> Option<Expr> {
        let segments: Vec<_> = path.segments.iter().map(|s| &s.ident).collect();
        let [ty, case] = segments.as_slice() else {
            self.errors.push(SemanticError::new(
                spanned_path(path),
                format!("an enum case is written `{}::Case`", spec.name),
            ));
            return None;
        };
        if *ty != &spec.name {
            self.errors.push(
                SemanticError::new(ty.span(), format!("expected a `{}` case, found `{ty}::{case}`", spec.name))
                    .label(format!("expected {}", spec.name)),
            );
            return None;
        }
        let case_name = case.to_string();
        if !spec.cases.contains(&case_name) {
            self.errors.push(
                SemanticError::new(case.span(), format!("no case `{case_name}` on `{}`", spec.name))
                    .help_opt(suggest(&case_name, spec.cases.iter().map(String::as_str)))
                    .label(format!("cases are {}", spec.cases.join(", "))),
            );
            return None;
        }
        Some(Expr::EnumCase { ty: spec.name.clone(), case: case_name })
    }

    /// `state.a.b` or `item.a.b`, resolved against the state struct or the
    /// loop's element when they are at hand: the value, and the Rust type
    /// it reads.
    fn resolve(&mut self, path: &DottedAst) -> Option<(Expr, Option<RustType>)> {
        let (head, rest) = path.segments.split_first().expect("a dotted path has a segment");
        let head_name = head.to_string();
        let (start, local) = if head == "state" {
            if let Some(scope) = self.scopes.last() {
                self.errors.push(
                    SemanticError::new(head.span(), "the state is not read inside a `for`")
                        .label(format!("read `{}` here", scope.name))
                        .help("derive what each element needs with a #[selector] that puts it on the element"),
                );
                return None;
            }
            (self.state_struct, None)
        } else if let Some(scope) = self.scopes.iter().rev().find(|s| s.name == head_name) {
            (scope.element, Some(head_name.clone()))
        } else {
            let names: Vec<&str> = self.scopes.iter().map(|s| s.name.as_str()).chain(["state"]).collect();
            self.errors.push(
                SemanticError::new(head.span(), format!("cannot find value `{head}`"))
                    .label("not the state or a loop item")
                    .help_opt(suggest(&head_name, names.iter().copied()).or_else(|| {
                        Some("a value path starts with `state` or a `for` item: `state.field`".into())
                    })),
            );
            return None;
        };
        if rest.is_empty() {
            self.errors.push(
                SemanticError::new(head.span(), format!("the whole `{head}` is not a value"))
                    .help(format!("name a field: `{head}.field`")),
            );
            return None;
        }
        let segments: Vec<String> = rest.iter().map(ToString::to_string).collect();
        let expr = match local {
            Some(local) => Expr::Local(LocalPath { local, segments }),
            None => Expr::State(StatePath { segments }),
        };
        let Some(types) = self.types else {
            return Some((expr, None));
        };
        // Already reported: the state type, or the loop's element, was not found.
        let mut current = start?;
        let mut field_ty = None;
        for (i, ident) in rest.iter().enumerate() {
            let name = ident.to_string();
            let Some(found) = current.field(&name) else {
                self.errors.push(
                    SemanticError::new(ident.span(), format!("no field `{name}` on type `{}`", current.name))
                        .help_opt(suggest(&name, current.names())),
                );
                return None;
            };
            if i + 1 < rest.len() {
                let nested = match found {
                    RustType::Other(n) => types.structs.get(n),
                    _ => None,
                };
                let Some(nested) = nested else {
                    self.errors.push(SemanticError::new(
                        ident.span(),
                        format!("`{name}` is a `{}`, which has no fields", found.rust()),
                    ));
                    return None;
                };
                current = nested;
            }
            field_ty = Some(found.clone());
        }
        Some((expr, field_ty))
    }

    /// A path read as a prop of type `ty`: a state field or a loop item's.
    fn path_value(&mut self, path: &DottedAst, ty: &PropType, prop: &str, component: &str) -> Option<Expr> {
        let (expr, field_ty) = self.resolve(path)?;
        if let Some(field_ty) = field_ty
            && !fits(ty, &field_ty)
        {
            let written: Vec<String> = path.segments.iter().map(ToString::to_string).collect();
            self.errors.push(
                SemanticError::new(
                    path.span(),
                    format!(
                        "property `{prop}` on `{component}` expects {}, but `{}` is a `{}`",
                        ty.name(),
                        written.join("."),
                        field_ty.rust()
                    ),
                )
                .label(format!("expected {}", ty.name())),
            );
            return None;
        }
        Some(expr)
    }

    /// `bind(state.x)`: only the state can be written back to.
    fn state_path(&mut self, path: &DottedAst, ty: &PropType, prop: &str, component: &str) -> Option<StatePath> {
        if path.segments[0] != "state" && self.scopes.iter().any(|s| path.segments[0] == s.name) {
            self.errors.push(
                SemanticError::new(path.span(), "only a state field can be bound")
                    .help("a loop item is a copy of the state's; change it with an action"),
            );
            return None;
        }
        match self.path_value(path, ty, prop, component)? {
            Expr::State(path) => Some(path),
            _ => None,
        }
    }

    fn nodes(&mut self, nodes: &[NodeAst]) -> Vec<UiNode> {
        nodes.iter().filter_map(|n| self.node(n)).collect()
    }

    fn if_node(&mut self, ast: &IfAst) -> Option<UiNode> {
        let value = self.path_value(&ast.condition, &PropType::Bool, "condition", "if");
        let then_branch = self.nodes(&ast.then_branch);
        let else_branch = self.nodes(&ast.else_branch);
        Some(UiNode::If {
            condition: Condition { negated: ast.negated, value: value? },
            then_branch,
            else_branch,
            source: self.loc(ast.keyword),
        })
    }

    fn for_node(&mut self, ast: &ForAst) -> Option<UiNode> {
        let item = ast.item.to_string();
        if !self.scopes.is_empty() {
            self.errors.push(
                SemanticError::new(ast.keyword, "a `for` inside a `for` is not supported yet")
                    .help("flatten the elements in a #[selector]"),
            );
            return None;
        }
        if item == "state" {
            self.errors.push(SemanticError::new(ast.item.span(), "a loop item cannot be called `state`"));
            return None;
        }
        let (collection, collection_ty) = self.resolve(&ast.collection)?;
        let Expr::State(collection) = collection else { unreachable!("outside a loop, a path is the state's") };

        let element = match (&collection_ty, self.types) {
            (None, _) => None,
            (Some(RustType::Vec(elem)), Some(types)) => match &**elem {
                RustType::Other(name) if types.structs.contains_key(name) => Some(&types.structs[name]),
                other => {
                    self.errors.push(
                        SemanticError::new(
                            ast.collection.span(),
                            format!("a `for` runs over a Vec of structs; this is a `Vec<{}>`", other.rust()),
                        )
                        .help("wrap each value in a struct with an `id`"),
                    );
                    return None;
                }
            },
            (Some(other), _) => {
                self.errors.push(
                    SemanticError::new(
                        ast.collection.span(),
                        format!("`state.{}` is a `{}`, not a Vec", collection.dotted(), other.rust()),
                    )
                    .label("expected a Vec"),
                );
                return None;
            }
        };

        self.scopes.push(Scope { name: item.clone(), element });
        let key = self.key(ast, element);
        let children = self.nodes(&ast.children);
        self.scopes.pop();
        Some(UiNode::ForEach {
            collection,
            item,
            element: element.map(|e| e.name.clone()).unwrap_or_default(),
            key: key?,
            children,
            source: self.loc(ast.keyword),
        })
    }

    /// The field that tells elements apart: `key: item.field`, or `id`.
    fn key(&mut self, ast: &ForAst, element: Option<&StructDef>) -> Option<LocalPath> {
        let item = ast.item.to_string();
        let Some(key) = &ast.key else {
            if let Some(element) = element
                && element.field("id").is_none()
            {
                self.errors.push(
                    SemanticError::new(ast.item.span(), format!("`{}` has no `id` to tell its elements apart", element.name))
                        .help(format!("name the field that does: `for {item} in …, key: {item}.field`")),
                );
                return None;
            }
            return Some(LocalPath { local: item, segments: vec!["id".into()] });
        };
        if key.segments[0] != item {
            self.errors.push(
                SemanticError::new(key.span(), format!("a key is a field of the loop item: `{item}.field`")),
            );
            return None;
        }
        match self.resolve(key)? {
            (Expr::Local(path), _) => Some(path),
            _ => None,
        }
    }

    /// The action an event entry sends: `.on_click(Action::Save)`,
    /// `on_change: Action::SetOn`, `.on_click(Action::Pick(state.id))`.
    fn event_action(&mut self, entry: &Entry, event: &EventSpec) -> Option<ActionRef> {
        let shape = || {
            SemanticError::new(
                entry.span,
                format!("`{}` takes one action: `{}(Action::Variant)`", entry.name, entry.name),
            )
        };
        match entry.value {
            Some(ValueAst::Path(path)) if is_action(path) => self.action(path, None, event),
            Some(ValueAst::Call(path, payload)) if is_action(path) => self.action(path, Some(payload), event),
            _ => {
                self.errors.push(shape());
                None
            }
        }
    }

    fn action(&mut self, path: &syn::Path, payload: Option<&ValueAst>, event: &EventSpec) -> Option<ActionRef> {
        let segments: Vec<_> = path.segments.iter().collect();
        if segments.len() != 2 {
            self.errors.push(SemanticError::new(spanned_path(path), "an action is written `Action::Variant`"));
            return None;
        }
        let variant_ident = &segments[1].ident;
        let variant = variant_ident.to_string();
        let Some(def) = self.action_enum else {
            // In the macro, rustc checks the variant and its payload.
            let payload = match payload {
                Some(value) => ActionPayload::Expr(self.payload_expr(value, None)?),
                None if event.payload.is_some() => ActionPayload::EventValue,
                None => ActionPayload::None,
            };
            return Some(ActionRef { variant, payload });
        };
        let Some(v) = def.variant(&variant) else {
            let candidates = def.variants.iter().map(|v| v.name.as_str());
            self.errors.push(
                SemanticError::new(variant_ident.span(), format!("no variant `{variant}` on action enum `{}`", def.name))
                    .help_opt(suggest(&variant, candidates)),
            );
            return None;
        };
        let field = match &v.fields {
            Fields::Unit => None,
            Fields::Tuple(types) if types.len() == 1 => Some(&types[0]),
            _ => {
                self.errors.push(
                    SemanticError::new(
                        variant_ident.span(),
                        format!("`{}::{variant}` carries several values; an action carries one or none", def.name),
                    )
                    .help("give the variant one tuple field, or none"),
                );
                return None;
            }
        };
        let payload = match (payload, field, &event.payload) {
            (None, None, None) => ActionPayload::None,
            // The value an event carries is for the action to take; an
            // action that drops it would hide a change the event reports.
            (None, None, Some(carried)) => {
                self.errors.push(
                    SemanticError::new(
                        variant_ident.span(),
                        format!("`{}::{variant}` carries no value, but `{}` gives it {}", def.name, event.name, carried.name()),
                    )
                    .label("expects no payload"),
                );
                return None;
            }
            (Some(value), Some(field), _) => ActionPayload::Expr(self.payload_expr(value, Some(field))?),
            (Some(value), None, _) => {
                self.errors.push(
                    SemanticError::new(value.span(), format!("`{}::{variant}` carries no value", def.name))
                        .help(format!("write `Action::{variant}`")),
                );
                return None;
            }
            (None, Some(field), Some(carried)) => {
                let fits = matches!(
                    (carried, field),
                    (PropType::Bool, RustType::Bool)
                        | (PropType::String, RustType::String)
                        | (PropType::Int, RustType::Int(_))
                );
                if !fits {
                    self.errors.push(
                        SemanticError::new(
                            variant_ident.span(),
                            format!(
                                "`{}::{variant}` carries a `{}`, but `{}` gives it {}",
                                def.name,
                                field.rust(),
                                event.name,
                                carried.name()
                            ),
                        )
                        .label("payload type differs"),
                    );
                    return None;
                }
                ActionPayload::EventValue
            }
            (None, Some(field), None) => {
                self.errors.push(
                    SemanticError::new(
                        variant_ident.span(),
                        format!("`{}::{variant}` carries a value, but `{}` has none to give it", def.name, event.name),
                    )
                    .label("expects a payload")
                    .help(format!("pass one: `Action::{variant}(…)`, a `{}`", field.rust())),
                );
                return None;
            }
        };
        Some(ActionRef { variant, payload })
    }

    /// The value an action is given, `Action::Pick(item.id)`: a literal, or
    /// a path of exactly the variant's field type.
    fn payload_expr(&mut self, value: &ValueAst, field: Option<&RustType>) -> Option<Expr> {
        let ty = match field {
            Some(RustType::Bool) => Some(PropType::Bool),
            Some(RustType::String) => Some(PropType::String),
            Some(RustType::Int(_)) => Some(PropType::Int),
            Some(other) => {
                self.errors.push(SemanticError::new(
                    value.span(),
                    format!("an action payload of type `{}` cannot cross to the native side yet", other.rust()),
                ));
                return None;
            }
            None => None,
        };
        match (value, ty) {
            (ValueAst::Dotted(path), _) => {
                let (expr, found) = self.resolve(path)?;
                // A field of the right kind but another width — `u8` for a
                // `u32` — would not convert on every side.
                if let (Some(field), Some(found)) = (field, found)
                    && *field != found
                {
                    let written: Vec<String> = path.segments.iter().map(ToString::to_string).collect();
                    self.errors.push(SemanticError::new(
                        value.span(),
                        format!("the action carries a `{}`, but `{}` is a `{}`", field.rust(), written.join("."), found.rust()),
                    ));
                    return None;
                }
                Some(expr)
            }
            (ValueAst::Lit(Lit::Str(s)), None | Some(PropType::String)) => Some(Expr::Literal(Value::String(s.value()))),
            (ValueAst::Lit(Lit::Bool(b)), None | Some(PropType::Bool)) => Some(Expr::Literal(Value::Bool(b.value))),
            (ValueAst::Lit(Lit::Int(_)) | ValueAst::NegInt(Lit::Int(_)), None | Some(PropType::Int)) => {
                self.int(value).map(|n| Expr::Literal(Value::Int(n)))
            }
            (_, ty) => {
                let expected = ty.map(|t| t.name()).unwrap_or_else(|| "a literal or a path".into());
                self.errors.push(SemanticError::new(
                    value.span(),
                    format!("an action payload is a literal or a path; expected {expected}"),
                ));
                None
            }
        }
    }
}

fn fits(ty: &PropType, field: &RustType) -> bool {
    matches!(
        (ty, field),
        (PropType::Bool, RustType::Bool) | (PropType::String, RustType::String) | (PropType::Int, RustType::Int(_))
    )
}

fn is_action(path: &syn::Path) -> bool {
    path.segments.first().is_some_and(|s| s.ident == "Action")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_types::RustTypes;

    fn types() -> RustTypes {
        let file: syn::File = syn::parse_str(
            "enum A { Save, SetOn(bool), Rename(String), Pick(u32) }
             struct S { on: bool, name: String, count: u32, small: u8, user: User, items: Vec<Item>, tags: Vec<String> }
             struct User { locked: bool }
             struct Item { id: u32, title: String, done: bool }
             struct Row { uuid: String }
             impl S { #[selector] fn open(&self) -> Vec<Item> { todo!() } #[selector] fn rows(&self) -> Vec<Row> { todo!() } }",
        )
        .unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items, "crate");
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
        let UiNode::Component(stack) = &page.root else { panic!() };
        assert_eq!(stack.prop("spacing"), Some(&Expr::Literal(Value::Int(0))));
        let UiNode::Component(button) = &stack.children[0] else { panic!() };
        assert_eq!(button.event("click").unwrap().variant, "Save");
        assert_eq!(button.prop("style"), Some(&Expr::EnumCase { ty: "ButtonStyle".into(), case: "Secondary".into() }));
        assert_eq!(button.prop("disabled"), Some(&Expr::Literal(Value::Bool(false))));
    }

    #[test]
    fn lowers_state_bindings_and_modifiers() {
        let page = lower_str(
            r#"page P(state: S, action: A) {
                VStack {
                    Text(state.name).style(TextStyle::Title)
                    Toggle { value: bind(state.on), on_change: Action::SetOn }.disabled(state.user.locked)
                    Button("Pick").style(ButtonStyle::Primary).on_click(Action::Pick(state.count))
                }
            }"#,
        )
        .unwrap();
        let UiNode::Component(stack) = &page.root else { panic!() };
        let [UiNode::Component(text), UiNode::Component(toggle), UiNode::Component(button)] = stack.children.as_slice()
        else {
            panic!("three children");
        };
        assert_eq!(text.prop("text"), Some(&Expr::State(StatePath { segments: vec!["name".into()] })));
        assert_eq!(text.prop("style"), Some(&Expr::EnumCase { ty: "TextStyle".into(), case: "Title".into() }));
        let binding = toggle.binding("value").unwrap();
        assert_eq!(binding.path.segments, ["on"]);
        assert_eq!(binding.on_change, ActionRef { variant: "SetOn".into(), payload: ActionPayload::EventValue });
        assert!(toggle.events.is_empty());
        assert_eq!(
            toggle.prop("disabled"),
            Some(&Expr::State(StatePath { segments: vec!["user".into(), "locked".into()] }))
        );
        assert_eq!(
            button.event("click").unwrap().payload,
            ActionPayload::Expr(Expr::State(StatePath { segments: vec!["count".into()] }))
        );
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
                "unknown property `titel` on component `Button`",
                "unknown modifier `on_tap` on component `Button`",
            ]
        );
    }

    #[test]
    fn checks_actions_against_the_enum() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack {
                Button("a").on_click(Action::Sav)
                Button("b").on_click(Action::SetOn)
                Button("c").on_click(Action::Save(true))
                Button("d").on_click(Action::Pick(state.small))
                Toggle { value: state.on, on_change: Action::Rename }
            } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no variant `Sav` on action enum `A`",
                "`A::SetOn` carries a value, but `click` has none to give it",
                "`A::Save` carries no value",
                "the action carries a `u32`, but `state.small` is a `u8`",
                "`A::Rename` carries a `String`, but `change` gives it Bool",
            ]
        );
    }

    #[test]
    fn checks_state_paths() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack {
                Text(state.nme)
                Text(state.on)
                Text(stat.name)
                Button("x").disabled(state.user.lockd)
                Toggle { value: bind(state.on) }
                Text("t") { Text(state.name) }
            } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no field `nme` on type `S`",
                "property `text` on `Text` expects String, but `state.on` is a `bool`",
                "cannot find value `stat`",
                "no field `lockd` on type `User`",
                "`bind` on `value` needs an action to write the change back",
                "`Text` takes no children",
            ]
        );
    }

    #[test]
    fn lowers_if_and_for() {
        let page = lower_str(
            r#"page P(state: S, action: A) { VStack {
                if !state.on { Text("off") } else { Text("on") }
                for item in state.open {
                    if item.done { Text(item.title) }
                    Button("Pick").on_click(Action::Pick(item.id))
                }
                for row in state.rows, key: row.uuid { Text(row.uuid) }
            } }"#,
        )
        .unwrap();
        let UiNode::Component(stack) = &page.root else { panic!() };
        let UiNode::If { condition, else_branch, .. } = &stack.children[0] else { panic!() };
        assert!(condition.negated);
        assert_eq!(condition.value, Expr::State(StatePath { segments: vec!["on".into()] }));
        assert_eq!(else_branch.len(), 1);
        let UiNode::ForEach { collection, element, key, children, .. } = &stack.children[1] else { panic!() };
        assert_eq!((collection.dotted().as_str(), element.as_str()), ("open", "Item"));
        assert_eq!(key, &LocalPath { local: "item".into(), segments: vec!["id".into()] });
        let UiNode::If { condition, .. } = &children[0] else { panic!() };
        assert_eq!(condition.value, Expr::Local(LocalPath { local: "item".into(), segments: vec!["done".into()] }));
        let UiNode::ForEach { key, .. } = &stack.children[2] else { panic!() };
        assert_eq!(key.segments, ["uuid"]);
    }

    #[test]
    fn checks_if_and_for() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack {
                if state.name { Text("a") }
                for item in state.count { Text("b") }
                for tag in state.tags { Text("c") }
                for row in state.rows { Text("d") }
                for item in state.items {
                    Text(state.name)
                    Text(item.titel)
                    Text(iten.title)
                    for other in state.items { Text("e") }
                }
                Text(item.title)
            } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "property `condition` on `if` expects Bool, but `state.name` is a `String`",
                "`state.count` is a `u32`, not a Vec",
                "a `for` runs over a Vec of structs; this is a `Vec<String>`",
                "`Row` has no `id` to tell its elements apart",
                "the state is not read inside a `for`",
                "no field `titel` on type `Item`",
                "cannot find value `iten`",
                "a `for` inside a `for` is not supported yet",
                "cannot find value `item`",
            ]
        );
    }

    const ROWS: &str = r#"
        [component.Section]
        children = { allow = ["Row"] }
        swiftui = { call = "Section", trailing = { children = true } }
        winui = { element = "Section" }

        [component.Row]
        positional = ["title"]
        props.title = { type = "string", required = true }
        slots.trailing = {}
        slots.icon = { required = true }
        swiftui = { call = "Row", args = [{ arg = "title", prop = "title" }, { arg = "trailing", slot = "trailing" }, { arg = "icon", slot = "icon" }] }
        winui = { element = "Row", slots.trailing = { property = "Control" }, slots.icon = { property = "Icon" } }
    "#;

    fn lower_with_rows(src: &str) -> Result<Page, Vec<String>> {
        let registry = Registry::from_tables(&[(crate::registry::BUILTIN, "registry.toml"), (ROWS, "app.toml")]).unwrap();
        let ast: PageAst = syn::parse_str(src).unwrap();
        lower(&ast, &registry, Some(&types()), "page.rs").map_err(|errors| errors.into_iter().map(|e| e.message).collect())
    }

    #[test]
    fn lowers_slots() {
        let page = lower_with_rows(
            r#"page P(state: S, action: A) { Section {
                Row("On", icon: Text("i"), trailing: Toggle { value: bind(state.on), on_change: Action::SetOn })
                if state.on { Row { title: "Name", icon: Text(state.name) } }
            } }"#,
        )
        .unwrap();
        let UiNode::Component(section) = &page.root else { panic!() };
        let UiNode::Component(row) = &section.children[0] else { panic!() };
        let Some(UiNode::Component(toggle)) = row.slot("trailing") else { panic!() };
        assert_eq!(toggle.binding("value").unwrap().on_change.variant, "SetOn");
        assert!(row.slot("icon").is_some());
    }

    #[test]
    fn checks_slots_and_allowed_children() {
        let errors = lower_with_rows(
            r#"page P(state: S, action: A) { Section {
                Row("a", icon: Text("i"), trailing: "x")
                Row("b", icon: Text("i"), icon: Text("j"))
                Row("c")
                Row("d", icon: Text(state.nme))
                Text("e")
                for item in state.items { Row(item.title, icon: Text("i")) }
            } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "`Section` takes only `Row` as children",
                "`Section` takes only `Row` as children",
                "slot `trailing` on `Row` takes a component",
                "slot `icon` is given twice",
                "missing required slot `icon` on component `Row`",
                "no field `nme` on type `S`",
            ]
        );
    }

    #[test]
    fn checks_types_and_duplicates() {
        let errors = lower_str(
            r#"page P(state: S, action: A) { VStack(spacing: "x") {
                Button("a", title: "b")
                Button("c").style(TextStyle::Title)
                Button("d").style(ButtonStyle::Primry)
                Text("e", style: TextStyle::Body).style(TextStyle::Title)
                Text("f").disabled(true)
                Button { title: bind(state.name) }
            } }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "property `spacing` on `VStack` expects Integer, found a string",
                "property `title` is given twice",
                "expected a `ButtonStyle` case, found `TextStyle::Title`",
                "no case `Primry` on `ButtonStyle`",
                "property `style` is given twice",
                "unknown modifier `disabled` on component `Text`",
                "property `title` on `Button` cannot be bound",
            ]
        );
    }
}
