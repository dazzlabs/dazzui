//! The demo app core: plain Rust that knows nothing of any UI toolkit, and
//! the `ui!` page that `dazzui-compiler generate` turns into SwiftUI and WinUI.

pub mod action;
pub mod state;
pub mod update;
pub mod views;

pub use action::SettingsAction;
pub use state::{Plugin, SettingsState};
pub use update::update;
