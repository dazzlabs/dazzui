//! The demo app core: plain Rust that knows nothing of any UI toolkit, and
//! the `ui!` page that `dazzui-compiler generate` turns into SwiftUI and WinUI.
//!
//! The page is a view of [`SettingsState`] that sends [`SettingsAction`]s.
//! What drives it is [`SettingsModel`], written by hand against the
//! `PageModel` seam: the compiler asks nothing more of it than a snapshot, a
//! way to take an action, and a notifier. How it is built inside — here, a
//! lock and a thread that saves — is this crate's own business.

// The bridge #[page_model] emits is UniFFI's, whose exports need this at
// the root.
uniffi::setup_scaffolding!();

pub mod action;
pub mod model;
pub mod state;
pub mod views;

pub use action::SettingsAction;
pub use model::{SettingsModel, settings};
pub use state::{Plugin, SettingsState};
