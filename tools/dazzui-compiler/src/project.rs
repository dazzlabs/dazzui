use std::fs;
use std::path::{Path, PathBuf};

use crate::ir::{Diagnostic, Page, SourceLocation};
use crate::syntax::{PageAst, parse_page};

use crate::bridge::Bridge;
use crate::error::SemanticError;
use crate::registry::Registry;
use crate::rust_types::{RustTypes, StructDef, module_path};
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

/// Everything a project compiles to: its pages, and the page models the
/// bridge carries to the native side. A page calls its model only through
/// a protocol of the model's methods, which a host may implement natively.
pub struct Compiled {
    pub pages: Vec<CompiledPage>,
    pub models: Vec<CompiledModel>,
}

/// A page, lowered, with the Rust type its state is.
pub struct CompiledPage {
    pub page: Page,
    pub file: String,
    pub state: StructDef,
    /// The components it was compiled against, whose mappings the backends
    /// lower it with.
    pub registry: Registry,
}

/// A `#[page_model] impl`, and the bridge that carries its model.
pub struct CompiledModel {
    pub bridge: Bridge,
    /// The file the function is in, as generated headers print it.
    pub file: String,
}

impl Project {
    /// Reads every `.rs` file under `src`, or `src` itself when it is a
    /// file. Paths are displayed relative to `base`.
    pub fn scan(src: &Path, base: &Path) -> std::io::Result<Self> {
        let mut paths = Vec::new();
        if src.is_file() {
            paths.push(src.to_path_buf());
        } else {
            walk(src, &mut paths)?;
        }
        paths.sort();

        let mut project = Project {
            files: Vec::new(),
            types: RustTypes::default(),
            pages: Vec::new(),
            diagnostics: Vec::new(),
        };
        for path in paths {
            let text = fs::read_to_string(&path)?;
            let display = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            let index = project.files.len();
            match syn::parse_file(&text) {
                Ok(file) => {
                    let module = module_path(path.strip_prefix(src).unwrap_or(&path));
                    let known = project.types.models.len();
                    project.types.collect(&file.items, &module);
                    for model in &mut project.types.models[known..] {
                        model.file = display.clone();
                    }
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
            self.diagnostics
                .push(SemanticError::from(e).to_diagnostic(file));
        }
    }

    pub fn file(&self, display: &str) -> Option<&SourceFile> {
        self.files.iter().find(|f| f.display == display)
    }

    /// Parses, resolves and type-checks every page, and checks every page
    /// model's types can cross. Generates nothing.
    pub fn compile(&self, registry: &Registry) -> Result<Compiled, Vec<Diagnostic>> {
        let mut diagnostics = self.diagnostics.clone();
        let mut pages = Vec::new();
        for (index, ast) in &self.pages {
            let file = &self.files[*index].display;
            let page = match lower(ast, registry, Some(&self.types), file) {
                Ok(page) => page,
                Err(errors) => {
                    diagnostics.extend(errors.iter().map(|e| e.to_diagnostic(file)));
                    continue;
                }
            };
            pages.push(CompiledPage {
                state: self.types.structs[&page.state_type].clone(),
                page,
                file: file.clone(),
                registry: registry.clone(),
            });
        }
        let mut models: Vec<CompiledModel> = Vec::new();
        for model in &self.types.models {
            let at = |message: String| {
                Diagnostic::error(message).at(
                    SourceLocation::new(&model.file, model.line, model.column),
                    model.width,
                )
            };
            let bridge = match Bridge::new(model, &self.types) {
                Ok(bridge) => bridge,
                Err(message) => {
                    diagnostics.push(at(message));
                    continue;
                }
            };
            // The bridge's types are named from the model: two of a name, or
            // of a stem, would export the same names.
            if let Some(first) = models.iter().find(|m| m.bridge.stem == bridge.stem) {
                diagnostics.push(
                    at(format!(
                        "`{}` and `{}` would both export `{}`; give one another name",
                        first.bridge.model,
                        bridge.model,
                        bridge.store()
                    ))
                    .with_help(format!("the first is in {}", first.file)),
                );
                continue;
            }
            models.push(CompiledModel {
                bridge,
                file: model.file.clone(),
            });
        }
        // In source order, as rustc reports.
        diagnostics.sort_by(|a, b| {
            let key = |d: &Diagnostic| {
                d.location
                    .as_ref()
                    .map(|l| (l.file.clone(), l.line, l.column))
            };
            key(a).cmp(&key(b))
        });
        // A page's files are named from it: two would write over each other.
        for (i, page) in pages.iter().enumerate() {
            let earlier = &pages[..i];
            let at = || page.page.source.clone();
            if let Some(first) = earlier.iter().find(|p| p.page.name == page.page.name) {
                diagnostics.push(
                    Diagnostic::error(format!("page `{}` is defined twice", page.page.name))
                        .at(at(), page.page.name.len() as u32)
                        .with_help(format!(
                            "the first is at {}:{}",
                            first.page.source.file, first.page.source.line
                        )),
                );
            }
        }
        if diagnostics.is_empty() {
            Ok(Compiled { pages, models })
        } else {
            Err(diagnostics)
        }
    }
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        // A hidden file is never a module rustc compiles — and macOS
        // leaves `._name.rs` resource forks beside files it copies away.
        if path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            continue;
        }
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
