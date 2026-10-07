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
    /// `ButtonVariant::Filled`
    EnumCase {
        ty: String,
        case: String,
    },
}

impl Expr {
    /// Whether this is `omit`, the value a platform's own default already
    /// draws, so that a mapping leaves the argument out.
    pub fn is_omitted(&self, omit: Option<&Value>) -> bool {
        match (self, omit) {
            (Expr::Literal(v), Some(omit)) => v == omit,
            (Expr::EnumCase { case, .. }, Some(Value::String(omit))) => case == omit,
            _ => false,
        }
    }
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

/// What an event calls: a method of the page's model, `model.save`, and
/// the one value it is given, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall {
    pub method: String,
    /// The Rust name of the method's parameter, when it has one: Swift
    /// labels the argument with it. Known when the model's methods are.
    pub param: Option<String>,
    pub argument: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    /// `on_click: model.save`
    None,
    /// `on_checked_change: model.set_on` — the value the event carries is
    /// the argument.
    EventValue,
    /// `on_click: model.pick(item.id)`
    Expr(Expr),
}
