use std::{env, path::PathBuf, process::Command};

fn main() {
    let source = "src/generated/icons";
    println!("cargo:rerun-if-changed={source}");
    let target = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("icons.gresource");
    let status = Command::new("glib-compile-resources")
        .arg(format!("{source}/icons.gresource.xml"))
        .arg(format!("--sourcedir={source}"))
        .arg("--target")
        .arg(target)
        .status()
        .expect("GTK's glib-compile-resources must be available");
    assert!(status.success(), "Failed to embed the Fluent glyphs");
}
