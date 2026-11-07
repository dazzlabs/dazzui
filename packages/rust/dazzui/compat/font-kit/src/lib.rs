//! Keep font-kit's public types and loaders. Only the macOS system UI
//! family's variable weight instances differ from upstream.

pub use upstream::*;

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
pub mod source {
    pub use crate::macos::SystemSource;
    pub use upstream::source::*;
}
