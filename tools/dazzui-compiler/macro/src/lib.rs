//! The `ui!`, `#[page_model]` and `#[selector]` proc macros. An app takes
//! them from `dazzui-runtime`, which re-exports them beside the
//! `PageModel` trait the bridge names.
//!
//! `ui!` parses the page, validates it against the built-in registry, and
//! expands to a function that is never called: a list of `let` bindings
//! that make rustc check every state path the page reads against the
//! page's state struct, and every model method it calls against the
//! model's impl. Each binding carries the DSL's span, so rustc's own error
//! lands on the line that wrote it.
//!
//! `#[page_model]` marks a model's impl, whose public methods are what a
//! page calls, and emits, beside it, the UniFFI exports the native side
//! reaches the model through.
//! `#[selector]` marks a method on the state whose value the page reads
//! like a field.
//!
//! Both `ui!` and `#[page_model]` read the crate's `src/` as the CLI does:
//! `#[page_model]` for the types the bridge carries, `ui!` only to know which
//! names in a path are selectors, to be called rather than read.

use std::path::Path;

use dazzui_compiler::bridge::Bridge;
use dazzui_compiler::ir::{ComponentSpec, PropType};
use dazzui_compiler::syntax::{ComponentAst, DottedAst, NodeAst, PageAst, ValueAst, parse_page};
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
    let model = &page.model_ty;
    let check = format_ident!("__dazzui_compiler_check_{}", page.name);
    // The value's struct, to know which names in a path are selectors.
    let model_name = page.model_ty.segments.last().map(|s| s.ident.to_string());
    let state_name = types.as_ref().and_then(|t| {
        let def = t.model(model_name.as_deref()?)?;
        t.model_state(def).ok()
    });
    let mut cx = Check {
        registry,
        model: &page.model_ty,
        types: types.as_ref(),
        state: state_name,
        scopes: Vec::new(),
    };
    let lets = cx.node(&page.root);
    quote! {
        #[doc(hidden)]
        const _: () = {
            #[allow(dead_code, unused_variables, non_snake_case, clippy::all)]
            fn #check(model: &#model) {
                // What `model.field` reads: the model's value.
                let state = <#model as ::dazzui_runtime::ValueListenable>::value(model);
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
    model: &'a syn::Path,
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
                // A component given to a prop is checked as a child is.
                let slots: Vec<NodeAst> = component
                    .props
                    .iter()
                    .filter_map(|p| match &p.value {
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
        let mut current = if head == "model" {
            self.state.clone()
        } else {
            self.scopes
                .iter()
                .rev()
                .find(|(name, _)| head == name)?
                .1
                .clone()
        };
        // `model.field` reads the value, bound to `state` in the check.
        let mut tokens = if head == "model" {
            let state = proc_macro2::Ident::new("state", head.span());
            quote!(#state)
        } else {
            quote!(#head)
        };
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

    /// One `let` per path and method the component names.
    fn component(
        &self,
        component: &ComponentAst,
        spec: &ComponentSpec,
        out: &mut Vec<TokenStream2>,
    ) {
        if let Some(ValueAst::Dotted(path)) = &component.text
            && let Some((access, _)) = self.access(path)
        {
            let span = path.span();
            out.push(quote_spanned! {span=> let _: &str = &#access; });
        }
        for prop in &component.props {
            let name = prop.name.to_string();
            if let Some(event) = spec.event(&name) {
                out.extend(self.call_check(&prop.value, event.payload.as_ref()));
            } else if let Some(spec) = spec.prop(&name)
                && let ValueAst::Dotted(path) = &prop.value
            {
                out.extend(self.value_check(path, &spec.ty));
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

    /// The check for a method an event calls, `model.set_on` or
    /// `model.pick(item.id)`, re-spanned onto the method's name, so rustc
    /// underlines `sav` in `model.sav` and nothing else.
    fn call_check(&self, value: &ValueAst, carried: Option<&PropType>) -> Option<TokenStream2> {
        let (path, argument) = match value {
            ValueAst::Dotted(path) => (path, None),
            ValueAst::Call(path, argument) => (path, Some(&**argument)),
            _ => return None,
        };
        let [head, method] = path.segments.as_slice() else {
            return None;
        };
        if head != "model" {
            return None;
        }
        let span = method.span();
        let ty = respan(self.model, span);
        Some(match (argument, carried) {
            (Some(argument), _) => {
                let value = self.payload_value(argument)?;
                quote_spanned! {span=> let _: () = #ty::#method(model, #value); }
            }
            (None, Some(carried)) => {
                let carried: TokenStream2 = match carried {
                    PropType::Bool => quote!(bool),
                    PropType::String => quote!(::std::string::String),
                    PropType::Int => quote!(i64),
                    PropType::Enum(_) | PropType::Spacing => return None,
                };
                quote_spanned! {span=> let _: fn(&#ty, #carried) = #ty::#method; }
            }
            (None, None) => quote_spanned! {span=> let _: fn(&#ty) = #ty::#method; },
        })
    }

    /// An argument as Rust would pass it: an owned clone of a field, or a
    /// literal.
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

/// Marks a model's impl: its public methods are what a page calls, and its
/// `pub fn new() -> Self` what the native side makes it with. Emits the
/// UniFFI exports the native side reaches the model through.
///
/// The macro reads the crate's `src/` for the model's `ValueListenable`
/// impl and the state struct it names, as the CLI does, so both derive the
/// ABI from the same Rust types.
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
        let imp: syn::ItemImpl = syn::parse2(item.clone()).map_err(|e| {
            syn::Error::new(
                e.span(),
                "#[page_model] marks a model's impl: `#[page_model] impl SettingsModel { … }`",
            )
        })?;
        let model = ModelDef::from_impl(&imp, "crate")?;
        let at = |message: String| syn::Error::new_spanned(&imp.self_ty, message);
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
        // The scan knows where the impl's type is; this one does not.
        let model = project.types.model(&model.name).cloned().unwrap_or(model);
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
