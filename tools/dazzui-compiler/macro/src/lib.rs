//! The `ui!` proc macro.
//!
//! It parses the page, validates it against the built-in registry, and
//! expands to a function that is never called: a list of `let` bindings
//! that make rustc check every action the page names against the page's
//! action enum. Each binding carries the DSL's span, so rustc's own error
//! lands on the line that wrote it.

use dazzui_compiler::{Registry, lower};
use dazzui_compiler::syntax::{ArgAst, ComponentAst, NodeAst, PageAst, ValueAst, parse_page};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, quote_spanned};

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    expand(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn expand(input: TokenStream2) -> syn::Result<TokenStream2> {
    let page = parse_page(input)?;
    // The type check goes out with the registry's errors rather than
    // instead of them, so one build reports both kinds.
    let errors = match lower(&page, &Registry::builtin(), None, "") {
        Ok(_) => TokenStream2::new(),
        Err(errors) => errors.iter().map(|e| e.to_syn().to_compile_error()).collect(),
    };
    let check = type_check(&page);
    Ok(quote! { #errors #check })
}

fn type_check(page: &PageAst) -> TokenStream2 {
    let state = &page.state_ty;
    let action = &page.action_ty;
    let check = format_ident!("__dazzui_compiler_check_{}", page.name);
    let mut lets = Vec::new();
    collect(&page.root, action, &mut lets);
    quote! {
        #[doc(hidden)]
        const _: () = {
            #[allow(dead_code, unused_variables, non_snake_case)]
            fn #check(state: &#state) {
                #(#lets)*
            }
        };
    }
}

fn collect(node: &NodeAst, action: &syn::Path, out: &mut Vec<TokenStream2>) {
    let NodeAst::Component(component) = node;
    actions(component, action, out);
    for child in &component.children {
        collect(child, action, out);
    }
}

/// `let _: Action = Action::Variant;` for each `.on_*(Action::Variant)`,
/// with `Action::` rewritten to the page's action type.
fn actions(component: &ComponentAst, action: &syn::Path, out: &mut Vec<TokenStream2>) {
    for modifier in &component.modifiers {
        for arg in &modifier.args {
            if let ArgAst::Positional(ValueAst::Path(path)) = arg
                && path.segments.first().is_some_and(|s| s.ident == "Action")
                && let Some(variant) = path.segments.iter().nth(1) {
                    let ident = &variant.ident;
                    // The page's action type, re-spanned onto the variant, so
                    // rustc underlines `Action::Sav` and nothing else.
                    let ty = respan(action, ident.span());
                    out.push(quote_spanned! {ident.span()=>
                        let _: #ty = #ty::#ident;
                    });
                }
        }
    }
}

fn respan(path: &syn::Path, span: proc_macro2::Span) -> TokenStream2 {
    quote::ToTokens::to_token_stream(path)
        .into_iter()
        .map(|mut tt| {
            tt.set_span(span);
            tt
        })
        .collect()
}
