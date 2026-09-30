//! The structural components: disclosures, trees, splitters, the sidebar
//! family, preferences, and what floats over a window — dialogs, drawers and
//! toasts.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dazzui::{
    Accordion, AccordionItem, Collapsible, Dialog, DialogBody, DialogClose, DialogFooter,
    DialogHeader, DialogTone, Drawer, DrawerSide, Length, PreferenceGroup, PreferenceRow,
    PreferenceSection, Preferences, Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup,
    SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarTrigger,
    SidebarVariant, Splitter, SplitterOrientation, SplitterPanel, ToastOptions, ToastPlacement,
    ToastProvider, ToastViewport, Toaster, Tree, TreeNode, TreeSize,
};
use gpui::{AnyElement, App, IntoElement, ParentElement, SharedString, Window, div};
use serde_json::Value;

use super::BuildFn;
use crate::lib::scene::{Callback, Node, held, text};

pub fn register(map: &mut HashMap<&'static str, BuildFn>) {
    map.insert("Accordion", accordion);
    map.insert("AccordionItem", accordion_item);
    map.insert("Collapsible", collapsible);
    map.insert("Tree", tree);
    map.insert("Splitter", splitter);
    map.insert("SplitterPanel", splitter_panel);
    map.insert("SidebarProvider", sidebar_provider);
    map.insert("Sidebar", sidebar);
    map.insert("SidebarHeader", sidebar_header);
    map.insert("SidebarContent", sidebar_content);
    map.insert("SidebarGroup", sidebar_group);
    map.insert("SidebarGroupLabel", sidebar_group_label);
    map.insert("SidebarGroupContent", sidebar_group_content);
    map.insert("SidebarMenu", sidebar_menu);
    map.insert("SidebarMenuItem", sidebar_menu_item);
    map.insert("SidebarMenuButton", sidebar_menu_button);
    map.insert("SidebarMenuBadge", sidebar_menu_badge);
    map.insert("SidebarInset", sidebar_inset);
    map.insert("SidebarTrigger", sidebar_trigger);
    map.insert("Preferences", preferences);
    map.insert("PreferenceGroup", preference_group);
    map.insert("PreferenceSection", preference_section);
    map.insert("PreferenceRow", preference_row);
    map.insert("Dialog", dialog);
    map.insert("DialogHeader", dialog_header);
    map.insert("DialogBody", dialog_body);
    map.insert("DialogFooter", dialog_footer);
    map.insert("DialogClose", dialog_close);
    map.insert("Drawer", drawer);
    map.insert("DrawerClose", dialog_close);
    map.insert("ToastProvider", toast_provider);
    map.insert("ToastViewport", toast_viewport);
}

/// A child of `node`, read as a node of its own at the path `kids` gives it.
fn child<'a>(node: &Node<'a>, index: usize, json: &'a Value) -> Node<'a> {
    Node {
        json,
        ctx: node.ctx.clone(),
        path: format!("{}/{index}", node.path),
    }
}

fn strings(value: &Value) -> Vec<SharedString> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| SharedString::from(text(item)))
        .collect()
}

// Disclosures ----------------------------------------------------------------

fn accordion(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut accordion = Accordion::new(node.id())
        .default_value(strings(&node.get("defaultValue")))
        .multiple(node.flag("multiple"))
        .divided(node.flag("divided"));
    for (index, json) in node.children().iter().enumerate() {
        let item = child(node, index, json);
        if item.component() == "AccordionItem" {
            accordion = accordion.item(accordion_item_of(&item, window, cx));
        }
    }
    accordion.into_any_element()
}

fn accordion_item_of(node: &Node, window: &mut Window, cx: &mut App) -> AccordionItem {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    AccordionItem::new(node.string("value").unwrap_or_default(), title)
        .children(node.kids(window, cx))
}

/// An item outside an accordion is an accordion of one.
fn accordion_item(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Accordion::new(node.id())
        .item(accordion_item_of(node, window, cx))
        .into_any_element()
}

fn collapsible(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    let mut collapsible = Collapsible::new(node.id(), title).default_open(node.flag("defaultOpen"));
    if let Some(meta) = node.element("meta", window, cx) {
        collapsible = collapsible.meta(meta);
    }
    collapsible
        .children(node.kids(window, cx))
        .into_any_element()
}

// Trees ----------------------------------------------------------------------

fn tree_nodes(
    node: &Node,
    items: &Value,
    path: &str,
    window: &mut Window,
    cx: &mut App,
) -> Vec<TreeNode> {
    items
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, item)| {
            let here = format!("{path}.{index}");
            let mut tree_node = TreeNode::new(text(&item["id"]), text(&item["label"]))
                .disabled(item["disabled"].as_bool().unwrap_or(false));
            if !item["icon"].is_null() {
                tree_node = tree_node.icon(node.draw_value(
                    &item["icon"],
                    format!("{here}.icon"),
                    window,
                    cx,
                ));
            }
            if !item["meta"].is_null() {
                tree_node = tree_node.meta(node.draw_value(
                    &item["meta"],
                    format!("{here}.meta"),
                    window,
                    cx,
                ));
            }
            if item.get("children").is_some() {
                let children = tree_nodes(node, &item["children"], &here, window, cx);
                tree_node = tree_node.children(children);
            }
            tree_node
        })
        .collect()
}

fn tree(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let items = node.get("items");
    let nodes = tree_nodes(node, &items, "items", window, cx);
    let mut tree = Tree::new(node.id(), nodes)
        .size(node.option("size", TreeSize::parse).unwrap_or_default())
        .default_expanded(strings(&node.get("defaultExpanded")))
        .selected(node.string("selected").map(SharedString::from))
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(handler) = node.handler("onSelectedChange") {
        tree = tree.on_selected_change(move |id, window, cx| {
            handler(Value::String(id.to_string()), window, cx)
        });
    }
    tree.into_any_element()
}

// Splitters ------------------------------------------------------------------

fn splitter_panel_of(node: &Node, window: &mut Window, cx: &mut App) -> SplitterPanel {
    let mut panel = SplitterPanel::new().children(node.kids(window, cx));
    if let Some(size) = node.number("defaultSize") {
        panel = panel.default_size(size as f32);
    }
    if let Some(size) = node.number("minSize") {
        panel = panel.min_size(size as f32);
    }
    if let Some(size) = node.number("maxSize") {
        panel = panel.max_size(size as f32);
    }
    panel
}

fn splitter(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut splitter = Splitter::new(node.id()).orientation(
        node.option("orientation", SplitterOrientation::parse)
            .unwrap_or_default(),
    );
    for (index, json) in node.children().iter().enumerate() {
        let panel = child(node, index, json);
        if panel.component() == "SplitterPanel" {
            splitter = splitter.panel(splitter_panel_of(&panel, window, cx));
        }
    }
    splitter.into_any_element()
}

/// A panel outside a splitter is a splitter of one.
fn splitter_panel(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Splitter::new(node.id())
        .panel(splitter_panel_of(node, window, cx))
        .into_any_element()
}

// The sidebar family ------------------------------------------------------------

/// A part that is a box of its children and nothing else.
macro_rules! container {
    ($name:ident, $kit:ident) => {
        fn $name(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
            $kit::new()
                .children(node.kids(window, cx))
                .into_any_element()
        }
    };
}

container!(sidebar_header, SidebarHeader);
container!(sidebar_content, SidebarContent);
container!(sidebar_group, SidebarGroup);
container!(sidebar_group_label, SidebarGroupLabel);
container!(sidebar_group_content, SidebarGroupContent);
container!(sidebar_menu, SidebarMenu);
container!(sidebar_inset, SidebarInset);
container!(preferences, Preferences);
container!(dialog_body, DialogBody);
container!(dialog_footer, DialogFooter);

fn sidebar_provider(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    SidebarProvider::new(node.id())
        .children(node.kids(window, cx))
        .into_any_element()
}

fn sidebar(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    Sidebar::new()
        .variant(
            node.option("variant", SidebarVariant::parse)
                .unwrap_or_default(),
        )
        .collapsible(
            node.option("collapsible", SidebarCollapsible::parse)
                .unwrap_or_default(),
        )
        .children(node.kids(window, cx))
        .into_any_element()
}

fn sidebar_menu_badge_of(node: &Node, window: &mut Window, cx: &mut App) -> SidebarMenuBadge {
    SidebarMenuBadge::new().children(node.kids(window, cx))
}

fn sidebar_menu_badge(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    sidebar_menu_badge_of(node, window, cx).into_any_element()
}

/// The count is the row's sibling in React; the kit takes it as the item's
/// badge, so the row can keep its label clear of it and hand it its ink.
fn sidebar_menu_item(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut item = SidebarMenuItem::new();
    for (index, json) in node.children().iter().enumerate() {
        let part = child(node, index, json);
        if part.component() == "SidebarMenuBadge" {
            item = item.badge(sidebar_menu_badge_of(&part, window, cx));
        } else {
            item = item.child(crate::lib::scene::draw(
                json, &node.ctx, &part.path, window, cx,
            ));
        }
    }
    item.into_any_element()
}

fn sidebar_menu_button(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut button = SidebarMenuButton::new(node.id())
        .active(node.flag("active"))
        .children(node.kids(window, cx));
    if let Some(icon) = node.element("icon", window, cx) {
        button = button.icon(icon);
    }
    // Accepted and not yet drawn: the kit's Tooltip is not written yet.
    if let Some(tooltip) = node.string("tooltip") {
        button = button.tooltip(tooltip);
    }
    if let Some(press) = node.press("onClick") {
        button = button.on_click(move |_, window, cx| press(window, cx));
    }
    button.into_any_element()
}

fn sidebar_trigger(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    SidebarTrigger::new(node.id()).into_any_element()
}

// Preferences -----------------------------------------------------------------------

fn preference_group(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    let mut group = PreferenceGroup::new(title);
    if let Some(description) = node.element("description", window, cx) {
        group = group.description(description);
    }
    group.children(node.kids(window, cx)).into_any_element()
}

fn preference_section(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut section = PreferenceSection::new();
    if let Some(label) = node.element("label", window, cx) {
        section = section.label(label);
    }
    if let Some(footer) = node.element("footer", window, cx) {
        section = section.footer(footer);
    }
    section.children(node.kids(window, cx)).into_any_element()
}

fn preference_row(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    let mut row = PreferenceRow::new(node.id(), title);
    if let Some(subtitle) = node.element("subtitle", window, cx) {
        row = row.subtitle(subtitle);
    }
    if let Some(open) = node.action("onOpen") {
        row = row.on_open(move |_, window, cx| open(window, cx));
    }
    row.children(node.kids(window, cx)).into_any_element()
}

// Dialogs and drawers ---------------------------------------------------------------

thread_local! {
    /// The close of each sheet being built, innermost last: a `DialogClose`
    /// deep inside a sheet's parts hands its control this press, so the
    /// control closes the sheet from the keyboard as well as the pointer.
    static CLOSERS: RefCell<Vec<Callback>> = const { RefCell::new(Vec::new()) };
}

/// Builds a sheet's parts with its close in reach.
fn with_close<R>(close: Callback, build: impl FnOnce() -> R) -> R {
    CLOSERS.with(|closers| closers.borrow_mut().push(close));
    let result = build();
    CLOSERS.with(|closers| closers.borrow_mut().pop());
    result
}

/// The open state a scene keeps for a sheet, and the presses that change it.
fn sheet_state(node: &Node, window: &mut Window, cx: &mut App) -> (bool, Callback, Callback) {
    let open = held(node, "open", window, cx, || false);
    let is_open = *open.borrow();
    let set = |value: bool| -> Callback {
        let open = open.clone();
        Rc::new(move |window: &mut Window, _: &mut App| {
            *open.borrow_mut() = value;
            window.refresh();
        })
    };
    (is_open, set(true), set(false))
}

fn length(node: &Node, name: &str) -> Option<Length> {
    match node.get(name) {
        Value::Number(number) => number.as_f64().map(|px| Length::Px(px as f32)),
        Value::String(text) => Length::parse(&text),
        _ => None,
    }
}

fn dialog(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let (open, show, hide) = sheet_state(node, window, cx);
    let trigger = node
        .in_ctx(node.ctx.with_press(show))
        .element("trigger", window, cx);
    let kids = with_close(hide.clone(), || node.kids(window, cx));
    let mut dialog = Dialog::new(node.id())
        .open(open)
        .on_open_change(move |next, window, cx| {
            if !next {
                hide(window, cx)
            }
        })
        .tone(node.option("tone", DialogTone::parse).unwrap_or_default())
        .children(kids);
    if let Some(width) = length(node, "width") {
        dialog = dialog.width(width);
    }
    if let Some(trigger) = trigger {
        dialog = dialog.trigger(trigger);
    }
    dialog.into_any_element()
}

fn dialog_header(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let title = node
        .element("title", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    let mut header = DialogHeader::new(title);
    if let Some(subtitle) = node.element("subtitle", window, cx) {
        header = header.subtitle(subtitle);
    }
    header.into_any_element()
}

fn dialog_close(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let close = CLOSERS.with(|closers| closers.borrow().last().cloned());
    let node = match close {
        Some(close) => node.in_ctx(node.ctx.with_press(close)),
        None => node.in_ctx(node.ctx.clone()),
    };
    let control = node
        .element("render", window, cx)
        .unwrap_or_else(|| div().into_any_element());
    DialogClose::new(node.id(), control).into_any_element()
}

fn drawer(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let (open, show, hide) = sheet_state(node, window, cx);
    let trigger = node
        .in_ctx(node.ctx.with_press(show))
        .element("trigger", window, cx);
    let (kids, footer) = with_close(hide.clone(), || {
        (node.kids(window, cx), node.element("footer", window, cx))
    });
    let mut drawer = Drawer::new(node.id())
        .open(open)
        .on_open_change(move |next, window, cx| {
            if !next {
                hide(window, cx)
            }
        })
        .side(node.option("side", DrawerSide::parse).unwrap_or_default())
        .children(kids);
    if let Some(title) = node.element("title", window, cx) {
        drawer = drawer.title(title);
    }
    if let Some(description) = node.element("description", window, cx) {
        drawer = drawer.description(description);
    }
    if let Some(footer) = footer {
        drawer = drawer.footer(footer);
    }
    if let Some(trigger) = trigger {
        drawer = drawer.trigger(trigger);
    }
    drawer.into_any_element()
}

// Toasts ------------------------------------------------------------------------------

fn toast_provider(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let slot = held(node, "toaster", window, cx, || None::<Toaster>);
    if slot.borrow().is_none() {
        *slot.borrow_mut() = Some(Toaster::new(cx));
    }
    let toaster = slot.borrow().clone().expect("a toaster, made above");
    let raise = toaster.clone();
    let ctx = node.ctx.with_toast(Rc::new(move |request, _, cx| {
        let mut options = ToastOptions::new(request.title).tint(request.tint);
        if let Some((label, run)) = request.action {
            options = options.action(label, move |window, cx| run(window, cx));
        }
        raise.add(options, cx);
    }));
    ToastProvider::new(toaster)
        .children(node.kids_in(&ctx, window, cx))
        .into_any_element()
}

fn toast_viewport(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    ToastViewport::new()
        .placement(
            node.option("placement", ToastPlacement::parse)
                .unwrap_or_default(),
        )
        .into_any_element()
}
