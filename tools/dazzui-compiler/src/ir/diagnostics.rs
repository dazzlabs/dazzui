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
        self.render_styled(source, false)
    }

    /// As [`Diagnostic::render`], in rustc's colours when `color` is set.
    pub fn render_styled(&self, source: Option<&str>, color: bool) -> String {
        let paint = |code: &str, text: &str| {
            if color {
                format!("\x1b[{code}m{text}\x1b[0m")
            } else {
                text.to_string()
            }
        };
        let (kind, kind_color) = match self.severity {
            Severity::Error => ("error", "1;31"),
            Severity::Warning => ("warning", "1;33"),
        };
        let mut out = format!(
            "{}{}\n",
            paint(kind_color, kind),
            paint("1", &format!(": {}", self.message))
        );
        if let Some(loc) = &self.location {
            let line_text = source.and_then(|s| s.lines().nth(loc.line.saturating_sub(1) as usize));
            let gutter = " ".repeat(loc.line.to_string().len());
            let bar = paint("1;34", "|");
            out += &format!(
                "{gutter}{} {}:{}:{}\n",
                paint("1;34", "-->"),
                loc.file,
                loc.line,
                loc.column
            );
            if let Some(text) = line_text {
                // A tab is drawn four columns wide, as rustc draws it, and
                // the underline moves with it.
                let before: String = text
                    .chars()
                    .take(loc.column.saturating_sub(1) as usize)
                    .collect();
                let pad = " ".repeat(before.chars().map(|c| if c == '\t' { 4 } else { 1 }).sum());
                let text = text.replace('\t', "    ");
                let marks = "^".repeat(self.width as usize);
                let label = self
                    .label
                    .as_deref()
                    .map(|l| format!(" {l}"))
                    .unwrap_or_default();
                out += &format!("{gutter} {bar}\n");
                out += &format!("{} {bar} {text}\n", paint("1;34", &loc.line.to_string()));
                out += &format!(
                    "{gutter} {bar} {pad}{}\n",
                    paint(kind_color, &format!("{marks}{label}"))
                );
                out += &format!("{gutter} {bar}\n");
            }
        }
        if let Some(help) = &self.help {
            out += &format!("{}: {help}\n", paint("1;36", "help"));
        }
        out
    }
}
