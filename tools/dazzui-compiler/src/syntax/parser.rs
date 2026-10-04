use syn::parse::{Parse, ParseStream};
use syn::{Ident, Lit, Path, Result, Token, braced, parenthesized};

use super::ast::*;

impl Parse for PageAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        if keyword != "page" {
            return Err(syn::Error::new(
                keyword.span(),
                "expected `page`: a ui! block declares `page Name(state: S, action: A) { … }`",
            ));
        }
        let name: Ident = input.parse()?;

        let header;
        parenthesized!(header in input);
        let state_ty = typed_param(&header, "state")?;
        header.parse::<Token![,]>()?;
        let action_ty = typed_param(&header, "action")?;
        if header.peek(Token![,]) {
            header.parse::<Token![,]>()?;
        }
        if !header.is_empty() {
            return Err(header.error("a page takes exactly `state` and `action`"));
        }

        let body;
        braced!(body in input);
        let root: NodeAst = body.parse()?;
        if !body.is_empty() {
            return Err(body.error("a page has one root node; wrap several in a VStack"));
        }
        if !input.is_empty() {
            return Err(input.error("one page per ui! block"));
        }
        Ok(PageAst { name, state_ty, action_ty, root })
    }
}

fn typed_param(input: ParseStream, expected: &str) -> Result<Path> {
    let name: Ident = input.parse()?;
    if name != expected {
        return Err(syn::Error::new(name.span(), format!("expected `{expected}`")));
    }
    input.parse::<Token![:]>()?;
    input.parse()
}

impl Parse for NodeAst {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) || input.peek(Token![for]) {
            return Err(input.error("`if` and `for` arrive in Phase 3"));
        }
        Ok(NodeAst::Component(input.parse()?))
    }
}

impl Parse for ComponentAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let mut args = Vec::new();
        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            args = parse_args(&content)?;
        }

        let mut props = Vec::new();
        let mut children = Vec::new();
        if input.peek(syn::token::Brace) {
            let body;
            braced!(body in input);
            while !body.is_empty() {
                if starts_named(&body) {
                    props.push(body.parse()?);
                    // A comma closes a prop; a line break before children may too.
                    if body.peek(Token![,]) {
                        body.parse::<Token![,]>()?;
                    }
                } else {
                    children.push(body.parse()?);
                }
            }
        }

        let mut modifiers = Vec::new();
        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let name: Ident = input.parse()?;
            let content;
            parenthesized!(content in input);
            modifiers.push(ModifierAst { name, args: parse_args(&content)? });
        }

        Ok(ComponentAst { name, args, props, children, modifiers })
    }
}

fn parse_args(input: ParseStream) -> Result<Vec<ArgAst>> {
    let mut args = Vec::new();
    while !input.is_empty() {
        if starts_named(input) {
            args.push(ArgAst::Named(input.parse()?));
        } else {
            args.push(ArgAst::Positional(input.parse()?));
        }
        if input.is_empty() {
            break;
        }
        input.parse::<Token![,]>()?;
    }
    Ok(args)
}

/// `ident :` and not `ident ::` — a named prop rather than a node or a path.
fn starts_named(input: ParseStream) -> bool {
    let fork = input.fork();
    fork.parse::<Ident>().is_ok() && fork.peek(Token![:]) && !fork.peek(Token![::])
}

impl Parse for PropAst {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        Ok(PropAst { name, value: input.parse()? })
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
                _ => Err(syn::Error::new(lit.span(), "only an integer can be negated")),
            };
        }
        let path: Path = input.parse()?;
        if path.segments.len() == 1 && input.peek(syn::token::Brace) {
            return Err(input.error("a component as a prop value is a named slot, which arrives in Phase 4"));
        }
        if input.peek(syn::token::Paren) {
            return Err(input.error("an action payload arrives in Phase 2"));
        }
        Ok(ValueAst::Path(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Result<PageAst> {
        syn::parse_str(src)
    }

    #[test]
    fn parses_a_page() {
        let page = parse(
            r#"page SettingsPage(state: SettingsState, action: SettingsAction) {
                VStack(spacing: 16) {
                    Text("General")
                    Button("Save").on_click(Action::Save)
                }
            }"#,
        )
        .unwrap();
        assert_eq!(page.name, "SettingsPage");
        let NodeAst::Component(stack) = page.root;
        assert_eq!(stack.name, "VStack");
        assert!(matches!(&stack.args[0], ArgAst::Named(p) if p.name == "spacing"));
        assert_eq!(stack.children.len(), 2);
        let NodeAst::Component(button) = &stack.children[1];
        assert_eq!(button.modifiers[0].name, "on_click");
        assert!(matches!(&button.modifiers[0].args[0], ArgAst::Positional(ValueAst::Path(_))));
    }

    #[test]
    fn tells_props_from_children() {
        let page = parse(
            r#"page P(state: S, action: A) {
                Button {
                    title: "Save",
                    disabled: false
                }
            }"#,
        )
        .unwrap();
        let NodeAst::Component(button) = page.root;
        assert_eq!(button.props.len(), 2);
        assert!(button.children.is_empty());
    }

    #[test]
    fn rejects_two_roots() {
        let err = parse(r#"page P(state: S, action: A) { Text("a") Text("b") }"#).unwrap_err();
        assert!(err.to_string().contains("one root node"));
    }
}
