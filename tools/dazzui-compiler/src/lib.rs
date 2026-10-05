//! Compiles `ui!` pages, written in Rust, to SwiftUI and WinUI source.
//!
//! [`syntax`] parses a page; [`lower`] resolves it against the [`Registry`]
//! and, when they are at hand, the app's Rust types, into the platform-free
//! [`ir`]; [`swiftui`] and [`winui`] lower that to source. The `ui!` macro
//! runs the same parser and checks without the Rust types, and leaves that
//! last check to rustc. [`bridge`] is what crosses to the native side, as
//! the UniFFI exports the `#[app]` macro emits; [`bindgen`] has UniFFI write
//! the Swift and C# for them.

#[cfg(feature = "bindgen")]
pub mod bindgen;
pub mod bridge;
pub mod ir;
pub mod swiftui;
pub mod syntax;
pub mod winui;

mod error;
mod generate;
mod project;
mod registry;
mod rust_types;
mod semantic;

pub use error::SemanticError;
pub use generate::{Options, Report, Target, compile, crate_lib_name, generate};
pub use project::{CompiledPage, Project, SourceFile};
pub use registry::{FILE as REGISTRY_FILE, Registry, RegistryError};
pub use rust_types::{
    AppDef, EnumDef, Fields, FloatType, IntType, RustType, RustTypes, StructDef, Variant, selector as read_selector,
};
pub use semantic::lower;
