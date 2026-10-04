/// A literal value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
}

/// A read-only value a prop is given.
///
/// Phase 1 has literals only; state paths, loop locals and enum cases join
/// in later phases (see DESIGN.md §3).
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(Value),
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
}
