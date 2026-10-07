use super::Value;

/// A component's interface — its props, events, slots and children, named
/// as the React kit names them — and, beside it, how each platform draws
/// it, as data. The registry loads these from TOML.
#[derive(Clone, Debug, PartialEq)]
pub struct ComponentSpec {
    pub name: String,
    pub props: Vec<PropSpec>,
    pub events: Vec<EventSpec>,
    pub slots: Vec<SlotSpec>,
    pub children: ChildrenSpec,
    /// React props the compiler does not draw yet: named so that a page
    /// setting one is told so, rather than that it does not exist.
    pub unsupported: Vec<String>,
    pub swiftui: Option<SwiftUIMapping>,
    pub winui: Option<WinUIMapping>,
}

impl ComponentSpec {
    pub fn prop(&self, name: &str) -> Option<&PropSpec> {
        self.props.iter().find(|p| p.name == name)
    }

    pub fn event(&self, name: &str) -> Option<&EventSpec> {
        self.events.iter().find(|e| e.name == name)
    }

    pub fn slot(&self, name: &str) -> Option<&SlotSpec> {
        self.slots.iter().find(|s| s.name == name)
    }

    /// Every name a page may give it: props, events, then slots.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.props
            .iter()
            .map(|p| p.name.as_str())
            .chain(self.events.iter().map(|e| e.name.as_str()))
            .chain(self.slots.iter().map(|s| s.name.as_str()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropSpec {
    pub name: String,
    pub ty: PropType,
    pub required: bool,
    pub default: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropType {
    String,
    Bool,
    Int,
    Enum(EnumSpec),
    /// A spacing step as the web writes it, `"3"` or `"0-5"`: a literal,
    /// which each backend reads from the theme.
    Spacing,
}

/// The spacing steps a theme has, as the web names them.
pub const SPACING_STEPS: &[&str] = &[
    "0", "px", "0-5", "1", "1-5", "2", "2-5", "3", "3-5", "4", "5", "6", "7", "8", "9", "10", "11",
    "12", "14", "16", "20",
];

/// A closed set of cases, `ButtonVariant::Filled`: a React string union's
/// values, in PascalCase, which each backend maps to its platform's own.
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
            PropType::Spacing => "a spacing step".into(),
        }
    }

    /// The Rust type a state value given to this prop must be, for the
    /// macro's type check. An enum's cases and a spacing step are checked
    /// by the compiler, not rustc, so they have none.
    pub fn rust_type(&self) -> Option<&'static str> {
        match self {
            PropType::String => Some("&str"),
            PropType::Bool => Some("bool"),
            PropType::Int => Some("i64"),
            PropType::Enum(_) | PropType::Spacing => None,
        }
    }
}

/// An event a component raises — `on_click`, `on_checked_change`, React's
/// callback prop in snake case — and the value it carries, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct EventSpec {
    pub name: String,
    pub payload: Option<PropType>,
}

/// A prop that takes a component, as a React prop takes a node:
/// `PreferenceRow(icon: Icon(…))`.
#[derive(Clone, Debug, PartialEq)]
pub struct SlotSpec {
    pub name: String,
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildrenSpec {
    None,
    /// Its text, given first as Flutter's `Text` takes it: `Text("Save")`.
    Text,
    /// `child: …`, at most one: a `PreferenceRow`'s control, a `Button`'s
    /// label; only these components, when `allow` says.
    One { allow: Option<Vec<String>> },
    /// `children: [ … ]`; only these components, when `allow` says.
    Many { allow: Option<Vec<String>> },
}

impl ChildrenSpec {
    pub fn allow(&self) -> Option<&[String]> {
        match self {
            ChildrenSpec::One { allow } | ChildrenSpec::Many { allow } => allow.as_deref(),
            _ => None,
        }
    }
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
    /// Its text, as a `Text`.
    Text,
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
/// property, through a wrapper. `text` names the attribute a plain `Text`
/// child is written in instead, `Content="Save"`.
#[derive(Clone, Debug, PartialEq)]
pub struct WinChildren {
    pub property: Option<String>,
    pub text: Option<String>,
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
    /// The component's children, as a labelled closure: `trailing: { … }`.
    Children,
}
