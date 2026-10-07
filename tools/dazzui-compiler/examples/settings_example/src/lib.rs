//! The demo app core: plain Rust that knows nothing of any UI toolkit, and
//! the `ui!` page that `dazzui-compiler generate` turns into SwiftUI and WinUI.
//!
//! The page reads a [`SettingsState`] and calls [`SettingsModel`]'s methods,
//! as a Flutter widget reads a `ValueListenable` and calls its model. How
//! the model works inside — here, a value and a thread that saves — is this
//! crate's own business; the compiler asks only for its public methods and
//! its value.

// The bridge #[page_model] emits is UniFFI's, whose exports need this at
// the root.
uniffi::setup_scaffolding!();

pub mod model;
pub mod state;
pub mod views;

pub use model::SettingsModel;
pub use state::{Plugin, SettingsState};
