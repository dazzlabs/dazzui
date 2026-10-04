use std::collections::BTreeMap;

/// The Rust types a page's state and actions are, read from the app's
/// source with syn, so that the native side's mirrors of them are derived
/// rather than written a second time.
#[derive(Clone, Debug, Default)]
pub struct RustTypes {
    pub enums: BTreeMap<String, EnumDef>,
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

/// The Rust types that cross to the native side.
#[derive(Clone, Debug, PartialEq)]
pub enum RustType {
    Bool,
    String,
    Int,
    Float,
    /// A type the bridge does not carry yet, by its written name.
    Other(String),
}

impl RustType {
    fn from_syn(ty: &syn::Type) -> Self {
        let syn::Type::Path(path) = ty else {
            return RustType::Other(quote_type(ty));
        };
        let Some(last) = path.path.segments.last() else {
            return RustType::Other(quote_type(ty));
        };
        match last.ident.to_string().as_str() {
            "bool" => RustType::Bool,
            "String" => RustType::String,
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                RustType::Int
            }
            "f32" | "f64" => RustType::Float,
            other => RustType::Other(other.into()),
        }
    }
}

fn quote_type(ty: &syn::Type) -> String {
    quote::ToTokens::to_token_stream(ty).to_string()
}

impl RustTypes {
    /// Collects the enums of one parsed file, including those in inline
    /// modules.
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
                syn::Item::Mod(m) => {
                    if let Some((_, items)) = &m.content {
                        self.collect(items);
                    }
                }
                _ => {}
            }
        }
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
