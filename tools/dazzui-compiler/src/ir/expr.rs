/// A literal value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
}

/// A read-only value a prop is given.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Value),
    /// `state.user.name`
    State(StatePath),
    /// `item.title`, inside `for item in …`
    Local(LocalPath),
    /// `TextStyle::Title`
    EnumCase { ty: String, case: String },
}

/// A path into the page's state, without the leading `state`:
/// `state.user.name` is `["user", "name"]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatePath {
    pub segments: Vec<String>,
}

impl StatePath {
    /// `user.name`, as the DSL wrote it after `state.`.
    pub fn dotted(&self) -> String {
        self.segments.join(".")
    }
}

/// A path into a loop's element: `item.owner.name` is `item` and
/// `["owner", "name"]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPath {
    pub local: String,
    pub segments: Vec<String>,
}

/// What an `if` tests: a bool, read from the state or a loop element, and
/// whether it is negated.
#[derive(Clone, Debug, PartialEq)]
pub struct Condition {
    pub negated: bool,
    /// `Expr::State` or `Expr::Local`, of a bool.
    pub value: Expr,
}

/// An action an event sends back to Rust: a variant of the page's action
/// enum.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionRef {
    pub variant: String,
    pub payload: ActionPayload,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionPayload {
    /// `Action::Save`
    None,
    /// `on_change: Action::SetX` — the value the event carries is the
    /// payload.
    EventValue,
    /// `Action::Select(state.current)`
    Expr(Expr),
}
