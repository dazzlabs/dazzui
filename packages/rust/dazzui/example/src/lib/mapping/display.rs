//! What shows rather than takes input: badges, cards, notices, readings,
//! the table.

use std::collections::HashMap;

use dazzui::{
    ActionBar, ActionBarAlign, Avatar, AvatarGroup, AvatarShape, Badge, BadgeVariant, Callout,
    Card, CardVariant, Code, CodeBlock, Divider, DividerOrientation, DividerTone, EmptyState, Icon,
    KeyCap, KeyCapVariant, Length, Link, LinkUnderline, Meter, Progress, SectionLabel, Size,
    Skeleton, SkeletonShape, SkeletonText, Spinner, Step, StepList, StepStatus, Table, TableCell,
    TableCellAlign, TableHead, TableRow, Tint,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, SharedString, Window};
use serde_json::Value;

use super::BuildFn;
use crate::lib::scene::{Node, glyph_path};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Icon", icon);
    map.insert("Badge", badge);
    map.insert("Card", card);
    map.insert("Callout", callout);
    map.insert("Divider", divider);
    map.insert("SectionLabel", section_label);
    map.insert("KeyCap", key_cap);
    map.insert("Code", code);
    map.insert("CodeBlock", code_block);
    map.insert("Link", link);
    map.insert("Spinner", spinner);
    map.insert("Skeleton", skeleton);
    map.insert("SkeletonText", skeleton_text);
    map.insert("Progress", progress);
    map.insert("Meter", meter);
    map.insert("Avatar", avatar);
    map.insert("AvatarGroup", avatar_group);
    map.insert("EmptyState", empty_state);
    map.insert("StepList", step_list);
    map.insert("Step", step);
    map.insert("ActionBar", action_bar);
    map.insert("Table", table);
    map.insert("TableHead", table_head);
    map.insert("TableRow", table_row);
    map.insert("TableCell", table_cell);
}

/// A length as a story writes one: a number of pixels, or a CSS string.
fn length(node: &Node, name: &str) -> Option<Length> {
    match node.get(name) {
        Value::Number(number) => number.as_f64().map(|n| Length::Px(n as f32)),
        Value::String(text) => Length::parse(&text),
        _ => None,
    }
}

/// A string-or-node prop, drawn — or nothing when the story leaves it out.
fn slot(node: &Node, name: &str, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
    match node.raw(name) {
        None | Some(Value::Null) => None,
        Some(_) => node.element(name, window, cx),
    }
}

fn icon(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut icon = Icon::new(glyph_path(&node.get("icon")));
    if let Some(size) = node.number("size") {
        icon = icon.size(size as f32);
    }
    icon.into_any_element()
}

fn badge(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Badge::new()
        .variant(
            node.option("variant", BadgeVariant::parse)
                .unwrap_or_default(),
        )
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .children(node.kids(window, cx))
        .into_any_element()
}

fn card(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Card::new()
        .variant(
            node.option("variant", CardVariant::parse)
                .unwrap_or_default(),
        )
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .children(node.kids(window, cx))
        .into_any_element()
}

fn callout(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut callout = Callout::new()
        .tint(node.option("tint", Tint::parse).unwrap_or(Tint::Neutral))
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .children(node.kids(window, cx));
    if let Some(title) = slot(node, "title", window, cx) {
        callout = callout.title(title);
    }
    if let Some(action) = slot(node, "action", window, cx) {
        callout = callout.action(action);
    }
    if let Some(icon) = slot(node, "icon", window, cx) {
        callout = callout.icon(icon);
    }
    callout.into_any_element()
}

fn divider(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    Divider::new()
        .tone(node.option("tone", DividerTone::parse).unwrap_or_default())
        .orientation(
            node.option("orientation", DividerOrientation::parse)
                .unwrap_or_default(),
        )
        .into_any_element()
}

fn section_label(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut label = SectionLabel::new()
        .faint(node.flag("faint"))
        .children(node.kids(window, cx));
    if let Some(tint) = node.option("tint", Tint::parse) {
        label = label.tint(tint);
    }
    label.into_any_element()
}

fn key_cap(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    KeyCap::new()
        .variant(
            node.option("variant", KeyCapVariant::parse)
                .unwrap_or_default(),
        )
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .children(node.kids(window, cx))
        .into_any_element()
}

fn code(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Code::new()
        .children(node.kids(window, cx))
        .into_any_element()
}

fn code_block(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut block = CodeBlock::new(node.id(), node.string("code").unwrap_or_default())
        .line_numbers(node.flag("lineNumbers"))
        .copyable(node.flag("copyable"))
        .wrap(node.flag("wrap"));
    if let Some(title) = slot(node, "title", window, cx) {
        block = block.title(title);
    }
    if let Some(language) = node.string("language") {
        block = block.language(language);
    }
    block.into_any_element()
}

fn link(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut link = Link::new(node.id())
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .underline(
            node.option("underline", LinkUnderline::parse)
                .unwrap_or_default(),
        )
        .external(node.flag("external"))
        .children(node.kids(window, cx));
    if let Some(href) = node.string("href") {
        link = link.href(href);
    }
    if let Some(press) = node.press("onClick") {
        link = link.on_click(move |_, window, cx| press(window, cx));
    }
    link.into_any_element()
}

fn spinner(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    Spinner::new()
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .on_accent(node.flag("onAccent"))
        .into_any_element()
}

fn skeleton(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut skeleton = Skeleton::new().shape(
        node.option("shape", SkeletonShape::parse)
            .unwrap_or_default(),
    );
    if let Some(width) = length(node, "width") {
        skeleton = skeleton.width(width);
    }
    if let Some(height) = length(node, "height") {
        skeleton = skeleton.height(height);
    }
    skeleton.into_any_element()
}

fn skeleton_text(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    SkeletonText::new()
        .lines(node.number("lines").map_or(3, |n| n.max(0.0) as usize))
        .into_any_element()
}

fn progress(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    // `aria-label` names the bar to assistive tech; GPUI has no
    // accessibility tree to hand it to, so it is read and set aside.
    let _ = node.string("aria-label");
    let mut progress = Progress::new()
        .size(node.option("size", Size::parse).unwrap_or(Size::Small))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .gradient(node.flag("gradient"));
    if let Some(value) = node.number("value") {
        progress = progress.value(value as f32);
    }
    progress.into_any_element()
}

fn meter(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    // As on `Progress`: no accessibility tree to carry the name to.
    let _ = node.string("aria-label");
    let mut meter = Meter::new(node.number("value").unwrap_or(0.0) as f32)
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .show_value(node.flag("showValue"));
    if let Some(label) = slot(node, "label", window, cx) {
        meter = meter.label(label);
    }
    meter.into_any_element()
}

fn avatar(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut avatar = Avatar::new()
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .shape(node.option("shape", AvatarShape::parse).unwrap_or_default())
        .tint(node.option("tint", Tint::parse).unwrap_or_default());
    if let Some(name) = node.string("name") {
        avatar = avatar.name(SharedString::from(name));
    }
    avatar.into_any_element()
}

fn avatar_group(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut group = AvatarGroup::new()
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .children(node.kids(window, cx));
    if let Some(max) = node.number("max") {
        group = group.max(max.max(0.0) as usize);
    }
    group.into_any_element()
}

fn empty_state(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = slot(node, "title", window, cx)
        .unwrap_or_else(|| SharedString::default().into_any_element());
    let mut state = EmptyState::new(title);
    if let Some(action) = slot(node, "action", window, cx) {
        state = state.action(action);
    }
    state.into_any_element()
}

fn step_list(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    StepList::new()
        .children(node.kids(window, cx))
        .into_any_element()
}

fn step(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let label = slot(node, "label", window, cx)
        .unwrap_or_else(|| SharedString::default().into_any_element());
    let mut step = Step::new(
        node.option("status", StepStatus::parse).unwrap_or_default(),
        label,
    )
    .tint(node.option("tint", Tint::parse).unwrap_or_default());
    if let Some(meta) = slot(node, "meta", window, cx) {
        step = step.meta(meta);
    }
    step.into_any_element()
}

fn action_bar(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    ActionBar::new()
        .align(
            node.option("align", ActionBarAlign::parse)
                .unwrap_or_default(),
        )
        .children(node.kids(window, cx))
        .into_any_element()
}

fn table(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Table::new()
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .children(node.kids(window, cx))
        .into_any_element()
}

fn table_head(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    TableHead::new()
        .children(node.kids(window, cx))
        .into_any_element()
}

fn table_row(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut row = TableRow::new()
        .active(node.flag("active"))
        .children(node.kids(window, cx));
    if let Some(press) = node.press("onClick") {
        row = row.on_click(node.id(), move |_, window, cx| press(window, cx));
    }
    row.into_any_element()
}

fn table_cell(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    TableCell::new()
        .head(node.flag("head"))
        .align(
            node.option("align", TableCellAlign::parse)
                .unwrap_or_default(),
        )
        .children(node.kids(window, cx))
        .into_any_element()
}
