//! React-only navigation scenes, read through their explicitly mapped grammar.
use super::BuildFn;
use crate::lib::scene::{Node, glyph_path, text};
use dazzui::{ActiveTheme, TabItem, TabStatus, Tabs};
use gpui::{AnyElement, App, IntoElement, ParentElement, Styled, Window, div, px};
use serde_json::Value;
use std::collections::HashMap;

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Tabs", tabs);
    map.insert("ChromeBar", chrome_bar);
}

fn tabs(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let items = node
        .get("items")
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| {
            let mut tab = TabItem::new(text(&item["value"]), text(&item["label"]))
                .closable(item["closable"].as_bool().unwrap_or(false))
                .disabled(item["disabled"].as_bool().unwrap_or(false))
                .busy(item["busy"].as_bool().unwrap_or(false));
            if item.get("icon").is_some() {
                tab = tab.icon(glyph_path(&item["icon"]));
            }
            if let Some(count) = item.get("count") {
                tab = tab.count(text(count));
            }
            if let Some(detail) = item["detail"].as_str() {
                tab = tab.detail(detail.to_string());
            }
            if let Some(title) = item["title"].as_str() {
                tab = tab.title(title.to_string());
            }
            if let Some(status) = item["status"].as_str().and_then(TabStatus::parse) {
                tab = tab.status(status);
            }
            tab
        })
        .collect();
    let mut tabs = Tabs::new(node.id(), items);
    if let Some(value) = node.string("value") {
        tabs = tabs.value(value);
    }
    if let Some(handler) = node.handler("onValueChange") {
        tabs = tabs.on_value_change(move |value, window, cx| {
            handler(Value::String(value.to_string()), window, cx)
        });
    }
    if let Some(handler) = node.handler("onClose") {
        tabs = tabs.on_close(move |value, window, cx| {
            handler(Value::String(value.to_string()), window, cx)
        });
    }
    if let Some(trailing) = node.element("trailing", window, cx) {
        tabs = tabs.trailing(trailing);
    }
    tabs.into_any_element()
}

fn chrome_bar(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let t = cx.theme();
    let padding = node.string("padding").unwrap_or_default();
    let padding = padding
        .strip_prefix("spacing.")
        .and_then(|step| t.spacing(&step.replace('.', "-")))
        .expect("a mapped header spacing token");
    assert_eq!(
        node.string("background").as_deref(),
        Some("surface-chrome"),
        "unmapped header background"
    );
    assert_eq!(
        node.string("borderColor").as_deref(),
        Some("border"),
        "unmapped header border color"
    );
    assert_eq!(
        node.string("borderWidth").as_deref(),
        Some("stroke.hairline"),
        "unmapped header border width"
    );
    div()
        .flex()
        .w(px(node.number("width").unwrap_or(0.) as f32))
        .max_w_full()
        .h(px(node.number("height").unwrap_or(0.) as f32))
        .px(px(padding))
        .border_b(px(t.stroke_hairline))
        .border_color(t.color_border.hsla())
        .bg(t.color_surface_chrome.hsla())
        .children(node.kids(window, cx))
        .into_any_element()
}
