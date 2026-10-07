use proc_macro2::Span;
use syn::{Ident, Lit, Path};

/// `page SettingsPage(model: SettingsModel) { … }`: a widget tree that reads
/// a model's value and calls its methods, as a Flutter `build` does.
#[derive(Clone, Debug)]
pub struct PageAst {
    pub name: Ident,
    pub model_ty: Path,
    pub root: NodeAst,
}

#[derive(Clone, Debug)]
pub enum NodeAst {
    Component(ComponentAst),
    /// In a `children: [ … ]` list, as Flutter's collection `if` is.
    If(IfAst),
    /// In a `children: [ … ]` list, as Flutter's collection `for` is.
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

/// `if [!]path { a, b } [else { c }]`; `else if` is an `if` alone in the
/// else branch.
#[derive(Clone, Debug)]
pub struct IfAst {
    pub keyword: Span,
    pub negated: bool,
    pub condition: DottedAst,
    pub then_branch: Vec<NodeAst>,
    pub else_branch: Vec<NodeAst>,
}

/// `for item in path [, key: item.field] { a, b }`
#[derive(Clone, Debug)]
pub struct ForAst {
    pub keyword: Span,
    pub item: Ident,
    pub collection: DottedAst,
    pub key: Option<DottedAst>,
    pub children: Vec<NodeAst>,
}

/// `Name(prop: value, …, child: …)` or `Name(…, children: [ … ])`, as a
/// Flutter widget is constructed; `Text("…")` takes its text first, as
/// Flutter's does. The names are React's.
#[derive(Clone, Debug)]
pub struct ComponentAst {
    pub name: Ident,
    /// `Text("Save")`, `Text(model.status)`
    pub text: Option<ValueAst>,
    pub props: Vec<PropAst>,
    /// How the children were given, and where: `child:` or `children:`.
    pub children_arg: Option<(ChildrenArg, Span)>,
    pub children: Vec<NodeAst>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildrenArg {
    /// `child: Widget(…)`
    One,
    /// `children: [ … ]`
    Many,
}

/// `name: value`
#[derive(Clone, Debug)]
pub struct PropAst {
    pub name: Ident,
    pub value: ValueAst,
}

/// A value as written. What it means — a literal, a method to call, an
/// enum case — is the compiler's to decide.
#[derive(Clone, Debug)]
pub enum ValueAst {
    /// `"Save"`, `16`, `true`
    Lit(Lit),
    /// `-1`
    NegInt(Lit),
    /// `ButtonVariant::Filled`
    Path(Path),
    /// `state.user.name`, `model.save`; `state` alone is a path of one.
    Dotted(DottedAst),
    /// `model.pick(item.id)`
    Call(DottedAst, Box<ValueAst>),
    /// `icon: Icon(…)`: a component given to a prop that takes one.
    Component(Box<ComponentAst>),
}

/// `a.b.c`: a path of field accesses.
#[derive(Clone, Debug)]
pub struct DottedAst {
    pub segments: Vec<Ident>,
}

impl DottedAst {
    pub fn span(&self) -> Span {
        join(
            self.segments.first().map(Ident::span),
            self.segments.last().map(Ident::span),
        )
    }
}

impl ValueAst {
    pub fn span(&self) -> Span {
        match self {
            ValueAst::Lit(lit) | ValueAst::NegInt(lit) => lit.span(),
            ValueAst::Path(path) => spanned_path(path),
            ValueAst::Dotted(path) | ValueAst::Call(path, _) => path.span(),
            ValueAst::Component(c) => c.name.span(),
        }
    }
}

/// The span of a whole path, first segment to last, where the compiler can
/// join them; the first segment's otherwise.
pub fn spanned_path(path: &Path) -> Span {
    join(
        path.segments.first().map(|s| s.ident.span()),
        path.segments.last().map(|s| s.ident.span()),
    )
}

fn join(first: Option<Span>, last: Option<Span>) -> Span {
    match (first, last) {
        (Some(a), Some(b)) => a.join(b).unwrap_or(a),
        _ => Span::call_site(),
    }
}
