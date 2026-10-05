//! What `dazzui-compiler check` prints, for one case of each kind of
//! error: `tests/diagnostics/<case>/page.rs`, with that directory's
//! `dazzui-compiler.toml` when it has one, against its `expected.stderr`.
//! `DAZZUI_BLESS=1 cargo test --test diagnostics` writes those afresh when
//! a change to a message is meant.

use std::fs;
use std::path::Path;

use dazzui_compiler::compile;

fn check(case: &str) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/diagnostics")
        .join(case);
    let report = match compile(&dir.join("page.rs"), &dir) {
        Ok(_) => panic!("{case} compiled; it is meant to fail"),
        Err(report) => report.render(false),
    };
    let expected = dir.join("expected.stderr");
    if std::env::var_os("DAZZUI_BLESS").is_some() {
        fs::write(&expected, &report).unwrap();
        return;
    }
    let committed =
        fs::read_to_string(&expected).unwrap_or_else(|e| panic!("{}: {e}", expected.display()));
    assert_eq!(
        committed, report,
        "{case}: the diagnostics differ from expected.stderr"
    );
}

macro_rules! cases {
    ($($case:ident)*) => { $( #[test] fn $case() { check(stringify!($case)); } )* };
}

cases! {
    syntax names types state_paths actions bindings control_flow slots bridge project registry_syntax registry_mapping
}
