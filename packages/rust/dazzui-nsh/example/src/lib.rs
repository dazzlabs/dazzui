use dazzui_nsh::prelude::*;
mod capture;
#[path = "lib/mod.rs"]
pub mod runtime;
pub mod stories;
mod window;

#[native_module("dazzui/example")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
}

/// The same desktop gallery as React, on NSH's AppKit / Win32 / GTK backend.
#[component]
pub fn App() -> impl IntoElement {
    if std::env::var_os("DAZZUI_NSH_CAPTURE").is_some() {
        capture::capture().into_element()
    } else {
        window::gallery().into_element()
    }
}
