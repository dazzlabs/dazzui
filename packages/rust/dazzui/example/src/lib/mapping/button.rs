use std::collections::HashMap;

use dazzui::{Button, Size, Tint, Variant};
use gpui::{AnyElement, App, IntoElement, ParentElement, Window};

use super::BuildFn;
use super::choice::is_glyph;
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
        .full_width(node.flag("fullWidth"));
    // A glyph child goes in as one, so the button sizes it and trims the
    // padding at the end it sits at, as React reads off its children.
    for (kid, element) in node.children().iter().zip(node.kids(window, cx)) {
        button = if is_glyph(kid) {
            button.glyph(element)
        } else {
            button.child(element)
        };
    }
    if let Some(shortcut) = node.element("shortcut", window, cx) {
        button = button.shortcut(shortcut);
    }
    if let Some(press) = node.press("onClick") {
        button = button.on_click(move |_, window, cx| press(window, cx));
    }
    button.into_any_element()
}
