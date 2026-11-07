//! A button that stays down, and a run of them joined into one control.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::icon_slot;
use crate::foundation::{Size, StyledExt, Tint, Variant, control, pointer_down};
use crate::theme::ActiveTheme;

type OnPressedChange = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;
type OnValueChange = Rc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>;

/// A single property that is on or off — bold, wrap, show whitespace.
/// Pressed is drawn as the system draws every selection: the `tinted` recipe,
/// and the label a weight heavier.
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    size: Size,
    variant: Variant,
    tint: Tint,
    label: Option<SharedString>,
    pressed: Option<bool>,
    default_pressed: bool,
    disabled: bool,
    icon_only: bool,
    /// Set by a group: whether this member opens and closes the run.
    grouped: Option<(bool, bool)>,
    focus: Option<(FocusHandle, bool)>,
    children: Vec<AnyElement>,
    on_pressed_change: Option<OnPressedChange>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Toggle {
            id: id.into(),
            size: Size::Tiny,
            variant: Variant::Plain,
            // Neutral, for the reason `IconButton` is: a toolbar of these is
            // chrome.
            tint: Tint::Neutral,
            label: None,
            pressed: None,
            default_pressed: false,
            disabled: false,
            icon_only: false,
            grouped: None,
            focus: None,
            children: Vec::new(),
            on_pressed_change: None,
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Accessible name and tooltip, for a toggle whose label is a glyph.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Controlled: whether it is down.
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// Uncontrolled: where it starts.
    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.default_pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The whole label is one glyph: square at the control height with no
    /// inline padding — what React's `:has(> svg:only-child)` finds.
    pub fn icon_only(mut self, icon_only: bool) -> Self {
        self.icon_only = icon_only;
        self
    }

    pub fn on_pressed_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_pressed_change = Some(Rc::new(handler));
        self
    }

    fn grouped(mut self, first: bool, last: bool) -> Self {
        self.grouped = Some((first, last));
        self
    }

    fn roving(mut self, handle: FocusHandle, tab_stop: bool) -> Self {
        self.focus = Some((handle, tab_stop));
        self
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for Toggle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let profile = self.size.profile(t);
        let state = window.use_keyed_state(kit::child_id(&self.id, "pressed"), cx, {
            let default = self.default_pressed;
            move |_, _| default
        });
        let (focus, tab_stop) = match self.focus {
            Some(focus) => focus,
            None => (kit::focus_handle(&self.id, window, cx), true),
        };
        let pressed = self.pressed.unwrap_or(*state.read(cx));
        let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);

        let mut toggle = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .profile(profile)
            .when(self.icon_only, |this| this.w(px(profile.size)).px_0())
            .when(pressed && !self.disabled, |this| {
                this.font_weight(t.label_strong.font_weight())
            });
        if let Some((first, last)) = self.grouped {
            toggle = toggle
                .when(!first, |this| {
                    // Shrinkable, though it never shrinks: taffy 0.9 sizes a
                    // row whose items pull back with a negative margin to
                    // nothing unless their shrink factor is 1 (its intrinsic
                    // pass scales the margin by the basis when it is 0).
                    this.flex_shrink()
                        .ml(px(-t.spacing_px))
                        .rounded_tl_none()
                        .rounded_bl_none()
                })
                .when(!last, |this| this.rounded_tr_none().rounded_br_none());
        }
        let variant = if pressed {
            Variant::Tinted
        } else {
            self.variant
        };
        toggle = control(toggle, t, variant, self.tint, self.disabled);
        toggle = kit::hint(toggle, self.label);
        toggle = crate::foundation::motion::color_transitions(
            toggle,
            self.id,
            t,
            Some((variant, self.tint)),
            window,
            cx,
        );

        if self.disabled {
            toggle = toggle.cursor_not_allowed();
        } else {
            let controlled = self.pressed.is_some();
            let handler = self.on_pressed_change;
            toggle = toggle
                .cursor_pointer()
                .track_focus(&focus.tab_stop(tab_stop))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |_, window, cx| {
                    let next = !pressed;
                    if !controlled {
                        state.update(cx, |value, cx| {
                            *value = next;
                            cx.notify();
                        });
                    }
                    if let Some(handler) = &handler {
                        handler(next, window, cx);
                    }
                });
        }

        icon_slot(
            profile.icon_size,
            toggle
                .children(self.children)
                .when(visible, |this| this.child(kit::outline(t, profile.radius))),
        )
    }
}

/// One member of a [`ToggleGroup`].
pub struct ToggleItem {
    pub value: SharedString,
    pub label: AnyElement,
    /// Accessible name and tooltip, where the label is a glyph.
    pub hint: Option<SharedString>,
    pub disabled: bool,
    /// The label is one glyph; see [`Toggle::icon_only`].
    pub icon_only: bool,
}

impl ToggleItem {
    pub fn new(value: impl Into<SharedString>, label: impl IntoElement) -> Self {
        ToggleItem {
            value: value.into(),
            label: crate::foundation::text_child(label),
            hint: None,
            disabled: false,
            icon_only: false,
        }
    }

    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_only(mut self, icon_only: bool) -> Self {
        self.icon_only = icon_only;
        self
    }
}

/// Several toggles that belong together, joined into one control: corners
/// only at the ends of the run, and shared edges collapsed to one. The group
/// is one tab stop and the arrow keys move inside it. `multiple` — the
/// default — lets several be on at once, or none.
#[derive(IntoElement)]
pub struct ToggleGroup {
    id: ElementId,
    items: Vec<ToggleItem>,
    value: Vec<SharedString>,
    size: Size,
    variant: Variant,
    tint: Tint,
    multiple: bool,
    disabled: bool,
    on_value_change: Option<OnValueChange>,
}

impl ToggleGroup {
    pub fn new(id: impl Into<ElementId>, items: Vec<ToggleItem>) -> Self {
        ToggleGroup {
            id: id.into(),
            items,
            value: Vec::new(),
            size: Size::Small,
            variant: Variant::Recessed,
            tint: Tint::Primary,
            multiple: true,
            disabled: false,
            on_value_change: None,
        }
    }

    /// The members that are down.
    pub fn value(mut self, value: Vec<SharedString>) -> Self {
        self.value = value;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let count = self.items.len();
        let handles = kit::focus_handles(&self.id, count, window, cx);
        let active = window.use_keyed_state(kit::child_id(&self.id, "active"), cx, |_, _| 0usize);
        let enabled: Vec<bool> = self
            .items
            .iter()
            .map(|item| !self.disabled && !item.disabled)
            .collect();
        let stored = *active.read(cx);
        let tab_stop = kit::focused(&handles, window)
            .or(Some(stored).filter(|index| *index < count && enabled[*index]))
            .or_else(|| enabled.iter().position(|e| *e));

        let group_id = self.id.clone();
        let value = self.value;
        let multiple = self.multiple;
        let handler = self.on_value_change;
        let toggles = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let pressed = value.contains(&item.value);
                let value = value.clone();
                let handler = handler.clone();
                let active = active.clone();
                let member = item.value.clone();
                let mut toggle = Toggle::new(kit::child_id(&group_id, &format!("item-{index}")))
                    .size(self.size)
                    .variant(self.variant)
                    .tint(self.tint)
                    .pressed(pressed)
                    .disabled(!enabled[index])
                    .icon_only(item.icon_only)
                    .grouped(index == 0, index + 1 == count)
                    .roving(handles[index].clone(), tab_stop == Some(index))
                    .on_pressed_change(move |down, window, cx| {
                        active.update(cx, |active, _| *active = index);
                        let next = match (multiple, down) {
                            (true, true) => {
                                let mut next = value.clone();
                                next.push(member.clone());
                                next
                            }
                            (true, false) => {
                                value.iter().filter(|v| **v != member).cloned().collect()
                            }
                            (false, true) => vec![member.clone()],
                            (false, false) => Vec::new(),
                        };
                        if let Some(handler) = &handler {
                            handler(next, window, cx);
                        }
                    })
                    .child(crate::foundation::text_child(item.label));
                if let Some(hint) = item.hint {
                    toggle = toggle.label(hint);
                }
                toggle
            })
            .collect::<Vec<_>>();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .on_key_down(move |event, window, cx| {
                let Some(step) = kit::arrow(&event.keystroke.key, false) else {
                    return;
                };
                let Some(current) = kit::focused(&handles, window) else {
                    return;
                };
                if let Some(next) = kit::target(current, &enabled, step) {
                    window.focus(&handles[next]);
                    active.update(cx, |active, _| *active = next);
                }
                cx.stop_propagation();
            })
            .children(toggles)
    }
}
