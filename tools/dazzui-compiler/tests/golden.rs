//! The settings example's committed `macos/Generated` and
//! `windows/Generated` are what the CLI writes today. Regenerate them with
//! `dazzui-compiler generate` from that directory when a change to a backend
//! is meant.

use std::fs;
use std::path::Path;

use dazzui_compiler::{Options, Target, compile, generate};

fn check(target: Target, namespace: &str, dir: &str) {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/settings_example");
    let pages = compile(&example.join("src"), &example).unwrap_or_else(|e| panic!("{e}"));
    let options = Options { namespace: namespace.into(), library: "settings_example".into() };
    let files = generate(&pages, target, &options);
    assert!(!files.is_empty());
    for file in files {
        let path = example.join(dir).join(&file.path);
        let committed = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert_eq!(committed, file.contents, "{} is stale", path.display());
    }
}

#[test]
fn swiftui_matches_the_committed_output() {
    check(Target::SwiftUI, "SettingsExample", "macos/Generated");
}

#[test]
fn winui_matches_the_committed_output() {
    check(Target::WinUI, "SettingsExample", "windows/Generated");
}
