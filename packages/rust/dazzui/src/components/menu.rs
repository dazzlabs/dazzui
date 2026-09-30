//! Menus: the one a button opens, the one a right-click opens, and the bar
//! that holds several.
//!
//! All three draw the same sheet and the same rows (popup.rs) through one
//! row renderer, [`menu_sheet`], so a right-click menu cannot drift from the
//! one a button opens. The sheet closes on an outside press and on Escape,
//! roves its highlight with the arrow keys, Home and End, chooses with Enter
//! or Space, and types ahead.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, Div, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString, Stateful,
    StatefulInteractiveElement, Styled, Window, div, prelude::*, px, size,
};

use crate::components::popup::{
    Align, Anchor, OnOpenChange, Placement, PopupRow, PopupState, Press, Side, TriggerFn,
    dismissable, measured, open_state, part_id, positioned, sheet, toggle_press,
};
use crate::foundation::{Tint, focus_visible, pointer_down, with_outline};
use crate::theme::ActiveTheme;

/// One row of a menu.
pub struct MenuItem {
    label: SharedString,
    icon: Option<AnyElement>,
    shortcut: Option<AnyElement>,
    checked: Option<bool>,
    disabled: bool,
    on_select: Option<Press>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        MenuItem {
            label: label.into(),
            icon: None,
            shortcut: None,
            checked: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// Trailing hint — a `KeyCap`, or the glyphs to set in one.
    pub fn shortcut(mut self, shortcut: impl IntoElement) -> Self {
        self.shortcut = Some(shortcut.into_any_element());
        self
    }

    /// Radio-style check state. Set it on every item of a group, including
    /// the unchecked ones, so the leading column stays aligned. As in React,
    /// the group's value is the first checked item: that one wears the mark.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

/// A key the sheet does not handle itself, offered to whoever holds it — the
/// bar's left and right arrows. Whether it was taken.
type ExtraKey = Rc<dyn Fn(&str, &mut Window, &mut App) -> bool>;

/// The rows, in the sheet every menu wears, wired to `state`: the pointer
/// and the keyboard highlight, a choice closes the menu and then runs.
fn menu_sheet(
    items: Vec<MenuItem>,
    state: &Entity<PopupState>,
    tint: Tint,
    keep: &Anchor,
    on_open_change: Option<OnOpenChange>,
    extra_key: Option<ExtraKey>,
    cx: &App,
) -> Stateful<Div> {
    let t = cx.theme();
    let highlighted = state.read(cx).highlighted;
    let checked = items.iter().position(|item| item.checked == Some(true));
    let enabled: Rc<Vec<bool>> = Rc::new(items.iter().map(|item| !item.disabled).collect());
    let labels: Rc<Vec<SharedString>> =
        Rc::new(items.iter().map(|item| item.label.clone()).collect());
    let selects: Rc<Vec<Option<Press>>> =
        Rc::new(items.iter().map(|item| item.on_select.clone()).collect());

    let choose: Rc<dyn Fn(usize, &mut Window, &mut App)> = {
        let state = state.clone();
        let enabled = enabled.clone();
        let notify = on_open_change.clone();
        Rc::new(move |index, window, cx| {
            if !enabled.get(index).copied().unwrap_or(false) {
                return;
            }
            state.update(cx, |state, cx| state.hide(window, cx));
            if let Some(notify) = &notify {
                notify(false, window, cx);
            }
            if let Some(Some(select)) = selects.get(index) {
                select(window, cx);
            }
        })
    };

    let rows = items.into_iter().enumerate().map(|(index, item)| {
        let hover = state.clone();
        let choose = choose.clone();
        let mut row = PopupRow::new(("row", index), item.label)
            .tint(tint)
            .highlighted(highlighted == Some(index))
            .disabled(item.disabled)
            .on_hover(move |hovered, _, cx| {
                hover.update(cx, |state, cx| {
                    if hovered {
                        state.highlighted = Some(index);
                    } else if state.highlighted == Some(index) {
                        state.highlighted = None;
                    }
                    cx.notify();
                })
            })
            .on_press(move |window, cx| choose(index, window, cx));
        if item.checked.is_some() {
            row = row.check(checked == Some(index));
        }
        if let Some(icon) = item.icon {
            row = row.icon(icon);
        }
        if let Some(shortcut) = item.shortcut {
            row = row.shortcut(shortcut);
        }
        row
    });

    let keys = state.clone();
    let sheet =
        sheet(t)
            .id("menu")
            .children(rows)
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                if let Some(extra) = &extra_key
                    && extra(key, window, cx)
                {
                    cx.stop_propagation();
                    return;
                }
                let handled = match key {
                    "enter" | "space" => {
                        if let Some(index) = keys.read(cx).highlighted {
                            choose(index, window, cx);
                        }
                        true
                    }
                    _ => keys.update(cx, |state, cx| {
                        let text = event.keystroke.key_char.as_deref().unwrap_or(key);
                        let handled = state.step(key, &enabled)
                            || (!event.keystroke.modifiers.modified()
                                && state.type_ahead(text, &labels, &enabled));
                        cx.notify();
                        handled
                    }),
                };
                if handled {
                    cx.stop_propagation();
                }
            });
    dismissable(sheet, state, keep, on_open_change, cx)
}

/// A menu a trigger opens.
#[derive(IntoElement)]
pub struct Menu {
    id: ElementId,
    trigger: Option<TriggerFn>,
    items: Vec<MenuItem>,
    side: Side,
    align: Align,
    tint: Tint,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
}

impl Menu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Menu {
            id: id.into(),
            trigger: None,
            items: Vec::new(),
            side: Side::Bottom,
            align: Align::End,
            tint: Tint::Neutral,
            open: None,
            default_open: false,
            on_open_change: None,
        }
    }

    /// What opens the menu, built with the press that does and whether the
    /// menu is open — React's trigger, or the function of the open state.
    pub fn trigger(
        mut self,
        trigger: impl FnOnce(Press, bool, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.trigger = Some(Box::new(trigger));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Menu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, self.default_open, window, cx);
        let open = open_state(&state, self.open, cx);
        let anchor = state.read(cx).anchor.clone();
        let press = toggle_press(&state, None, self.on_open_change.clone());
        let trigger = self
            .trigger
            .map(|trigger| measured(&anchor, trigger(press, open, window, cx)));
        let popup = open.then(|| {
            let sheet = menu_sheet(
                self.items,
                &state,
                self.tint,
                &anchor,
                self.on_open_change,
                None,
                cx,
            );
            positioned(&anchor, Placement::side(self.side, self.align, t), sheet)
        });
        div().id(self.id).flex().children(trigger).children(popup)
    }
}

/// The menu a right-click opens, at the pointer, over the region it wraps.
#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    items: Vec<MenuItem>,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl ContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        ContextMenu {
            id: id.into(),
            items: Vec::new(),
            tint: Tint::Neutral,
            children: Vec::new(),
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl ParentElement for ContextMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let (open, point, region) = {
            let state = state.read(cx);
            (state.open, state.point, state.anchor.clone())
        };
        let popup = open.then(|| {
            let at = Anchor::at(Bounds::new(point.unwrap_or_default(), size(px(0.), px(0.))));
            let sheet = menu_sheet(self.items, &state, self.tint, &region, None, None, cx);
            positioned(&at, Placement::side(Side::Bottom, Align::Start, t), sheet)
        });
        let right = state.clone();
        let left = state.clone();
        let wrapped = div()
            .id(self.id)
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                right.update(cx, |state, cx| {
                    state.point = Some(event.position);
                    state.show(None, window, cx);
                });
                cx.stop_propagation();
            })
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                left.update(cx, |state, cx| state.hide(window, cx));
            })
            .children(self.children)
            .children(popup);
        measured(&region, wrapped)
    }
}

/// One menu on a bar.
pub struct MenubarMenu {
    label: SharedString,
    items: Vec<MenuItem>,
    disabled: bool,
}

impl MenubarMenu {
    /// The name on the bar — File, Edit, View.
    pub fn new(label: impl Into<SharedString>) -> Self {
        MenubarMenu {
            label: label.into(),
            items: Vec::new(),
            disabled: false,
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Which of a bar's menus is the one open, and where each name was drawn.
struct Bar {
    active: usize,
    names: Vec<Anchor>,
    focus: Vec<FocusHandle>,
}

/// The application menu bar: several menus that behave as one. Once one is
/// open, moving the pointer along the bar opens the next without a second
/// click, and the left and right arrows walk the bar.
#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    menus: Vec<MenubarMenu>,
    tint: Tint,
    disabled: bool,
}

impl Menubar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Menubar {
            id: id.into(),
            menus: Vec::new(),
            tint: Tint::Neutral,
            disabled: false,
        }
    }

    pub fn menus(mut self, menus: impl IntoIterator<Item = MenubarMenu>) -> Self {
        self.menus.extend(menus);
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Menubar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let count = self.menus.len();
        let bar = window.use_keyed_state(part_id(&self.id, "bar"), cx, |_, _| Bar {
            active: 0,
            names: Vec::new(),
            focus: Vec::new(),
        });
        bar.update(cx, |bar, cx| {
            bar.names.resize_with(count, Anchor::default);
            while bar.focus.len() < count {
                bar.focus.push(cx.focus_handle());
            }
        });
        let open = state.read(cx).open;
        let (active, names, focus) = {
            let bar = bar.read(cx);
            (bar.active, bar.names.clone(), bar.focus.clone())
        };
        let whole = state.read(cx).anchor.clone();
        let disabled: Vec<bool> = self
            .menus
            .iter()
            .map(|menu| self.disabled || menu.disabled)
            .collect();

        // Opens menu `index` — or closes it, when it is the one open.
        let switch: Rc<dyn Fn(usize, bool, &mut Window, &mut App)> = {
            let state = state.clone();
            let bar = bar.clone();
            Rc::new(move |index, toggle, window, cx| {
                let already = state.read(cx).open && bar.read(cx).active == index;
                bar.update(cx, |bar, cx| {
                    bar.active = index;
                    cx.notify();
                });
                state.update(cx, |state, cx| {
                    if already && toggle {
                        state.hide(window, cx);
                    } else {
                        state.show(None, window, cx);
                    }
                });
            })
        };

        let mut sheets = Vec::new();
        let mut names_row = Vec::new();
        for (index, menu) in self.menus.into_iter().enumerate() {
            let is_open = open && active == index;
            let disabled = disabled[index];
            let visible = focus_visible(&focus[index], window, cx);
            let mut name = div()
                .id(("menubar-item", index))
                .flex()
                .flex_none()
                .items_center()
                .h(px(t.control_small_size))
                .px(px(t.spacing_2))
                .rounded(px(t.radius_small))
                .whitespace_nowrap()
                .font_family(t.control_small_content.face.family())
                .text_size(px(t.control_small_content.size))
                .font_weight(t.control_small_content.font_weight())
                .line_height(px(t.control_small_content.size))
                .child(menu.label);
            let ramp = self.tint.ramp(t);
            let wash = t.control_color_normal_surface_hovered.resolve(ramp);
            if disabled {
                name = name.text_color(t.control_color_normal_content_disabled.resolve(ramp));
            } else {
                let click = switch.clone();
                let follow = switch.clone();
                let bar_open = state.clone();
                let bar_active = bar.clone();
                name = name
                    .text_color(t.color_content.hsla())
                    .track_focus(&focus[index].clone().tab_stop(true))
                    .when(visible, |this| with_outline(this, t))
                    .when(is_open, |this| this.bg(wash))
                    .hover(move |style| style.bg(wash))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                    .on_click(move |_, window, cx| click(index, true, window, cx))
                    .on_hover(move |hovered, window, cx| {
                        if *hovered && bar_open.read(cx).open && bar_active.read(cx).active != index
                        {
                            follow(index, false, window, cx);
                        }
                    });
            }
            names_row.push(measured(&names[index], name).into_any_element());
            sheets.push((index, menu.items));
        }

        let popup = sheets
            .into_iter()
            .find(|(index, _)| open && *index == active)
            .map(|(index, items)| {
                let walk: ExtraKey = {
                    let switch = switch.clone();
                    let disabled = disabled.clone();
                    Rc::new(move |key, window, cx| {
                        let step: isize = match key {
                            "right" => 1,
                            "left" => -1,
                            _ => return false,
                        };
                        let count = disabled.len() as isize;
                        let mut next = index as isize;
                        for _ in 0..count {
                            next = (next + step).rem_euclid(count);
                            if !disabled[next as usize] {
                                break;
                            }
                        }
                        switch(next as usize, false, window, cx);
                        true
                    })
                };
                let sheet = menu_sheet(items, &state, self.tint, &whole, None, Some(walk), cx);
                positioned(
                    &names[index],
                    Placement::side(Side::Bottom, Align::Start, t),
                    sheet,
                )
            });

        measured(
            &whole,
            div()
                .id(self.id)
                .flex()
                .items_center()
                .gap(px(t.spacing_0_5))
                .children(names_row)
                .children(popup),
        )
    }
}
