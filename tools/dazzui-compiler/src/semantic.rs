use std::collections::HashMap;

use crate::ir::{
    Argument, ChildrenSpec, Component, ComponentId, ComponentSpec, Condition, EventSpec, Expr,
    LocalPath, MethodCall, Page, PropType, SPACING_STEPS, SourceLocation, StatePath, UiEvent,
    UiNode, UiProp, UiSlot, Value,
};
use crate::syntax::{
    ChildrenArg, ComponentAst, DottedAst, ForAst, IfAst, NodeAst, PageAst, PropAst, ValueAst,
    spanned_path,
};
use proc_macro2::Span;
use syn::Lit;

use crate::error::{SemanticError, closest, locate, suggest};
use crate::registry::Registry;
use crate::rust_types::{ModelDef, RustType, RustTypes, StructDef};

/// Lowers a parsed page to IR.
///
/// `file` is the path locations are recorded against. With `types`, every
/// state path and model method is also checked against the page's state
/// struct and model; without them — in the macro — that check is rustc's.
pub fn lower(
    page: &PageAst,
    registry: &Registry,
    types: Option<&RustTypes>,
    file: &str,
) -> Result<Page, Vec<SemanticError>> {
    let model_type = last_ident(&page.model_ty);
    let mut state_type = String::new();
    let mut cx = Cx {
        registry,
        scopes: Vec::new(),
        file,
        errors: Vec::new(),
        types,
        model: None,
        state_struct: None,
    };

    if let Some(types) = types {
        match types.model(&model_type) {
            Some(def) => {
                cx.model = Some(def);
                match types.model_state(def) {
                    Ok(state) => {
                        match types.structs.get(&state) {
                            Some(s) => cx.state_struct = Some(s),
                            None => cx.errors.push(SemanticError::new(
                                spanned_path(&page.model_ty),
                                format!("cannot find struct `{state}`, `{model_type}`'s value, in this crate"),
                            )),
                        }
                        state_type = state;
                    }
                    Err(message) => cx
                        .errors
                        .push(SemanticError::new(spanned_path(&page.model_ty), message)),
                }
            }
            None => cx.errors.push(
                SemanticError::new(
                    spanned_path(&page.model_ty),
                    format!("cannot find `#[page_model] impl {model_type}` in this crate"),
                )
                .help("the page's model is a type whose impl is marked `#[page_model]`, which the CLI can read from `--src`"),
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
        model_type,
        root: root.expect("a node with no errors lowers"),
        source: cx.loc(page.name.span()),
    })
}

fn last_ident(path: &syn::Path) -> String {
    path.segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default()
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
    model: Option<&'a ModelDef>,
    state_struct: Option<&'a StructDef>,
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

        let mut seen: HashMap<String, Span> = HashMap::new();
        // Names a misspelling was taken for: already reported, so not
        // reported again as missing.
        let mut meant: Vec<String> = Vec::new();
        let mut props = Vec::new();
        let mut events: Vec<UiEvent> = Vec::new();
        let mut slots: Vec<UiSlot> = Vec::new();
        for PropAst { name: ident, value } in &ast.props {
            let prop = ident.to_string();
            let span = ident.span();
            if seen.insert(prop.clone(), span).is_some() {
                self.errors.push(
                    SemanticError::new(span, format!("`{prop}` is given twice"))
                        .label("given again here"),
                );
                continue;
            }
            if let Some(event) = spec.event(&prop) {
                if let Some(call) = self.event_call(value, event, &prop) {
                    events.push(UiEvent {
                        name: prop,
                        call,
                        source: self.loc(span),
                    });
                }
            } else if let Some(slot) = spec.slot(&prop) {
                match value {
                    ValueAst::Component(node) => {
                        if let Some(node) = self.component(node) {
                            slots.push(UiSlot {
                                name: slot.name.clone(),
                                node: UiNode::Component(node),
                                source: self.loc(span),
                            });
                        }
                    }
                    _ => self.errors.push(
                        SemanticError::new(span, format!("`{prop}` on `{name}` takes a component"))
                            .help(format!("`{prop}: Component(…)`")),
                    ),
                }
            } else if let Some(prop_spec) = spec.prop(&prop) {
                if let Some(expr) = self.expr(value, &prop_spec.ty, &prop, &name) {
                    props.push(UiProp {
                        name: prop,
                        value: expr,
                        source: self.loc(span),
                    });
                }
            } else if spec.unsupported.contains(&prop) {
                self.errors.push(
                    SemanticError::new(
                        span,
                        format!(
                            "`{prop}` on `{name}` is a React prop dazzui-compiler does not draw yet"
                        ),
                    )
                    .label("not drawn on the native side yet"),
                );
            } else {
                let closest = closest(&prop, spec.names());
                meant.extend(closest.map(str::to_owned));
                self.errors.push(
                    SemanticError::new(
                        span,
                        format!("unknown property `{prop}` on component `{name}`"),
                    )
                    .help_opt(
                        closest
                            .map(|c| format!("did you mean `{c}`?"))
                            .or_else(|| react_spelling(&prop, spec)),
                    ),
                );
            }
        }

        // A default is resolved here rather than by each backend, so the IR
        // says what is drawn and no platform's own default leaks in.
        for prop_spec in &spec.props {
            if let (false, Some(default)) = (seen.contains_key(&prop_spec.name), &prop_spec.default)
            {
                let value = match (&prop_spec.ty, default) {
                    (PropType::Enum(e), Value::String(case)) => Expr::EnumCase {
                        ty: e.name.clone(),
                        case: case.clone(),
                    },
                    _ => Expr::Literal(default.clone()),
                };
                props.push(UiProp {
                    name: prop_spec.name.clone(),
                    value,
                    source: self.loc(ast.name.span()),
                });
            }
        }

        for prop_spec in spec.props.iter().filter(|p| p.required) {
            if !seen.contains_key(&prop_spec.name) && !meant.contains(&prop_spec.name) {
                self.errors.push(
                    SemanticError::new(
                        ast.name.span(),
                        format!(
                            "missing required property `{}` on component `{name}`",
                            prop_spec.name
                        ),
                    )
                    .help(format!("add it: `{name}({}: …)`", prop_spec.name)),
                );
            }
        }
        for slot in spec.slots.iter().filter(|s| s.required) {
            if !seen.contains_key(&slot.name) && !meant.contains(&slot.name) {
                self.errors.push(
                    SemanticError::new(
                        ast.name.span(),
                        format!("missing required `{}` on component `{name}`", slot.name),
                    )
                    .help(format!("add it: `{name}({}: Component(…))`", slot.name)),
                );
            }
        }

        let mut text = Vec::new();
        match (&ast.text, &spec.children) {
            (Some(value), ChildrenSpec::Text) => text.extend(self.text(value, &name)),
            (None, ChildrenSpec::Text) => self.errors.push(
                SemanticError::new(ast.name.span(), format!("`{name}` needs its text"))
                    .help(format!("give it first: `{name}(\"…\")`")),
            ),
            (Some(value), ChildrenSpec::One { .. }) => self.errors.push(
                SemanticError::new(value.span(), format!("only `Text` takes its text first; `{name}` takes a child"))
                    .help(format!("`{name}(…, child: Text(\"…\"))`")),
            ),
            (Some(value), _) => self.errors.push(
                SemanticError::new(value.span(), "props are named, as Flutter's and React's are: `prop: value`")
                    .label("not named"),
            ),
            (None, _) => {}
        }

        let mut children = Vec::new();
        match (&spec.children, ast.children_arg) {
            (_, None) => {}
            (ChildrenSpec::None | ChildrenSpec::Text, Some((_, span))) => self.errors.push(
                SemanticError::new(span, format!("`{name}` takes no children")).label("given here"),
            ),
            (ChildrenSpec::One { .. }, Some((ChildrenArg::Many, span))) => self.errors.push(
                SemanticError::new(span, format!("`{name}` takes one child: `child: …`, not `children: [ … ]`"))
                    .label("a list"),
            ),
            (ChildrenSpec::Many { .. }, Some((ChildrenArg::One, span))) => self.errors.push(
                SemanticError::new(span, format!("`{name}` takes a list: `children: [ … ]`, not `child: …`"))
                    .label("one child"),
            ),
            (spec_children, Some(_)) => {
                if let Some(allow) = spec_children.allow() {
                    self.check_allowed(&ast.children, allow, &name);
                }
                children = self.nodes(&ast.children);
            }
        }

        if self.errors.len() > errors_before {
            return None;
        }
        Some(Component {
            id: ComponentId(name),
            props,
            events,
            slots,
            children,
            text,
            source: self.loc(ast.name.span()),
        })
    }

    /// `Text`'s text: a string literal, or a string of the model's value or
    /// a loop item.
    fn text(&mut self, value: &ValueAst, component: &str) -> Option<Expr> {
        match value {
            ValueAst::Lit(Lit::Str(s)) => Some(Expr::Literal(Value::String(s.value()))),
            ValueAst::Dotted(path) => self.path_value(path, &PropType::String, "text", component),
            other => {
                self.errors.push(
                    SemanticError::new(
                        other.span(),
                        format!("`{component}` takes a string, or a string of the model's value"),
                    )
                    .label("not text"),
                );
                None
            }
        }
    }

    /// Only `allow`ed components as children, directly or in an `if`; a
    /// `for` cannot promise what it repeats is one.
    fn check_allowed(&mut self, children: &[NodeAst], allow: &[String], parent: &str) {
        let list = allow
            .iter()
            .map(|a| format!("`{a}`"))
            .collect::<Vec<_>>()
            .join(", ");
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
                    SemanticError::new(
                        other.span(),
                        format!("`{parent}` takes only {list} as children"),
                    )
                    .label("not allowed here"),
                ),
            }
        }
    }

    /// Whether `model.name` names one of the model's methods rather than
    /// a field of its value.
    fn is_method(&self, path: &DottedAst) -> bool {
        match (path.segments.as_slice(), self.model) {
            ([head, name], Some(model)) => head == "model" && model.method(&name.to_string()).is_some(),
            _ => false,
        }
    }

    /// A read-only value for a prop of type `ty`.
    fn expr(
        &mut self,
        value: &ValueAst,
        ty: &PropType,
        prop: &str,
        component: &str,
    ) -> Option<Expr> {
        let found = match value {
            ValueAst::Lit(Lit::Str(s)) if *ty == PropType::Spacing => {
                let step = s.value();
                if SPACING_STEPS.contains(&step.as_str()) {
                    return Some(Expr::Literal(Value::String(step)));
                }
                self.errors.push(
                    SemanticError::new(s.span(), format!("no spacing step `{step}`"))
                        .help_opt(suggest(&step, SPACING_STEPS.iter().copied()))
                        .label(format!("steps are {}", SPACING_STEPS.join(", "))),
                );
                return None;
            }
            ValueAst::Lit(Lit::Str(s)) if *ty == PropType::String => {
                return Some(Expr::Literal(Value::String(s.value())));
            }
            ValueAst::Lit(Lit::Bool(b)) if *ty == PropType::Bool => {
                return Some(Expr::Literal(Value::Bool(b.value)));
            }
            ValueAst::Lit(Lit::Int(_)) | ValueAst::NegInt(Lit::Int(_)) if *ty == PropType::Int => {
                return self.int(value).map(|n| Expr::Literal(Value::Int(n)));
            }
            ValueAst::Path(path) => {
                if let PropType::Enum(spec) = ty {
                    return self.enum_case(path, spec);
                }
                "an enum case"
            }
            ValueAst::Dotted(path) if self.is_method(path) => "a model method",
            ValueAst::Dotted(path) => {
                // A mapping passes an integer on as it is, so a mapped
                // component's may read the state. A spacing step is read from
                // the theme, and an enum's case written as platform code, so
                // those stay literals.
                let mapped = self
                    .registry
                    .get(component)
                    .is_some_and(|s| s.swiftui.is_some() && s.winui.is_some());
                if matches!(ty, PropType::Enum(_) | PropType::Spacing)
                    || (*ty == PropType::Int && !mapped)
                {
                    self.errors.push(SemanticError::new(
                        path.span(),
                        format!("property `{prop}` on `{component}` takes a literal for now, not a value of the model"),
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
            ValueAst::Call(..) => "a model method",
            ValueAst::Component(_) => "a component",
        };
        let mut error = SemanticError::new(
            value.span(),
            format!(
                "property `{prop}` on `{component}` expects {}, found {found}",
                ty.name()
            ),
        )
        .label(format!("expected {}", ty.name()));
        if found == "a model method" {
            error =
                error.help("a model method is given to an event, such as `on_click: model.save`");
        }
        self.errors.push(error);
        None
    }

    fn int(&mut self, value: &ValueAst) -> Option<i64> {
        let parsed = match value {
            ValueAst::Lit(Lit::Int(i)) => i.base10_parse::<i64>().map_err(|e| e.to_string()),
            ValueAst::NegInt(Lit::Int(i)) => format!("-{}", i.base10_digits())
                .parse::<i64>()
                .map_err(|e| e.to_string()),
            _ => unreachable!("not an integer literal"),
        };
        parsed
            .map_err(|e| self.errors.push(SemanticError::new(value.span(), e)))
            .ok()
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
                SemanticError::new(
                    ty.span(),
                    format!("expected a `{}` case, found `{ty}::{case}`", spec.name),
                )
                .label(format!("expected {}", spec.name)),
            );
            return None;
        }
        let case_name = case.to_string();
        if !spec.cases.contains(&case_name) {
            self.errors.push(
                SemanticError::new(
                    case.span(),
                    format!("no case `{case_name}` on `{}`", spec.name),
                )
                .help_opt(suggest(&case_name, spec.cases.iter().map(String::as_str)))
                .label(format!("cases are {}", spec.cases.join(", "))),
            );
            return None;
        }
        Some(Expr::EnumCase {
            ty: spec.name.clone(),
            case: case_name,
        })
    }

    /// `state.a.b` or `item.a.b`, resolved against the state struct or the
    /// loop's element when they are at hand: the value, and the Rust type
    /// it reads.
    fn resolve(&mut self, path: &DottedAst) -> Option<(Expr, Option<RustType>)> {
        let (head, rest) = path
            .segments
            .split_first()
            .expect("a dotted path has a segment");
        let head_name = head.to_string();
        let (start, local) = if head == "model" {
            if let Some(scope) = self.scopes.last() {
                self.errors.push(
                    SemanticError::new(head.span(), "the model is not read inside a `for`")
                        .label(format!("read `{}` here", scope.name))
                        .help("derive what each element needs with a #[selector] that puts it on the element"),
                );
                return None;
            }
            (self.state_struct, None)
        } else if let Some(scope) = self.scopes.iter().rev().find(|s| s.name == head_name) {
            (scope.element, Some(head_name.clone()))
        } else {
            let names: Vec<&str> = self
                .scopes
                .iter()
                .map(|s| s.name.as_str())
                .chain(["model"])
                .collect();
            self.errors.push(
                SemanticError::new(head.span(), format!("cannot find value `{head}`"))
                    .label("not the model or a loop item")
                    .help_opt(suggest(&head_name, names.iter().copied()).or_else(|| {
                        Some(
                            "a value path starts with `model` or a `for` item: `model.field`"
                                .into(),
                        )
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
                    SemanticError::new(
                        ident.span(),
                        format!("no field `{name}` on type `{}`", current.name),
                    )
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
    fn path_value(
        &mut self,
        path: &DottedAst,
        ty: &PropType,
        prop: &str,
        component: &str,
    ) -> Option<Expr> {
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

    fn nodes(&mut self, nodes: &[NodeAst]) -> Vec<UiNode> {
        nodes.iter().filter_map(|n| self.node(n)).collect()
    }

    fn if_node(&mut self, ast: &IfAst) -> Option<UiNode> {
        let value = self.path_value(&ast.condition, &PropType::Bool, "condition", "if");
        let then_branch = self.nodes(&ast.then_branch);
        let else_branch = self.nodes(&ast.else_branch);
        Some(UiNode::If {
            condition: Condition {
                negated: ast.negated,
                value: value?,
            },
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
        if item == "model" {
            self.errors.push(SemanticError::new(
                ast.item.span(),
                "a loop item cannot be called `model`",
            ));
            return None;
        }
        let (collection, collection_ty) = self.resolve(&ast.collection)?;
        let Expr::State(collection) = collection else {
            unreachable!("outside a loop, a path is the state's")
        };

        let element = match (&collection_ty, self.types) {
            (None, _) => None,
            (Some(RustType::Vec(elem)), Some(types)) => match &**elem {
                RustType::Other(name) if types.structs.contains_key(name) => {
                    Some(&types.structs[name])
                }
                other => {
                    self.errors.push(
                        SemanticError::new(
                            ast.collection.span(),
                            format!(
                                "a `for` runs over a Vec of structs; this is a `Vec<{}>`",
                                other.rust()
                            ),
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
                        format!(
                            "`model.{}` is a `{}`, not a Vec",
                            collection.dotted(),
                            other.rust()
                        ),
                    )
                    .label("expected a Vec"),
                );
                return None;
            }
        };

        self.scopes.push(Scope {
            name: item.clone(),
            element,
        });
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
                    SemanticError::new(
                        ast.item.span(),
                        format!("`{}` has no `id` to tell its elements apart", element.name),
                    )
                    .help(format!(
                        "name the field that does: `for {item} in …, key: {item}.field`"
                    )),
                );
                return None;
            }
            return Some(LocalPath {
                local: item,
                segments: vec!["id".into()],
            });
        };
        if key.segments[0] != item {
            self.errors.push(SemanticError::new(
                key.span(),
                format!("a key is a field of the loop item: `{item}.field`"),
            ));
            return None;
        }
        match self.resolve(key)? {
            (Expr::Local(path), _) => Some(path),
            _ => None,
        }
    }

    /// The model method an event calls: `on_click: model.save`,
    /// `on_checked_change: model.set_on`, `on_click: model.pick(item.id)`.
    fn event_call(
        &mut self,
        value: &ValueAst,
        event: &EventSpec,
        prop: &str,
    ) -> Option<MethodCall> {
        let (path, given) = match value {
            ValueAst::Dotted(path) => (path, None),
            ValueAst::Call(path, argument) => (path, Some(&**argument)),
            _ => {
                self.errors.push(
                    SemanticError::new(
                        value.span(),
                        format!("`{prop}` calls a method of the model: `{prop}: model.method`"),
                    )
                    .label("not a model method"),
                );
                return None;
            }
        };
        let [head, method_ident] = path.segments.as_slice() else {
            self.errors.push(SemanticError::new(
                path.span(),
                format!("`{prop}` calls a method of the model: `{prop}: model.method`"),
            ));
            return None;
        };
        if head != "model" {
            self.errors.push(
                SemanticError::new(head.span(), format!("`{prop}` calls a method of the model"))
                    .label("not `model`")
                    .help(format!("`{prop}: model.{method_ident}`")),
            );
            return None;
        }
        let method = method_ident.to_string();
        let Some(model) = self.model else {
            // In the macro, rustc checks the method and its argument.
            let argument = match given {
                Some(value) => Argument::Expr(self.payload_expr(value, None)?),
                None if event.payload.is_some() => Argument::EventValue,
                None => Argument::None,
            };
            return Some(MethodCall {
                method,
                param: None,
                argument,
            });
        };
        let Some(def) = model.method(&method) else {
            let candidates = model.methods.iter().map(|m| m.name.as_str());
            self.errors.push(
                SemanticError::new(
                    method_ident.span(),
                    format!("no method `{method}` on model `{}`", model.name),
                )
                .help_opt(suggest(&method, candidates).or_else(|| {
                    Some(format!(
                        "a page calls the public methods of `#[page_model] impl {}`",
                        model.name
                    ))
                })),
            );
            return None;
        };
        let what = format!("`{}::{method}`", model.name);
        let argument = match (given, &def.param, &event.payload) {
            (None, None, None) => Argument::None,
            // The value an event carries is for the method to take; one that
            // drops it would hide a change the event reports.
            (None, None, Some(carried)) => {
                self.errors.push(
                    SemanticError::new(
                        method_ident.span(),
                        format!(
                            "{what} takes no value, but `{prop}` gives it {}",
                            carried.name()
                        ),
                    )
                    .label("takes no value")
                    .help(format!(
                        "give the method a parameter: `pub fn {method}(&self, value: …)`"
                    )),
                );
                return None;
            }
            (Some(value), Some((_, ty)), _) => Argument::Expr(self.payload_expr(value, Some(ty))?),
            (Some(value), None, _) => {
                self.errors.push(
                    SemanticError::new(value.span(), format!("{what} takes no value"))
                        .help(format!("write `model.{method}`")),
                );
                return None;
            }
            (None, Some((_, ty)), Some(carried)) => {
                if !fits(carried, ty) {
                    self.errors.push(
                        SemanticError::new(
                            method_ident.span(),
                            format!(
                                "{what} takes a `{}`, but `{prop}` gives it {}",
                                ty.rust(),
                                carried.name()
                            ),
                        )
                        .label("parameter type differs"),
                    );
                    return None;
                }
                Argument::EventValue
            }
            (None, Some((_, ty)), None) => {
                self.errors.push(
                    SemanticError::new(
                        method_ident.span(),
                        format!("{what} takes a value, but `{prop}` has none to give it"),
                    )
                    .label("takes a value")
                    .help(format!("pass one: `model.{method}(…)`, a `{}`", ty.rust())),
                );
                return None;
            }
        };
        Some(MethodCall {
            method,
            param: def.param.as_ref().map(|(name, _)| name.clone()),
            argument,
        })
    }

    fn payload_expr(&mut self, value: &ValueAst, field: Option<&RustType>) -> Option<Expr> {
        let ty = match field {
            Some(RustType::Bool) => Some(PropType::Bool),
            Some(RustType::String) => Some(PropType::String),
            Some(RustType::Int(_)) => Some(PropType::Int),
            Some(other) => {
                self.errors.push(SemanticError::new(
                    value.span(),
                    format!(
                        "a method parameter of type `{}` cannot cross to the native side yet",
                        other.rust()
                    ),
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
                    let written: Vec<String> =
                        path.segments.iter().map(ToString::to_string).collect();
                    self.errors.push(SemanticError::new(
                        value.span(),
                        format!(
                            "the method takes a `{}`, but `{}` is a `{}`",
                            field.rust(),
                            written.join("."),
                            found.rust()
                        ),
                    ));
                    return None;
                }
                Some(expr)
            }
            (ValueAst::Lit(Lit::Str(s)), None | Some(PropType::String)) => {
                Some(Expr::Literal(Value::String(s.value())))
            }
            (ValueAst::Lit(Lit::Bool(b)), None | Some(PropType::Bool)) => {
                Some(Expr::Literal(Value::Bool(b.value)))
            }
            (
                ValueAst::Lit(Lit::Int(_)) | ValueAst::NegInt(Lit::Int(_)),
                None | Some(PropType::Int),
            ) => self.int(value).map(|n| Expr::Literal(Value::Int(n))),
            (_, ty) => {
                let expected = ty
                    .map(|t| t.name())
                    .unwrap_or_else(|| "a literal or a path".into());
                self.errors.push(SemanticError::new(
                    value.span(),
                    format!("a method's argument is a literal or a path; expected {expected}"),
                ));
                None
            }
        }
    }
}

fn fits(ty: &PropType, field: &RustType) -> bool {
    matches!(
        (ty, field),
        (PropType::Bool, RustType::Bool)
            | (PropType::String, RustType::String)
            | (PropType::Int, RustType::Int(_))
    )
}

/// For a prop given in React's camel case, the snake case it is written in
/// here: `onClick` → `on_click`.
fn react_spelling(prop: &str, spec: &ComponentSpec) -> Option<String> {
    let snake = crate::ir::snake_case(prop);
    (snake != prop && spec.names().any(|n| n == snake))
        .then(|| format!("props are in snake case here: `{snake}`"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_types::RustTypes;

    fn types() -> RustTypes {
        let file: syn::File = syn::parse_str(
            "struct S { on: bool, name: String, count: u32, small: u8, user: User, items: Vec<Item>, tags: Vec<String> }
             struct User { locked: bool }
             struct Item { id: u32, title: String, done: bool }
             struct Row { uuid: String }
             impl S { #[selector] fn open(&self) -> Vec<Item> { todo!() } #[selector] fn rows(&self) -> Vec<Row> { todo!() } }
             struct T { on: bool }
             struct M;
             impl ValueListenable for M { type Value = S; }
             #[page_model]
             impl M {
                 pub fn new() -> Self { M }
                 pub fn save(&self) {}
                 pub fn set_on(&self, on: bool) {}
                 pub fn rename(&self, name: String) {}
                 pub fn pick(&self, id: u32) {}
             }",
        )
        .unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items, "crate");
        types
    }

    fn lower_in(registry: &Registry, src: &str) -> Result<Page, Vec<String>> {
        let ast: PageAst = syn::parse_str(src).unwrap();
        lower(&ast, registry, Some(&types()), "page.rs")
            .map_err(|errors| errors.into_iter().map(|e| e.message).collect())
    }

    fn lower_str(src: &str) -> Result<Page, Vec<String>> {
        lower_in(&Registry::builtin(), src)
    }

    fn root(page: &Page) -> &Component {
        let UiNode::Component(c) = &page.root else {
            panic!()
        };
        c
    }

    #[test]
    fn lowers_a_page_and_fills_react_defaults() {
        let page = lower_str(
            r#"page P(model: M) { Col(children: [Button(on_click: model.save, child: Text("Save"))]) }"#,
        )
        .unwrap();
        let col = root(&page);
        assert_eq!(page.state_type, "S");
        assert_eq!(col.prop("gap"), Some(&Expr::Literal(Value::String("3".into()))));
        let UiNode::Component(button) = &col.children[0] else {
            panic!()
        };
        assert_eq!(
            button.event("on_click"),
            Some(&MethodCall {
                method: "save".into(),
                param: None,
                argument: Argument::None
            })
        );
        assert_eq!(
            button.prop("variant"),
            Some(&Expr::EnumCase {
                ty: "ButtonVariant".into(),
                case: "Recessed".into()
            })
        );
        let UiNode::Component(label) = &button.children[0] else {
            panic!()
        };
        assert_eq!(label.text, [Expr::Literal(Value::String("Save".into()))]);
    }

    #[test]
    fn lowers_props_text_and_calls() {
        let page = lower_str(
            r#"page P(model: M) {
                Row(gap: "0-5", align: Align::Start, children: [
                    Text(model.name, tone: TextTone::Secondary),
                    Switch(checked: model.on, disabled: model.user.locked, on_checked_change: model.set_on),
                    Button(variant: ButtonVariant::Filled, on_click: model.pick(model.count), child: Text("Pick")),
                ])
            }"#,
        )
        .unwrap();
        let row = root(&page);
        assert_eq!(row.prop("gap"), Some(&Expr::Literal(Value::String("0-5".into()))));
        let [
            UiNode::Component(text),
            UiNode::Component(switch),
            UiNode::Component(button),
        ] = row.children.as_slice()
        else {
            panic!("three children");
        };
        assert_eq!(
            text.text,
            [Expr::State(StatePath {
                segments: vec!["name".into()]
            })]
        );
        assert_eq!(
            switch.event("on_checked_change"),
            Some(&MethodCall {
                method: "set_on".into(),
                param: Some("on".into()),
                argument: Argument::EventValue
            })
        );
        assert_eq!(
            switch.prop("disabled"),
            Some(&Expr::State(StatePath {
                segments: vec!["user".into(), "locked".into()]
            }))
        );
        assert_eq!(
            button.event("on_click").unwrap().argument,
            Argument::Expr(Expr::State(StatePath {
                segments: vec!["count".into()]
            }))
        );
    }

    #[test]
    fn reports_names_react_does_not_have() {
        let errors = lower_str(
            r#"page P(model: M) { Col(children: [
                Buton(child: Text("x")),
                Button(varient: ButtonVariant::Filled, child: Text("a")),
                Button(onClick: model.save, child: Text("b")),
                Button(full_width: true, child: Text("c")),
                Switch(checked: true, checked: false),
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "unknown component `Buton`",
                "unknown property `varient` on component `Button`",
                "unknown property `onClick` on component `Button`",
                "`full_width` on `Button` is a React prop dazzui-compiler does not draw yet",
                "`checked` is given twice",
            ]
        );
    }

    #[test]
    fn checks_calls_against_the_model() {
        let errors = lower_str(
            r#"page P(model: M) { Col(children: [
                Button(on_click: model.sav, child: Text("a")),
                Switch(checked: model.on, on_checked_change: model.save),
                Button(on_click: model.set_on, child: Text("b")),
                Button(on_click: model.save(1), child: Text("c")),
                Button(on_click: model.pick(model.small), child: Text("d")),
                Button(on_click: state.save, child: Text("e")),
                Button(on_click: "save", child: Text("f")),
                Button(disabled: model.save, child: Text("g")),
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no method `sav` on model `M`",
                "`M::save` takes no value, but `on_checked_change` gives it Bool",
                "`M::set_on` takes a value, but `on_click` has none to give it",
                "`M::save` takes no value",
                "the method takes a `u32`, but `model.small` is a `u8`",
                "`on_click` calls a method of the model",
                "`on_click` calls a method of the model: `on_click: model.method`",
                "property `disabled` on `Button` expects Bool, found a model method",
            ]
        );
    }

    #[test]
    fn checks_the_page_against_its_model() {
        assert_eq!(
            lower_str(r#"page P(model: N) { Col() }"#).unwrap_err(),
            ["cannot find `#[page_model] impl N` in this crate"]
        );
    }

    #[test]
    fn checks_value_paths_and_children() {
        let errors = lower_str(
            r#"page P(model: M) { Col(children: [
                Text(model.nme),
                Text(model.on),
                Text(modl.name),
                Switch(checked: model.user.lockd),
                Text(),
                Text("a", child: Text("b")),
                Switch(checked: true, child: Text("c")),
                Button(child: Text("d"), on_click: model.save, variant: ButtonVariant::Plain),
                Button("e", on_click: model.save),
                PreferenceRow(title: "f", children: [Text("g")]),
                Col(child: Text("h")),
                Button(child: Switch(checked: true), on_click: model.save),
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no field `nme` on type `S`",
                "property `text` on `Text` expects String, but `model.on` is a `bool`",
                "cannot find value `modl`",
                "no field `lockd` on type `User`",
                "`Text` needs its text",
                "`Text` takes no children",
                "`Switch` takes no children",
                "only `Text` takes its text first; `Button` takes a child",
                "`PreferenceRow` takes one child: `child: …`, not `children: [ … ]`",
                "`Col` takes a list: `children: [ … ]`, not `child: …`",
                "`Button` takes only `Text` as children",
            ]
        );
    }

    #[test]
    fn lowers_if_and_for() {
        let page = lower_str(
            r#"page P(model: M) { Col(children: [
                if !model.on { Text("off") } else { Text("on") },
                for item in model.open {
                    if item.done { Text(item.title) },
                    Button(on_click: model.pick(item.id), child: Text("Pick")),
                },
                for row in model.rows, key: row.uuid { Text(row.uuid) },
            ]) }"#,
        )
        .unwrap();
        let col = root(&page);
        let UiNode::If {
            condition,
            else_branch,
            ..
        } = &col.children[0]
        else {
            panic!()
        };
        assert!(condition.negated);
        assert_eq!(
            condition.value,
            Expr::State(StatePath {
                segments: vec!["on".into()]
            })
        );
        assert_eq!(else_branch.len(), 1);
        let UiNode::ForEach {
            collection,
            element,
            key,
            children,
            ..
        } = &col.children[1]
        else {
            panic!()
        };
        assert_eq!(
            (collection.dotted().as_str(), element.as_str()),
            ("open", "Item")
        );
        assert_eq!(
            key,
            &LocalPath {
                local: "item".into(),
                segments: vec!["id".into()]
            }
        );
        let UiNode::If { condition, .. } = &children[0] else {
            panic!()
        };
        assert_eq!(
            condition.value,
            Expr::Local(LocalPath {
                local: "item".into(),
                segments: vec!["done".into()]
            })
        );
        let UiNode::ForEach { key, .. } = &col.children[2] else {
            panic!()
        };
        assert_eq!(key.segments, ["uuid"]);
    }

    #[test]
    fn checks_if_and_for() {
        let errors = lower_str(
            r#"page P(model: M) { Col(children: [
                if model.name { Text("a") },
                for item in model.count { Text("b") },
                for tag in model.tags { Text("c") },
                for row in model.rows { Text("d") },
                for item in model.items {
                    Text(model.name),
                    Text(item.titel),
                    Text(iten.title),
                    for other in model.items { Text("e") },
                },
                Text(item.title),
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "property `condition` on `if` expects Bool, but `model.name` is a `String`",
                "`model.count` is a `u32`, not a Vec",
                "a `for` runs over a Vec of structs; this is a `Vec<String>`",
                "`Row` has no `id` to tell its elements apart",
                "the model is not read inside a `for`",
                "no field `titel` on type `Item`",
                "cannot find value `iten`",
                "a `for` inside a `for` is not supported yet",
                "cannot find value `item`",
            ]
        );
    }

    /// An app's own components: a list that takes only its rows, and a row
    /// with a required icon and a control.
    const ROWS: &str = r#"
        [component.List]
        children = { allow = ["ListRow"] }
        swiftui = { call = "List", trailing = { children = true } }
        winui = { element = "List" }

        [component.ListRow]
        children = "one"
        props.title = { type = "string", required = true }
        slots.icon = { required = true }
        swiftui = { call = "ListRow", args = [{ arg = "title", prop = "title" }, { arg = "icon", slot = "icon" }, { arg = "trailing", children = true }] }
        winui = { element = "ListRow", slots.icon = { property = "Icon" }, children = { property = "Control" } }
    "#;

    fn rows() -> Registry {
        Registry::from_tables(&[
            (crate::registry::BUILTIN, "registry.toml"),
            (ROWS, "app.toml"),
        ])
        .unwrap()
    }

    #[test]
    fn lowers_a_widget_given_to_a_prop() {
        let page = lower_in(
            &rows(),
            r#"page P(model: M) { List(children: [
                ListRow(title: "On", icon: Text("i"), child: Switch(checked: model.on, on_checked_change: model.set_on)),
                if model.on { ListRow(title: "Name", icon: Text(model.name)) },
            ]) }"#,
        )
        .unwrap();
        let UiNode::Component(row) = &root(&page).children[0] else {
            panic!()
        };
        let UiNode::Component(switch) = &row.children[0] else {
            panic!()
        };
        assert_eq!(switch.event("on_checked_change").unwrap().method, "set_on");
        assert!(row.slot("icon").is_some());
    }

    #[test]
    fn checks_slots_and_allowed_children() {
        let errors = lower_in(
            &rows(),
            r#"page P(model: M) { List(children: [
                ListRow(title: "a", icon: "x"),
                ListRow(title: "c"),
                ListRow(title: "d", icon: Text(model.nme)),
                Text("h"),
                for item in model.items { ListRow(title: "i", icon: Text("i")) },
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "`List` takes only `ListRow` as children",
                "`List` takes only `ListRow` as children",
                "`icon` on `ListRow` takes a component",
                "missing required `icon` on component `ListRow`",
                "no field `nme` on type `S`",
            ]
        );
    }

    #[test]
    fn a_mapped_component_reads_an_integer_from_the_value() {
        const COLUMN: &str = r#"
            [component.Column]
            props.width = { type = "int", required = true }
            swiftui = { call = "Column", args = [{ arg = "width", prop = "width" }] }
            winui = { element = "Column", attrs = [{ attr = "Width", prop = "width" }] }
        "#;
        let registry = Registry::from_tables(&[
            (crate::registry::BUILTIN, "registry.toml"),
            (COLUMN, "app.toml"),
        ])
        .unwrap();
        let page = lower_in(
            &registry,
            r#"page P(model: M) { Col(children: [Column(width: model.count), for item in model.items { Column(width: item.id) }]) }"#,
        )
        .unwrap();
        let UiNode::Component(column) = &root(&page).children[0] else {
            panic!()
        };
        assert!(
            matches!(column.prop("width"), Some(Expr::State(path)) if path.segments == ["count"])
        );
        // A spacing step is still a literal, and the type still checked.
        assert_eq!(
            lower_in(&registry, r#"page P(model: M) { Row(gap: model.name, children: [Column(width: model.name)]) }"#)
                .unwrap_err(),
            [
                "property `gap` on `Row` takes a literal for now, not a value of the model",
                "property `width` on `Column` expects Integer, but `model.name` is a `String`",
            ]
        );
    }

    #[test]
    fn checks_literals_and_cases() {
        let errors = lower_str(
            r#"page P(model: M) { Col(gap: "13", children: [
                Row(gap: 3),
                Button(variant: TextTone::Secondary, child: Text("a")),
                Button(variant: ButtonVariant::Primry, child: Text("b")),
                Text("c", strong: "yes"),
            ]) }"#,
        )
        .unwrap_err();
        assert_eq!(
            errors,
            [
                "no spacing step `13`",
                "property `gap` on `Row` expects a spacing step, found an integer",
                "expected a `ButtonVariant` case, found `TextTone::Secondary`",
                "no case `Primry` on `ButtonVariant`",
                "property `strong` on `Text` expects Bool, found a string",
            ]
        );
    }
}
