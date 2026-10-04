use proc_macro2::Span;
use syn::{Ident, Lit, Path};

/// `page SettingsPage(state: SettingsState, action: SettingsAction) { … }`
#[derive(Clone, Debug)]
pub struct PageAst {
    pub name: Ident,
    pub state_ty: Path,
    pub action_ty: Path,
    pub root: NodeAst,
}

#[derive(Clone, Debug)]
pub enum NodeAst {
    Component(ComponentAst),
    If(IfAst),
    For(ForAst),
}

impl NodeAst {
    /// Where the node starts: a component's name, or its keyword.
    pub fn span(&self) -> Span {
        match self {
            NodeAst::Component(c) => c.name.span(),
            NodeAst::If(i) => i.keyword,
            NodeAst::For(f) => f.keyword,
        }
    }
}

/// `if [!]path { … } [else { … }]`; `else if` is an `if` alone in the
/// else branch.
#[derive(Clone, Debug)]
pub struct IfAst {
    pub keyword: Span,
    pub negated: bool,
    pub condition: DottedAst,
    pub then_branch: Vec<NodeAst>,
    pub else_branch: Vec<NodeAst>,
}

/// `for item in path [, key: item.field] { … }`
#[derive(Clone, Debug)]
pub struct ForAst {
    pub keyword: Span,
    pub item: Ident,
    pub collection: DottedAst,
    pub key: Option<DottedAst>,
    pub children: Vec<NodeAst>,
}

/// `Name(args) { body } .modifier(args)…`
#[derive(Clone, Debug)]
pub struct ComponentAst {
    pub name: Ident,
    pub args: Vec<ArgAst>,
    pub props: Vec<PropAst>,
    pub children: Vec<NodeAst>,
    pub modifiers: Vec<ModifierAst>,
}

/// An argument in parentheses: positional, or named like a body prop.
#[derive(Clone, Debug)]
pub enum ArgAst {
    Positional(ValueAst),
    Named(PropAst),
}

/// `name: value`, in a body or in parentheses.
#[derive(Clone, Debug)]
pub struct PropAst {
    pub name: Ident,
    pub value: ValueAst,
}

/// `.name(args)`
#[derive(Clone, Debug)]
pub struct ModifierAst {
    pub name: Ident,
    pub args: Vec<ArgAst>,
}

/// A value as written. What it means — a literal, an action, an enum case —
/// is the compiler's to decide.
#[derive(Clone, Debug)]
pub enum ValueAst {
    /// `"Save"`, `16`, `true`
    Lit(Lit),
    /// `-1`
    NegInt(Lit),
    /// `Action::Save`, `TextStyle::Title`
    Path(Path),
    /// `state.user.name`; `state` alone is a path of one.
    Dotted(DottedAst),
    /// `bind(state.x)`
    Bind(DottedAst),
    /// `Action::Select(state.current)`
    Call(Path, Box<ValueAst>),
}

/// `a.b.c`: a path of field accesses.
#[derive(Clone, Debug)]
pub struct DottedAst {
    pub segments: Vec<Ident>,
}

impl DottedAst {
    pub fn span(&self) -> Span {
        join(self.segments.first().map(Ident::span), self.segments.last().map(Ident::span))
    }
}

impl ValueAst {
    pub fn span(&self) -> Span {
        match self {
            ValueAst::Lit(lit) | ValueAst::NegInt(lit) => lit.span(),
            ValueAst::Path(path) => spanned_path(path),
            ValueAst::Dotted(path) | ValueAst::Bind(path) => path.span(),
            ValueAst::Call(path, _) => spanned_path(path),
        }
    }
}

/// The span of a whole path, first segment to last, where the compiler can
/// join them; the first segment's otherwise.
pub fn spanned_path(path: &Path) -> Span {
    join(path.segments.first().map(|s| s.ident.span()), path.segments.last().map(|s| s.ident.span()))
}

fn join(first: Option<Span>, last: Option<Span>) -> Span {
    match (first, last) {
        (Some(a), Some(b)) => a.join(b).unwrap_or(a),
        _ => Span::call_site(),
    }
}
