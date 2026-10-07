//! The settings example's committed `macos/Generated` and
//! `windows/Generated` are what the CLI writes today. Regenerate them with
//! `dazzui-compiler generate` from that directory when a change to a backend
//! is meant. `tests/protocol_only` checks a page holds its model only as a
//! protocol.

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

/// A page holds its model only as the protocol of the methods it calls:
/// nothing in it names the store or the Rust model's native object.
#[test]
fn a_page_calls_its_model_through_a_protocol() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/protocol_only");
    let compiled = compile(&dir.join("page.rs"), &dir).unwrap_or_else(|e| panic!("{e}"));
    let options = Options {
        namespace: "Notes".into(),
        library: "notes".into(),
    };
    let swift = generate(&compiled, Target::SwiftUI, &options);
    let names: Vec<_> = swift.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(names, ["P.swift", "NotesActions.swift", "NotesStore.swift"]);
    let page = &swift[0].contents;
    assert!(
        page.contains("public let model: any NotesActions\n"),
        "{page}"
    );
    assert!(
        page.contains("onCheckedChange: { model.setOn(on: $0) }"),
        "{page}"
    );
    assert!(
        !page.contains("NotesStore") && !page.contains("NotesModel"),
        "{page}"
    );
    assert!(swift[1].contents.contains("func setOn(on: Bool)"));
    let winui = generate(&compiled, Target::WinUI, &options);
    let names: Vec<_> = winui.iter().map(|f| f.path.as_str()).collect();
    assert_eq!(
        names,
        ["P.xaml", "P.xaml.cs", "NotesActions.cs", "NotesStore.cs"]
    );
    assert!(
        winui[1]
            .contents
            .contains("public INotesActions? Model { get; set; }")
    );
    assert!(!winui[1].contents.contains("NotesStore") && !winui[1].contents.contains("NotesModel"));
    assert!(winui[2].contents.contains("void SetOn(bool on);"));
}
