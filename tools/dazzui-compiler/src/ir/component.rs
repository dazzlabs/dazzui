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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropType {
    String,
    Bool,
    Int,
}

impl PropType {
    pub fn name(self) -> &'static str {
        match self {
            PropType::String => "String",
            PropType::Bool => "Bool",
            PropType::Int => "Integer",
        }
    }

    /// The Rust type a value of this prop is, for the macro's type check.
    pub fn rust_type(self) -> &'static str {
        match self {
            PropType::String => "&str",
            PropType::Bool => "bool",
            PropType::Int => "i64",
        }
    }
}

/// An event a component raises, and the modifier that wires it:
/// `click` / `.on_click(…)`.
#[derive(Clone, Debug, PartialEq)]
pub struct EventSpec {
    pub name: String,
    pub modifier: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildrenSpec {
    None,
    Many,
}
