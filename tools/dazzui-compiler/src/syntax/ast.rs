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
}

impl ValueAst {
    pub fn span(&self) -> Span {
        match self {
            ValueAst::Lit(lit) | ValueAst::NegInt(lit) => lit.span(),
            ValueAst::Path(path) => spanned_path(path),
        }
    }
}

/// The span of a whole path, first segment to last, where the compiler can
/// join them; the first segment's otherwise.
pub fn spanned_path(path: &Path) -> Span {
    let first = path.segments.first().map(|s| s.ident.span());
    let last = path.segments.last().map(|s| s.ident.span());
    match (first, last) {
        (Some(a), Some(b)) => a.join(b).unwrap_or(a),
        _ => Span::call_site(),
    }
}
