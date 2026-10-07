use std::collections::BTreeMap;

/// The Rust types a page's state and actions are, read from the app's
/// source with syn, so that the native side's mirrors of them are derived
/// rather than written a second time.
#[derive(Clone, Debug, Default)]
pub struct RustTypes {
    pub enums: BTreeMap<String, EnumDef>,
    pub structs: BTreeMap<String, StructDef>,
    /// `#[page_model] impl`s: the models pages call, and the bridge
    /// carries to the native side.
    pub models: Vec<ModelDef>,
    /// `impl ValueListenable for T`: each `T`, and its `Value`.
    pub listenable_impls: BTreeMap<String, String>,
    /// Whether the crate's root calls `uniffi::setup_scaffolding!()`, which
    /// the bridge's UniFFI exports need.
    pub scaffolding: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub name: String,
    /// `crate::action::SettingsAction`: where it is, so that generated Rust
    /// can name it from any module.
    pub path: String,
    pub variants: Vec<Variant>,
}

impl EnumDef {
    pub fn variant(&self, name: &str) -> Option<&Variant> {
        self.variants.iter().find(|v| v.name == name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub name: String,
    pub fields: Fields,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fields {
    Unit,
    Tuple(Vec<RustType>),
    Named(Vec<(String, RustType)>),
}

/// A struct with named fields, and the selectors defined on it.
#[derive(Clone, Debug, PartialEq)]
pub struct StructDef {
    pub name: String,
    /// `crate::state::Plugin`
    pub path: String,
    pub fields: Vec<(String, RustType)>,
    /// `#[selector] fn visible_items(&self) -> Vec<Item>`: values derived
    /// from the fields, which a page reads as if they were fields.
    pub selectors: Vec<(String, RustType)>,
}

impl StructDef {
    /// A field or a selector, by name.
    pub fn field(&self, name: &str) -> Option<&RustType> {
        self.fields
            .iter()
            .chain(&self.selectors)
            .find(|(n, _)| n == name)
            .map(|(_, ty)| ty)
    }

    pub fn is_selector(&self, name: &str) -> bool {
        self.selectors.iter().any(|(n, _)| n == name)
    }

    /// Every name a page can read: fields, then selectors.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.fields
            .iter()
            .chain(&self.selectors)
            .map(|(n, _)| n.as_str())
    }
}

/// `#[page_model] impl SettingsModel { … }`: a model, and the methods a
/// page may call on it.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelDef {
    pub name: String,
    /// `crate::model::SettingsModel`
    pub path: String,
    pub methods: Vec<MethodDef>,
    /// Where the type's name is in the impl, for a diagnostic: the file, as
    /// the project displays it, and the line, column and width.
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub width: u32,
}

impl ModelDef {
    pub fn method(&self, name: &str) -> Option<&MethodDef> {
        self.methods.iter().find(|m| m.name == name)
    }
}

/// `pub fn set_on(&self, on: bool)`: a method a page may call, and its one
/// parameter, if it has one.
#[derive(Clone, Debug, PartialEq)]
pub struct MethodDef {
    pub name: String,
    pub param: Option<(String, RustType)>,
}

/// Names the bridge gives the native object itself, which a model's method
/// cannot also take.
const RESERVED: &[&str] = &["value", "add_listener", "remove_listener", "with_model"];

/// The Rust types that cross to the native side.
#[derive(Clone, Debug, PartialEq)]
pub enum RustType {
    Bool,
    String,
    Int(IntType),
    Float(FloatType),
    Vec(Box<RustType>),
    /// Any other type: a struct by its name, or what the bridge does not
    /// carry, as written.
    Other(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Isize,
    Usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FloatType {
    F32,
    F64,
}

impl IntType {
    pub fn rust(self) -> &'static str {
        match self {
            IntType::I8 => "i8",
            IntType::I16 => "i16",
            IntType::I32 => "i32",
            IntType::I64 => "i64",
            IntType::U8 => "u8",
            IntType::U16 => "u16",
            IntType::U32 => "u32",
            IntType::U64 => "u64",
            IntType::Isize => "isize",
            IntType::Usize => "usize",
        }
    }
}

impl FloatType {
    pub fn rust(self) -> &'static str {
        match self {
            FloatType::F32 => "f32",
            FloatType::F64 => "f64",
        }
    }
}

impl RustType {
    pub fn from_syn(ty: &syn::Type) -> Self {
        let syn::Type::Path(path) = ty else {
            return RustType::Other(quote_type(ty));
        };
        let Some(last) = path.path.segments.last() else {
            return RustType::Other(quote_type(ty));
        };
        match last.ident.to_string().as_str() {
            "bool" => RustType::Bool,
            "String" => RustType::String,
            "i8" => RustType::Int(IntType::I8),
            "i16" => RustType::Int(IntType::I16),
            "i32" => RustType::Int(IntType::I32),
            "i64" => RustType::Int(IntType::I64),
            "u8" => RustType::Int(IntType::U8),
            "u16" => RustType::Int(IntType::U16),
            "u32" => RustType::Int(IntType::U32),
            "u64" => RustType::Int(IntType::U64),
            "isize" => RustType::Int(IntType::Isize),
            "usize" => RustType::Int(IntType::Usize),
            "f32" => RustType::Float(FloatType::F32),
            "f64" => RustType::Float(FloatType::F64),
            "Vec" => match &last.arguments {
                syn::PathArguments::AngleBracketed(args) => match args.args.first() {
                    Some(syn::GenericArgument::Type(elem)) if args.args.len() == 1 => {
                        RustType::Vec(Box::new(RustType::from_syn(elem)))
                    }
                    _ => RustType::Other(quote_type(ty)),
                },
                _ => RustType::Other(quote_type(ty)),
            },
            name if last.arguments.is_none() => RustType::Other(name.into()),
            _ => RustType::Other(quote_type(ty)),
        }
    }

    /// The type as Rust writes it.
    pub fn rust(&self) -> String {
        match self {
            RustType::Bool => "bool".into(),
            RustType::String => "String".into(),
            RustType::Int(i) => i.rust().into(),
            RustType::Float(f) => f.rust().into(),
            RustType::Vec(elem) => format!("Vec<{}>", elem.rust()),
            RustType::Other(name) => name.clone(),
        }
    }
}

fn quote_type(ty: &syn::Type) -> String {
    quote::ToTokens::to_token_stream(ty)
        .to_string()
        .replace(' ', "")
}

impl RustTypes {
    /// Collects the enums, structs, selectors, `#[page_model] impl`s and
    /// `impl ValueListenable`s of
    /// one parsed file, including those in inline modules. `module` is the
    /// file's module path, `crate::state`.
    pub fn collect(&mut self, items: &[syn::Item], module: &str) {
        for item in items {
            match item {
                syn::Item::Enum(e) => {
                    let def = EnumDef {
                        name: e.ident.to_string(),
                        path: format!("{module}::{}", e.ident),
                        variants: e
                            .variants
                            .iter()
                            .map(|v| Variant {
                                name: v.ident.to_string(),
                                fields: fields(&v.fields),
                            })
                            .collect(),
                    };
                    self.enums.insert(def.name.clone(), def);
                }
                syn::Item::Struct(s) => {
                    if let Fields::Named(fields) = fields(&s.fields) {
                        let name = s.ident.to_string();
                        let def = self
                            .structs
                            .entry(name.clone())
                            .or_insert_with(|| StructDef {
                                name: name.clone(),
                                path: String::new(),
                                fields: Vec::new(),
                                selectors: Vec::new(),
                            });
                        def.path = format!("{module}::{name}");
                        def.fields = fields;
                    }
                }
                syn::Item::Impl(imp) if has_attr(&imp.attrs, "page_model") => {
                    if let Ok(model) = ModelDef::from_impl(imp, module) {
                        self.models.push(model);
                    }
                }
                syn::Item::Impl(imp) if imp.trait_.is_none() => {
                    let syn::Type::Path(self_ty) = &*imp.self_ty else {
                        continue;
                    };
                    let Some(name) = self_ty.path.segments.last().map(|s| s.ident.to_string())
                    else {
                        continue;
                    };
                    let selectors: Vec<_> = imp
                        .items
                        .iter()
                        .filter_map(|item| match item {
                            syn::ImplItem::Fn(f) if has_attr(&f.attrs, "selector") => {
                                selector(&f.sig).ok()
                            }
                            _ => None,
                        })
                        .collect();
                    if !selectors.is_empty() {
                        // An impl may come before its struct, or in another file.
                        let def = self
                            .structs
                            .entry(name.clone())
                            .or_insert_with(|| StructDef {
                                name,
                                path: String::new(),
                                fields: Vec::new(),
                                selectors: Vec::new(),
                            });
                        def.selectors.extend(selectors);
                    }
                }
                syn::Item::Impl(imp) if implements(imp, "ValueListenable") => {
                    if let (Some(name), Some(value)) =
                        (type_name(&imp.self_ty), associated(imp, "Value"))
                    {
                        self.listenable_impls.insert(name, value);
                    }
                }
                syn::Item::Macro(m)
                    if module == "crate"
                        && m.mac
                            .path
                            .segments
                            .last()
                            .is_some_and(|s| s.ident == "setup_scaffolding") =>
                {
                    self.scaffolding = true;
                }
                syn::Item::Mod(m) => {
                    if let Some((_, items)) = &m.content {
                        self.collect(items, &format!("{module}::{}", m.ident));
                    }
                }
                _ => {}
            }
        }
    }

    /// The state a model's pages show — its `ValueListenable` value — or
    /// why it cannot be told.
    pub fn model_state(&self, model: &ModelDef) -> Result<String, String> {
        self.listenable_impls.get(&model.name).cloned().ok_or_else(|| {
            format!(
                "`{}` is not a `ValueListenable`: implement it, naming the state its pages show: `impl ValueListenable for {} {{ type Value = …; }}`",
                model.name, model.name
            )
        })
    }

    pub fn model(&self, name: &str) -> Option<&ModelDef> {
        self.models.iter().find(|m| m.name == name)
    }
}

impl ModelDef {
    /// Reads `#[page_model] impl Name { pub fn new() -> Self; pub fn m(&self[, v: T]) … }`.
    pub fn from_impl(imp: &syn::ItemImpl, module: &str) -> syn::Result<Self> {
        if imp.trait_.is_some() || !imp.generics.params.is_empty() {
            return Err(syn::Error::new_spanned(
                &imp.self_ty,
                "#[page_model] marks a model's own `impl`, with no generics: `#[page_model] impl SettingsModel { … }`",
            ));
        }
        let Some(name) = type_name(&imp.self_ty) else {
            return Err(syn::Error::new_spanned(
                &imp.self_ty,
                "a model is a named type",
            ));
        };
        let mut methods = Vec::new();
        let mut new = false;
        for item in &imp.items {
            let syn::ImplItem::Fn(f) = item else { continue };
            if !matches!(f.vis, syn::Visibility::Public(_)) {
                continue;
            }
            let sig = &f.sig;
            if sig.ident == "new" {
                let returns_self = matches!(&sig.output, syn::ReturnType::Type(_, ty)
                    if type_name(ty).is_some_and(|t| t == "Self" || t == name));
                if !sig.inputs.is_empty() || !returns_self {
                    return Err(syn::Error::new_spanned(
                        sig,
                        "the native side makes the model with `pub fn new() -> Self`, which takes nothing",
                    ));
                }
                new = true;
                continue;
            }
            let shape = || {
                syn::Error::new_spanned(
                    sig,
                    "a model's public method is what a page calls: `pub fn name(&self)` or `pub fn name(&self, value: T)`, returning nothing; make anything else private",
                )
            };
            if RESERVED.contains(&sig.ident.to_string().as_str()) {
                return Err(syn::Error::new(
                    sig.ident.span(),
                    format!(
                        "`{}` is a name the bridge gives the native object; call the method something else",
                        sig.ident
                    ),
                ));
            }
            let mut inputs = sig.inputs.iter();
            match inputs.next() {
                Some(syn::FnArg::Receiver(r))
                    if r.reference.is_some() && r.mutability.is_none() => {}
                _ => return Err(shape()),
            }
            let param = match inputs.next() {
                None => None,
                Some(syn::FnArg::Typed(t)) => {
                    let syn::Pat::Ident(ident) = &*t.pat else {
                        return Err(shape());
                    };
                    Some((ident.ident.to_string(), RustType::from_syn(&t.ty)))
                }
                Some(_) => return Err(shape()),
            };
            if inputs.next().is_some()
                || sig.output != syn::ReturnType::Default
                || sig.asyncness.is_some()
                || !sig.generics.params.is_empty()
            {
                return Err(shape());
            }
            methods.push(MethodDef {
                name: sig.ident.to_string(),
                param,
            });
        }
        if !new {
            return Err(syn::Error::new_spanned(
                &imp.self_ty,
                "the native side makes the model with `pub fn new() -> Self`; add one to this impl",
            ));
        }
        let span = match &*imp.self_ty {
            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.span()),
            _ => None,
        }
        .unwrap_or_else(proc_macro2::Span::call_site);
        let (start, end) = (span.start(), span.end());
        Ok(ModelDef {
            path: format!("{module}::{name}"),
            name,
            methods,
            file: String::new(),
            line: start.line as u32,
            column: start.column as u32 + 1,
            width: end.column.saturating_sub(start.column) as u32,
        })
    }
}

/// `Name` for a path type `a::b::Name`.
fn type_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

fn implements(imp: &syn::ItemImpl, name: &str) -> bool {
    imp.trait_
        .as_ref()
        .and_then(|(_, path, _)| path.segments.last())
        .is_some_and(|s| s.ident == name)
}

/// The type an impl's `type Name = …;` names.
fn associated(imp: &syn::ItemImpl, name: &str) -> Option<String> {
    imp.items.iter().find_map(|item| match item {
        syn::ImplItem::Type(t) if t.ident == name => type_name(&t.ty),
        _ => None,
    })
}

fn has_attr(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs
        .iter()
        .any(|a| a.path().segments.last().is_some_and(|s| s.ident == name))
}

/// Reads `fn name(&self) -> T`.
pub fn selector(sig: &syn::Signature) -> syn::Result<(String, RustType)> {
    let shape = || syn::Error::new_spanned(sig, "a #[selector] is `fn name(&self) -> T`");
    let [syn::FnArg::Receiver(receiver)] = sig.inputs.iter().collect::<Vec<_>>().as_slice() else {
        return Err(shape());
    };
    if receiver.reference.is_none() || receiver.mutability.is_some() {
        return Err(shape());
    }
    match &sig.output {
        syn::ReturnType::Type(_, ty) => Ok((sig.ident.to_string(), RustType::from_syn(ty))),
        syn::ReturnType::Default => Err(shape()),
    }
}

/// `src/state.rs` → `crate::state`; `src/views/mod.rs` → `crate::views`;
/// `src/lib.rs` and `src/main.rs` → `crate`. `relative` is the file's
/// path under `src`.
pub fn module_path(relative: &std::path::Path) -> String {
    let mut parts: Vec<String> = relative
        .with_extension("")
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    if matches!(parts.last().map(String::as_str), Some("mod"))
        || matches!(parts.as_slice(), [only] if only == "lib" || only == "main")
    {
        parts.pop();
    }
    std::iter::once("crate".to_string())
        .chain(parts)
        .collect::<Vec<_>>()
        .join("::")
}

fn fields(fields: &syn::Fields) -> Fields {
    match fields {
        syn::Fields::Unit => Fields::Unit,
        syn::Fields::Unnamed(f) => Fields::Tuple(
            f.unnamed
                .iter()
                .map(|f| RustType::from_syn(&f.ty))
                .collect(),
        ),
        syn::Fields::Named(f) => Fields::Named(
            f.named
                .iter()
                .map(|f| {
                    (
                        f.ident.as_ref().unwrap().to_string(),
                        RustType::from_syn(&f.ty),
                    )
                })
                .collect(),
        ),
    }
}
