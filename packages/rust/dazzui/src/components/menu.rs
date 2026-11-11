//! Menus: the one a button opens, the one a right-click opens, and the bar
//! that holds several.
//!
//! All three draw the same sheet and the same rows (popup.rs) through one
//! row renderer, [`menu_sheet`], so a right-click menu cannot drift from the
//! one a button opens. The sheet closes on an outside press and on Escape,
//! roves its highlight with the arrow keys, Home and End, chooses with Enter
//! or Space, and types ahead.

use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    Styled, Window, div, prelude::*, px, size,
};

use crate::components::native_menu::{
    NativeMenuEntry, arm_native_menu, close_native_menu, native_mode_active, native_pointer,
    native_press, native_state_for, present_native_menu, set_on_dismiss, set_open,
};
use crate::components::popup::{
    Align, Anchor, DismissOverrides, OnOpenChange, Placement, PopupRow, PopupState, Press, Side,
    TriggerFn, dismissable_with, measured, open_state, part_id, part_index, positioned, sheet,
    toggle_press,
};
use crate::foundation::{Tint, focus_visible, pointer_down, with_outline};
use crate::theme::ActiveTheme;

/// One row of a menu.
pub struct MenuItem {
    label: SharedString,
    icon: Option<AnyElement>,
    shortcut: Option<AnyElement>,
    /// The plain-text form of the shortcut slot — what a native popup row
    /// wears for display. A hint, never an accelerator.
    shortcut_hint: Option<SharedString>,
    checked: Option<bool>,
    disabled: bool,
    on_select: Option<Press>,
    /// What a right arrow opens: the row becomes a branch wearing the
    /// chevron, and its list hangs beside it, itself holding branches.
    children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        MenuItem {
            label: label.into(),
            icon: None,
            shortcut: None,
            shortcut_hint: None,
            checked: None,
            disabled: false,
            on_select: None,
            children: Vec::new(),
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

    /// The shortcut slot as plain display text, what a native popup row
    /// carries when `MenuPresentation::Native` is in force. Display only —
    /// never an accelerator registered with the OS.
    pub fn shortcut_hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.shortcut_hint = Some(hint.into());
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

    /// The rows a right arrow opens beside this one — the menu nests as
    /// deep as its data does.
    pub fn submenu(mut self, items: impl IntoIterator<Item = MenuItem>) -> Self {
        self.children.extend(items);
        self
    }
}

/// A key the sheet does not handle itself, offered to whoever holds it — the
/// bar's left and right arrows. Whether it was taken.
type ExtraKey = Rc<dyn Fn(&str, &mut Window, &mut App) -> bool>;

/// The rows an OS popup wears: the items flattened, in order. Element slots
/// — an icon, a `KeyCap` — do not survive the trip to a native row; the
/// plain-text hint does. An item holding `submenu` rows hands the nesting
/// to the OS.
pub(crate) fn native_entries(items: &[MenuItem]) -> Vec<NativeMenuEntry> {
    fn entries(items: &[MenuItem]) -> Vec<NativeMenuEntry> {
        items
            .iter()
            .map(|item| {
                NativeMenuEntry::new(item.label.to_string())
                    .enabled(!item.disabled)
                    .checked(item.checked)
                    .hint(item.shortcut_hint.as_ref().map(|hint| hint.to_string()))
                    .on_select(item.on_select.clone())
                    .children(entries(&item.children))
            })
            .collect()
    }
    entries(items)
}

/// What a sheet level inherits from the one above it: what its final close
/// and its Escape run, the id its branches hang from, and the bar's arrows
/// when this sheet belongs to one.
struct SheetScope {
    /// Choosing a leaf, Tab, or an outside press: the whole menu goes —
    /// every level, then this one, then the host hears it close.
    close: Press,
    /// Escape: one level. The root's is its own close; a child's closes it
    /// and hands the keyboard back to its parent.
    escape: Press,
    /// The bar's left and right arrows, when this sheet belongs to one.
    extra_key: Option<ExtraKey>,
    /// The id this level's branch states hang from.
    base: ElementId,
    /// Where the sheet this level opens from was drawn, when there is one —
    /// a press on the parent's rows is theirs, not this sheet's dismissal.
    outer_sheet: Option<Anchor>,
}

/// The scope a root sheet hands [`menu_sheet`]: its close is hide-and-
/// notify, its Escape the same, and its branches hang from the menu's id.
fn root_scope(
    state: &Entity<PopupState>,
    on_open_change: Option<OnOpenChange>,
    extra_key: Option<ExtraKey>,
    base: ElementId,
) -> SheetScope {
    let close: Press = {
        let state = state.clone();
        Rc::new(move |window, cx| {
            state.update(cx, |state, cx| {
                state.close_descendants(window, cx);
                state.hide(window, cx);
            });
            if let Some(notify) = &on_open_change {
                notify(false, window, cx);
            }
        })
    };
    SheetScope {
        escape: close.clone(),
        close,
        extra_key,
        base,
        outer_sheet: None,
    }
}

/// The rows, in the sheet every menu wears, wired to `state`: the pointer
/// and the keyboard highlight, a choice closes the menu and then runs. A
/// branch row opens its own level beside itself — the child sheet draws
/// inside this one, so its portal registers under this sheet's nest and a
/// press outside is judged against both.
fn menu_sheet(
    items: Vec<MenuItem>,
    state: &Entity<PopupState>,
    tint: Tint,
    keep: &Anchor,
    scope: &SheetScope,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let t = *cx.theme();
    let highlighted = state.read(cx).highlighted;
    let open = state.read(cx).open_submenu;
    // The sheet reports where it was drawn — the box the child level names
    // as its outside press's exempt zone.
    let sheet_anchor = Anchor::default();
    let checked = items.iter().position(|item| item.checked == Some(true));
    let enabled: Rc<Vec<bool>> = Rc::new(items.iter().map(|item| !item.disabled).collect());
    let labels: Rc<Vec<SharedString>> =
        Rc::new(items.iter().map(|item| item.label.clone()).collect());
    let selects: Rc<Vec<Option<Press>>> =
        Rc::new(items.iter().map(|item| item.on_select.clone()).collect());
    let close = scope.close.clone();

    let choose: Rc<dyn Fn(usize, &mut Window, &mut App)> = {
        let enabled = enabled.clone();
        let close = close.clone();
        Rc::new(move |index, window, cx| {
            if !enabled.get(index).copied().unwrap_or(false) {
                return;
            }
            close(window, cx);
            if let Some(Some(select)) = selects.get(index) {
                select(window, cx);
            }
        })
    };

    // One state per branch row: the level's own highlight, focus, and the
    // branch it may open in turn. A level that closed without walking down
    // — a leaf chosen below it, or the whole menu going — is brought down
    // quietly here, since its sheet is not drawn to hear it.
    let branches: Rc<HashMap<usize, (Entity<PopupState>, Rc<Vec<bool>>)>> = Rc::new(
        items
            .iter()
            .enumerate()
            .filter(|(_, item)| !item.children.is_empty())
            .map(|(index, item)| {
                let id = part_index(&scope.base, "sub", index);
                let child = PopupState::keyed(&id, false, window, cx);
                if child.read(cx).open && open != Some(index) {
                    child.update(cx, |child, _| {
                        child.open = false;
                        child.highlighted = None;
                        child.child = None;
                    });
                }
                let child_enabled: Rc<Vec<bool>> =
                    Rc::new(item.children.iter().map(|child| !child.disabled).collect());
                (index, (child, child_enabled))
            })
            .collect(),
    );

    // What opens a branch: the open one goes first — its own descendants,
    // then it — and the branch's level takes the keyboard, as a native menu
    // hands it across. From a pointer the level opens unhighlighted; from
    // the keys, on its first enabled row.
    let open_branch: Rc<dyn Fn(usize, Option<usize>, &mut Window, &mut App)> = {
        let state = state.clone();
        let branches = branches.clone();
        Rc::new(move |index, highlighted, window, cx| {
            let Some((child, _)) = branches.get(&index) else {
                return;
            };
            let child = child.clone();
            state.update(cx, |state, cx| {
                if state.open_submenu == Some(index) {
                    return;
                }
                state.close_descendants(window, cx);
                state.open_submenu = Some(index);
                child.update(cx, |child, cx| child.show(highlighted, window, cx));
            });
        })
    };

    // What closes the branch this level holds open — its descendants first,
    // then it, the keyboard stepping back here.
    let close_child: Press = {
        let state = state.clone();
        Rc::new(move |window, cx| {
            state.update(cx, |state, cx| {
                state.close_child(window, cx);
            });
        })
    };

    let mut rows: Vec<AnyElement> = Vec::new();
    let mut anchors: HashMap<usize, Anchor> = HashMap::new();
    let mut nested: HashMap<usize, Vec<MenuItem>> = HashMap::new();
    for (index, item) in items.into_iter().enumerate() {
        let branch = !item.children.is_empty();
        let hover = state.clone();
        let choose = choose.clone();
        let branches_for_hover = branches.clone();
        let mut row = PopupRow::new(("row", index), item.label)
            .tint(tint)
            .highlighted(highlighted == Some(index))
            .disabled(item.disabled)
            .on_hover(move |hovered, window, cx| {
                hover.update(cx, |state, cx| {
                    if hovered {
                        state.highlighted = Some(index);
                        // The branch under the pointer opens, as the
                        // highlight lands on it; a leaf closes what is
                        // open. The way a native menu reads a passing
                        // pointer, and without a timer to hold it.
                        if let Some((child, _)) = branches_for_hover.get(&index) {
                            if state.open_submenu != Some(index) {
                                let child = child.clone();
                                state.close_descendants(window, cx);
                                state.open_submenu = Some(index);
                                child.update(cx, |child, cx| child.show(None, window, cx));
                            }
                        } else {
                            state.close_descendants(window, cx);
                        }
                    } else if state.highlighted == Some(index) {
                        state.highlighted = None;
                    }
                    cx.notify();
                })
            });
        if branch {
            let toggle_state = state.clone();
            let open_branch_for_press = open_branch.clone();
            row = row.branch().on_press(move |window, cx| {
                let already = toggle_state.read(cx).open_submenu == Some(index);
                if already {
                    toggle_state.update(cx, |state, cx| {
                        state.close_child(window, cx);
                    });
                } else {
                    open_branch_for_press(index, None, window, cx);
                }
            });
            if item.checked.is_some() {
                row = row.check(checked == Some(index));
            }
            if let Some(icon) = item.icon {
                row = row.icon(icon);
            }
            if let Some(shortcut) = item.shortcut {
                row = row.shortcut(shortcut);
            }
            // The child sheet hangs from the row: its anchor is where the
            // row was drawn, written when the row paints.
            let anchor = Anchor::default();
            anchors.insert(index, anchor.clone());
            nested.insert(index, item.children);
            rows.push(measured(&anchor, row).into_any_element());
        } else {
            row = row.on_press(move |window, cx| choose(index, window, cx));
            if item.checked.is_some() {
                row = row.check(checked == Some(index));
            }
            if let Some(icon) = item.icon {
                row = row.icon(icon);
            }
            if let Some(shortcut) = item.shortcut {
                row = row.shortcut(shortcut);
            }
            rows.push(row.into_any_element());
        }
    }

    // The open branch's sheet, inside this one so its portal nests.
    if let Some(index) = open
        && let Some(anchor) = anchors.remove(&index)
        && let Some(children) = nested.remove(&index)
    {
        let id = part_index(&scope.base, "sub", index);
        // Escape closes this level alone, handing the keyboard back.
        let child_escape = close_child.clone();
        // Left closes it too, once nothing of its own is open.
        let left = {
            let close_child = close_child.clone();
            Rc::new(move |key: &str, window: &mut Window, cx: &mut App| {
                if key == "left" {
                    close_child(window, cx);
                    true
                } else {
                    false
                }
            })
        };
        let child_scope = SheetScope {
            close: scope.close.clone(),
            escape: child_escape,
            extra_key: Some(left),
            base: id,
            outer_sheet: Some(sheet_anchor.clone()),
        };
        // The parent hears this level as its child, closable as one.
        let attach_state = state.clone();
        let (attach_child, _) = branches.get(&index).expect("the open branch is one").clone();
        let child_for_close = attach_child.clone();
        let child_for_link = attach_child.clone();
        attach_state.update(cx, |state, _| {
            state.attach_child(
                child_for_link,
                Rc::new(move |window, cx| {
                    child_for_close.update(cx, |child, cx| {
                        child.close_descendants(window, cx);
                        child.hide(window, cx);
                    });
                }),
            );
        });
        let sheet = menu_sheet(children, &attach_child, tint, &anchor, &child_scope, window, cx);
        rows.push(
            positioned(&anchor, Placement::side(Side::Right, Align::Start, &t), sheet)
                .into_any_element(),
        );
    }

    let keys = state.clone();
    let branches_for_keys = branches.clone();
    let open_branch_for_keys = open_branch.clone();
    let close_for_keys = close.clone();
    let scope_extra = scope.extra_key.clone();
    let sheet =
        sheet(&t)
            .id("menu")
            .children(rows)
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                let key = event.keystroke.key.as_str();
                let highlighted = keys.read(cx).highlighted;
                // The submenu keys outrank the bar's arrows: a right arrow
                // on a branch opens it onto its first enabled row, a left
                // one closes the level open.
                if let Some(at) = highlighted
                    && key == "right"
                    && let Some((_, child_enabled)) = branches_for_keys.get(&at)
                {
                    let first = child_enabled.iter().position(|&enabled| enabled);
                    open_branch_for_keys(at, first, window, cx);
                    cx.stop_propagation();
                    return;
                }
                if key == "left" && keys.read(cx).open_submenu.is_some() {
                    keys.update(cx, |state, cx| {
                        state.close_child(window, cx);
                    });
                    cx.stop_propagation();
                    return;
                }
                if let Some(extra) = &scope_extra
                    && extra(key, window, cx)
                {
                    cx.stop_propagation();
                    return;
                }
                let handled = match key {
                    "tab"
                        if !event.keystroke.modifiers.control
                            && !event.keystroke.modifiers.alt
                            && !event.keystroke.modifiers.platform =>
                    {
                        close_for_keys(window, cx);
                        // Leave traversal to the enclosing modal or the kit's
                        // unhandled-Tab fallback, starting at the trigger.
                        false
                    }
                    "enter" | "space" => {
                        if let Some(index) = keys.read(cx).highlighted {
                            if let Some((_, child_enabled)) = branches_for_keys.get(&index) {
                                let first = child_enabled.iter().position(|&enabled| enabled);
                                open_branch_for_keys(index, first, window, cx);
                            } else {
                                choose(index, window, cx);
                            }
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
    let sheet = dismissable_with(
        sheet,
        state,
        keep,
        None,
        DismissOverrides {
            outside: Some(scope.close.clone()),
            escape: Some(scope.escape.clone()),
            outer: scope.outer_sheet.clone(),
        },
        cx,
    );
    // The sheet reports where it was drawn — the box the child level names
    // as its outside press's exempt zone.
    measured(&sheet_anchor, sheet).into_any_element()
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
        let native = native_state_for(&self.id, window, cx);
        let native_mode = native_mode_active(cx);
        let failed = native.read(cx).failed;
        // A close — or a scope that changed its mind about native — takes
        // the OS popup with it.
        if !open || !native_mode {
            close_native_menu(&native, cx);
        }
        let entries = native_entries(&self.items);
        // What the OS's dismissal runs: the host hears the close.
        set_on_dismiss(
            &native,
            if native_mode {
                self.on_open_change.clone()
            } else {
                None
            },
            cx,
        );
        let native_open = native_mode && !failed;
        let press = if native_open {
            // The trigger presents the OS popup itself, at where the last
            // frame drew it.
            native_press(
                &state,
                &native,
                entries.clone(),
                anchor.clone(),
                self.align == Align::End,
                self.on_open_change.clone(),
            )
        } else {
            toggle_press(&state, None, self.on_open_change.clone())
        };
        let trigger = self
            .trigger
            .map(|trigger| measured(&anchor, trigger(press, open, window, cx)));
        let popup = if open && native_open {
            // The list is the OS's; the trigger is all this element draws. A
            // `default_open` or a host's controlled open arrives with no
            // press to present from — the next frame does it.
            arm_native_menu(
                &state,
                &native,
                entries,
                anchor,
                self.align == Align::End,
                window,
                cx,
            );
            None
        } else {
            open.then(|| {
                let scope = root_scope(&state, self.on_open_change.clone(), None, self.id.clone());
                let sheet = menu_sheet(self.items, &state, self.tint, &anchor, &scope, window, cx);
                positioned(&anchor, Placement::side(self.side, self.align, t), sheet)
            })
        };
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
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let native = native_state_for(&self.id, window, cx);
        let native_mode = native_mode_active(cx);
        let failed = native.read(cx).failed;
        let (open, point, region) = {
            let state = state.read(cx);
            (state.open, state.point, state.anchor.clone())
        };
        if !open || !native_mode {
            close_native_menu(&native, cx);
        }
        let entries = native_entries(&self.items);
        let popup = if open && native_mode && !failed {
            // The list is the OS's, at the right-click point; the press
            // below presents it, and this element stops drawing its own.
            None
        } else {
            open.then(|| {
                let at = Anchor::at(Bounds::new(point.unwrap_or_default(), size(px(0.), px(0.))));
                let scope = root_scope(&state, None, None, self.id.clone());
                let sheet = menu_sheet(self.items, &state, self.tint, &region, &scope, window, cx);
                positioned(&at, Placement::side(Side::Bottom, Align::Start, t), sheet)
            })
        };
        let right = state.clone();
        let right_state = state.clone();
        let right_native = native.clone();
        let right_entries = entries;
        let left = state.clone();
        let native_click = native_mode && !failed;
        let wrapped = div()
            .id(self.id)
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                let position = window
                    .painted_bounds(Bounds::new(event.position, size(px(0.), px(0.))))
                    .origin;
                right.update(cx, |state, cx| {
                    state.point = Some(position);
                    if native_click {
                        // No sheet to hand the keyboard to.
                        state.open = true;
                        cx.notify();
                    } else {
                        state.show(None, window, cx);
                    }
                });
                if native_click {
                    // The list is the OS's, at the pointer.
                    let at = Anchor::at(Bounds::new(position, size(px(0.), px(0.))));
                    present_native_menu(
                        right_entries.clone(),
                        &at,
                        false,
                        &right_state,
                        &right_native,
                        window,
                        cx,
                    );
                }
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
        let (names, focus) = {
            let bar = bar.read(cx);
            (bar.names.clone(), bar.focus.clone())
        };
        let whole = state.read(cx).anchor.clone();
        let disabled: Vec<bool> = self
            .menus
            .iter()
            .map(|menu| self.disabled || menu.disabled)
            .collect();

        let native = native_state_for(&self.id, window, cx);
        let native_mode = native_mode_active(cx);
        if !native_mode {
            close_native_menu(&native, cx);
        }

        // What the OS's dismissal runs: the slide the drawn bar does while a
        // menu is open. The OS owns the tracking loop, so the pointer can
        // only be followed between its menus — when one closes, whichever
        // name the pointer has come to rest on opens next. Weakly tied to
        // the state, which is what holds the closure.
        if native_mode {
            let follow: OnOpenChange = {
                let state = state.downgrade();
                let bar = bar.clone();
                let native = native.downgrade();
                let names = names.clone();
                let disabled = disabled.clone();
                let entries: Vec<Vec<NativeMenuEntry>> = self
                    .menus
                    .iter()
                    .map(|menu| native_entries(&menu.items))
                    .collect();
                Rc::new(move |open, window, cx| {
                    if open {
                        return;
                    }
                    let (Some(state), Some(native)) = (state.upgrade(), native.upgrade()) else {
                        return;
                    };
                    let active = bar.read(cx).active;
                    let at = native_pointer(window);
                    let next = at.and_then(|at| {
                        names
                            .iter()
                            .enumerate()
                            .find(|(index, name)| {
                                *index != active && !disabled[*index] && name.contains(at)
                            })
                            .map(|(index, _)| index)
                    });
                    let Some(next) = next else {
                        return;
                    };
                    bar.update(cx, |bar, _| bar.active = next);
                    set_open(&state, true, cx);
                    present_native_menu(
                        entries[next].clone(),
                        &names[next],
                        false,
                        &state,
                        &native,
                        window,
                        cx,
                    );
                })
            };
            set_on_dismiss(&native, Some(follow), cx);
        }

        let open = state.read(cx).open;
        let active = bar.read(cx).active;

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
                .debug_selector(move || format!("dz-menubar-name-{index}").into())
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
                .child(crate::foundation::text_child(menu.label.clone()));
            let ramp = self.tint.ramp(t);
            let wash = t.control_color_normal_surface_hovered.resolve(ramp);
            if disabled {
                name = name.text_color(t.control_color_normal_content_disabled.resolve(ramp));
            } else {
                name = name
                    .text_color(t.color_content.hsla())
                    .track_focus(&focus[index].clone().tab_stop(true))
                    .when(visible, |this| with_outline(this, t))
                    .when(is_open, |this| this.bg(wash))
                    .hover(move |style| style.bg(wash))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx));
                if native_mode {
                    // The name hands its items to the OS; there is no sheet
                    // open for a hover to slide from, so the pointer only
                    // ever follows between the OS's own menus — the
                    // dismissal hook above does that.
                    let state = state.clone();
                    let bar = bar.clone();
                    let native = native.clone();
                    let entries = native_entries(&menu.items);
                    let anchor = names[index].clone();
                    name = name.on_click(move |_, window, cx| {
                        let already = state.read(cx).open && bar.read(cx).active == index;
                        bar.update(cx, |bar, _| bar.active = index);
                        if already {
                            close_native_menu(&native, cx);
                            set_open(&state, false, cx);
                        } else {
                            // Whatever else is out — a menu on another name
                            // — goes away first.
                            close_native_menu(&native, cx);
                            set_open(&state, true, cx);
                            present_native_menu(
                                entries.clone(),
                                &anchor,
                                false,
                                &state,
                                &native,
                                window,
                                cx,
                            );
                        }
                    });
                } else {
                    let click = switch.clone();
                    let follow = switch.clone();
                    let bar_open = state.clone();
                    let bar_active = bar.clone();
                    name = name
                        .on_click(move |_, window, cx| click(index, true, window, cx))
                        .on_hover(move |hovered, window, cx| {
                            if *hovered
                                && bar_open.read(cx).open
                                && bar_active.read(cx).active != index
                            {
                                follow(index, false, window, cx);
                            }
                        });
                }
            }
            names_row.push(measured(&names[index], name).into_any_element());
            sheets.push((index, menu.items));
        }

        let popup = (!native_mode)
            .then(|| {
                sheets
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
                        let scope = root_scope(&state, None, Some(walk), self.id.clone());
                        let sheet =
                            menu_sheet(items, &state, self.tint, &whole, &scope, window, cx);
                        positioned(
                            &names[index],
                            Placement::side(Side::Bottom, Align::Start, t),
                            sheet,
                        )
                    })
            })
            .flatten();

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
