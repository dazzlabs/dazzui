//! The six pages, in the order the sidebar lists them.
//!
//! Each is a function rather than a view: the playground's state lives in one
//! place, and a page is a way of looking at it.

mod controls;
mod data;
mod feedback;
mod forms;
mod settings;
mod themes;

pub use controls::controls;
pub use data::data;
pub use feedback::feedback;
pub use forms::forms;
pub use settings::settings;
pub use themes::themes;
