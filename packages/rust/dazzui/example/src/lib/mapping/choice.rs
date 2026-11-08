//! The choice controls: what a person picks, latches or sets.

use std::collections::HashMap;

use dazzui::{
    ActiveStyle, Checkbox, CheckboxGroup, IconButton, OptionCard, PillTabItem, PillTabs,
    RadioGroup, RadioGroupOption, SegmentedControl, SegmentedItem, Size, Slider, Switch, Tag, Tint,
    Toggle, ToggleGroup, ToggleItem, Variant,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, SharedString, Window};
use serde_json::Value;

use super::BuildFn;
use crate::lib::scene::{Node, text};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Checkbox", checkbox);
    map.insert("CheckboxGroup", checkbox_group);
    map.insert("RadioGroup", radio_group);
    map.insert("Switch", switch);
    map.insert("Toggle", toggle);
    map.insert("ToggleGroup", toggle_group);
    map.insert("Tag", tag);
    map.insert("SegmentedControl", segmented_control);
    map.insert("PillTabs", pill_tabs);
    map.insert("OptionCard", option_card);
    map.insert("IconButton", icon_button);
    map.insert("Slider", slider);
}

/// A list prop's items, as objects.
fn items(node: &Node, name: &str) -> Vec<Value> {
    node.get(name).as_array().cloned().unwrap_or_default()
}

/// An item's label, drawn at a path of its own.
fn label(node: &Node, item: &Value, part: String, window: &mut Window, cx: &mut App) -> AnyElement {
    node.draw_value(&item["label"], part, window, cx)
}

/// Whether a label is one glyph and nothing else — what React's
/// `:has(> svg:only-child)` finds. The button reads its ends with it too.
pub(super) fn is_glyph(value: &Value) -> bool {
    value.get("$icon").is_some() || value["$el"].as_str() == Some("Icon")
}

fn value_of(item: &Value) -> SharedString {
    text(&item["value"]).into()
}

fn checkbox(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut checkbox = Checkbox::new(node.id())
        .default_checked(node.flag("defaultChecked"))
        .indeterminate(node.flag("indeterminate"))
        .disabled(node.flag("disabled"))
        .read_only(node.flag("readOnly"))
        .parent(node.flag("parent"))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .children(node.kids(window, cx));
    if let Some(note) = node.element("note", window, cx) {
        checkbox = checkbox.note(note);
    }
    if let Some(value) = node.string("value") {
        checkbox = checkbox.value(value);
    }
    if node.raw("checked").is_some() {
        checkbox = checkbox.checked(node.flag("checked"));
    }
    if let Some(change) = node.handler("onCheckedChange") {
        checkbox = checkbox.on_checked_change(move |value, window, cx| {
            change(Value::Bool(value), window, cx);
        });
    }
    checkbox.into_any_element()
}

fn checkbox_group(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut group = CheckboxGroup::new(node.id())
        .disabled(node.flag("disabled"))
        .children(node.kids(window, cx));
    for name in ["defaultValue", "value", "allValues"] {
        if node.raw(name).is_some() {
            let values = items(node, name).iter().map(text).collect::<Vec<_>>();
            group = match name {
                "value" => group.value(values),
                "allValues" => group.all_values(values),
                _ => group.default_value(values),
            };
        }
    }
    if let Some(change) = node.handler("onValueChange") {
        group = group.on_value_change(move |values, window, cx| {
            change(
                Value::Array(
                    values
                        .into_iter()
                        .map(|value| Value::String(value.to_string()))
                        .collect(),
                ),
                window,
                cx,
            );
        });
    }
    group.into_any_element()
}

fn radio_group(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let options = items(node, "options")
        .iter()
        .enumerate()
        .map(|(index, option)| {
            RadioGroupOption::new(
                value_of(option),
                label(node, option, format!("options.{index}"), window, cx),
            )
            .disabled(option["disabled"].as_bool().unwrap_or(false))
        })
        .collect();
    let mut group = RadioGroup::new(node.id(), options)
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium));
    if let Some(value) = node.string("defaultValue") {
        group = group.default_value(value);
    }
    if let Some(label) = node.string("aria-label") {
        group = group.aria_label(label);
    }
    group.into_any_element()
}

fn switch(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut switch = Switch::new(node.id())
        .default_checked(node.flag("defaultChecked"))
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .disabled(node.flag("disabled"));
    if let Some(label) = node.string("aria-label") {
        switch = switch.aria_label(label);
    }
    switch.into_any_element()
}

fn toggle(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let glyph = matches!(node.children(), [only] if is_glyph(only));
    let mut toggle = Toggle::new(node.id())
        .size(node.option("size", Size::parse).unwrap_or(Size::Tiny))
        .variant(
            node.option("variant", Variant::parse)
                .unwrap_or(Variant::Plain),
        )
        .default_pressed(node.flag("defaultPressed"))
        .icon_only(glyph)
        .children(node.kids(window, cx));
    if let Some(label) = node.string("label") {
        toggle = toggle.label(label);
    }
    toggle.into_any_element()
}

fn toggle_group(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let members = items(node, "items")
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let mut member = ToggleItem::new(
                value_of(item),
                label(node, item, format!("items.{index}"), window, cx),
            )
            .icon_only(is_glyph(&item["label"]))
            .disabled(item["disabled"].as_bool().unwrap_or(false));
            if let Some(hint) = item["hint"].as_str() {
                member = member.hint(hint.to_string());
            }
            member
        })
        .collect();
    let value = node
        .get("value")
        .as_array()
        .map(|values| values.iter().map(|v| SharedString::from(text(v))).collect())
        .unwrap_or_default();
    let mut group = ToggleGroup::new(node.id(), members)
        .value(value)
        .size(node.option("size", Size::parse).unwrap_or(Size::Small))
        .multiple(node.get("multiple").as_bool().unwrap_or(true));
    if let Some(label) = node.string("aria-label") {
        group = group.aria_label(label);
    }
    if let Some(handler) = node.handler("onValueChange") {
        group = group.on_value_change(move |next, window, cx| {
            let next = next.into_iter().map(|v| Value::String(v.to_string()));
            handler(Value::Array(next.collect()), window, cx)
        });
    }
    group.into_any_element()
}

fn tag(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut tag = Tag::new(node.id())
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .children(node.kids(window, cx));
    if node.has("selected") {
        tag = tag.selected(node.flag("selected"));
    }
    if let Some(handler) = node.handler("onSelectedChange") {
        tag = tag.on_selected_change(move |selected, window, cx| {
            handler(Value::Bool(selected), window, cx)
        });
    }
    if let Some(remove) = node.action("onRemove") {
        tag = tag.on_remove(move |window, cx| remove(window, cx));
    }
    tag.into_any_element()
}

fn segmented_control(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let segments = items(node, "items")
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let mut segment = SegmentedItem::new(
                value_of(item),
                label(node, item, format!("items.{index}"), window, cx),
            )
            .disabled(item["disabled"].as_bool().unwrap_or(false));
            if !item["icon"].is_null() {
                segment = segment.icon(node.draw_value(
                    &item["icon"],
                    format!("items.{index}.icon"),
                    window,
                    cx,
                ));
            }
            segment
        })
        .collect();
    let mut control = SegmentedControl::new(
        node.id(),
        segments,
        node.string("value").unwrap_or_default(),
    )
    .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
    .tint(node.option("tint", Tint::parse).unwrap_or_default())
    .active_style(
        node.option("activeStyle", ActiveStyle::parse)
            .unwrap_or_default(),
    )
    .stretch(node.flag("stretch"))
    .icon_only(node.flag("iconOnly"));
    if let Some(label) = node.string("aria-label") {
        control = control.aria_label(label);
    }
    if let Some(handler) = node.handler("onValueChange") {
        control = control.on_value_change(move |value, window, cx| {
            handler(Value::String(value.to_string()), window, cx)
        });
    }
    control.into_any_element()
}

fn pill_tabs(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let pills = items(node, "items")
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let mut pill = PillTabItem::new(
                value_of(item),
                label(node, item, format!("items.{index}"), window, cx),
            )
            .disabled(item["disabled"].as_bool().unwrap_or(false));
            if !item["count"].is_null() {
                pill = pill.count(text(&item["count"]));
            }
            pill
        })
        .collect();
    let mut tabs = PillTabs::new(node.id(), pills);
    if let Some(value) = node.string("defaultValue") {
        tabs = tabs.default_value(value);
    }
    if let Some(label) = node.string("aria-label") {
        tabs = tabs.aria_label(label);
    }
    tabs.into_any_element()
}

fn option_card(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| "".into_any_element());
    let mut card = OptionCard::new(node.id(), title)
        .selected(node.flag("selected"))
        .disabled(node.flag("disabled"));
    if let Some(description) = node.element("description", window, cx) {
        card = card.description(description);
    }
    if let Some(press) = node.press("onClick") {
        card = card.on_click(move |_, window, cx| press(window, cx));
    }
    card.into_any_element()
}

fn icon_button(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut button = IconButton::new(node.id(), node.string("label").unwrap_or_default())
        .variant(
            node.option("variant", Variant::parse)
                .unwrap_or(Variant::Plain),
        )
        .tint(node.option("tint", Tint::parse).unwrap_or(Tint::Neutral))
        .size(node.option("size", Size::parse).unwrap_or(Size::Tiny))
        .children(node.kids(window, cx));
    if let Some(press) = node.press("onClick") {
        button = button.on_click(move |_, window, cx| press(window, cx));
    }
    button.into_any_element()
}

fn slider(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let values: Vec<f64> = match node.get("defaultValue") {
        Value::Array(values) => values.iter().filter_map(Value::as_f64).collect(),
        value => value.as_f64().into_iter().collect(),
    };
    let mut slider = Slider::new(node.id())
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .show_value(node.flag("showValue"));
    if !values.is_empty() {
        slider = slider.default_value(values);
    }
    if let Some(label) = node.string("aria-label") {
        slider = slider.aria_label(label);
    }
    slider.into_any_element()
}
