//! What the `dazzui-compiler` binary does, as functions the golden tests share.

use std::path::Path;

use crate::ir::{Diagnostic, GeneratedFile};
use crate::{CompiledPage, Project, REGISTRY_FILE, Registry};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    SwiftUI,
    WinUI,
}

impl Target {
    pub const ALL: [Target; 2] = [Target::SwiftUI, Target::WinUI];

    /// The platform folder of an app whose host this target writes for,
    /// as a Flutter app has one per platform: `macos`, `windows`.
    pub fn platform(self) -> &'static str {
        match self {
            Target::SwiftUI => "macos",
            Target::WinUI => "windows",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Target::SwiftUI => "swiftui",
            Target::WinUI => "winui",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "swiftui" => Some(Target::SwiftUI),
            "winui" => Some(Target::WinUI),
            _ => None,
        }
    }
}

/// Diagnostics that stopped a compile, with the text of the files they
/// point into, so they can be rendered quoting them.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub diagnostics: Vec<Diagnostic>,
    sources: Vec<(String, String)>,
}

impl Report {
    fn source(&self, file: &str) -> Option<&str> {
        self.sources.iter().find(|(f, _)| f == file).map(|(_, t)| t.as_str())
    }

    /// Every diagnostic rustc-style, then rustc's closing line.
    pub fn render(&self, color: bool) -> String {
        let mut out = String::new();
        for d in &self.diagnostics {
            let text = d.location.as_ref().and_then(|l| self.source(&l.file));
            out += &d.render_styled(text, color);
            out.push('\n');
        }
        let n = self.diagnostics.len();
        let closing = format!("could not compile due to {n} previous error{}", if n == 1 { "" } else { "s" });
        out += &Diagnostic::error(closing).render_styled(None, color);
        out
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.render(false))
    }
}

/// Reads and compiles every page under `src` — a directory, or one file —
/// against the built-in components and those in `base`'s
/// `dazzui-compiler.toml`, with paths shown relative to `base`.
pub fn compile(src: &Path, base: &Path) -> Result<Vec<CompiledPage>, Report> {
    let registry = match Registry::load(base) {
        Ok(registry) => registry,
        Err(e) => {
            let text = std::fs::read_to_string(base.join(REGISTRY_FILE)).unwrap_or_default();
            let diagnostic = e.to_diagnostic(&text);
            return Err(Report { diagnostics: vec![diagnostic], sources: vec![(e.file.clone(), text)] });
        }
    };
    let project = Project::scan(src, base).map_err(|e| Report {
        diagnostics: vec![Diagnostic::error(format!("cannot read {}: {e}", src.display()))],
        sources: Vec::new(),
    })?;
    project.compile(&registry).map_err(|diagnostics| Report {
        diagnostics,
        sources: project.files.iter().map(|f| (f.display.clone(), f.text.clone())).collect(),
    })
}

/// What the platform side needs to know that the Rust source does not say.
#[derive(Clone, Debug)]
pub struct Options {
    /// The C# namespace the WinUI files go in.
    pub namespace: String,
    /// The native library the bridge loads: the app crate's lib name,
    /// `settings_example`.
    pub library: String,
}

pub fn generate(pages: &[CompiledPage], target: Target, options: &Options) -> Vec<GeneratedFile> {
    pages
        .iter()
        .flat_map(|page| match target {
            Target::SwiftUI => crate::swiftui::generate(page),
            Target::WinUI => crate::winui::generate(page, options),
        })
        .collect()
}

/// `settings_example`, the lib name of the crate whose `Cargo.toml` is
/// `manifest`: its `[package]` name with `-` as `_`. Read by hand rather
/// than with a TOML parser, which this one line does not warrant.
pub fn crate_lib_name(manifest: &str) -> Option<String> {
    let mut in_package = false;
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            in_package = line == "[package]";
        } else if in_package && let Some(value) = line.strip_prefix("name").map(str::trim_start) {
            let value = value.strip_prefix('=')?.trim().trim_matches('"');
            return Some(value.replace('-', "_"));
        }
    }
    None
}
