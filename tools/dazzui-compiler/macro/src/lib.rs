//! The `ui!`, `#[page_model]` and `#[selector]` proc macros. An app takes
//! them from `dazzui-runtime`, which re-exports them beside the
//! `PageModel` trait the bridge names.
//!
//! `ui!` parses the page, validates it against the built-in registry, and
//! expands to a function that is never called: a list of `let` bindings
//! that make rustc check every state path the page reads against the
//! page's state struct, and every action it names against its action enum.
//! Each binding carries the DSL's span, so rustc's own error lands on the
//! line that wrote it.
//!
//! `#[page_model]` marks the function that makes a page model and emits,
//! beside it, the UniFFI exports the native side reaches the model through.
//! `#[selector]` marks a method on the state whose value the page reads
//! like a field.
//!
//! Both `ui!` and `#[page_model]` read the crate's `src/` as the CLI does:
//! `#[page_model]` for the types the bridge carries, `ui!` only to know which
//! names in a path are selectors, to be called rather than read.

use std::path::Path;

use dazzui_compiler::bridge::Bridge;
use dazzui_compiler::ir::{ComponentSpec, PropType};
use dazzui_compiler::syntax::{
    ArgAst, ComponentAst, DottedAst, NodeAst, PageAst, ValueAst, parse_page,
};
use dazzui_compiler::{ModelDef, Project, Registry, RustType, RustTypes, lower, read_selector};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::Lit;

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    expand(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand(input: TokenStream2) -> syn::Result<TokenStream2> {
    let page = parse_page(input)?;
    let dir = std::env::var("CARGO_MANIFEST_DIR").ok();
    let registry = match &dir {
        Some(dir) => {
            Registry::load(Path::new(dir)).map_err(|e| syn::Error::new(page.name.span(), e))?
        }
        None => Registry::builtin(),
    };
    // Read so that cargo builds again when the app's components change.
    let track = dir
        .map(|dir| Path::new(&dir).join(dazzui_compiler::REGISTRY_FILE))
        .filter(|p| p.exists())
        .map(|p| {
            let path = p.to_string_lossy().into_owned();
            quote!(
                const _: &[u8] = include_bytes!(#path);
            )
        });
    // The type check goes out with the registry's errors rather than
    // instead of them, so one build reports both kinds.
    let errors = match lower(&page, &registry, None, "") {
        Ok(_) => TokenStream2::new(),
        Err(errors) => errors
            .iter()
            .map(|e| e.to_syn().to_compile_error())
            .collect(),
    };
    let check = type_check(
        &page,
        &registry,
        crate_types().map_err(|e| syn::Error::new(page.name.span(), e))?,
    );
    Ok(quote! { #errors #check #track })
}

fn type_check(page: &PageAst, registry: &Registry, types: Option<RustTypes>) -> TokenStream2 {
    let state = &page.state_ty;
    let check = format_ident!("__dazzui_compiler_check_{}", page.name);
    let state_name = page.state_ty.segments.last().map(|s| s.ident.to_string());
    let mut cx = Check {
        registry,
        action: &page.action_ty,
        types: types.as_ref(),
        state: state_name,
        scopes: Vec::new(),
    };
    let lets = cx.node(&page.root);
    quote! {
        #[doc(hidden)]
        const _: () = {
            #[allow(dead_code, unused_variables, non_snake_case, clippy::all)]
            fn #check(state: &#state) {
                #lets
            }
        };
    }
}

/// The Rust types of the crate being compiled, read from its `src/`; none
/// outside cargo or for a crate with no `src/`. A `src/` that cannot be read
/// is an error: without its types, a selector would be read as a field.
fn crate_types() -> syn::Result<Option<RustTypes>> {
    let Ok(dir) = std::env::var("CARGO_MANIFEST_DIR") else {
        return Ok(None);
    };
    let src = Path::new(&dir).join("src");
    if !src.is_dir() {
        return Ok(None);
    }
    Project::scan(&src, Path::new(&dir))
        .map(|p| Some(p.types))
        .map_err(|e| {
            syn::Error::new(
                Span::call_site(),
                format!("cannot read {}: {e}", src.display()),
            )
        })
}

/// What the type check is walking: the page, and the loop items in scope.
struct Check<'a> {
    registry: &'a Registry,
    action: &'a syn::Path,
    types: Option<&'a RustTypes>,
    /// The state struct's name.
    state: Option<String>,
    /// Each loop item, and the struct its elements are when it is known.
    scopes: Vec<(String, Option<String>)>,
}

impl Check<'_> {
    fn node(&mut self, node: &NodeAst) -> TokenStream2 {
        match node {
            NodeAst::Component(component) => {
                let mut out = Vec::new();
                // An unknown component is the registry's error, already reported.
                if let Some(spec) = self.registry.get(&component.name.to_string()) {
                    self.component(component, spec, &mut out);
                }
                // What a slot holds is checked as a child is.
                let named = component.args.iter().filter_map(|a| match a {
                    ArgAst::Named(p) => Some(&p.value),
                    ArgAst::Positional(_) => None,
                });
                let slots: Vec<NodeAst> = named
                    .chain(component.props.iter().map(|p| &p.value))
                    .filter_map(|v| match v {
                        ValueAst::Component(c) => Some(NodeAst::Component((**c).clone())),
                        _ => None,
                    })
                    .collect();
                let slots = slots.iter().map(|n| self.node(n)).collect::<Vec<_>>();
                let children = component
                    .children
                    .iter()
                    .map(|c| self.node(c))
                    .collect::<Vec<_>>();
                quote!(#(#out)* #(#slots)* #(#children)*)
            }
            NodeAst::If(ast) => {
                let condition = self.access(&ast.condition).map(|(access, _)| {
                    let span = ast.condition.span();
                    quote_spanned! {span=> let _: bool = #access; }
                });
                let then_branch = ast
                    .then_branch
                    .iter()
                    .map(|n| self.node(n))
                    .collect::<Vec<_>>();
                let else_branch = ast
                    .else_branch
                    .iter()
                    .map(|n| self.node(n))
                    .collect::<Vec<_>>();
                quote!(#condition #(#then_branch)* #(#else_branch)*)
            }
            NodeAst::For(ast) => {
                let Some((collection, ty)) = self.access(&ast.collection) else {
                    return TokenStream2::new();
                };
                let element = match ty {
                    Some(RustType::Vec(elem)) => match *elem {
                        RustType::Other(name) => Some(name),
                        _ => None,
                    },
                    _ => None,
                };
                let item = &ast.item;
                self.scopes.push((item.to_string(), element));
                let key = match &ast.key {
                    Some(key) => self.access(key).map(|(access, _)| {
                        let span = key.span();
                        quote_spanned! {span=> let _ = &#access; }
                    }),
                    // Without `key:`, the element's `id` is.
                    None => Some(quote_spanned! {item.span()=> let _ = &#item.id; }),
                };
                let children = ast
                    .children
                    .iter()
                    .map(|n| self.node(n))
                    .collect::<Vec<_>>();
                self.scopes.pop();
                let span = ast.collection.span();
                quote_spanned! {span=>
                    for #item in ::core::iter::IntoIterator::into_iter(&#collection) {
                        #key
                        #(#children)*
                    }
                }
            }
        }
    }

    /// `state.a.b()` for `state.a.b`, with a selector called and a field
    /// read, each segment keeping its own span; and the Rust type it reads,
    /// when the crate's types say. `None` for a path the compiler has
    /// already reported.
    fn access(&self, path: &DottedAst) -> Option<(TokenStream2, Option<RustType>)> {
        let (head, rest) = path.segments.split_first()?;
        if rest.is_empty() {
            return None;
        }
        let mut current = if head == "state" {
            self.state.clone()
        } else {
            self.scopes
                .iter()
                .rev()
                .find(|(name, _)| head == name)?
                .1
                .clone()
        };
        let mut tokens = quote!(#head);
        let mut ty = None;
        for segment in rest {
            let def = current
                .as_ref()
                .and_then(|name| self.types?.structs.get(name));
            if def.is_some_and(|d| d.is_selector(&segment.to_string())) {
                tokens = quote_spanned! {segment.span()=> #tokens.#segment() };
            } else {
                tokens = quote!(#tokens.#segment);
            }
            ty = def.and_then(|d| d.field(&segment.to_string())).cloned();
            current = match &ty {
                Some(RustType::Other(name)) => Some(name.clone()),
                _ => None,
            };
        }
        Some((tokens, ty))
    }

    /// One `let` per path and action the component names.
    fn component(
        &self,
        component: &ComponentAst,
        spec: &ComponentSpec,
        out: &mut Vec<TokenStream2>,
    ) {
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
        entries.extend(
            component
                .props
                .iter()
                .map(|p| (p.name.to_string(), &p.value)),
        );
        for modifier in &component.modifiers {
            if let [ArgAst::Positional(value)] = modifier.args.as_slice() {
                entries.push((modifier.name.to_string(), value));
            }
        }

        for (name, value) in entries {
            if let Some(event) = spec.event_by_modifier(&name) {
                out.extend(self.action_check(value, event.payload.as_ref()));
            } else if let Some(prop) = spec.prop(&name)
                && let ValueAst::Dotted(path) | ValueAst::Bind(path) = value
            {
                out.extend(self.value_check(path, &prop.ty));
            }
        }
    }

    /// `let _: bool = state.saving;` — or `&str` for a string, which a
    /// `String` field coerces to.
    fn value_check(&self, path: &DottedAst, ty: &PropType) -> Option<TokenStream2> {
        let (access, _) = self.access(path)?;
        let span = path.span();
        match ty.rust_type()? {
            "&str" => Some(quote_spanned! {span=> let _: &str = &#access; }),
            "bool" => Some(quote_spanned! {span=> let _: bool = #access; }),
            _ => None,
        }
    }

    /// The check for an action an event sends, with `Action::` rewritten to
    /// the page's action type and re-spanned onto the variant, so rustc
    /// underlines `Action::Sav` and nothing else.
    fn action_check(&self, value: &ValueAst, carried: Option<&PropType>) -> Option<TokenStream2> {
        let action = self.action;
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
                let value = self.payload_value(payload)?;
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

    /// A payload as Rust would build the action with it: an owned clone of
    /// a field, or a literal.
    fn payload_value(&self, value: &ValueAst) -> Option<TokenStream2> {
        match value {
            ValueAst::Dotted(path) => {
                let (access, _) = self.access(path)?;
                let span = path.span();
                // Bound first, so its type is the field's and a mismatch reads
                // as one, not as an argument to `clone`.
                Some(
                    quote_spanned! {span=> { let value = ::core::clone::Clone::clone(&#access); value } },
                )
            }
            ValueAst::Lit(Lit::Str(s)) => {
                Some(quote_spanned! {s.span()=> ::core::convert::Into::into(#s) })
            }
            ValueAst::Lit(lit) => Some(lit.to_token_stream()),
            ValueAst::NegInt(lit) => Some(quote!(-#lit)),
            _ => None,
        }
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

/// Marks `fn name() -> Model`, where `Model: PageModel`, as what drives the
/// pages of the model's state, and emits the UniFFI exports the native side
/// reaches the model through: the native side makes one by calling it.
///
/// The macro reads the crate's `src/` for the state struct and action enum,
/// as the CLI does, so both derive the ABI from the same Rust types: from
/// `Reducer<State, Action>`, or from the `impl PageModel` of the type the
/// function returns.
#[proc_macro_attribute]
pub fn page_model(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let result = (|| {
        if !attr.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[page_model] takes no arguments",
            ));
        }
        let function: syn::ItemFn = syn::parse2(item.clone())?;
        let model = ModelDef::from_fn(&function.sig)?;
        let at = |message: String| syn::Error::new(function.sig.ident.span(), message);
        let dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| at("CARGO_MANIFEST_DIR is not set".into()))?;
        let src = Path::new(&dir).join("src");
        let project = Project::scan(&src, Path::new(&dir))
            .map_err(|e| at(format!("cannot read {}: {e}", src.display())))?;
        if !project.types.scaffolding {
            return Err(at(
                "the bridge is UniFFI's, which needs `uniffi::setup_scaffolding!();` at the crate's root, in src/lib.rs"
                    .into(),
            ));
        }
        let bridge = Bridge::new(&model, &project.types).map_err(at)?;
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

/// Marks `fn name(&self) -> T` on the state, or on a struct in it, as a
/// value the page reads like a field: `state.visible_items`. It is computed
/// in Rust each time a snapshot is taken. The method itself is unchanged.
#[proc_macro_attribute]
pub fn selector(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let check = (|| {
        if !attr.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[selector] takes no arguments",
            ));
        }
        let method: syn::ImplItemFn = syn::parse2(item.clone())?;
        read_selector(&method.sig).map(|_| ())
    })();
    match check {
        Ok(()) => item,
        Err(error) => {
            let error = error.to_compile_error();
            quote!(#item #error)
        }
    }
    .into()
}
