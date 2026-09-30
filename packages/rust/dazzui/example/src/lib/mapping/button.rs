use std::collections::HashMap;

use dazzui::{Button, Size, Tint, Variant};
use gpui::{AnyElement, App, IntoElement, ParentElement, Window};

use super::BuildFn;
use crate::lib::scene::Node;

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Button", button);
}

fn button(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut button = Button::new(node.id())
        .variant(node.option("variant", Variant::parse).unwrap_or_default())
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .size(node.option("size", Size::parse).unwrap_or(Size::Small))
        .disabled(node.flag("disabled"))
        .full_width(node.flag("fullWidth"))
        .children(node.kids(window, cx));
    if let Some(shortcut) = node.element("shortcut", window, cx) {
        button = button.shortcut(shortcut);
    }
    if let Some(press) = node.press("onClick") {
        button = button.on_click(move |_, window, cx| press(window, cx));
    }
    button.into_any_element()
}
