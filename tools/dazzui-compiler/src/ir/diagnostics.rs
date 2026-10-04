use super::SourceLocation;

/// A compiler message about the Rust source, rendered the way rustc renders
/// its own.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Option<SourceLocation>,
    /// How many columns the underline spans on `location`'s line.
    pub width: u32,
    /// The text beside the underline.
    pub label: Option<String>,
    pub help: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            location: None,
            width: 1,
            label: None,
            help: None,
        }
    }

    pub fn at(mut self, location: SourceLocation, width: u32) -> Self {
        self.location = Some(location);
        self.width = width.max(1);
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Renders the message rustc-style, quoting `source` — the text of
    /// `location`'s file — when it is given.
    pub fn render(&self, source: Option<&str>) -> String {
        let kind = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let mut out = format!("{kind}: {}\n", self.message);
        if let Some(loc) = &self.location {
            let line_text = source.and_then(|s| s.lines().nth(loc.line as usize - 1));
            let gutter = " ".repeat(loc.line.to_string().len());
            out += &format!("{gutter}--> {}:{}:{}\n", loc.file, loc.line, loc.column);
            if let Some(text) = line_text {
                let pad = " ".repeat(loc.column.saturating_sub(1) as usize);
                let marks = "^".repeat(self.width as usize);
                let label = self.label.as_deref().map(|l| format!(" {l}")).unwrap_or_default();
                out += &format!("{gutter} |\n");
                out += &format!("{} | {text}\n", loc.line);
                out += &format!("{gutter} | {pad}{marks}{label}\n");
                out += &format!("{gutter} |\n");
            }
        }
        if let Some(help) = &self.help {
            out += &format!("help: {help}\n");
        }
        out
    }
}
