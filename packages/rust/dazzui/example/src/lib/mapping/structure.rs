//! The structural components: disclosures, trees, splitters, the sidebar
//! family, preferences, and what floats over a window — dialogs, drawers and
//! toasts.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use dazzui::{
    Accordion, AccordionItem, Collapsible, Dialog, DialogBody, DialogClose, DialogFooter,
    DialogHeader, DialogTone, Drawer, DrawerSide, Length, PreferenceGroup, PreferenceRow,
    PreferenceSection, PreferenceThemeOption, PreferenceThemeSwatches, Preferences, Sidebar,
    SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupAction, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuAction,
    SidebarMenuBadge, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubButtonSize, SidebarMenuSubItem, SidebarProvider,
    SidebarRail, SidebarTrigger, SidebarVariant, Size, Splitter, SplitterOrientation,
    SplitterPanel, Tint, ToastOptions, ToastPlacement, ToastProvider, ToastViewport, Toaster, Tree,
    TreeNode, TreeSize, Variant,
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
    map.insert("SidebarGroupAction", sidebar_group_action);
    map.insert("SidebarMenuAction", sidebar_menu_action);
    map.insert("SidebarGroupContent", sidebar_group_content);
    map.insert("SidebarMenu", sidebar_menu);
    map.insert("SidebarMenuItem", sidebar_menu_item);
    map.insert("SidebarMenuButton", sidebar_menu_button);
    map.insert("SidebarMenuBadge", sidebar_menu_badge);
    map.insert("SidebarMenuSub", sidebar_menu_sub);
    map.insert("SidebarMenuSubItem", sidebar_menu_sub_item);
    map.insert("SidebarMenuSubButton", sidebar_menu_sub_button);
    map.insert("SidebarInset", sidebar_inset);
    map.insert("SidebarTrigger", sidebar_trigger);
    map.insert("SidebarRail", sidebar_rail);
    map.insert("Preferences", preferences);
    map.insert("PreferenceGroup", preference_group);
    map.insert("PreferenceSection", preference_section);
    map.insert("PreferenceRow", preference_row);
    map.insert("PreferenceThemeSwatches", preference_theme_swatches);
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

/// A node the mapping hands a kit component as data has no element of its
/// own until the kit draws it: this records the box the kit drew for it at
/// the node's address.
fn recorded(node: &Node) -> impl FnOnce(AnyElement) -> AnyElement + 'static {
    let at = node.json.get("$at").cloned();
    let path = node.path.clone();
    move |element| crate::lib::specimen::wrap(at.as_ref(), element, &path)
}

fn recorded_surface(node: &Node) -> impl FnOnce(AnyElement) -> AnyElement + 'static {
    let at = node.json.get("$at").cloned();
    move |element| crate::lib::specimen::surface(at.as_ref(), element)
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
            accordion =
                accordion.item(accordion_item_of(&item, window, cx).wrap_with(recorded(&item)));
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
            let name = if item["text"].is_null() {
                text(&item["label"])
            } else {
                text(&item["text"])
            };
            let mut tree_node = TreeNode::new(text(&item["id"]), name)
                .disabled(item["disabled"].as_bool().unwrap_or(false));
            if item["label"].is_object() {
                tree_node = tree_node.label_view(node.draw_value(
                    &item["label"],
                    format!("{here}.label"),
                    window,
                    cx,
                ));
            }
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
                let children = tree_nodes(
                    node,
                    &item["children"],
                    &format!("{here}.children"),
                    window,
                    cx,
                );
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
        .tint(node.option("tint", Tint::parse).unwrap_or_default())
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(handler) = node.handler("onSelectedChange") {
        tree = tree.on_selected_change(move |id, window, cx| {
            handler(Value::String(id.to_string()), window, cx)
        });
    }
    if node.has("expanded") {
        tree = tree.expanded(strings(&node.get("expanded")));
    }
    if let Some(handler) = node.handler("onExpandedChange") {
        tree = tree.on_expanded_change(move |ids, window, cx| {
            handler(
                Value::Array(
                    ids.into_iter()
                        .map(|id| Value::String(id.to_string()))
                        .collect(),
                ),
                window,
                cx,
            )
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
            splitter =
                splitter.panel(splitter_panel_of(&panel, window, cx).wrap_with(recorded(&panel)));
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
container!(sidebar_menu_sub, SidebarMenuSub);
container!(sidebar_menu_sub_item, SidebarMenuSubItem);
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
    let mut item = SidebarMenuItem::new().id(node.id());
    for (index, json) in node.children().iter().enumerate() {
        let part = child(node, index, json);
        if part.component() == "SidebarMenuBadge" {
            let badge = sidebar_menu_badge_of(&part, window, cx);
            item = item.badge(recorded(&part)(badge.into_any_element()));
        } else if part.component() == "SidebarMenuAction" {
            item = item.action(crate::lib::scene::draw(
                json, &node.ctx, &part.path, window, cx,
            ));
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
        .disabled(node.flag("disabled"))
        .size(
            node.option("size", SidebarMenuButtonSize::parse)
                .unwrap_or_default(),
        )
        .variant(
            node.option("variant", Variant::parse)
                .unwrap_or(Variant::Filled),
        )
        .tint(node.option("tint", Tint::parse).unwrap_or(Tint::Primary))
        .children(node.kids(window, cx));
    if let Some(icon) = node.element("icon", window, cx) {
        button = button.icon(icon);
    }
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

macro_rules! sidebar_action {
    ($function:ident, $component:ident $(, $method:ident: $prop:literal)*) => {
        fn $function(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
            let mut button = $component::new(node.id(), node.string("label").unwrap_or_default())
                .size(node.option("size", Size::parse).unwrap_or(Size::Tiny))
                .variant(node.option("variant", Variant::parse).unwrap_or(Variant::Plain))
                .tint(node.option("tint", Tint::parse).unwrap_or(Tint::Neutral))
                .disabled(node.flag("disabled"))
                $(.$method(node.flag($prop)))*
                .children(node.kids(window, cx));
            if let Some(press) = node.press("onClick") {button = button.on_click(move |_, window, cx| press(window, cx));}
            button.into_any_element()
        }
    }
}
sidebar_action!(sidebar_group_action, SidebarGroupAction);
sidebar_action!(sidebar_menu_action, SidebarMenuAction, show_on_hover: "showOnHover");

fn sidebar_menu_sub_button(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let mut button = SidebarMenuSubButton::new(node.id())
        .active(node.flag("active"))
        .disabled(node.flag("disabled"))
        .size(
            node.option("size", SidebarMenuSubButtonSize::parse)
                .unwrap_or_default(),
        )
        .variant(
            node.option("variant", Variant::parse)
                .unwrap_or(Variant::Filled),
        )
        .tint(node.option("tint", Tint::parse).unwrap_or(Tint::Primary))
        .children(node.kids(window, cx));
    if let Some(icon) = node.element("icon", window, cx) {
        button = button.icon(icon);
    }
    if let Some(press) = node.press("onClick") {
        button = button.on_click(move |_, window, cx| press(window, cx));
    }
    button.into_any_element()
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
        .wrap_with(recorded_surface(node))
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
        .wrap_with(recorded_surface(node))
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
        .wrap_with(recorded_surface(node))
        .placement(
            node.option("placement", ToastPlacement::parse)
                .unwrap_or_default(),
        )
        .into_any_element()
}

fn sidebar_rail(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let mut rail = SidebarRail::new(node.id()).disabled(node.flag("disabled"));
    if let Some(label) = node.string("label") {
        rail = rail.label(label);
    }
    if let Some(press) = node.press("onClick") {
        rail = rail.on_click(move |_, window, cx| press(window, cx));
    }
    rail.into_any_element()
}

fn preference_theme_swatches(node: &Node, _: &mut Window, _: &mut App) -> AnyElement {
    let options = node
        .get("options")
        .as_array()
        .into_iter()
        .flatten()
        .map(|option| {
            let mut result = PreferenceThemeOption::new(
                option["value"].as_str().unwrap_or_default().to_owned(),
                option["label"].as_str().unwrap_or_default().to_owned(),
            );
            if let Some(theme) = option["theme"].as_str() {
                result = result.theme(theme.to_owned());
            }
            result
        })
        .collect();
    let mut swatches = PreferenceThemeSwatches::new(node.id(), options)
        .aria_label(node.string("aria-label").unwrap_or_default());
    if let Some(value) = node.string("value") {
        swatches = swatches.value(value);
    }
    if let Some(value) = node.string("defaultValue") {
        swatches = swatches.default_value(value);
    }
    if let Some(change) = node.handler("onValueChange") {
        swatches = swatches.on_value_change(move |value, window, cx| {
            change(Value::String(value.to_string()), window, cx)
        });
    }
    swatches.into_any_element()
}
