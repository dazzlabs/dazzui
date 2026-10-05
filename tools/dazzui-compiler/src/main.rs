//! `dazzui-compiler check` and `dazzui-compiler generate`.

use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::ExitCode;
use std::{env, fs};

use dazzui_compiler::{Options, Target, compile, crate_lib_name, generate};

const USAGE: &str = "\
usage:
  dazzui-compiler check    [--src <dir>]
  dazzui-compiler generate --target swiftui|winui [--src <dir>] [--output <dir>]
                           [--namespace <C# ns>] [--library <lib name>]
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
    let mut output = PathBuf::from("generated");
    let mut target = None;
    let mut namespace = None;
    let mut library = None;
    let mut rest = rest.iter();
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().cloned().ok_or_else(|| format!("error: `{flag}` takes a value\n\n{USAGE}"));
        match flag.as_str() {
            "--src" => src = value()?.into(),
            "--output" => output = value()?.into(),
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
            for target in [Target::SwiftUI, Target::WinUI] {
                generate(&pages, target, &options);
            }
            eprintln!("checked {} page{}", pages.len(), if pages.len() == 1 { "" } else { "s" });
            Ok(())
        }
        "generate" => {
            let target = target.ok_or_else(|| format!("error: `generate` needs `--target`\n\n{USAGE}"))?;
            let library = match library {
                Some(library) => library,
                None => fs::read_to_string(base.join("Cargo.toml"))
                    .ok()
                    .and_then(|manifest| crate_lib_name(&manifest))
                    .ok_or("error: no `--library`, and no package name in ./Cargo.toml to take it from\n")?,
            };
            let options = Options { namespace: namespace.unwrap_or_else(|| "App".into()), library };
            let files = generate(&pages, target, &options);
            for file in &files {
                let path = output.join(&file.path);
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir).map_err(|e| format!("error: {}: {e}\n", dir.display()))?;
                }
                fs::write(&path, &file.contents).map_err(|e| format!("error: {}: {e}\n", path.display()))?;
                eprintln!("wrote {}", path.display());
            }
            Ok(())
        }
        other => Err(format!("error: unknown command `{other}`\n\n{USAGE}")),
    }
}
