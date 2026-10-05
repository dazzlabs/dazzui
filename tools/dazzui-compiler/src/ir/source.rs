/// A place in the Rust source: where a node was written, so that a
/// diagnostic, a source map or a generated `// source:` comment can point
/// back at it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl SourceLocation {
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: file.into(),
            line,
            column,
        }
    }

    /// `settings.rs:23` — the file's own name rather than its path, which is
    /// what a reader of generated code needs to find the line.
    pub fn short(&self) -> String {
        let name = self.file.rsplit(['/', '\\']).next().unwrap_or(&self.file);
        format!("{name}:{}", self.line)
    }
}
