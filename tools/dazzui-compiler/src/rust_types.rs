use std::collections::BTreeMap;

/// The Rust types a page's state and actions are, read from the app's
/// source with syn, so that the native side's mirrors of them are derived
/// rather than written a second time.
#[derive(Clone, Debug, Default)]
pub struct RustTypes {
    pub enums: BTreeMap<String, EnumDef>,
    pub structs: BTreeMap<String, StructDef>,
    /// Functions marked `#[app]`: the update functions the bridge wraps.
    pub apps: Vec<AppDef>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    pub name: String,
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

/// A struct with named fields.
#[derive(Clone, Debug, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, RustType)>,
}

impl StructDef {
    pub fn field(&self, name: &str) -> Option<&RustType> {
        self.fields.iter().find(|(n, _)| n == name).map(|(_, ty)| ty)
    }
}

/// `#[app] fn update(state: &mut State, action: Action)`.
#[derive(Clone, Debug, PartialEq)]
pub struct AppDef {
    pub function: String,
    pub state: String,
    pub action: String,
}

/// The Rust types that cross to the native side.
#[derive(Clone, Debug, PartialEq)]
pub enum RustType {
    Bool,
    String,
    Int(IntType),
    Float(FloatType),
    /// A type the bridge does not carry yet, by its written name.
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
            RustType::Other(name) => name.clone(),
        }
    }
}

fn quote_type(ty: &syn::Type) -> String {
    quote::ToTokens::to_token_stream(ty).to_string().replace(' ', "")
}

impl RustTypes {
    /// Collects the enums, structs and `#[app]` functions of one parsed
    /// file, including those in inline modules.
    pub fn collect(&mut self, items: &[syn::Item]) {
        for item in items {
            match item {
                syn::Item::Enum(e) => {
                    let def = EnumDef {
                        name: e.ident.to_string(),
                        variants: e
                            .variants
                            .iter()
                            .map(|v| Variant { name: v.ident.to_string(), fields: fields(&v.fields) })
                            .collect(),
                    };
                    self.enums.insert(def.name.clone(), def);
                }
                syn::Item::Struct(s) => {
                    if let Fields::Named(fields) = fields(&s.fields) {
                        self.structs.insert(s.ident.to_string(), StructDef { name: s.ident.to_string(), fields });
                    }
                }
                syn::Item::Fn(f) if f.attrs.iter().any(|a| a.path().segments.last().is_some_and(|s| s.ident == "app")) => {
                    if let Ok(app) = AppDef::from_fn(&f.sig) {
                        self.apps.push(app);
                    }
                }
                syn::Item::Mod(m) => {
                    if let Some((_, items)) = &m.content {
                        self.collect(items);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn app_for(&self, state: &str) -> Option<&AppDef> {
        self.apps.iter().find(|a| a.state == state)
    }
}

impl AppDef {
    /// Reads `fn name(state: &mut State, action: Action)`.
    pub fn from_fn(sig: &syn::Signature) -> syn::Result<Self> {
        let shape = || {
            syn::Error::new_spanned(sig, "an #[app] function is `fn update(state: &mut State, action: Action)`")
        };
        let args: Vec<_> = sig.inputs.iter().collect();
        let [syn::FnArg::Typed(state), syn::FnArg::Typed(action)] = args.as_slice() else {
            return Err(shape());
        };
        let syn::Type::Reference(state_ref) = &*state.ty else {
            return Err(shape());
        };
        if state_ref.mutability.is_none() || sig.output != syn::ReturnType::Default {
            return Err(shape());
        }
        let name = |ty: &syn::Type| match ty {
            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()).ok_or_else(shape),
            _ => Err(shape()),
        };
        Ok(AppDef { function: sig.ident.to_string(), state: name(&state_ref.elem)?, action: name(&action.ty)? })
    }
}

fn fields(fields: &syn::Fields) -> Fields {
    match fields {
        syn::Fields::Unit => Fields::Unit,
        syn::Fields::Unnamed(f) => {
            Fields::Tuple(f.unnamed.iter().map(|f| RustType::from_syn(&f.ty)).collect())
        }
        syn::Fields::Named(f) => Fields::Named(
            f.named
                .iter()
                .map(|f| (f.ident.as_ref().unwrap().to_string(), RustType::from_syn(&f.ty)))
                .collect(),
        ),
    }
}
