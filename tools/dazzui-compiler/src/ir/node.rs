use super::{Condition, Expr, LocalPath, MethodCall, SourceLocation, StatePath};

/// One `ui!` page: a root node, the Rust type of the state it reads, and of
/// the model whose methods it calls.
#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub name: String,
    pub state_type: String,
    pub model_type: String,
    pub root: UiNode,
    pub source: SourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UiNode {
    Component(Component),
    /// Either branch's nodes stand in the parent as if written there.
    If {
        condition: Condition,
        then_branch: Vec<UiNode>,
        else_branch: Vec<UiNode>,
        source: SourceLocation,
    },
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

/// The name of a component — `Button`, `Row` — as the React kit names it,
/// and the key it is found by in the registry.
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
    pub slots: Vec<UiSlot>,
    pub children: Vec<UiNode>,
    /// A text child's runs, in order: `Button(…) { "Save" }`,
    /// `Text { state.status }`.
    pub text: Vec<Expr>,
    pub source: SourceLocation,
}

impl Component {
    pub fn prop(&self, name: &str) -> Option<&Expr> {
        self.props.iter().find(|p| p.name == name).map(|p| &p.value)
    }

    pub fn event(&self, name: &str) -> Option<&MethodCall> {
        self.events.iter().find(|e| e.name == name).map(|e| &e.call)
    }

    pub fn slot(&self, name: &str) -> Option<&UiNode> {
        self.slots.iter().find(|s| s.name == name).map(|s| &s.node)
    }
}

/// `icon: Icon(…)`: a component given to a prop that takes one.
#[derive(Clone, Debug, PartialEq)]
pub struct UiSlot {
    pub name: String,
    pub node: UiNode,
    pub source: SourceLocation,
}

/// A read-only value given to a prop.
#[derive(Clone, Debug, PartialEq)]
pub struct UiProp {
    pub name: String,
    pub value: Expr,
    pub source: SourceLocation,
}

/// An event wired to a model method: `on_click: model.save`. A control's
/// value is never written on the native side: a change goes to the model,
/// and the new value comes back in the next snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct UiEvent {
    pub name: String,
    pub call: MethodCall,
    pub source: SourceLocation,
}
