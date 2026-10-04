//! The C ABI between the Rust app core and the native page (DESIGN.md §8).
//!
//! [`Bridge`] is the one description of that ABI — which state fields cross,
//! in what C type, and what tag each action carries — and every side is
//! derived from it: the `#[app]` macro's `extern "C"` functions
//! ([`Bridge::rust`]), the C header Swift imports ([`Bridge::c_header`]),
//! and the Swift and C# glue the backends write. One description is what
//! keeps the four in step.
//!
//! The data flow is a whole snapshot each time: the native side sends an
//! action, Rust's `update` runs, every subscriber is called back on the
//! sending thread, and the native side reads the state again. Strings in
//! the snapshot are Rust's, handed back with `<prefix>_state_free`; strings
//! in an action are the caller's, borrowed for the length of the call.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ir::{pascal_case, snake_case};
use crate::rust_types::{AppDef, EnumDef, Fields, FloatType, IntType, RustType, RustTypes, StructDef};

#[derive(Clone, Debug, PartialEq)]
pub struct Bridge {
    /// `Settings`, from `SettingsState`: what the bridge's types are named
    /// from.
    pub stem: String,
    /// `settings`: what its C functions are named from.
    pub prefix: String,
    pub state: String,
    pub action: String,
    pub update: String,
    pub fields: Vec<BridgeField>,
    pub variants: Vec<BridgeVariant>,
}

/// A state field as it crosses.
#[derive(Clone, Debug, PartialEq)]
pub struct BridgeField {
    pub name: String,
    pub ty: Scalar,
}

/// An action variant: its tag, and the one value it carries, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct BridgeVariant {
    pub name: String,
    pub tag: u32,
    pub payload: Option<Scalar>,
}

impl BridgeVariant {
    /// `set_auto_update`: its member of the payload union.
    pub fn member(&self) -> String {
        snake_case(&self.name)
    }
}

/// The types that cross. A string crosses as `NuiStr`, bytes and a length.
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
            RustType::Other(_) => None,
        }
    }

    pub fn c(self) -> &'static str {
        match self {
            Scalar::Bool => "bool",
            Scalar::Int(IntType::I8) => "int8_t",
            Scalar::Int(IntType::I16) => "int16_t",
            Scalar::Int(IntType::I32) => "int32_t",
            Scalar::Int(IntType::I64) => "int64_t",
            Scalar::Int(IntType::U8) => "uint8_t",
            Scalar::Int(IntType::U16) => "uint16_t",
            Scalar::Int(IntType::U32) => "uint32_t",
            Scalar::Int(IntType::U64) => "uint64_t",
            Scalar::Int(IntType::Isize) => "intptr_t",
            Scalar::Int(IntType::Usize) => "uintptr_t",
            Scalar::Float(FloatType::F32) => "float",
            Scalar::Float(FloatType::F64) => "double",
            Scalar::Str => "NuiStr",
        }
    }

    /// The type a Rust DTO holds it in.
    fn rust_dto(self) -> TokenStream {
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
            Scalar::Str => quote!(NuiStr),
        }
    }
}

impl Bridge {
    /// The bridge for `app`, checked against the Rust types it names.
    pub fn new(app: &AppDef, types: &RustTypes) -> Result<Self, String> {
        let state: &StructDef = types
            .structs
            .get(&app.state)
            .ok_or_else(|| format!("cannot find struct `{}`, the state `{}` updates", app.state, app.function))?;
        let action: &EnumDef = types
            .enums
            .get(&app.action)
            .ok_or_else(|| format!("cannot find enum `{}`, the action `{}` takes", app.action, app.function))?;

        let fields = state
            .fields
            .iter()
            .map(|(name, ty)| {
                Scalar::from_rust(ty).map(|ty| BridgeField { name: name.clone(), ty }).ok_or_else(|| {
                    format!("`{}.{name}` is a `{}`, which cannot cross to the native side yet", state.name, ty.rust())
                })
            })
            .collect::<Result<_, _>>()?;
        let variants = action
            .variants
            .iter()
            .enumerate()
            .map(|(tag, v)| {
                let payload = match &v.fields {
                    Fields::Unit => None,
                    Fields::Tuple(types) if types.len() == 1 => Some(Scalar::from_rust(&types[0]).ok_or_else(|| {
                        format!("`{}::{}` carries a `{}`, which cannot cross yet", action.name, v.name, types[0].rust())
                    })?),
                    _ => {
                        return Err(format!(
                            "`{}::{}` carries several values; an action carries one or none",
                            action.name, v.name
                        ));
                    }
                };
                Ok(BridgeVariant { name: v.name.clone(), tag: tag as u32, payload })
            })
            .collect::<Result<_, _>>()?;

        let stem = app.state.strip_suffix("State").filter(|s| !s.is_empty()).unwrap_or(&app.state).to_string();
        Ok(Bridge {
            prefix: snake_case(&stem),
            stem,
            state: app.state.clone(),
            action: app.action.clone(),
            update: app.function.clone(),
            fields,
            variants,
        })
    }

    pub fn has_payloads(&self) -> bool {
        self.variants.iter().any(|v| v.payload.is_some())
    }

    pub fn has_strings(&self) -> bool {
        self.fields.iter().any(|f| f.ty == Scalar::Str)
    }

    /// `settings_send`
    pub fn function(&self, name: &str) -> String {
        format!("{}_{name}", self.prefix)
    }

    /// The `extern "C"` side, which the `#[app]` macro emits beside the
    /// update function it marks. It names the state, action and update by
    /// what they are called there.
    pub fn rust(&self) -> TokenStream {
        let module = format_ident!("__dazzui_bridge_{}", self.prefix);
        let state = format_ident!("{}", self.state);
        let action = format_ident!("{}", self.action);
        let update = format_ident!("{}", self.update);
        let app = format_ident!("{}App", self.stem);
        let state_dto = format_ident!("{}StateDto", self.stem);
        let action_dto = format_ident!("{}ActionDto", self.stem);
        let payload = format_ident!("{}ActionPayload", self.stem);
        let f = |name: &str| format_ident!("{}", self.function(name));
        let (create, destroy, get_state, state_free, send, subscribe) =
            (f("create"), f("destroy"), f("get_state"), f("state_free"), f("send"), f("subscribe"));

        let dto_fields = self.fields.iter().map(|field| {
            let name = format_ident!("{}", field.name);
            let ty = field.ty.rust_dto();
            quote!(pub #name: #ty)
        });
        let to_dto = self.fields.iter().map(|field| {
            let name = format_ident!("{}", field.name);
            match field.ty {
                Scalar::Str => quote!(#name: NuiStr::owned(&state.#name)),
                _ => quote!(#name: state.#name),
            }
        });
        let frees = self.fields.iter().filter(|f| f.ty == Scalar::Str).map(|field| {
            let name = format_ident!("{}", field.name);
            quote!(unsafe { state.#name.free() };)
        });

        let members = self.variants.iter().filter_map(|v| {
            let member = format_ident!("{}", v.member());
            let ty = v.payload?.rust_dto();
            Some(quote!(pub #member: #ty))
        });
        let payload_type = self.has_payloads().then(|| {
            quote! {
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union #payload { #(#members,)* }
            }
        });
        let payload_field = self.has_payloads().then(|| quote!(pub payload: #payload,));
        let arms = self.variants.iter().map(|v| {
            let tag = v.tag;
            let variant = format_ident!("{}", v.name);
            let member = format_ident!("{}", v.member());
            match v.payload {
                None => quote!(#tag => #action::#variant,),
                Some(Scalar::Str) => quote!(#tag => #action::#variant(unsafe { action.payload.#member.borrowed() }),),
                Some(_) => quote!(#tag => #action::#variant(unsafe { action.payload.#member }),),
            }
        });

        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types, clippy::missing_safety_doc)]
            pub mod #module {
                use super::*;
                use ::core::ffi::c_void;

                /// UTF-8 bytes and their length. Rust owns those in a state
                /// snapshot; the caller owns those in an action.
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct NuiStr {
                    pub ptr: *const u8,
                    pub len: usize,
                }

                impl NuiStr {
                    fn owned(text: &str) -> Self {
                        let bytes: Box<[u8]> = text.as_bytes().into();
                        let len = bytes.len();
                        NuiStr { ptr: Box::into_raw(bytes) as *const u8, len }
                    }

                    unsafe fn free(self) {
                        if !self.ptr.is_null() {
                            let slice = ::core::ptr::slice_from_raw_parts_mut(self.ptr as *mut u8, self.len);
                            drop(unsafe { Box::from_raw(slice) });
                        }
                    }

                    unsafe fn borrowed(self) -> String {
                        if self.ptr.is_null() || self.len == 0 {
                            return String::new();
                        }
                        let bytes = unsafe { ::core::slice::from_raw_parts(self.ptr, self.len) };
                        String::from_utf8_lossy(bytes).into_owned()
                    }
                }

                #[repr(C)]
                pub struct #state_dto { #(#dto_fields,)* }

                #payload_type

                #[repr(C)]
                pub struct #action_dto {
                    pub tag: u32,
                    #payload_field
                }

                pub type NuiStateChanged = unsafe extern "C" fn(context: *mut c_void);

                pub struct #app {
                    state: #state,
                    subscribers: Vec<(NuiStateChanged, *mut c_void)>,
                }

                #[unsafe(no_mangle)]
                pub extern "C" fn #create() -> *mut #app {
                    Box::into_raw(Box::new(#app { state: <#state as Default>::default(), subscribers: Vec::new() }))
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #destroy(app: *mut #app) {
                    if !app.is_null() {
                        drop(unsafe { Box::from_raw(app) });
                    }
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #get_state(app: *const #app) -> #state_dto {
                    let state = unsafe { &(*app).state };
                    #state_dto { #(#to_dto,)* }
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #state_free(state: #state_dto) {
                    let _ = &state;
                    #(#frees)*
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #send(app: *mut #app, action: #action_dto) {
                    let action = match action.tag {
                        #(#arms)*
                        _ => return,
                    };
                    unsafe { super::#update(&mut (*app).state, action) };
                    // Copied out, so that no borrow of the app is live while a
                    // subscriber reads the state again.
                    let subscribers = unsafe { (*app).subscribers.clone() };
                    for (callback, context) in subscribers {
                        unsafe { callback(context) };
                    }
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #subscribe(app: *mut #app, callback: NuiStateChanged, context: *mut c_void) {
                    unsafe { (*app).subscribers.push((callback, context)) };
                }
            }
        }
    }

    /// The C header the Swift side imports.
    pub fn c_header(&self) -> String {
        let stem = &self.stem;
        let upper = self.prefix.to_uppercase();
        let mut h = String::new();
        h += &format!("#ifndef {upper}_BRIDGE_H\n#define {upper}_BRIDGE_H\n\n");
        h += "#include <stdbool.h>\n#include <stddef.h>\n#include <stdint.h>\n\n";
        h += "#ifndef NUI_STR_DEFINED\n#define NUI_STR_DEFINED\n";
        h += "/// UTF-8 bytes and their length. Rust owns those in a state snapshot;\n";
        h += "/// the caller owns those in an action, for the length of the call.\n";
        h += "typedef struct {\n  const uint8_t *ptr;\n  size_t len;\n} NuiStr;\n#endif\n\n";
        h += "typedef void (*NuiStateChanged)(void *context);\n\n";
        h += &format!("typedef struct {stem}App {stem}App;\n\n");

        h += &format!("/// A snapshot of `{}`. Hand it back to `{}`.\n", self.state, self.function("state_free"));
        h += "typedef struct {\n";
        for field in &self.fields {
            h += &format!("  {} {};\n", field.ty.c(), field.name);
        }
        h += &format!("}} {stem}StateDto;\n\n");

        for v in &self.variants {
            h += &format!("#define {upper}_ACTION_{} {}u\n", snake_case(&v.name).to_uppercase(), v.tag);
        }
        h += "\n";
        if self.has_payloads() {
            h += "typedef union {\n";
            for v in &self.variants {
                if let Some(ty) = v.payload {
                    h += &format!("  {} {};\n", ty.c(), v.member());
                }
            }
            h += &format!("}} {stem}ActionPayload;\n\n");
        }
        h += &format!("/// A `{}`: a tag, and the value its variant carries.\n", self.action);
        h += "typedef struct {\n  uint32_t tag;\n";
        if self.has_payloads() {
            h += &format!("  {stem}ActionPayload payload;\n");
        }
        h += &format!("}} {stem}ActionDto;\n\n");

        let f = |name: &str| self.function(name);
        h += &format!("{stem}App *{}(void);\n", f("create"));
        h += &format!("void {}({stem}App *app);\n", f("destroy"));
        h += &format!("{stem}StateDto {}(const {stem}App *app);\n", f("get_state"));
        h += &format!("void {}({stem}StateDto state);\n", f("state_free"));
        h += &format!("void {}({stem}App *app, {stem}ActionDto action);\n", f("send"));
        h += "/// `callback` runs after every action, on the thread that sent it.\n";
        h += &format!("void {}({stem}App *app, NuiStateChanged callback, void *context);\n", f("subscribe"));
        h += "\n#endif\n";
        h
    }

    /// The C# property a field is: `launch_at_startup` → `LaunchAtStartup`.
    pub fn property(field: &str) -> String {
        pascal_case(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bridge(src: &str) -> Result<Bridge, String> {
        let file: syn::File = syn::parse_str(src).unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items);
        Bridge::new(&types.apps[0].clone(), &types)
    }

    #[test]
    fn lays_out_state_and_actions() {
        let b = bridge(
            "struct NoteState { done: bool, count: u32, title: String }
             enum NoteAction { Toggle, Rename(String), Bump(u32) }
             #[app] fn update(state: &mut NoteState, action: NoteAction) {}",
        )
        .unwrap();
        assert_eq!(b.prefix, "note");
        assert_eq!(b.fields.iter().map(|f| f.ty).collect::<Vec<_>>(), [Scalar::Bool, Scalar::Int(IntType::U32), Scalar::Str]);
        assert_eq!(b.variants[1], BridgeVariant { name: "Rename".into(), tag: 1, payload: Some(Scalar::Str) });
        let header = b.c_header();
        assert!(header.contains("  uint32_t count;\n"));
        assert!(header.contains("  NuiStr rename;\n"));
        assert!(header.contains("void note_send(NoteApp *app, NoteActionDto action);"));
        syn::parse2::<syn::File>(b.rust()).expect("the Rust side parses");
    }

    #[test]
    fn rejects_what_cannot_cross() {
        let err = bridge(
            "struct S { items: Vec<String> } enum A { Go }
             #[app] fn update(state: &mut S, action: A) {}",
        )
        .unwrap_err();
        assert_eq!(err, "`S.items` is a `Vec<String>`, which cannot cross to the native side yet");
    }
}
