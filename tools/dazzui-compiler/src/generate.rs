//! What the `dazzui-compiler` binary does, as functions the golden tests share.

use std::path::Path;

use crate::{CompiledPage, Project, Registry};
use crate::ir::GeneratedFile;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    SwiftUI,
    WinUI,
}

impl Target {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "swiftui" => Some(Target::SwiftUI),
            "winui" => Some(Target::WinUI),
            _ => None,
        }
    }
}

/// Reads and compiles every page under `src`, against the built-in
/// components and those in `base`'s `dazzui-compiler.toml`, with paths
/// shown relative to `base`. On failure, the diagnostics come back
/// rendered.
pub fn compile(src: &Path, base: &Path) -> Result<Vec<CompiledPage>, String> {
    let registry = Registry::load(base).map_err(|e| format!("error: {e}\n"))?;
    let project = Project::scan(src, base).map_err(|e| format!("error: cannot read {}: {e}\n", src.display()))?;
    project.compile(&registry).map_err(|diagnostics| {
        let mut out = String::new();
        for d in &diagnostics {
            let text = d.location.as_ref().and_then(|l| project.file(&l.file)).map(|f| f.text.as_str());
            out += &d.render(text);
            out.push('\n');
        }
        let n = diagnostics.len();
        out += &format!("error: could not compile due to {n} previous error{}\n", if n == 1 { "" } else { "s" });
        out
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
