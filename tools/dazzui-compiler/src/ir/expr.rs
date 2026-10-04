/// A literal value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
}

/// A read-only value a prop is given.
///
/// Loop locals join in Phase 3 (see DESIGN.md §3).
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Value),
    /// `state.user.name`
    State(StatePath),
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
