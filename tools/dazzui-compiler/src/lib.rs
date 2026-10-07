//! Compiles `ui!` pages, written in Rust, to SwiftUI and WinUI source.
//!
//! [`syntax`] parses a page; [`lower`] resolves it against the [`Registry`]
//! and, when they are at hand, the app's Rust types, into the platform-free
//! [`ir`]; [`swiftui`] and [`winui`] lower that to source. The `ui!` macro
//! runs the same parser and checks without the Rust types, and leaves that
//! last check to rustc.
//!
//! A page reads a state and calls a model's methods, as a Flutter widget
//! does, and its components and props are the React kit's. When the model
//! is Rust's, [`bridge`] is what crosses to the native side, as the UniFFI
//! exports the `#[page_model]` macro emits, and [`bindgen`] has UniFFI write
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
pub use project::{Compiled, CompiledModel, CompiledPage, Project, SourceFile};
pub use registry::{FILE as REGISTRY_FILE, Registry, RegistryError};
pub use rust_types::{
    EnumDef, Fields, FloatType, IntType, MethodDef, ModelDef, RustType, RustTypes, StructDef,
    Variant, selector as read_selector,
};
pub use semantic::lower;
