use super::{ActionRef, Expr, SourceLocation};

/// One `ui!` page: a root node, and the Rust types its state and actions
/// are.
#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub name: String,
    pub state_type: String,
    pub action_type: String,
    pub root: UiNode,
    pub source: SourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiNode {
    Component(Component),
}

/// The semantic name of a component — `VStack`, `Button` — and the key it
/// is found by in the registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    pub id: ComponentId,
    pub props: Vec<UiProp>,
    pub events: Vec<UiEvent>,
    pub children: Vec<UiNode>,
    pub source: SourceLocation,
}

impl Component {
    pub fn prop(&self, name: &str) -> Option<&Expr> {
        self.props.iter().find(|p| p.name == name).map(|p| &p.value)
    }

    pub fn event(&self, name: &str) -> Option<&ActionRef> {
        self.events.iter().find(|e| e.name == name).map(|e| &e.action)
    }
}

/// A read-only value given to a prop.
#[derive(Clone, Debug, PartialEq)]
pub struct UiProp {
    pub name: String,
    pub value: Expr,
    pub source: SourceLocation,
}

/// An event wired to an action.
#[derive(Clone, Debug, PartialEq)]
pub struct UiEvent {
    pub name: String,
    pub action: ActionRef,
    pub source: SourceLocation,
}
