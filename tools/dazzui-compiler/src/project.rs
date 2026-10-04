use std::fs;
use std::path::{Path, PathBuf};

use crate::ir::{Diagnostic, Page};
use crate::syntax::{PageAst, parse_page};

use crate::error::SemanticError;
use crate::registry::Registry;
use crate::rust_types::{EnumDef, RustTypes};
use crate::semantic::lower;

/// An app crate's source as the CLI reads it: every `.rs` file under a
/// directory, the `ui!` pages in them, and the Rust types they define.
pub struct Project {
    pub files: Vec<SourceFile>,
    pub types: RustTypes,
    pages: Vec<(usize, PageAst)>,
    diagnostics: Vec<Diagnostic>,
}

pub struct SourceFile {
    /// The path as diagnostics and generated headers print it.
    pub display: String,
    pub text: String,
}

/// A page, lowered, with the Rust enum its actions come from.
pub struct CompiledPage {
    pub page: Page,
    pub file: String,
    pub actions: EnumDef,
}

impl Project {
    /// Reads every `.rs` file under `src`. Paths are displayed relative to
    /// `base`.
    pub fn scan(src: &Path, base: &Path) -> std::io::Result<Self> {
        let mut paths = Vec::new();
        walk(src, &mut paths)?;
        paths.sort();

        let mut project =
            Project { files: Vec::new(), types: RustTypes::default(), pages: Vec::new(), diagnostics: Vec::new() };
        for path in paths {
            let text = fs::read_to_string(&path)?;
            let display = path.strip_prefix(base).unwrap_or(&path).to_string_lossy().replace('\\', "/");
            let index = project.files.len();
            match syn::parse_file(&text) {
                Ok(file) => {
                    project.types.collect(&file.items);
                    let mut macros = Vec::new();
                    find_ui_macros(&file.items, &mut macros);
                    for tokens in macros {
                        match parse_page(tokens) {
                            Ok(page) => project.pages.push((index, page)),
                            Err(error) => project.push_syn(&display, error),
                        }
                    }
                }
                Err(error) => project.push_syn(&display, error),
            }
            project.files.push(SourceFile { display, text });
        }
        Ok(project)
    }

    fn push_syn(&mut self, file: &str, error: syn::Error) {
        for e in error {
            self.diagnostics.push(SemanticError::from(e).to_diagnostic(file));
        }
    }

    pub fn file(&self, display: &str) -> Option<&SourceFile> {
        self.files.iter().find(|f| f.display == display)
    }

    /// Parses, resolves and type-checks every page. Generates nothing.
    pub fn compile(&self, registry: &Registry) -> Result<Vec<CompiledPage>, Vec<Diagnostic>> {
        let mut diagnostics = self.diagnostics.clone();
        let mut pages = Vec::new();
        for (index, ast) in &self.pages {
            let file = &self.files[*index].display;
            match lower(ast, registry, Some(&self.types), file) {
                Ok(page) => {
                    let actions = self.types.enums[&page.action_type].clone();
                    pages.push(CompiledPage { page, file: file.clone(), actions });
                }
                Err(errors) => diagnostics.extend(errors.iter().map(|e| e.to_diagnostic(file))),
            }
        }
        if diagnostics.is_empty() { Ok(pages) } else { Err(diagnostics) }
    }
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// `ui! { … }` in item position — at the top of a module, or of an inline
/// module — whether written `ui!` or `dazzui_compiler::ui!`.
fn find_ui_macros(items: &[syn::Item], out: &mut Vec<proc_macro2::TokenStream>) {
    for item in items {
        match item {
            syn::Item::Macro(m) if m.mac.path.segments.last().is_some_and(|s| s.ident == "ui") => {
                out.push(m.mac.tokens.clone());
            }
            syn::Item::Mod(m) => {
                if let Some((_, items)) = &m.content {
                    find_ui_macros(items, out);
                }
            }
            _ => {}
        }
    }
}
