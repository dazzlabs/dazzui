//! The bridge between a Rust page model and the native side (DESIGN.md §8):
//! UniFFI's.
//!
//! [`Bridge`] describes what crosses — the state's fields and selectors,
//! the structs they hold, the actions — and [`Bridge::rust`] is what the
//! `#[page_model]` macro emits from it: UniFFI records that mirror the state
//! as the page reads it, the action enum, an object that holds the app's
//! model, and a listener trait the native side implements. UniFFI then
//! writes the Swift and C# for those; the backends add only a thin store
//! over them, which hands a page its state and a way to send.
//!
//! The bridge holds no state and runs no logic of its own: whatever the
//! model is — a reducer, a view model over use cases — it is reached only
//! through `PageModel`. The mirrors keep the app free of UniFFI: its own
//! types derive nothing, and the exported ones take their names. The data
//! flow is a whole snapshot each time: the native side sends an action, the
//! model calls its notifier when the state changes, on whatever thread, and
//! the native side reads the state again. A selector is a field of the
//! snapshot, computed when it is taken.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{pascal_case, snake_case};
use crate::rust_types::{
    EnumDef, Fields, FloatType, IntType, ModelDef, RustType, RustTypes, StructDef,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Bridge {
    /// `Settings`, from `SettingsState`: what the bridge's types are named
    /// from.
    pub stem: String,
    /// `settings`: what its hidden module is named from.
    pub prefix: String,
    pub state: String,
    pub action: String,
    /// `crate::action::SettingsAction`
    pub action_path: String,
    /// The `#[page_model]` function that makes the model.
    pub function: String,
    /// Every struct that crosses, each after the structs it holds; the
    /// state is last.
    pub structs: Vec<BridgeStruct>,
    pub variants: Vec<BridgeVariant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeStruct {
    pub name: String,
    /// `crate::state::Plugin`
    pub path: String,
    pub fields: Vec<BridgeField>,
}

/// A field as it crosses.
#[derive(Clone, Debug, PartialEq)]
pub struct BridgeField {
    pub name: String,
    pub ty: FieldType,
    /// Read by calling the selector rather than the field.
    pub selector: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldType {
    Scalar(Scalar),
    /// Another struct, held by value.
    Struct(String),
    /// A `Vec` of structs.
    List(String),
}

/// An action variant, and the one value it carries, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct BridgeVariant {
    pub name: String,
    pub payload: Option<Scalar>,
}

/// The scalar types that cross.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scalar {
    Bool,
    Int(IntType),
    Float(FloatType),
    Str,
}

impl Scalar {
    fn from_rust(ty: &RustType) -> Option<Self> {
        match ty {
            RustType::Bool => Some(Scalar::Bool),
            RustType::String => Some(Scalar::Str),
            RustType::Int(i) => Some(Scalar::Int(*i)),
            RustType::Float(f) => Some(Scalar::Float(*f)),
            RustType::Vec(_) | RustType::Other(_) => None,
        }
    }

    /// The type a mirror holds it in.
    fn rust(self) -> TokenStream {
        match self {
            Scalar::Bool => quote!(bool),
            Scalar::Int(i) => {
                let ident = format_ident!("{}", i.rust());
                quote!(#ident)
            }
            Scalar::Float(f) => {
                let ident = format_ident!("{}", f.rust());
                quote!(#ident)
            }
            Scalar::Str => quote!(::std::string::String),
        }
    }
}

impl Bridge {
    /// The bridge for `model`, checked against the Rust types it names.
    pub fn new(model: &ModelDef, types: &RustTypes) -> Result<Self, String> {
        let (state, action) = types.model_types(model)?;
        let action_def: &EnumDef = types.enums.get(&action).ok_or_else(|| {
            format!(
                "cannot find enum `{action}`, the action `{}`'s model takes",
                model.function
            )
        })?;
        if !types.structs.contains_key(&state) {
            return Err(format!(
                "cannot find struct `{state}`, the state `{}`'s model shows",
                model.function
            ));
        }
        let mut structs = Vec::new();
        collect_struct(&state, types, &mut Vec::new(), &mut structs)?;

        let action = action_def;
        let variants = action
            .variants
            .iter()
            .map(|v| {
                let payload = match &v.fields {
                    Fields::Unit => None,
                    Fields::Tuple(types) if types.len() == 1 => {
                        Some(Scalar::from_rust(&types[0]).ok_or_else(|| {
                            format!(
                                "`{}::{}` carries type `{}`, which cannot cross yet",
                                action.name,
                                v.name,
                                types[0].rust()
                            )
                        })?)
                    }
                    _ => {
                        return Err(format!(
                            "`{}::{}` carries several values; an action carries one or none",
                            action.name, v.name
                        ));
                    }
                };
                Ok(BridgeVariant {
                    name: v.name.clone(),
                    payload,
                })
            })
            .collect::<Result<_, _>>()?;

        let stem = state
            .strip_suffix("State")
            .filter(|s| !s.is_empty())
            .unwrap_or(&state)
            .to_string();
        Ok(Bridge {
            prefix: snake_case(&stem),
            stem,
            state: state.clone(),
            action: action.name.clone(),
            action_path: action.path.clone(),
            function: model.function.clone(),
            structs,
            variants,
        })
    }

    pub fn state_struct(&self) -> &BridgeStruct {
        self.structs.last().expect("the state crosses")
    }

    /// `SettingsSource`: the object the native side holds, over the model.
    pub fn source(&self) -> String {
        format!("{}Source", self.stem)
    }

    /// `SettingsStore`: the native side's observable over the source, which
    /// hands a page its state and `send`.
    pub fn store(&self) -> String {
        format!("{}Store", self.stem)
    }

    /// `SettingsListener`: the trait the native side implements to hear of
    /// a new state.
    pub fn listener(&self) -> String {
        format!("{}Listener", self.stem)
    }

    /// The UniFFI exports, which the `#[page_model]` macro emits beside the
    /// function it marks. The app's types are named by their paths, so it
    /// compiles wherever the function is; the crate's root must hold
    /// `uniffi::setup_scaffolding!()`.
    pub fn rust(&self) -> TokenStream {
        let module = format_ident!("__dazzui_bridge_{}", self.prefix);
        let path = |p: &str| syn::parse_str::<syn::Path>(p).expect("a Rust path");
        let core_state = path(&self.state_struct().path);
        let core_action = path(&self.action_path);
        let function = format_ident!("{}", self.function);
        let state = format_ident!("{}", self.state);
        let action = format_ident!("{}", self.action);
        let source = format_ident!("{}", self.source());
        let listener = format_ident!("{}", self.listener());
        let to_snapshot = format_ident!("mirror_{}", snake_case(&self.state));

        let structs = self.structs.iter().map(|s| self.mirror(s));
        let variants = self.variants.iter().map(|v| {
            let name = format_ident!("{}", v.name);
            match v.payload {
                Some(ty) => {
                    let ty = ty.rust();
                    quote!(#name(#ty))
                }
                None => quote!(#name),
            }
        });
        let arms = self.variants.iter().map(|v| {
            let name = format_ident!("{}", v.name);
            match v.payload {
                Some(_) => quote!(#action::#name(value) => #core_action::#name(value)),
                None => quote!(#action::#name => #core_action::#name),
            }
        });

        quote! {
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub mod #module {
                use ::std::sync::Arc;
                use ::dazzui_runtime::PageModel;

                #(#structs)*

                #[derive(::uniffi::Enum, Clone, Debug, PartialEq)]
                pub enum #action { #(#variants,)* }

                impl From<#action> for #core_action {
                    fn from(action: #action) -> Self {
                        match action { #(#arms,)* }
                    }
                }

                /// Called back when the model's state changes, on whichever
                /// thread changed it.
                #[::uniffi::export(with_foreign)]
                pub trait #listener: Send + Sync {
                    fn state_changed(&self);
                }

                /// The app's model, as the native side holds it.
                #[derive(::uniffi::Object)]
                pub struct #source {
                    model: Box<dyn PageModel<State = #core_state, Action = #core_action>>,
                }

                impl #source {
                    /// A source over a model made in Rust, as a test makes one.
                    pub fn with_model(
                        model: impl PageModel<State = #core_state, Action = #core_action>,
                    ) -> Arc<Self> {
                        Arc::new(Self { model: Box::new(model) })
                    }
                }

                #[::uniffi::export]
                impl #source {
                    /// A source over the model the marked function makes.
                    #[uniffi::constructor]
                    pub fn new() -> Arc<Self> {
                        Self::with_model(super::#function())
                    }

                    /// A snapshot of the state, selectors computed.
                    pub fn state(&self) -> #state {
                        #to_snapshot(&self.model.state())
                    }

                    pub fn send(&self, action: #action) {
                        self.model.send(action.into());
                    }

                    pub fn subscribe(&self, listener: Arc<dyn #listener>) {
                        self.model
                            .notifier()
                            .subscribe(move || listener.state_changed());
                    }
                }
            }
        }
    }

    /// A struct's mirror — its fields, then its selectors — and the
    /// function that fills one from the core's.
    fn mirror(&self, s: &BridgeStruct) -> TokenStream {
        let core = syn::parse_str::<syn::Path>(&s.path).expect("a Rust path");
        let name = format_ident!("{}", s.name);
        let to_mirror = format_ident!("mirror_{}", snake_case(&s.name));
        let fields = s.fields.iter().map(|f| {
            let field = format_ident!("{}", f.name);
            let ty = match &f.ty {
                FieldType::Scalar(scalar) => scalar.rust(),
                FieldType::Struct(t) => {
                    let t = format_ident!("{}", t);
                    quote!(#t)
                }
                FieldType::List(t) => {
                    let t = format_ident!("{}", t);
                    quote!(Vec<#t>)
                }
            };
            quote!(pub #field: #ty)
        });
        let values = s.fields.iter().map(|f| {
            let field = format_ident!("{}", f.name);
            let read = if f.selector {
                quote!(value.#field())
            } else {
                quote!(value.#field)
            };
            match &f.ty {
                FieldType::Scalar(Scalar::Str) => {
                    quote!(#field: ::std::clone::Clone::clone(&#read))
                }
                FieldType::Scalar(_) => quote!(#field: #read),
                FieldType::Struct(t) => {
                    let to = format_ident!("mirror_{}", snake_case(t));
                    quote!(#field: #to(&#read))
                }
                FieldType::List(t) => {
                    let to = format_ident!("mirror_{}", snake_case(t));
                    quote!(#field: #read.iter().map(#to).collect())
                }
            }
        });
        quote! {
            #[derive(::uniffi::Record, Clone, Debug, PartialEq)]
            pub struct #name { #(#fields,)* }

            fn #to_mirror(value: &#core) -> #name {
                #name { #(#values,)* }
            }
        }
    }

    /// The C# property a field is: `launch_at_startup` → `LaunchAtStartup`.
    pub fn property(field: &str) -> String {
        pascal_case(field)
    }
}

/// Adds `name` and every struct it holds to `out`, those it holds first.
/// `path` is the chain being collected, to refuse a struct that holds
/// itself.
fn collect_struct(
    name: &str,
    types: &RustTypes,
    path: &mut Vec<String>,
    out: &mut Vec<BridgeStruct>,
) -> Result<(), String> {
    if out.iter().any(|s| s.name == name) {
        return Ok(());
    }
    if path.iter().any(|p| p == name) {
        return Err(format!(
            "`{name}` holds itself, which cannot cross: {} → {name}",
            path.join(" → ")
        ));
    }
    let def: &StructDef = &types.structs[name];
    if def.path.is_empty() {
        return Err(format!(
            "`{name}` has selectors but no struct the CLI can read"
        ));
    }
    path.push(name.into());
    let mut fields = Vec::new();
    let named = def
        .fields
        .iter()
        .map(|f| (f, false))
        .chain(def.selectors.iter().map(|f| (f, true)));
    for ((field, ty), selector) in named {
        let what = if selector { "selector" } else { "field" };
        let struct_name = |ty: &RustType| match ty {
            RustType::Other(n) if types.structs.contains_key(n) => Some(n.clone()),
            _ => None,
        };
        let ty = if let Some(scalar) = Scalar::from_rust(ty) {
            FieldType::Scalar(scalar)
        } else if let Some(n) = struct_name(ty) {
            collect_struct(&n, types, path, out)?;
            FieldType::Struct(n)
        } else if let RustType::Vec(elem) = ty {
            let n = struct_name(elem).ok_or_else(|| {
                format!("`{name}.{field}`, a {what}, has type `{}`: a Vec crosses only as a Vec of structs", ty.rust())
            })?;
            collect_struct(&n, types, path, out)?;
            FieldType::List(n)
        } else {
            return Err(format!(
                "`{name}.{field}` has type `{}`, which cannot cross to the native side yet",
                ty.rust()
            ));
        };
        fields.push(BridgeField {
            name: field.clone(),
            ty,
            selector,
        });
    }
    path.pop();
    out.push(BridgeStruct {
        name: name.into(),
        path: def.path.clone(),
        fields,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bridge(src: &str) -> Result<Bridge, String> {
        let file: syn::File = syn::parse_str(src).unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items, "crate");
        Bridge::new(&types.models[0].clone(), &types)
    }

    #[test]
    fn mirrors_state_and_actions() {
        let b = bridge(
            "struct NoteState { done: bool, count: u32, title: String }
             enum NoteAction { Toggle, Rename(String), Bump(u32) }
             #[page_model] fn note() -> Reducer<NoteState, NoteAction> { todo!() }",
        )
        .unwrap();
        assert_eq!(
            (
                b.prefix.as_str(),
                b.source().as_str(),
                b.listener().as_str()
            ),
            ("note", "NoteSource", "NoteListener")
        );
        let types: Vec<_> = b
            .state_struct()
            .fields
            .iter()
            .map(|f| f.ty.clone())
            .collect();
        assert_eq!(
            types,
            [
                FieldType::Scalar(Scalar::Bool),
                FieldType::Scalar(Scalar::Int(IntType::U32)),
                FieldType::Scalar(Scalar::Str)
            ]
        );
        assert_eq!(
            b.variants[1],
            BridgeVariant {
                name: "Rename".into(),
                payload: Some(Scalar::Str)
            }
        );
        let rust = b.rust().to_string();
        assert!(rust.contains("uniffi :: Record"), "{rust}");
        assert!(rust.contains("pub enum NoteAction { Toggle , Rename (:: std :: string :: String) , Bump (u32) , }"), "{rust}");
        syn::parse2::<syn::File>(b.rust()).expect("the Rust side parses");
    }

    #[test]
    fn carries_lists_nested_structs_and_selectors() {
        let b = bridge(
            "struct S { items: Vec<Item>, owner: User }
             struct Item { id: u32, tags: Vec<Tag> }
             struct Tag { name: String }
             struct User { name: String }
             impl S { #[selector] fn first(&self) -> Vec<Item> { todo!() } }
             enum A { Go }
             struct M;
             impl PageModel for M { type State = S; type Action = A; }
             #[page_model] fn model() -> M { M }",
        )
        .unwrap();
        let names: Vec<_> = b.structs.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["Tag", "Item", "User", "S"]);
        let first = &b.state_struct().fields[2];
        assert_eq!(
            (first.name.as_str(), &first.ty, first.selector),
            ("first", &FieldType::List("Item".into()), true)
        );
        let rust = b.rust().to_string();
        assert!(
            rust.contains("first : value . first () . iter () . map (mirror_item) . collect ()"),
            "{rust}"
        );
        syn::parse2::<syn::File>(b.rust()).expect("the Rust side parses");
    }

    #[test]
    fn rejects_what_cannot_cross() {
        let err = |src: &str| {
            bridge(&format!(
                "{src} enum A {{ Go }} #[page_model] fn model() -> Reducer<S, A> {{ todo!() }}"
            ))
            .unwrap_err()
        };
        assert_eq!(
            err("struct S { tags: Vec<String> }"),
            "`S.tags`, a field, has type `Vec<String>`: a Vec crosses only as a Vec of structs"
        );
        assert_eq!(
            err("struct S { at: Instant }"),
            "`S.at` has type `Instant`, which cannot cross to the native side yet"
        );
        assert_eq!(
            err("struct S { child: Vec<S> }"),
            "`S` holds itself, which cannot cross: S → S"
        );
    }

    #[test]
    fn finds_a_models_types_from_its_impl() {
        let err = bridge(
            "struct S { on: bool } enum A { Go } struct M;
             #[page_model] fn model() -> M { M }",
        )
        .unwrap_err();
        assert_eq!(
            err,
            "`model` returns `M`, and there is no `impl PageModel for M` naming its `State` and `Action`"
        );
        let b = bridge(
            "struct S { on: bool } enum A { Go } struct M;
             impl dazzui_runtime::PageModel for M { type State = S; type Action = crate::A; }
             #[page_model] fn model() -> M { M }",
        )
        .unwrap();
        assert_eq!((b.state.as_str(), b.action.as_str()), ("S", "A"));
    }
}
