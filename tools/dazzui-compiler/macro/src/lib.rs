//! The `ui!` and `#[app]` proc macros.
//!
//! `ui!` parses the page, validates it against the built-in registry, and
//! expands to a function that is never called: a list of `let` bindings
//! that make rustc check every state path the page reads against the
//! page's state struct, and every action it names against its action enum.
//! Each binding carries the DSL's span, so rustc's own error lands on the
//! line that wrote it.
//!
//! `#[app]` marks the update function and emits, beside it, the `extern "C"`
//! side of the bridge the native page talks to it through.

use std::path::Path;

use dazzui_compiler::bridge::Bridge;
use dazzui_compiler::ir::{ComponentSpec, PropType};
use dazzui_compiler::syntax::{ArgAst, ComponentAst, DottedAst, NodeAst, PageAst, ValueAst, parse_page};
use dazzui_compiler::{AppDef, Project, Registry, lower};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::Lit;

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    expand(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn expand(input: TokenStream2) -> syn::Result<TokenStream2> {
    let page = parse_page(input)?;
    let registry = Registry::builtin();
    // The type check goes out with the registry's errors rather than
    // instead of them, so one build reports both kinds.
    let errors = match lower(&page, &registry, None, "") {
        Ok(_) => TokenStream2::new(),
        Err(errors) => errors.iter().map(|e| e.to_syn().to_compile_error()).collect(),
    };
    let check = type_check(&page, &registry);
    Ok(quote! { #errors #check })
}

fn type_check(page: &PageAst, registry: &Registry) -> TokenStream2 {
    let state = &page.state_ty;
    let check = format_ident!("__dazzui_compiler_check_{}", page.name);
    let mut lets = Vec::new();
    collect(&page.root, &page.action_ty, registry, &mut lets);
    quote! {
        #[doc(hidden)]
        const _: () = {
            #[allow(dead_code, unused_variables, non_snake_case, clippy::all)]
            fn #check(state: &#state) {
                #(#lets)*
            }
        };
    }
}

fn collect(node: &NodeAst, action: &syn::Path, registry: &Registry, out: &mut Vec<TokenStream2>) {
    let NodeAst::Component(component) = node;
    // An unknown component is the registry's error, already reported.
    if let Some(spec) = registry.get(&component.name.to_string()) {
        check_component(component, spec, action, out);
    }
    for child in &component.children {
        collect(child, action, registry, out);
    }
}

/// One `let` per state path and action the component names.
fn check_component(component: &ComponentAst, spec: &ComponentSpec, action: &syn::Path, out: &mut Vec<TokenStream2>) {
    let mut entries: Vec<(String, &ValueAst)> = Vec::new();
    let mut positional = spec.positional.iter();
    for arg in &component.args {
        match arg {
            ArgAst::Named(prop) => entries.push((prop.name.to_string(), &prop.value)),
            ArgAst::Positional(value) => {
                if let Some(name) = positional.next() {
                    entries.push((name.clone(), value));
                }
            }
        }
    }
    entries.extend(component.props.iter().map(|p| (p.name.to_string(), &p.value)));
    for modifier in &component.modifiers {
        if let [ArgAst::Positional(value)] = modifier.args.as_slice() {
            entries.push((modifier.name.to_string(), value));
        }
    }

    for (name, value) in entries {
        if let Some(event) = spec.event_by_modifier(&name) {
            out.extend(action_check(value, action, event.payload.as_ref()));
        } else if let Some(prop) = spec.prop(&name)
            && let ValueAst::Dotted(path) | ValueAst::Bind(path) = value
        {
            out.extend(state_check(path, &prop.ty));
        }
    }
}

/// `let _: bool = state.saving;` — or `&str` for a string, which a
/// `String` field coerces to.
fn state_check(path: &DottedAst, ty: &PropType) -> Option<TokenStream2> {
    if path.segments.first()? != "state" || path.segments.len() < 2 {
        return None;
    }
    let span = path.span();
    let access = access(path);
    match ty.rust_type()? {
        "&str" => Some(quote_spanned! {span=> let _: &str = &#access; }),
        "bool" => Some(quote_spanned! {span=> let _: bool = #access; }),
        _ => None,
    }
}

/// `state.a.b`, each segment keeping its own span.
fn access(path: &DottedAst) -> TokenStream2 {
    let segments = &path.segments;
    quote!(#(#segments).*)
}

/// The check for an action an event sends, with `Action::` rewritten to
/// the page's action type and re-spanned onto the variant, so rustc
/// underlines `Action::Sav` and nothing else.
fn action_check(value: &ValueAst, action: &syn::Path, carried: Option<&PropType>) -> Option<TokenStream2> {
    let (path, payload) = match value {
        ValueAst::Path(path) => (path, None),
        ValueAst::Call(path, payload) => (path, Some(&**payload)),
        _ => return None,
    };
    if path.segments.first()?.ident != "Action" || path.segments.len() != 2 {
        return None;
    }
    let variant = &path.segments[1].ident;
    let span = variant.span();
    let ty = respan(action, span);
    Some(match (payload, carried) {
        (Some(payload), _) => {
            let value = payload_value(payload)?;
            quote_spanned! {span=> let _: #ty = #ty::#variant(#value); }
        }
        (None, Some(carried)) => {
            let carried: TokenStream2 = match carried {
                PropType::Bool => quote!(bool),
                PropType::String => quote!(::std::string::String),
                PropType::Int => quote!(i64),
                PropType::Enum(_) => return None,
            };
            quote_spanned! {span=> let _: fn(#carried) -> #ty = #ty::#variant; }
        }
        (None, None) => quote_spanned! {span=> let _: #ty = #ty::#variant; },
    })
}

/// A payload as Rust would build the action with it: an owned clone of a
/// state field, or a literal.
fn payload_value(value: &ValueAst) -> Option<TokenStream2> {
    match value {
        ValueAst::Dotted(path) if path.segments.first()? == "state" => {
            let access = access(path);
            let span = path.span();
            Some(quote_spanned! {span=> ::core::clone::Clone::clone(&#access) })
        }
        ValueAst::Lit(Lit::Str(s)) => Some(quote_spanned! {s.span()=> ::core::convert::Into::into(#s) }),
        ValueAst::Lit(lit) => Some(lit.to_token_stream()),
        ValueAst::NegInt(lit) => Some(quote!(-#lit)),
        _ => None,
    }
}

fn respan(path: &syn::Path, span: Span) -> TokenStream2 {
    path.to_token_stream()
        .into_iter()
        .map(|mut tt| {
            tt.set_span(span);
            tt
        })
        .collect()
}

/// Marks `fn update(state: &mut State, action: Action)` as the app's update
/// function and emits the C ABI the native page reaches it through.
///
/// The macro reads the crate's `src/` for the state struct and action enum,
/// as the CLI does, so both derive the ABI from the same Rust types.
#[proc_macro_attribute]
pub fn app(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let result = (|| {
        if !attr.is_empty() {
            return Err(syn::Error::new(Span::call_site(), "#[app] takes no arguments"));
        }
        let function: syn::ItemFn = syn::parse2(item.clone())?;
        let app = AppDef::from_fn(&function.sig)?;
        let at = |message: String| syn::Error::new(function.sig.ident.span(), message);
        let dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| at("CARGO_MANIFEST_DIR is not set".into()))?;
        let src = Path::new(&dir).join("src");
        let project = Project::scan(&src, Path::new(&dir)).map_err(|e| at(format!("cannot read {}: {e}", src.display())))?;
        let bridge = Bridge::new(&app, &project.types).map_err(at)?;
        Ok(bridge.rust())
    })();
    match result {
        Ok(bridge) => quote!(#item #bridge),
        Err(error) => {
            let error = error.to_compile_error();
            quote!(#item #error)
        }
    }
    .into()
}
