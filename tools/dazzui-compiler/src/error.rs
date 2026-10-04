use crate::ir::{Diagnostic, SourceLocation};
use proc_macro2::Span;

/// A semantic error, still holding the span it is about. The macro turns it
/// into a `syn::Error` at that span; the CLI turns it into a [`Diagnostic`]
/// with the span's line and column.
#[derive(Clone, Debug)]
pub struct SemanticError {
    pub span: Span,
    pub message: String,
    pub label: Option<String>,
    pub help: Option<String>,
}

impl SemanticError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self { span, message: message.into(), label: None, help: None }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn help_opt(mut self, help: Option<String>) -> Self {
        self.help = help;
        self
    }

    pub fn to_syn(&self) -> syn::Error {
        match &self.help {
            Some(help) => syn::Error::new(self.span, format!("{}\n\nhelp: {help}", self.message)),
            None => syn::Error::new(self.span, &self.message),
        }
    }

    pub fn to_diagnostic(&self, file: &str) -> Diagnostic {
        let mut d = Diagnostic::error(&self.message);
        let (location, width) = locate(file, self.span);
        d = d.at(location, width);
        d.label = self.label.clone();
        d.help = self.help.clone();
        d
    }
}

impl From<syn::Error> for SemanticError {
    fn from(error: syn::Error) -> Self {
        SemanticError::new(error.span(), error.to_string())
    }
}

/// Where a span starts, and how wide its underline is on that line.
pub fn locate(file: &str, span: Span) -> (SourceLocation, u32) {
    let start = span.start();
    let end = span.end();
    let width = if end.line == start.line { end.column.saturating_sub(start.column) } else { 1 };
    (SourceLocation::new(file, start.line as u32, start.column as u32 + 1), width as u32)
}

/// The candidate closest to `name`, if any is close enough to be a typo.
pub fn closest<'a>(name: &str, candidates: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    candidates
        .into_iter()
        .map(|c| (strsim::jaro_winkler(name, c), c))
        .filter(|(score, _)| *score >= 0.8)
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, c)| c)
}

/// `did you mean `…`?` for the closest candidate, if there is one.
pub fn suggest<'a>(name: &str, candidates: impl IntoIterator<Item = &'a str>) -> Option<String> {
    closest(name, candidates).map(|c| format!("did you mean `{c}`?"))
}
