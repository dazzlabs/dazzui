use super::Value;

/// A component's semantic interface — what it takes, not how any platform
/// draws it — and, beside it, how each platform draws it, as data. The
/// registry loads these from TOML.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentSpec {
    pub name: String,
    pub props: Vec<PropSpec>,
    /// Prop names positional arguments fill, in order.
    pub positional: Vec<String>,
    pub events: Vec<EventSpec>,
    pub slots: Vec<SlotSpec>,
    pub children: ChildrenSpec,
    pub swiftui: Option<SwiftUIMapping>,
    pub winui: Option<WinUIMapping>,
}

impl ComponentSpec {
    pub fn prop(&self, name: &str) -> Option<&PropSpec> {
        self.props.iter().find(|p| p.name == name)
    }

    /// The prop `.name(…)` sets, when it may be written as a modifier.
    pub fn modifier_prop(&self, name: &str) -> Option<&PropSpec> {
        self.prop(name).filter(|p| p.modifier)
    }

    pub fn event(&self, name: &str) -> Option<&EventSpec> {
        self.events.iter().find(|e| e.name == name)
    }

    pub fn event_by_modifier(&self, modifier: &str) -> Option<&EventSpec> {
        self.events.iter().find(|e| e.modifier == modifier)
    }

    pub fn slot(&self, name: &str) -> Option<&SlotSpec> {
        self.slots.iter().find(|s| s.name == name)
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

/// A named place for one node: `SettingsRow { trailing: Toggle { … } }`.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotSpec {
    pub name: String,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildrenSpec {
    None,
    /// Any number of children; only these components, when `allow` says.
    Many {
        allow: Option<Vec<String>>,
    },
}

/// How SwiftUI draws a component: a call, its arguments in the order the
/// initializer declares them, and what its trailing closure holds.
#[derive(Clone, Debug, PartialEq)]
pub struct SwiftUIMapping {
    /// `DazzUI.Button`
    pub call: String,
    pub args: Vec<SwiftArg>,
    pub trailing: Option<Trailing>,
}

/// One argument: `variant: .filled`.
#[derive(Clone, Debug, PartialEq)]
pub struct SwiftArg {
    pub arg: String,
    pub source: Source,
    /// Code written when the source is absent: `action: {}`.
    pub fallback: Option<String>,
}

/// What a trailing closure holds.
#[derive(Clone, Debug, PartialEq)]
pub enum Trailing {
    /// The component's children.
    Children,
    /// A prop wrapped in a view: `{ Text(title) }`.
    Prop {
        prop: String,
        view: String,
    },
    Slot(String),
}

/// How WinUI draws a component: an element, its attributes, its events,
/// and where its children and slots go.
#[derive(Clone, Debug, PartialEq)]
pub struct WinUIMapping {
    /// `dazz:Button`
    pub element: String,
    pub attrs: Vec<WinAttr>,
    pub events: Vec<WinEvent>,
    pub slots: Vec<(String, String)>,
    pub children: Option<WinChildren>,
}

/// One attribute: `Variant="Recessed"`.
#[derive(Clone, Debug, PartialEq)]
pub struct WinAttr {
    pub attr: String,
    pub source: Source,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WinEvent {
    pub name: String,
    /// `Click`
    pub event: String,
    /// `RoutedEventArgs`
    pub args: String,
    /// The C# that reads the value the event carries, from `sender`.
    pub value: Option<String>,
    /// A namespace that code needs.
    pub using: Option<String>,
}

/// Where children go when not straight inside the element: into a
/// property, through a wrapper.
#[derive(Clone, Debug, PartialEq)]
pub struct WinChildren {
    pub property: Option<String>,
    /// A wrapper element and the property of it the children go in:
    /// `dazz:PreferenceGroup`, `Rows`.
    pub wrap: Option<(String, Option<String>)>,
}

/// Where an argument's or attribute's value comes from.
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    Prop {
        prop: String,
        /// Semantic case → platform code.
        cases: Vec<(String, String)>,
        invert: bool,
        /// The semantic value at which the platform's own default already
        /// draws it, so the argument is left out.
        omit: Option<Value>,
    },
    Event(String),
    Slot(String),
}
