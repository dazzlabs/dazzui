//! `dazzui-compiler check` and `dazzui-compiler generate`.

use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use dazzui_compiler::ir::pascal_case;
use dazzui_compiler::{Options, Target, compile, crate_lib_name, generate};

const USAGE: &str = "\
usage:
  dazzui-compiler check    [--src <dir>]
  dazzui-compiler generate [--target swiftui|winui] [--src <dir>] [--output <dir>]
                           [--namespace <C# ns>] [--library <lib name>]

`generate` writes each target into its platform folder's Generated/ —
macos/Generated for swiftui, windows/Generated for winui — and, without
--target, writes every target whose platform folder is there. --output
takes one --target. It builds the crate first: UniFFI reads the bridge's
bindings from the built library (C# through uniffi-bindgen-cs, on PATH or
in DAZZUI_UNIFFI_BINDGEN_CS).
";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprint!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(USAGE.into());
    };
    let mut src = PathBuf::from("src");
    let mut output: Option<PathBuf> = None;
    let mut target = None;
    let mut namespace = None;
    let mut library = None;
    let mut rest = rest.iter();
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().cloned().ok_or_else(|| format!("error: `{flag}` takes a value\n\n{USAGE}"));
        match flag.as_str() {
            "--src" => src = value()?.into(),
            "--output" => output = Some(value()?.into()),
            "--namespace" => namespace = Some(value()?),
            "--library" => library = Some(value()?),
            "--target" => {
                let name = value()?;
                target = Some(Target::parse(&name).ok_or_else(|| format!("error: unknown target `{name}`\n\n{USAGE}"))?);
            }
            other => return Err(format!("error: unknown argument `{other}`\n\n{USAGE}")),
        }
    }

    let base = env::current_dir().map_err(|e| e.to_string())?;
    // rustc's colours on a terminal, unless NO_COLOR asks otherwise.
    let color = std::io::stderr().is_terminal() && env::var_os("NO_COLOR").is_none();
    let pages = compile(&src, &base).map_err(|report| report.render(color))?;
    match command.as_str() {
        "check" => {
            // Both backends run too, their output dropped, so that a page
            // `check` passes is one `generate` writes for either target.
            let options = Options { namespace: "Check".into(), library: "check".into() };
            for target in Target::ALL {
                generate(&pages, target, &options);
            }
            eprintln!("checked {} page{}", pages.len(), if pages.len() == 1 { "" } else { "s" });
            Ok(())
        }
        "generate" => {
            let package = fs::read_to_string(base.join("Cargo.toml")).ok().and_then(|m| crate_lib_name(&m));
            let library = library
                .or_else(|| package.clone())
                .ok_or("error: no `--library`, and no package name in ./Cargo.toml to take it from\n")?;
            // `settings_example` → `SettingsExample`
            let namespace = namespace.or_else(|| package.map(|p| pascal_case(&p))).unwrap_or_else(|| "App".into());
            let options = Options { namespace, library };
            let targets: Vec<Target> = match target {
                Some(target) => vec![target],
                None if output.is_some() => return Err(format!("error: `--output` takes one `--target`\n\n{USAGE}")),
                None => {
                    let found: Vec<Target> = Target::ALL.into_iter().filter(|t| base.join(t.platform()).is_dir()).collect();
                    if found.is_empty() {
                        return Err(format!("error: no `macos/` or `windows/` here to generate into; pass `--target`\n\n{USAGE}"));
                    }
                    found
                }
            };
            for target in targets {
                let dir = output.clone().unwrap_or_else(|| Path::new(target.platform()).join("Generated"));
                // The pages and their view models, then UniFFI's bindings for
                // the bridge they talk to, from the freshly built library.
                let mut files = generate(&pages, target, &options);
                files.extend(dazzui_compiler::bindgen::bindings(&base, target, &options).map_err(|e| format!("error: {e}\n"))?);
                for file in files {
                    let path = dir.join(&file.path);
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent).map_err(|e| format!("error: {}: {e}\n", parent.display()))?;
                    }
                    fs::write(&path, &file.contents).map_err(|e| format!("error: {}: {e}\n", path.display()))?;
                    eprintln!("wrote {}", path.display());
                }
            }
            Ok(())
        }
        other => Err(format!("error: unknown command `{other}`\n\n{USAGE}")),
    }
}
