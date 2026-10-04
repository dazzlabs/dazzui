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

/// Reads and compiles every page under `src`, with paths shown relative to
/// `base`. On failure, the diagnostics come back rendered.
pub fn compile(src: &Path, base: &Path) -> Result<Vec<CompiledPage>, String> {
    let project = Project::scan(src, base).map_err(|e| format!("error: cannot read {}: {e}\n", src.display()))?;
    project.compile(&Registry::builtin()).map_err(|diagnostics| {
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

pub fn generate(pages: &[CompiledPage], target: Target, namespace: &str) -> Vec<GeneratedFile> {
    pages
        .iter()
        .flat_map(|page| match target {
            Target::SwiftUI => crate::swiftui::generate(page),
            Target::WinUI => crate::winui::generate(page, namespace),
        })
        .collect()
}
