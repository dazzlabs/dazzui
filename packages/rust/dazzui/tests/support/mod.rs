#[allow(dead_code)]
pub mod paint;

/// The same editing gesture using the host platform's native modifiers.
pub fn keys(mac: &'static str, other: &'static str) -> &'static str {
    if cfg!(target_os = "macos") {
        mac
    } else {
        other
    }
}
