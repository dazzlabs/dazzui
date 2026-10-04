use super::Value;

/// A component's semantic interface: what it takes, not how any platform
/// draws it. Phase 4 loads these from a TOML registry; until then the
/// compiler carries the built-in ones.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentSpec {
    pub name: String,
    pub props: Vec<PropSpec>,
    /// Prop names positional arguments fill, in order.
    pub positional: Vec<String>,
    pub events: Vec<EventSpec>,
    pub children: ChildrenSpec,
}

impl ComponentSpec {
    pub fn prop(&self, name: &str) -> Option<&PropSpec> {
        self.props.iter().find(|p| p.name == name)
    }

    /// The prop `.name(…)` sets, when it may be written as a modifier.
    pub fn modifier_prop(&self, name: &str) -> Option<&PropSpec> {
        self.prop(name).filter(|p| p.modifier)
    }

    pub fn event_by_modifier(&self, modifier: &str) -> Option<&EventSpec> {
        self.events.iter().find(|e| e.modifier == modifier)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropSpec {
    pub name: String,
    pub ty: PropType,
    pub required: bool,
    pub default: Option<Value>,
    /// Whether `.name(value)` sets it too, as well as `name: value`.
    pub modifier: bool,
    /// Whether it takes `bind(state.x)`.
    pub bindable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropType {
    String,
    Bool,
    Int,
    Enum(EnumSpec),
}

/// A closed set of semantic cases, `ButtonStyle::Primary`, that each
/// backend maps to its platform's own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumSpec {
    pub name: String,
    pub cases: Vec<String>,
}

impl PropType {
    pub fn name(&self) -> String {
        match self {
            PropType::String => "String".into(),
            PropType::Bool => "Bool".into(),
            PropType::Int => "Integer".into(),
            PropType::Enum(e) => e.name.clone(),
        }
    }

    /// The Rust type a state value given to this prop must be, for the
    /// macro's type check. An enum's cases are checked by the compiler,
    /// not rustc, so it has none.
    pub fn rust_type(&self) -> Option<&'static str> {
        match self {
            PropType::String => Some("&str"),
            PropType::Bool => Some("bool"),
            PropType::Int => Some("i64"),
            PropType::Enum(_) => None,
        }
    }
}

/// An event a component raises, the modifier that wires it — `click` /
/// `.on_click(…)` — and the value it carries, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct EventSpec {
    pub name: String,
    pub modifier: String,
    pub payload: Option<PropType>,
    /// The bindable prop whose write-back this event is: a Toggle's
    /// `change` carries the new `value`.
    pub binds: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildrenSpec {
    None,
    Many,
}
