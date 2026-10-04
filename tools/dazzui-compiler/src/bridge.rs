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
//! sending thread, and the native side reads the state again. Strings and
//! lists in the snapshot are Rust's, handed back with `<prefix>_state_free`;
//! strings in an action are the caller's, borrowed for the length of the
//! call. A selector crosses as the field it reads as, computed when the
//! snapshot is taken.

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
    /// `crate::action::SettingsAction`
    pub action_path: String,
    pub update: String,
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
    /// A `Vec` of structs, as a pointer and a length.
    List(String),
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

/// The scalar types that cross. A string crosses as `NuiStr`, bytes and a
/// length.
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
        let action: &EnumDef = types
            .enums
            .get(&app.action)
            .ok_or_else(|| format!("cannot find enum `{}`, the action `{}` takes", app.action, app.function))?;
        if !types.structs.contains_key(&app.state) {
            return Err(format!("cannot find struct `{}`, the state `{}` updates", app.state, app.function));
        }
        let mut structs = Vec::new();
        collect_struct(&app.state, types, &mut Vec::new(), &mut structs)?;

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
            action_path: action.path.clone(),
            update: app.function.clone(),
            structs,
            variants,
        })
    }

    pub fn state_struct(&self) -> &BridgeStruct {
        self.structs.last().expect("the state crosses")
    }

    pub fn has_payloads(&self) -> bool {
        self.variants.iter().any(|v| v.payload.is_some())
    }

    /// Whether a string crosses anywhere in the snapshot.
    pub fn has_strings(&self) -> bool {
        self.structs.iter().flat_map(|s| &s.fields).any(|f| f.ty == FieldType::Scalar(Scalar::Str))
    }

    /// Whether `name` is the struct some field holds a list of.
    pub fn is_listed(&self, name: &str) -> bool {
        self.structs.iter().flat_map(|s| &s.fields).any(|f| f.ty == FieldType::List(name.into()))
    }

    /// `settings_send`
    pub fn function(&self, name: &str) -> String {
        format!("{}_{name}", self.prefix)
    }

    /// The C name of a struct's DTO: `SettingsStateDto` for the state,
    /// `SettingsPluginDto` for another, prefixed so that two bridges in one
    /// program cannot collide.
    pub fn dto(&self, name: &str) -> String {
        if name == self.state { format!("{}StateDto", self.stem) } else { format!("{}{name}Dto", self.stem) }
    }

    /// `SettingsPluginList`
    pub fn list(&self, name: &str) -> String {
        format!("{}{name}List", self.stem)
    }

    /// The `extern "C"` side, which the `#[app]` macro emits beside the
    /// update function it marks. Types are named by their paths, so it
    /// compiles wherever the update function is.
    pub fn rust(&self) -> TokenStream {
        let module = format_ident!("__dazzui_bridge_{}", self.prefix);
        let path = |p: &str| syn::parse_str::<syn::Path>(p).expect("a Rust path");
        let state = path(&self.state_struct().path);
        let action = path(&self.action_path);
        let update = format_ident!("{}", self.update);
        let app = format_ident!("{}App", self.stem);
        let state_dto = format_ident!("{}", self.dto(&self.state));
        let action_dto = format_ident!("{}ActionDto", self.stem);
        let payload = format_ident!("{}ActionPayload", self.stem);
        let state_to_dto = format_ident!("dto_{}", snake_case(&self.state));
        let state_free_dto = format_ident!("free_{}", snake_case(&self.state));
        let f = |name: &str| format_ident!("{}", self.function(name));
        let (create, destroy, get_state, state_free, send, subscribe) =
            (f("create"), f("destroy"), f("get_state"), f("state_free"), f("send"), f("subscribe"));

        let structs = self.structs.iter().map(|s| self.rust_struct(s));

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

                /// Hands `items` over as a pointer and a length, which
                /// `take_list` takes back.
                #[allow(dead_code)]
                fn give_list<T>(items: Box<[T]>) -> (*const T, usize) {
                    let len = items.len();
                    (Box::into_raw(items) as *const T, len)
                }

                #[allow(dead_code)]
                unsafe fn take_list<T>(ptr: *const T, len: usize) -> Vec<T> {
                    let slice = ::core::ptr::slice_from_raw_parts_mut(ptr as *mut T, len);
                    unsafe { Box::from_raw(slice) }.into_vec()
                }

                #(#structs)*

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
                    #state_to_dto(unsafe { &(*app).state })
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #state_free(state: #state_dto) {
                    unsafe { #state_free_dto(state) }
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

    /// One struct's DTO, its list type if a field holds a list of it, and
    /// the functions that build and free them.
    fn rust_struct(&self, s: &BridgeStruct) -> TokenStream {
        let path = syn::parse_str::<syn::Path>(&s.path).expect("a Rust path");
        let dto = format_ident!("{}", self.dto(&s.name));
        let to_dto = format_ident!("dto_{}", snake_case(&s.name));
        let free = format_ident!("free_{}", snake_case(&s.name));
        let ident = |n: &str| format_ident!("{}", n);
        let fields = s.fields.iter().map(|f| {
            let name = ident(&f.name);
            let ty = match &f.ty {
                FieldType::Scalar(scalar) => scalar.rust_dto(),
                FieldType::Struct(t) => {
                    let t = ident(&self.dto(t));
                    quote!(#t)
                }
                FieldType::List(t) => {
                    let t = ident(&self.list(t));
                    quote!(#t)
                }
            };
            quote!(pub #name: #ty)
        });
        let reads = s.fields.iter().map(|f| {
            let name = ident(&f.name);
            if f.selector { quote!(let #name = &value.#name();) } else { quote!(let #name = &value.#name;) }
        });
        let builds = s.fields.iter().map(|f| {
            let name = ident(&f.name);
            match &f.ty {
                FieldType::Scalar(Scalar::Str) => quote!(#name: NuiStr::owned(#name)),
                FieldType::Scalar(_) => quote!(#name: *#name),
                FieldType::Struct(t) => {
                    let to = format_ident!("dto_{}", snake_case(t));
                    quote!(#name: #to(#name))
                }
                FieldType::List(t) => {
                    let to = format_ident!("dto_{}", snake_case(t));
                    let list = ident(&self.list(t));
                    quote!(#name: {
                        let (ptr, len) = give_list(#name.iter().map(#to).collect());
                        #list { ptr, len }
                    })
                }
            }
        });
        let frees = s.fields.iter().filter_map(|f| {
            let name = ident(&f.name);
            match &f.ty {
                FieldType::Scalar(Scalar::Str) => Some(quote!(unsafe { dto.#name.free() };)),
                FieldType::Scalar(_) => None,
                FieldType::Struct(t) => {
                    let free = format_ident!("free_{}", snake_case(t));
                    Some(quote!(unsafe { #free(dto.#name) };))
                }
                FieldType::List(t) => {
                    let free = format_ident!("free_{}", snake_case(t));
                    Some(quote! {
                        for item in unsafe { take_list(dto.#name.ptr, dto.#name.len) } {
                            unsafe { #free(item) };
                        }
                    })
                }
            }
        });
        let list = self.is_listed(&s.name).then(|| {
            let list = ident(&self.list(&s.name));
            quote! {
                #[repr(C)]
                pub struct #list {
                    pub ptr: *const #dto,
                    pub len: usize,
                }
            }
        });
        quote! {
            #[repr(C)]
            pub struct #dto { #(#fields,)* }

            #list

            fn #to_dto(value: &#path) -> #dto {
                #(#reads)*
                #dto { #(#builds,)* }
            }

            #[allow(unused_variables)]
            unsafe fn #free(dto: #dto) {
                #(#frees)*
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

        for s in &self.structs {
            if s.name == self.state {
                h += &format!("/// A snapshot of `{}`. Hand it back to `{}`.\n", self.state, self.function("state_free"));
            } else {
                h += &format!("/// A `{}`.\n", s.name);
            }
            h += "typedef struct {\n";
            for field in &s.fields {
                let ty = match &field.ty {
                    FieldType::Scalar(scalar) => scalar.c().to_string(),
                    FieldType::Struct(t) => self.dto(t),
                    FieldType::List(t) => self.list(t),
                };
                h += &format!("  {ty} {};\n", field.name);
            }
            h += &format!("}} {};\n\n", self.dto(&s.name));
            if self.is_listed(&s.name) {
                h += &format!(
                    "typedef struct {{\n  const {} *ptr;\n  size_t len;\n}} {};\n\n",
                    self.dto(&s.name),
                    self.list(&s.name)
                );
            }
        }

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
        let state_dto = self.dto(&self.state);
        h += &format!("{stem}App *{}(void);\n", f("create"));
        h += &format!("void {}({stem}App *app);\n", f("destroy"));
        h += &format!("{state_dto} {}(const {stem}App *app);\n", f("get_state"));
        h += &format!("void {}({state_dto} state);\n", f("state_free"));
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
        return Err(format!("`{name}` holds itself, which cannot cross: {} → {name}", path.join(" → ")));
    }
    let def: &StructDef = &types.structs[name];
    if def.path.is_empty() {
        return Err(format!("`{name}` has selectors but no struct the CLI can read"));
    }
    path.push(name.into());
    let mut fields = Vec::new();
    let named = def.fields.iter().map(|f| (f, false)).chain(def.selectors.iter().map(|f| (f, true)));
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
                format!("`{name}.{field}`, a {what}, is a `{}`: a Vec crosses only as a Vec of structs", ty.rust())
            })?;
            collect_struct(&n, types, path, out)?;
            FieldType::List(n)
        } else {
            return Err(format!("`{name}.{field}` is a `{}`, which cannot cross to the native side yet", ty.rust()));
        };
        fields.push(BridgeField { name: field.clone(), ty, selector });
    }
    path.pop();
    out.push(BridgeStruct { name: name.into(), path: def.path.clone(), fields });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bridge(src: &str) -> Result<Bridge, String> {
        let file: syn::File = syn::parse_str(src).unwrap();
        let mut types = RustTypes::default();
        types.collect(&file.items, "crate");
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
        let types: Vec<_> = b.state_struct().fields.iter().map(|f| f.ty.clone()).collect();
        assert_eq!(
            types,
            [FieldType::Scalar(Scalar::Bool), FieldType::Scalar(Scalar::Int(IntType::U32)), FieldType::Scalar(Scalar::Str)]
        );
        assert_eq!(b.variants[1], BridgeVariant { name: "Rename".into(), tag: 1, payload: Some(Scalar::Str) });
        let header = b.c_header();
        assert!(header.contains("  uint32_t count;\n"));
        assert!(header.contains("  NuiStr rename;\n"));
        assert!(header.contains("void note_send(NoteApp *app, NoteActionDto action);"));
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
             #[app] fn update(state: &mut S, action: A) {}",
        )
        .unwrap();
        let names: Vec<_> = b.structs.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, ["Tag", "Item", "User", "S"]);
        let first = &b.state_struct().fields[2];
        assert_eq!((first.name.as_str(), &first.ty, first.selector), ("first", &FieldType::List("Item".into()), true));
        let header = b.c_header();
        let tag = header.find("} STagDto;").unwrap();
        let tag_list = header.find("} STagList;").unwrap();
        let item = header.find("} SItemDto;").unwrap();
        assert!(tag < tag_list && tag_list < item, "each C type after the ones it holds");
        assert!(header.contains("  SItemList items;\n  SUserDto owner;\n  SItemList first;\n"));
        syn::parse2::<syn::File>(b.rust()).expect("the Rust side parses");
    }

    #[test]
    fn rejects_what_cannot_cross() {
        let err = |src: &str| {
            bridge(&format!("{src} enum A {{ Go }} #[app] fn update(state: &mut S, action: A) {{}}")).unwrap_err()
        };
        assert_eq!(
            err("struct S { tags: Vec<String> }"),
            "`S.tags`, a field, is a `Vec<String>`: a Vec crosses only as a Vec of structs"
        );
        assert_eq!(err("struct S { at: Instant }"), "`S.at` is a `Instant`, which cannot cross to the native side yet");
        assert_eq!(err("struct S { child: Vec<S> }"), "`S` holds itself, which cannot cross: S → S");
    }
}
