//! The demo app core: plain Rust that knows nothing of any UI toolkit, and
//! the `ui!` page that `dazzui-compiler generate` turns into SwiftUI and WinUI.

// The bridge #[app] emits is UniFFI's, whose exports need this at the root.
uniffi::setup_scaffolding!();

pub mod action;
pub mod state;
pub mod update;
pub mod views;

pub use action::SettingsAction;
pub use state::{Plugin, SettingsState};
pub use update::update;
