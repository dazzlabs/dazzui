//! The settings example's committed `macos/Generated` and
//! `windows/Generated` are what the CLI writes today. Regenerate them with
//! `dazzui-compiler generate` from that directory when a change to a backend
//! is meant. The view-only case checks a page with no page model.

use std::fs;
use std::path::Path;

use dazzui_compiler::{Options, Target, compile, generate};

fn check(target: Target, namespace: &str, dir: &str) {
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/settings_example");
    let pages = compile(&example.join("src"), &example).unwrap_or_else(|e| panic!("{e}"));
    let options = Options {
        namespace: namespace.into(),
        library: "settings_example".into(),
    };
    let files = generate(&pages, target, &options);
    assert!(!files.is_empty());
    for file in files {
        let path = example.join(dir).join(&file.path);
        let committed =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
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

/// A page with no page model compiles to its view alone: no store, and
/// nothing that names the bridge.
#[test]
fn a_page_needs_no_model() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/view_only");
    let compiled = compile(&dir.join("page.rs"), &dir).unwrap_or_else(|e| panic!("{e}"));
    assert!(compiled.models.is_empty());
    let options = Options {
        namespace: "ViewOnly".into(),
        library: "view_only".into(),
    };
    let swift = generate(&compiled, Target::SwiftUI, &options);
    let names: Vec<_> = swift.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(names, ["P.swift"]);
    let page = &swift[0].contents;
    assert!(page.contains("public let state: S\n"), "{page}");
    assert!(
        page.contains("onCheckedChange: { send(.setOn($0)) }"),
        "{page}"
    );
    assert!(
        !page.contains("Store") && !page.contains("Listener"),
        "{page}"
    );
    let winui = generate(&compiled, Target::WinUI, &options);
    let names: Vec<_> = winui.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(names, ["P.xaml", "P.xaml.cs"]);
    assert!(
        winui[1]
            .contents
            .contains("public Action<A>? Send { get; set; }")
    );
}
