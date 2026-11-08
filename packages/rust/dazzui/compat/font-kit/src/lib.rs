//! Keep font-kit's public types and loaders. Only the macOS system UI
//! family's variable weight instances differ from upstream: it hands out
//! faces instanced at CSS weights, and matches on the weight a face draws at
//! rather than the one its file declares. AAT tracking tables retain the
//! fractional interpolation that browser shaping uses.

pub use upstream::*;

pub mod tracking;

#[cfg(all(
    target_os = "macos",
    feature = "source",
    not(feature = "loader-freetype-default")
))]
mod macos;

#[cfg(all(
    target_os = "macos",
    feature = "source",
    not(feature = "loader-freetype-default")
))]
pub use crate::macos::{css_properties, css_weight};

#[cfg(all(
    target_os = "macos",
    feature = "source",
    not(feature = "loader-freetype-default")
))]
pub mod source {
    pub use crate::macos::SystemSource;
    pub use upstream::source::*;
}
