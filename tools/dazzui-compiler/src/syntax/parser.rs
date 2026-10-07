use syn::parse::{Parse, ParseStream};
use syn::{Ident, Lit, Path, Result, Token, braced, bracketed, parenthesized};

use super::ast::*;

impl Parse for PageAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        if keyword != "page" {
            return Err(syn::Error::new(
                keyword.span(),
                "expected `page`: a ui! block declares `page Name(model: Model) { … }`",
            ));
        }
        let name: Ident = input.parse()?;

        let header;
        parenthesized!(header in input);
        let shape = "a page takes its model: `page Name(model: Model)`; it reads the model's value as `model.field`";
        let param: Ident = header.parse()?;
        if param != "model" {
            return Err(syn::Error::new(param.span(), shape));
        }
        header.parse::<Token![:]>()?;
        let model_ty: Path = header.parse()?;
        if header.peek(Token![,]) {
            header.parse::<Token![,]>()?;
        }
        if !header.is_empty() {
            return Err(header.error(shape));
        }

        let body;
        braced!(body in input);
        let root: NodeAst = body.parse()?;
        if !matches!(root, NodeAst::Component(_)) {
            return Err(syn::Error::new(
                root.span(),
                "a page's root is a widget; put an `if` or a `for` in its `children: [ … ]`",
            ));
        }
        if body.peek(Token![,]) {
            body.parse::<Token![,]>()?;
        }
        if !body.is_empty() {
            return Err(body.error("a page has one root widget; put several in a Col's `children: [ … ]`"));
        }
        if !input.is_empty() {
            return Err(input.error("one page per ui! block"));
        }
        Ok(PageAst {
            name,
            model_ty,
            root,
        })
    }
}

impl Parse for NodeAst {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            return Ok(NodeAst::If(input.parse()?));
        }
        if input.peek(Token![for]) {
            return Ok(NodeAst::For(input.parse()?));
        }
        if input.peek(Lit) {
            return Err(input.error("text is a widget, as in Flutter: `Text(\"…\")`"));
        }
        Ok(NodeAst::Component(input.parse()?))
    }
}

/// `a, b, …` up to the end of `input`, a trailing comma allowed.
fn node_list(input: ParseStream) -> Result<Vec<NodeAst>> {
    let mut nodes = Vec::new();
    while !input.is_empty() {
        nodes.push(input.parse()?);
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(nodes)
}

/// `{ a, b }`: an `if` or `for` body.
fn nodes(input: ParseStream) -> Result<Vec<NodeAst>> {
    let content;
    braced!(content in input);
    node_list(&content)
}

impl Parse for IfAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword = input.parse::<Token![if]>()?.span;
        let negated = input.peek(Token![!]);
        if negated {
            input.parse::<Token![!]>()?;
        }
        let first: Ident = input.parse()?;
        let condition = dotted(first, input)?;
        if !input.peek(syn::token::Brace) {
            return Err(input.error("a condition is a path to a bool, `model.flag` or `!model.flag`; compute anything else in Rust"));
        }
        let then_branch = nodes(input)?;
        let mut else_branch = Vec::new();
        if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;
            if input.peek(Token![if]) {
                else_branch.push(NodeAst::If(input.parse()?));
            } else {
                else_branch = nodes(input)?;
            }
        }
        Ok(IfAst {
            keyword,
            negated,
            condition,
            then_branch,
            else_branch,
        })
    }
}

impl Parse for ForAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword = input.parse::<Token![for]>()?.span;
        let item: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let first: Ident = input.parse()?;
        let collection = dotted(first, input)?;
        let mut key = None;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let name: Ident = input.parse()?;
            if name != "key" {
                return Err(syn::Error::new(name.span(), "expected `key: item.field`"));
            }
            input.parse::<Token![:]>()?;
            let first: Ident = input.parse()?;
            key = Some(dotted(first, input)?);
        }
        let children = nodes(input)?;
        Ok(ForAst {
            keyword,
            item,
            collection,
            key,
            children,
        })
    }
}

impl Parse for ComponentAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        if !input.peek(syn::token::Paren) {
            let message = if input.peek(syn::token::Brace) {
                format!("a widget is constructed as Flutter's are, its children named: `{name}(children: [ … ])`")
            } else {
                format!("a widget is constructed as Flutter's are: `{name}(…)`")
            };
            return Err(syn::Error::new(name.span(), message));
        }
        let content;
        parenthesized!(content in input);
        let mut component = ComponentAst {
            name,
            text: None,
            props: Vec::new(),
            children_arg: None,
            children: Vec::new(),
        };
        let mut first = true;
        while !content.is_empty() {
            if starts_named(&content) {
                let prop: Ident = content.fork().parse()?;
                if prop == "child" || prop == "children" {
                    let ident: Ident = content.parse()?;
                    content.parse::<Token![:]>()?;
                    if component.children_arg.is_some() {
                        return Err(syn::Error::new(ident.span(), "a widget takes `child:` or `children:`, once"));
                    }
                    if ident == "child" {
                        component.children = vec![content.parse()?];
                        component.children_arg = Some((ChildrenArg::One, ident.span()));
                    } else {
                        if !content.peek(syn::token::Bracket) {
                            return Err(content.error("`children` is a list: `children: [ … ]`"));
                        }
                        let list;
                        bracketed!(list in content);
                        component.children = node_list(&list)?;
                        component.children_arg = Some((ChildrenArg::Many, ident.span()));
                    }
                } else {
                    component.props.push(content.parse()?);
                }
            } else if first {
                component.text = Some(content.parse()?);
            } else {
                return Err(content.error("props are named, as Flutter's and React's are: `prop: value`"));
            }
            first = false;
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }

        if input.peek(syn::token::Brace) {
            return Err(input.error(format!(
                "children are named, as in Flutter: `{}(…, children: [ … ])`, or `child: …` for one",
                component.name
            )));
        }
        if input.peek(Token![.]) && input.peek2(Ident) {
            let fork = input.fork();
            fork.parse::<Token![.]>()?;
            let modifier: Ident = fork.parse()?;
            return Err(syn::Error::new(
                modifier.span(),
                format!(
                    "there are no modifiers; `{modifier}` is a prop: `{}({modifier}: …)`",
                    component.name
                ),
            ));
        }
        Ok(component)
    }
}

/// `ident :` and not `ident ::` — a named prop rather than a value.
fn starts_named(input: ParseStream) -> bool {
    let fork = input.fork();
    fork.parse::<Ident>().is_ok() && fork.peek(Token![:]) && !fork.peek(Token![::])
}

impl Parse for PropAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        Ok(PropAst {
            name,
            value: input.parse()?,
        })
    }
}

impl Parse for ValueAst {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Lit) {
            return Ok(ValueAst::Lit(input.parse()?));
        }
        if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            let lit: Lit = input.parse()?;
            return match lit {
                Lit::Int(_) => Ok(ValueAst::NegInt(lit)),
                _ => Err(syn::Error::new(
                    lit.span(),
                    "only an integer can be negated",
                )),
            };
        }
        let fork = input.fork();
        if let Ok(ident) = fork.parse::<Ident>() {
            if ident == "bind" && fork.peek(syn::token::Paren) {
                return Err(syn::Error::new(
                    ident.span(),
                    "there is no `bind`: give the value and the change apart, as React's controlled props do: `checked: model.on, on_checked_change: model.set_on`",
                ));
            }
            // `Name(`: a widget, for a prop that takes one.
            if fork.peek(syn::token::Paren) {
                return Ok(ValueAst::Component(Box::new(input.parse()?)));
            }
            if !fork.peek(Token![::]) {
                let first: Ident = input.parse()?;
                let path = dotted(first, input)?;
                if input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    let argument: ValueAst = content.parse()?;
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                    if !content.is_empty() {
                        return Err(content.error("an event's method takes one value"));
                    }
                    return Ok(ValueAst::Call(path, Box::new(argument)));
                }
                return Ok(ValueAst::Dotted(path));
            }
        }
        Ok(ValueAst::Path(input.parse()?))
    }
}

/// The rest of `first.b.c`.
fn dotted(first: Ident, input: ParseStream) -> Result<DottedAst> {
    let mut segments = vec![first];
    while input.peek(Token![.]) && input.peek2(Ident) {
        input.parse::<Token![.]>()?;
        segments.push(input.parse()?);
    }
    Ok(DottedAst { segments })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Result<PageAst> {
        syn::parse_str(src)
    }

    fn component(node: &NodeAst) -> &ComponentAst {
        let NodeAst::Component(c) = node else {
            panic!("not a component")
        };
        c
    }

    #[test]
    fn parses_a_page() {
        let page = parse(
            r#"page SettingsPage(model: SettingsModel) {
                Col(gap: "4", children: [
                    Text("General"),
                    Button(variant: ButtonVariant::Filled, on_click: model.save, child: Text("Save")),
                ])
            }"#,
        )
        .unwrap();
        assert_eq!(page.name, "SettingsPage");
        let col = component(&page.root);
        assert_eq!(col.name, "Col");
        assert_eq!(col.props[0].name, "gap");
        assert!(matches!(col.children_arg, Some((ChildrenArg::Many, _))));
        assert_eq!(col.children.len(), 2);
        assert!(matches!(&component(&col.children[0]).text, Some(ValueAst::Lit(_))));
        let button = component(&col.children[1]);
        assert!(matches!(&button.props[0].value, ValueAst::Path(_)));
        assert!(matches!(&button.props[1].value, ValueAst::Dotted(p) if p.segments.len() == 2));
        assert!(matches!(button.children_arg, Some((ChildrenArg::One, _))));
        assert_eq!(component(&button.children[0]).name, "Text");
    }

    #[test]
    fn parses_values_and_calls() {
        let page = parse(
            r#"page P(model: M) {
                Row(children: [
                    Text(model.status, tone: TextTone::Secondary),
                    Switch(checked: model.on, on_checked_change: model.set_on),
                    Button(on_click: model.pick(model.id), child: Text("Pick")),
                ])
            }"#,
        )
        .unwrap();
        let row = component(&page.root);
        let text = component(&row.children[0]);
        assert!(matches!(&text.text, Some(ValueAst::Dotted(p)) if p.segments.len() == 2));
        assert_eq!(text.props[0].name, "tone");
        assert!(component(&row.children[1]).children.is_empty());
        assert!(matches!(
            &component(&row.children[2]).props[0].value,
            ValueAst::Call(method, argument) if method.segments[1] == "pick" && matches!(**argument, ValueAst::Dotted(_))
        ));
    }

    #[test]
    fn parses_collection_if_and_for() {
        let page = parse(
            r#"page P(model: M) {
                Col(children: [
                    if !model.signed_in { Text("a") } else if model.busy { Text("b") } else { Text("c"), Text("d") },
                    for item in model.items, key: item.uuid { Text(item.title) },
                ])
            }"#,
        )
        .unwrap();
        let col = component(&page.root);
        let NodeAst::If(cond) = &col.children[0] else {
            panic!()
        };
        assert!(cond.negated);
        let NodeAst::If(nested) = &cond.else_branch[0] else {
            panic!()
        };
        assert_eq!(nested.else_branch.len(), 2);
        let NodeAst::For(each) = &col.children[1] else {
            panic!()
        };
        assert_eq!(each.item, "item");
        assert_eq!(each.key.as_ref().unwrap().segments[1], "uuid");
    }

    #[test]
    fn parses_a_widget_given_to_a_prop() {
        let page = parse(r#"page P(model: M) { PreferenceRow(title: "Launch", icon: Icon(name: "gear")) }"#).unwrap();
        let row = component(&page.root);
        assert!(matches!(&row.props[1].value, ValueAst::Component(c) if c.name == "Icon"));
    }

    /// What Flutter does not write is refused with how Flutter writes it.
    #[test]
    fn refuses_what_flutter_does_not_write() {
        let err = |src: &str| parse(src).unwrap_err().to_string();
        assert!(err(r#"page P(state: S, model: M) { Text("a") }"#).contains("a page takes its model"));
        assert!(err(r#"page P(model: M) { Col { Text("a") } }"#).contains("children: [ … ]"));
        assert!(err(r#"page P(model: M) { Col(children: [Text("a")]) { Text("b") } }"#).contains("children are named"));
        assert!(err(r#"page P(model: M) { Col(children: ["a"]) }"#).contains("text is a widget"));
        assert!(err(r#"page P(model: M) { Button(on_click: model.save, "Save") }"#).contains("props are named"));
        assert!(err(r#"page P(model: M) { Button(child: Text("a")).disabled(true) }"#).contains("`disabled` is a prop"));
        assert!(err(r#"page P(model: M) { Switch(checked: bind(model.on)) }"#).contains("there is no `bind`"));
        assert!(err(r#"page P(model: M) { Text("a") Text("b") }"#).contains("one root widget"));
        assert!(err(r#"page P(model: M) { Col(child: Text("a"), children: []) }"#).contains("once"));
    }
}
