//! Every component a story draws, by the name React exports it under, and
//! the builder that draws it from a node. `tools/codegen/gpui.mjs` lists what
//! this maps; the two change together.

use std::collections::HashMap;
use std::sync::OnceLock;

use gpui::{AnyElement, App, IntoElement, ParentElement, Styled, Window, div};

use crate::lib::scene::Node;
use crate::lib::stage;

mod button;
mod calendar;
mod choice;
mod display;
mod input;
mod navigation;
mod overlay;
mod structure;
mod window_frame;

pub type Build = fn(&Node, &mut Window, &mut App) -> AnyElement;

fn registry() -> &'static HashMap<&'static str, Build> {
    static REGISTRY: OnceLock<HashMap<&'static str, Build>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut map: HashMap<&'static str, Build> = HashMap::new();
        map.insert("Row", stage::row);
        map.insert("Col", stage::col);
        map.insert("Frame", stage::frame);
        map.insert("Text", stage::text);
        button::register(&mut map);
        calendar::register(&mut map);
        display::register(&mut map);
        choice::register(&mut map);
        input::register(&mut map);
        navigation::register(&mut map);
        overlay::register(&mut map);
        structure::register(&mut map);
        window_frame::register(&mut map);
        map
    })
}

/// Draws a node with its component's builder. A component nothing maps is
/// drawn as a marked gap, so a missing mapping shows rather than vanishes.
pub fn build(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    match registry().get(node.component()) {
        Some(build) => build(node, window, cx),
        None => div()
            .border_1()
            .border_color(gpui::red())
            .text_color(gpui::red())
            .p_1()
            .child(format!("<{}>", node.component()))
            .into_any_element(),
    }
}

/// Whether a component has a mapping.
#[cfg(test)]
pub fn maps(component: &str) -> bool {
    registry().contains_key(component)
}

pub use Build as BuildFn;
