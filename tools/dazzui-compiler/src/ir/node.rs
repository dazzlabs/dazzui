use super::{ActionRef, Condition, Expr, LocalPath, SourceLocation, StatePath};

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
    /// Either branch's nodes stand in the parent as if written there.
    If { condition: Condition, then_branch: Vec<UiNode>, else_branch: Vec<UiNode>, source: SourceLocation },
    /// The children, once per element, standing in the parent as if written
    /// there. `element` is the Rust struct each element is; `key` is the
    /// field that tells one element from another.
    ForEach {
        collection: StatePath,
        item: String,
        element: String,
        key: LocalPath,
        children: Vec<UiNode>,
        source: SourceLocation,
    },
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
    pub bindings: Vec<UiBinding>,
    pub events: Vec<UiEvent>,
    pub children: Vec<UiNode>,
    pub source: SourceLocation,
}

impl Component {
    pub fn prop(&self, name: &str) -> Option<&Expr> {
        self.props.iter().find(|p| p.name == name).map(|p| &p.value)
    }

    pub fn binding(&self, name: &str) -> Option<&UiBinding> {
        self.bindings.iter().find(|b| b.name == name)
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

/// `value: bind(state.x)`: a prop that shows a state field and writes a
/// change back. The write goes through Rust — `on_change` is the action the
/// new value travels in — so the native side never changes its copy of the
/// state on its own.
#[derive(Clone, Debug, PartialEq)]
pub struct UiBinding {
    pub name: String,
    pub path: StatePath,
    pub on_change: ActionRef,
    pub source: SourceLocation,
}

/// An event wired to an action.
#[derive(Clone, Debug, PartialEq)]
pub struct UiEvent {
    pub name: String,
    pub action: ActionRef,
    pub source: SourceLocation,
}
