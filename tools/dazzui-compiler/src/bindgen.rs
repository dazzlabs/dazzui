//! The UniFFI bindings `generate` writes beside the pages: UniFFI's Swift,
//! generated in process, and uniffi-bindgen-cs's C#. Both read the bridge
//! from the app's built library, so the app crate is built first.
//!
//! The files come back as [`GeneratedFile`]s, for the CLI to write where it
//! writes the pages:
//!
//! - Swift: `<lib>.swift`, and the C module it imports as
//!   `<lib>FFI/<lib>FFI.h` and `<lib>FFI/module.modulemap`.
//! - C#: `<lib>.cs`, in the pages' namespace.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use camino::Utf8PathBuf;

use crate::generate::{Options, Target};
use crate::ir::GeneratedFile;

/// Builds the crate whose manifest is in `dir` and generates `target`'s
/// bindings from its library.
pub fn bindings(dir: &Path, target: Target, options: &Options) -> Result<Vec<GeneratedFile>, String> {
    let library = build(dir, &options.library)?;
    let scratch = env::temp_dir().join(format!("dazzui-compiler-{}-{}", std::process::id(), target.name()));
    let _ = fs::remove_dir_all(&scratch);
    fs::create_dir_all(&scratch).map_err(|e| format!("{}: {e}", scratch.display()))?;
    let files = match target {
        Target::SwiftUI => swift(dir, &library, &scratch, &options.library),
        Target::WinUI => csharp(dir, &library, &scratch, options),
    };
    let _ = fs::remove_dir_all(&scratch);
    files
}

/// `cargo build`s the crate in `dir` and returns its cdylib.
fn build(dir: &Path, library: &str) -> Result<PathBuf, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(&cargo)
        .args(["build", "--quiet", "--lib"])
        .current_dir(dir)
        .status()
        .map_err(|e| format!("cannot run cargo: {e}"))?;
    if !status.success() {
        return Err("`cargo build` failed; the bindings are read from the built library".into());
    }
    let metadata = Command::new(&cargo)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(dir)
        .output()
        .map_err(|e| format!("cannot run cargo metadata: {e}"))?;
    let target_dir = target_directory(&String::from_utf8_lossy(&metadata.stdout))
        .ok_or("cannot find the target directory in `cargo metadata`")?;
    let file = if cfg!(target_os = "windows") {
        format!("{library}.dll")
    } else if cfg!(target_os = "macos") {
        format!("lib{library}.dylib")
    } else {
        format!("lib{library}.so")
    };
    let path = PathBuf::from(target_dir).join("debug").join(file);
    if path.exists() { Ok(path) } else { Err(format!("no library at {}: is the crate a cdylib?", path.display())) }
}

/// `"target_directory":"…"` from cargo metadata's JSON, unescaped — enough
/// JSON for one string field.
fn target_directory(json: &str) -> Option<String> {
    let start = json.find("\"target_directory\":\"")? + "\"target_directory\":\"".len();
    let mut out = String::new();
    let mut chars = json[start..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => out.push(chars.next()?),
            c => out.push(c),
        }
    }
    None
}

fn swift(dir: &Path, library: &Path, scratch: &Path, lib: &str) -> Result<Vec<GeneratedFile>, String> {
    let utf8 = |p: &Path| Utf8PathBuf::from_path_buf(p.to_path_buf()).map_err(|p| format!("not UTF-8: {}", p.display()));
    // UniFFI finds the crate's config through cargo metadata, from here.
    let previous = env::current_dir().map_err(|e| e.to_string())?;
    env::set_current_dir(dir).map_err(|e| e.to_string())?;
    let result = uniffi_bindgen::bindings::generate(uniffi_bindgen::bindings::GenerateOptions {
        languages: vec![uniffi_bindgen::bindings::TargetLanguage::Swift],
        source: utf8(library)?,
        out_dir: utf8(scratch)?,
        config_override: None,
        format: false,
        crate_filter: Some(lib.into()),
        metadata_no_deps: true,
    });
    env::set_current_dir(previous).map_err(|e| e.to_string())?;
    result.map_err(|e| format!("UniFFI could not generate Swift: {e:#}"))?;

    let read = |name: &str| fs::read_to_string(scratch.join(name)).map_err(|e| format!("UniFFI wrote no {name}: {e}"));
    let ffi = format!("{lib}FFI");
    Ok(vec![
        GeneratedFile::new(format!("{lib}.swift"), read(&format!("{lib}.swift"))?),
        GeneratedFile::new(format!("{ffi}/{ffi}.h"), read(&format!("{ffi}.h"))?),
        GeneratedFile::new(format!("{ffi}/module.modulemap"), read(&format!("{ffi}.modulemap"))?),
    ])
}

/// uniffi-bindgen-cs, from PATH or `DAZZUI_UNIFFI_BINDGEN_CS`, with a config
/// that puts the bindings in the pages' namespace.
fn csharp(dir: &Path, library: &Path, scratch: &Path, options: &Options) -> Result<Vec<GeneratedFile>, String> {
    let lib = &options.library;
    let config = scratch.join("uniffi.toml");
    let toml = format!(
        "[bindings.csharp]\nnamespace = \"{}\"\ncdylib_name = \"{lib}\"\naccess_modifier = \"public\"\n",
        options.namespace
    );
    fs::write(&config, toml).map_err(|e| e.to_string())?;
    let program = env::var_os("DAZZUI_UNIFFI_BINDGEN_CS").unwrap_or_else(|| "uniffi-bindgen-cs".into());
    let output = Command::new(&program)
        .arg("--library")
        .arg(library)
        .args(["--crate", lib, "--no-format", "--config"])
        .arg(&config)
        .arg("--out-dir")
        .arg(scratch)
        .current_dir(dir)
        .output()
        .map_err(|e| {
            format!(
                "cannot run uniffi-bindgen-cs ({e}); install it with `cargo install uniffi-bindgen-cs \
                 --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.11.0+v0.31.0`"
            )
        })?;
    if !output.status.success() {
        return Err(format!("uniffi-bindgen-cs failed:\n{}", String::from_utf8_lossy(&output.stderr)));
    }
    let name = format!("{lib}.cs");
    let text = fs::read_to_string(scratch.join(&name)).map_err(|e| format!("uniffi-bindgen-cs wrote no {name}: {e}"))?;
    Ok(vec![GeneratedFile::new(name, text)])
}

#[cfg(test)]
mod tests {
    use super::target_directory;

    #[test]
    fn reads_the_target_directory() {
        let json = r#"{"packages":[],"target_directory":"C:\\Users\\me\\target","version":1}"#;
        assert_eq!(target_directory(json).as_deref(), Some(r"C:\Users\me\target"));
    }
}
