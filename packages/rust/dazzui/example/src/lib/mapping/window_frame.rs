//! The presentation windows authored in React's WindowFrame page.
use super::BuildFn;
use crate::lib::scene::Node;
use dazzui::{
    ActiveTheme, Length, WindowBody, WindowContent, WindowControlState, WindowFooter, WindowFrame,
    WindowFrameControls, WindowFramePlatform, WindowMain, WindowTitlebar, WindowTitlebarSlots,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, Styled, Window, div, px};
use serde_json::Value;
use std::collections::HashMap;

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("WindowFrame", frame);
    map.insert("WindowTitlebar", titlebar);
    map.insert("WindowBody", |node, window, cx| {
        WindowBody::new()
            .children(node.kids(window, cx))
            .into_any_element()
    });
    map.insert("WindowMain", |node, window, cx| {
        WindowMain::new()
            .children(node.kids(window, cx))
            .into_any_element()
    });
    map.insert("WindowContent", |node, window, cx| {
        WindowContent::new()
            .children(node.kids(window, cx))
            .into_any_element()
    });
    map.insert("WindowFooter", |node, window, cx| {
        WindowFooter::new()
            .children(node.kids(window, cx))
            .into_any_element()
    });
    map.insert("WindowPane", |node, window, cx| {
        let t = cx.theme();
        div()
            .p(px(t.spacing_5))
            .pt_0()
            .children(node.kids(window, cx))
            .into_any_element()
    });
}
fn length(node: &Node, name: &str) -> Option<Length> {
    match node.get(name) {
        Value::Number(number) => number.as_f64().map(|n| Length::Px(n as f32)),
        Value::String(text) => Length::parse(&text),
        _ => None,
    }
}
fn controls(value: &Value) -> WindowFrameControls {
    let state = |key| match &value[key] {
        Value::Bool(value) => Some(WindowControlState::from(*value)),
        Value::String(value) => WindowControlState::parse(value),
        _ => None,
    };
    WindowFrameControls {
        close: state("close"),
        minimize: state("minimize"),
        maximize: state("maximize"),
    }
}
fn frame(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut frame = WindowFrame::new(node.id())
        .platform(
            node.option("platform", WindowFramePlatform::parse)
                .unwrap_or_default(),
        )
        .title_hidden(node.flag("titleHidden"))
        .full_size_content(node.flag("fullSizeContent"))
        .inactive(node.flag("inactive"));
    if let Some(value) = (!node.get("title").is_null())
        .then(|| node.element("title", window, cx))
        .flatten()
    {
        frame = frame.title(value);
    }
    if let Some(value) = (!node.get("subtitle").is_null())
        .then(|| node.element("subtitle", window, cx))
        .flatten()
    {
        frame = frame.subtitle(value);
    }
    if node.get("titlebar") == Value::Bool(false) {
        frame = frame.without_titlebar();
    } else if let Some(value) = node.raw("titlebar") {
        let mut slots = WindowTitlebarSlots::new();
        if let Some(value) = value.get("leading").filter(|value| !value.is_null()) {
            slots = slots.leading(node.draw_value(value, "titlebar.leading", window, cx));
        }
        if let Some(value) = value.get("toolbar").filter(|value| !value.is_null()) {
            let children = match value.as_array() {
                Some(values) => values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        node.draw_value(value, format!("titlebar.toolbar.{index}"), window, cx)
                    })
                    .collect(),
                None => vec![node.draw_value(value, "titlebar.toolbar", window, cx)],
            };
            slots = slots.toolbar_elements(children);
        }
        if let Some(value) = value.get("trailing").filter(|value| !value.is_null()) {
            slots = slots.trailing(node.draw_value(value, "titlebar.trailing", window, cx));
        }
        if let Some(value) = value.get("accessory").filter(|value| !value.is_null()) {
            slots = slots.accessory(node.draw_value(value, "titlebar.accessory", window, cx));
        }
        frame = frame.titlebar(slots);
    }
    if node.get("controls") == Value::Bool(false) {
        frame = frame.without_controls();
    } else if node.has("controls") {
        frame = frame.controls(controls(&node.get("controls")));
    }
    if let Some(value) = length(node, "width") {
        frame = frame.width(value);
    }
    if let Some(value) = length(node, "height") {
        frame = frame.height(value);
    }
    if let Some(value) = length(node, "minWidth") {
        frame = frame.min_width(value);
    }
    if let Some(value) = length(node, "maxWidth") {
        frame = frame.max_width(value);
    }
    if let Some(value) = length(node, "minHeight") {
        frame = frame.min_height(value);
    }
    if let Some(value) = length(node, "maxHeight") {
        frame = frame.max_height(value);
    }
    if let Some(action) = node.action("onClose") {
        frame = frame.on_close(move |window, cx| action(window, cx));
    }
    if let Some(action) = node.action("onMinimize") {
        frame = frame.on_minimize(move |window, cx| action(window, cx));
    }
    if let Some(action) = node.action("onMaximize") {
        frame = frame.on_maximize(move |window, cx| action(window, cx));
    }
    frame.children(node.kids(window, cx)).into_any_element()
}
fn titlebar(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut bar = WindowTitlebar::new(node.id()).title_hidden(node.flag("titleHidden"));
    if let Some(value) = (!node.get("title").is_null())
        .then(|| node.element("title", window, cx))
        .flatten()
    {
        bar = bar.title(value);
    }
    if let Some(value) = (!node.get("subtitle").is_null())
        .then(|| node.element("subtitle", window, cx))
        .flatten()
    {
        bar = bar.subtitle(value);
    }
    if let Some(value) = (!node.get("leading").is_null())
        .then(|| node.element("leading", window, cx))
        .flatten()
    {
        bar = bar.leading(value);
    }
    if let Some(value) = node.raw("toolbar").filter(|value| !value.is_null()) {
        let children = match value.as_array() {
            Some(values) => values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    node.draw_value(value, format!("toolbar.{index}"), window, cx)
                })
                .collect(),
            None => vec![node.draw_value(value, "toolbar", window, cx)],
        };
        bar = bar.toolbar_elements(children);
    }
    if let Some(value) = (!node.get("trailing").is_null())
        .then(|| node.element("trailing", window, cx))
        .flatten()
    {
        bar = bar.trailing(value);
    }
    if let Some(value) = (!node.get("accessory").is_null())
        .then(|| node.element("accessory", window, cx))
        .flatten()
    {
        bar = bar.accessory(value);
    }
    if node.get("controls") == Value::Bool(false) {
        bar = bar.without_controls();
    } else if node.has("controls") {
        bar = bar.controls(controls(&node.get("controls")));
    }
    if let Some(action) = node.action("onClose") {
        bar = bar.on_close(move |window, cx| action(window, cx));
    }
    if let Some(action) = node.action("onMinimize") {
        bar = bar.on_minimize(move |window, cx| action(window, cx));
    }
    if let Some(action) = node.action("onMaximize") {
        bar = bar.on_maximize(move |window, cx| action(window, cx));
    }
    bar.into_any_element()
}
