//! The bridge between a Rust page model and the native side (DESIGN.md §8):
//! UniFFI's.
//!
//! [`Bridge`] describes what crosses — the state's fields and selectors,
//! the structs they hold, the model's methods — and [`Bridge::rust`] is what
//! the `#[page_model]` macro emits from it: UniFFI records that mirror the
//! state as the page reads it, an object of the model's own name that holds
//! the model and forwards its methods, and a listener trait the native side
//! implements. UniFFI then writes the Swift and C# for those; the backends
//! add only a thin store over them, and a protocol of the methods, which is
//! what a page calls.
//!
//! The bridge holds no state and runs no logic of its own: it reaches the
//! model through its methods and `ValueListenable`, as a Flutter widget
//! reaches its model. The mirrors keep the app free of UniFFI: its own types
//! derive nothing, and the exported ones take their names. The data flow is
//! a whole snapshot each time: the native side calls a method, the model
//! notifies its listeners when the state changes, on whatever thread, and
//! the native side reads the value again. A selector is a field of the
//! snapshot, computed when it is taken.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{model_stem, pascal_case, snake_case};
use crate::rust_types::{FloatType, IntType, ModelDef, RustType, RustTypes, StructDef};

#[derive(Clone, Debug, PartialEq)]
pub struct Bridge {
    /// `SettingsModel`: the model's type, and the name of the native object
    /// over it.
    pub model: String,
    /// `crate::model::SettingsModel`
    pub model_path: String,
    /// `Settings`, from `SettingsModel`: what the bridge's other types are
    /// named from.
    pub stem: String,
    /// `settings`: what its hidden module is named from.
    pub prefix: String,
    pub state: String,
    /// Every struct that crosses, each after the structs it holds; the
    /// state is last.
    pub structs: Vec<BridgeStruct>,
    pub methods: Vec<BridgeMethod>,
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

/// A model method a page may call, and its one parameter, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct BridgeMethod {
    pub name: String,
    pub param: Option<(String, Scalar)>,
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

    /// The Swift type UniFFI gives it.
    pub fn swift(self) -> &'static str {
        match self {
            Scalar::Bool => "Bool",
            Scalar::Str => "String",
            Scalar::Float(FloatType::F32) => "Float",
            Scalar::Float(FloatType::F64) => "Double",
            Scalar::Int(i) => match i {
                IntType::I8 => "Int8",
                IntType::I16 => "Int16",
                IntType::I32 => "Int32",
                IntType::I64 | IntType::Isize => "Int64",
                IntType::U8 => "UInt8",
                IntType::U16 => "UInt16",
                IntType::U32 => "UInt32",
                IntType::U64 | IntType::Usize => "UInt64",
            },
        }
    }

    /// The C# type uniffi-bindgen-cs gives it.
    pub fn csharp(self) -> &'static str {
        match self {
            Scalar::Bool => "bool",
            Scalar::Str => "string",
            Scalar::Float(FloatType::F32) => "float",
            Scalar::Float(FloatType::F64) => "double",
            Scalar::Int(i) => match i {
                IntType::I8 => "sbyte",
                IntType::I16 => "short",
                IntType::I32 => "int",
                IntType::I64 | IntType::Isize => "long",
                IntType::U8 => "byte",
                IntType::U16 => "ushort",
                IntType::U32 => "uint",
                IntType::U64 | IntType::Usize => "ulong",
            },
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
        let state = types.model_state(model)?;
        if !types.structs.contains_key(&state) {
            return Err(format!(
                "cannot find struct `{state}`, the value of `{}`",
                model.name
            ));
        }
        let mut structs = Vec::new();
        collect_struct(&state, types, &mut Vec::new(), &mut structs)?;
        let methods = model
            .methods
            .iter()
            .map(|m| {
                let param = match &m.param {
                    None => None,
                    Some((name, ty)) => Some((
                        name.clone(),
                        Scalar::from_rust(ty).ok_or_else(|| {
                            format!(
                                "`{}::{}` takes `{}`, a type that cannot cross yet",
                                model.name,
                                m.name,
                                ty.rust()
                            )
                        })?,
                    )),
                };
                Ok(BridgeMethod {
                    name: m.name.clone(),
                    param,
                })
            })
            .collect::<Result<_, String>>()?;
        let stem = model_stem(&model.name).to_string();
        Ok(Bridge {
            prefix: snake_case(&stem),
            stem,
            model: model.name.clone(),
            model_path: model.path.clone(),
            state,
            structs,
            methods,
        })
    }

    pub fn state_struct(&self) -> &BridgeStruct {
        self.structs.last().expect("the state crosses")
    }

    /// `SettingsActions`: the protocol of the model's methods, which is
    /// what a page calls.
    pub fn actions(&self) -> String {
        format!("{}Actions", self.stem)
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
    /// impl it marks. The app's types are named by their paths, so it
    /// compiles wherever the impl is; the crate's root must hold
    /// `uniffi::setup_scaffolding!()`.
    pub fn rust(&self) -> TokenStream {
        let module = format_ident!("__dazzui_bridge_{}", self.prefix);
        let path = |p: &str| syn::parse_str::<syn::Path>(p).expect("a Rust path");
        let core_model = path(&self.model_path);
        let model = format_ident!("{}", self.model);
        let state = format_ident!("{}", self.state);
        let listener = format_ident!("{}", self.listener());
        let to_snapshot = format_ident!("mirror_{}", snake_case(&self.state));

        let structs = self.structs.iter().map(|s| self.mirror(s));
        let methods = self.methods.iter().map(|m| {
            let name = format_ident!("{}", m.name);
            match &m.param {
                Some((param, ty)) => {
                    let param = format_ident!("{}", param);
                    let ty = ty.rust();
                    quote! {
                        pub fn #name(&self, #param: #ty) {
                            self.model.#name(#param);
                        }
                    }
                }
                None => quote! {
                    pub fn #name(&self) {
                        self.model.#name();
                    }
                },
            }
        });

        quote! {
            #[doc(hidden)]
            #[allow(clippy::all)]
            pub mod #module {
                use ::std::sync::Arc;
                use ::dazzui_runtime::{ListenerId, ValueListenable};

                #(#structs)*

                /// Called back when the model's value changes, on whichever
                /// thread changed it.
                #[::uniffi::export(with_foreign)]
                pub trait #listener: Send + Sync {
                    fn changed(&self);
                }

                /// The app's model, as the native side holds it.
                #[derive(::uniffi::Object)]
                pub struct #model {
                    model: #core_model,
                }

                impl #model {
                    /// The native object over a model made in Rust, as a test
                    /// makes one.
                    pub fn with_model(model: #core_model) -> Arc<Self> {
                        Arc::new(Self { model })
                    }
                }

                #[::uniffi::export]
                impl #model {
                    #[uniffi::constructor]
                    pub fn new() -> Arc<Self> {
                        Self::with_model(#core_model::new())
                    }

                    /// A snapshot of the value, selectors computed.
                    pub fn value(&self) -> #state {
                        #to_snapshot(&ValueListenable::value(&self.model))
                    }

                    pub fn add_listener(&self, listener: Arc<dyn #listener>) -> u64 {
                        ValueListenable::notifier(&self.model)
                            .add_listener(move || listener.changed())
                            .0
                    }

                    pub fn remove_listener(&self, id: u64) {
                        ValueListenable::notifier(&self.model).remove_listener(ListenerId(id));
                    }

                    #(#methods)*
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

    /// A model over `state`, with `methods`.
    fn model(state: &str, methods: &str) -> String {
        format!(
            "struct M;
             impl ValueListenable for M {{ type Value = {state}; }}
             #[page_model] impl M {{ pub fn new() -> Self {{ M }} {methods} }}"
        )
    }

    #[test]
    fn mirrors_the_state_and_forwards_the_methods() {
        let b = bridge(&format!(
            "struct NoteState {{ done: bool, count: u32, title: String }} {}",
            model(
                "NoteState",
                "pub fn toggle(&self) {} pub fn rename(&self, title: String) {} fn helper(&self) -> u8 { 0 }"
            )
        ))
        .unwrap();
        assert_eq!(
            (b.model.as_str(), b.stem.as_str(), b.listener().as_str()),
            ("M", "M", "MListener")
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
        // A private method is not the page's to call.
        assert_eq!(
            b.methods,
            [
                BridgeMethod {
                    name: "toggle".into(),
                    param: None
                },
                BridgeMethod {
                    name: "rename".into(),
                    param: Some(("title".into(), Scalar::Str))
                },
            ]
        );
        let rust = b.rust().to_string();
        assert!(rust.contains("uniffi :: Record"), "{rust}");
        assert!(
            rust.contains("pub fn rename (& self , title : :: std :: string :: String) { self . model . rename (title) ; }"),
            "{rust}"
        );
        syn::parse2::<syn::File>(b.rust()).expect("the Rust side parses");
    }

    #[test]
    fn names_the_bridge_from_the_model() {
        let b = bridge(
            &format!(
                "struct SettingsState {{ on: bool }} {}",
                model("SettingsState", "")
            )
            .replace("M", "SettingsModel"),
        )
        .unwrap();
        assert_eq!(
            (
                b.stem.as_str(),
                b.store().as_str(),
                b.actions().as_str(),
                b.prefix.as_str()
            ),
            ("Settings", "SettingsStore", "SettingsActions", "settings")
        );
    }

    #[test]
    fn carries_lists_nested_structs_and_selectors() {
        let b = bridge(&format!(
            "struct S {{ items: Vec<Item>, owner: User }}
             struct Item {{ id: u32, tags: Vec<Tag> }}
             struct Tag {{ name: String }}
             struct User {{ name: String }}
             impl S {{ #[selector] fn first(&self) -> Vec<Item> {{ todo!() }} }}
             {}",
            model("S", "")
        ))
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
        let err = |state: &str, methods: &str| {
            bridge(&format!("{state} {}", model("S", methods))).unwrap_err()
        };
        assert_eq!(
            err("struct S { tags: Vec<String> }", ""),
            "`S.tags`, a field, has type `Vec<String>`: a Vec crosses only as a Vec of structs"
        );
        assert_eq!(
            err("struct S { at: Instant }", ""),
            "`S.at` has type `Instant`, which cannot cross to the native side yet"
        );
        assert_eq!(
            err("struct S { child: Vec<S> }", ""),
            "`S` holds itself, which cannot cross: S → S"
        );
        assert_eq!(
            err(
                "struct S { on: bool }",
                "pub fn at(&self, when: Instant) {}"
            ),
            "`M::at` takes `Instant`, a type that cannot cross yet"
        );
        let file: syn::File = syn::parse_str(
            "struct S { on: bool } struct M; #[page_model] impl M { pub fn new() -> Self { M } }",
        )
        .unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items, "crate");
        assert!(
            Bridge::new(&types.models[0], &types)
                .unwrap_err()
                .contains("is not a `ValueListenable`")
        );
    }
}
