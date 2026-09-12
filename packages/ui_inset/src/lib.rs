#![feature(arbitrary_self_types)]
//! DazzUI controls for Inset, with generated design tokens and twelve themes.
//!
//! Install [`Theme`] under the host's `WidgetsApp`; controlled widgets report
//! changes through callbacks. See the package README and PORTING.md for host
//! setup, examples, and remaining Flutter parity differences.
pub mod foundation;
pub mod generated;
pub use foundation::*;
pub use generated::theme_variables::*;
pub mod theme;
pub use theme::*;
pub mod widgets;
pub use widgets::*;
