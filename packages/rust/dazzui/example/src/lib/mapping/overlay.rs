//! The floating components: what a trigger opens, a pointer rests on, or a
//! right-click raises.
//!
//! A trigger is a prop holding a component. The kit builds it once it knows
//! what pressing it does, so the mapping draws it then, in a context whose
//! press is that one — the trigger's own mapping (`Button`'s) takes the press
//! as its `onClick`.

use std::collections::HashMap;

use dazzui::{
    Align, ContextMenu, Length, Menu, MenuItem, Menubar, MenubarMenu, Popover, PopoverClose,
    PreviewCard, Select, SelectEntry, SelectOption, Side, Size, Tooltip,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, Window, div};
use serde_json::Value;

use super::BuildFn;
use crate::lib::scene::{Callback, Ctx, Node, glyph_path, text, truthy};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Popover", popover);
    map.insert("PopoverClose", popover_close);
    map.insert("Tooltip", tooltip);
    map.insert("Menu", menu);
    map.insert("ContextMenu", context_menu);
    map.insert("Menubar", menubar);
    map.insert("Select", select);
    map.insert("PreviewCard", preview_card);
}

/// A builder for the component prop `name`, drawn later with `press` handed
/// to it — what the kit's trigger slots take.
fn later(
    node: &Node,
    name: &'static str,
) -> impl FnOnce(Callback, &mut Window, &mut App) -> AnyElement + 'static {
    let json = node.json.clone();
    let ctx = node.ctx.clone();
    let path = node.path.clone();
    move |press, window, cx| {
        let node = Node {
            json: &json,
            ctx: ctx.with_press(press),
            path,
        };
        node.element(name, window, cx)
            .unwrap_or_else(|| div().into_any_element())
    }
}

fn width(node: &Node) -> Option<Length> {
    node.string("width").and_then(|width| Length::parse(&width))
}

fn popover(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let trigger = later(node, "trigger");
    let mut popover = Popover::new(node.id())
        .trigger(move |press, _, window, cx| trigger(press, window, cx))
        .side(node.option("side", Side::parse).unwrap_or(Side::Bottom))
        .children(node.kids(window, cx));
    if let Some(title) = node.element("title", window, cx) {
        popover = popover.title(title);
    }
    if let Some(description) = node.element("description", window, cx) {
        popover = popover.description(description);
    }
    if let Some(width) = width(node) {
        popover = popover.width(width);
    }
    popover.into_any_element()
}

fn popover_close(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let render = later(node, "render");
    PopoverClose::new(render).into_any_element()
}

fn tooltip(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Tooltip::new(node.id(), node.string("label").unwrap_or_default())
        .side(node.option("side", Side::parse).unwrap_or(Side::Top))
        .children(node.kids(window, cx))
        .into_any_element()
}

/// The glyph an icon slot holds — a bare `$icon`, or an `<Icon icon={…} />`
/// — as an asset path. `None` for anything else.
fn icon_glyph(value: &Value) -> Option<gpui::SharedString> {
    if value.get("$icon").is_some() {
        return Some(glyph_path(value));
    }
    let glyph = value.get("props")?.get("icon")?;
    (value.get("$el")?.as_str()? == "Icon" && glyph.get("$icon").is_some())
        .then(|| glyph_path(glyph))
}

/// A story's `MenuItem[]`: each row's label, icon, shortcut, check state,
/// whether it is disabled, what choosing it does, and the rows a branch
/// opens beside it.
fn menu_items(
    node: &Node,
    items: Option<&Value>,
    part: &str,
    window: &mut Window,
    cx: &mut App,
) -> Vec<MenuItem> {
    let ctx: &Ctx = &node.ctx;
    let items = items.and_then(Value::as_array).cloned().unwrap_or_default();
    items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let read = |key: &str| item.get(key).map(|value| ctx.resolve(value, &[]));
            let mut row = MenuItem::new(text(&read("label").unwrap_or(Value::Null)))
                .disabled(read("disabled").is_some_and(|v| truthy(&v)));
            if let Some(checked) = read("checked") {
                row = row.checked(truthy(&checked));
            }
            if let Some(icon) = item.get("icon") {
                row = row.icon(node.draw_value(icon, format!("{part}.{index}.icon"), window, cx));
                // The glyph beside the drawn icon, for a native row to wear.
                if let Some(glyph) = icon_glyph(icon) {
                    row = row.glyph(glyph);
                }
            }
            if let Some(shortcut) = item.get("shortcut") {
                row = row.shortcut(node.draw_value(
                    shortcut,
                    format!("{part}.{index}.shortcut"),
                    window,
                    cx,
                ));
            }
            if let Some(action) = item.get("onSelect").cloned() {
                let ctx = ctx.clone();
                row = row.on_select(move |window, cx| ctx.run(&action, &[], window, cx));
            }
            if let Some(children) = item.get("items") {
                let part = format!("{part}.{index}.items");
                row = row.submenu(menu_items(node, Some(children), &part, window, cx));
            }
            row
        })
        .collect()
}

fn menu(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let trigger = later(node, "trigger");
    Menu::new(node.id())
        .trigger(move |press, _, window, cx| trigger(press, window, cx))
        .items(menu_items(node, node.raw("items"), "items", window, cx))
        .into_any_element()
}

fn context_menu(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    ContextMenu::new(node.id())
        .items(menu_items(node, node.raw("items"), "items", window, cx))
        .children(node.kids(window, cx))
        .into_any_element()
}

fn menubar(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let menus = node
        .raw("menus")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let menus: Vec<MenubarMenu> = menus
        .iter()
        .enumerate()
        .map(|(index, menu)| {
            let read = |key: &str| {
                menu.get(key)
                    .map(|value| node.ctx.resolve(value, &[]))
                    .unwrap_or(Value::Null)
            };
            let items = menu_items(
                node,
                menu.get("items"),
                &format!("menus.{index}"),
                window,
                cx,
            );
            MenubarMenu::new(text(&read("label")))
                .disabled(truthy(&read("disabled")))
                .items(items)
        })
        .collect();
    Menubar::new(node.id()).menus(menus).into_any_element()
}

fn select_option(node: &Node, value: &Value) -> SelectOption {
    let read = |key: &str| {
        value
            .get(key)
            .map(|value| node.ctx.resolve(value, &[]))
            .unwrap_or(Value::Null)
    };
    SelectOption::new(text(&read("value")), text(&read("label")))
        .disabled(truthy(&read("disabled")))
}

fn select(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let entries: Vec<SelectEntry> = node
        .raw("options")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(
            |entry| match entry.get("options").and_then(Value::as_array) {
                Some(options) => SelectEntry::group(
                    text(&node.ctx.resolve(&entry["label"], &[])),
                    options
                        .iter()
                        .map(|option| select_option(node, option))
                        .collect(),
                ),
                None => SelectEntry::Option(select_option(node, entry)),
            },
        )
        .collect();
    // GPUI has no accessibility tree to hand `aria-label` to; it is read so
    // the prop is accounted for, and names nothing drawn.
    let _accessible_name = node.string("aria-label");
    let mut select = Select::new(node.id())
        .options(entries)
        .size(node.option("size", Size::parse).unwrap_or(Size::Medium))
        .disabled(node.flag("disabled"));
    if let Some(value) = node.string("defaultValue") {
        select = select.default_value(value);
    }
    if let Some(placeholder) = node.string("placeholder") {
        select = select.placeholder(placeholder);
    }
    select.into_any_element()
}

fn preview_card(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut card = PreviewCard::new(node.id())
        .align(Align::Center)
        .children(node.kids(window, cx));
    if let Some(trigger) = node.element("trigger", window, cx) {
        card = card.trigger(trigger);
    }
    if let Some(width) = width(node) {
        card = card.width(width);
    }
    card.into_any_element()
}
