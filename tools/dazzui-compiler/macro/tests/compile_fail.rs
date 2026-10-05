//! What the macros make rustc say, for each kind of mistake: the files in
//! `ui/` each fail to compile with the `.stderr` beside them, and
//! `ui/pass.rs` compiles. `TRYBUILD=overwrite cargo test -p
//! dazzui-compiler-macro` writes the `.stderr` files afresh when a change to
//! a message is meant.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass.rs");
    t.compile_fail("tests/ui/fail_*.rs");
}
