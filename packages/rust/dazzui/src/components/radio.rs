//! One choice of several, where exactly one holds.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::foundation::{ControlColors, Size, State, StyledExt, Tint, Variant, pointer_down};
use crate::theme::ActiveTheme;

type OnSelect = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
type OnValueChange = Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>;

/// One choice. It only means something inside a [`RadioGroup`], which is
/// where the selection lives — a radio outside one is never checked.
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    size: Size,
    tint: Tint,
    checked: bool,
    disabled: bool,
    children: Vec<AnyElement>,
    focus: Option<(FocusHandle, bool)>,
    on_select: Option<OnSelect>,
}

impl Radio {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Radio {
            id: id.into(),
            size: Size::Medium,
            tint: Tint::Primary,
            checked: false,
            disabled: false,
            children: Vec::new(),
            focus: None,
            on_select: None,
        }
    }

    /// `small`, `medium` or `large`; `tiny` draws as `small`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

    /// The group's roving focus: this radio's handle, and whether it is the
    /// group's one tab stop.
    pub(crate) fn roving(mut self, handle: FocusHandle, tab_stop: bool) -> Self {
        self.focus = Some((handle, tab_stop));
        self
    }
}

impl ParentElement for Radio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Radio {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let (focus, tab_stop) = match self.focus {
            Some(focus) => focus,
            None => (kit::focus_handle(&self.id, window, cx), true),
        };
        let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);
        let (edge, dot) = match self.size {
            Size::Tiny | Size::Small => (t.radio_small_box, t.radio_small_dot),
            Size::Medium => (t.radio_medium_box, t.radio_medium_dot),
            Size::Large => (t.radio_large_box, t.radio_large_dot),
        };
        let accent = ControlColors::resolve(t, Variant::Filled, self.tint, State::Normal).surface;

        let ring = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(edge))
            .border(px(t.radio_thickness))
            .rounded(px(t.radius_full))
            .border_color(t.color_border_muted.hsla())
            .when(self.checked, |this| {
                this.border_color(accent)
                    .child(div().size(px(dot)).rounded(px(t.radius_full)).bg(accent))
            });

        let mut row = div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .gap(px(t.spacing_2_5))
            .type_style(t.label_quiet)
            .text_color(if self.checked {
                t.color_content.hsla()
            } else {
                t.color_content_muted.hsla()
            })
            .when(self.checked, |this| {
                this.font_weight(t.label_medium.font_weight())
            })
            .child(ring)
            .child(div().flex().min_w_0().children(self.children))
            .when(visible, |this| this.child(kit::outline(t, t.radius_tiny)));

        if self.disabled {
            row = row.opacity(0.6).cursor_not_allowed();
        } else {
            let on_select = self.on_select;
            row = row
                .cursor_pointer()
                .track_focus(&focus.tab_stop(tab_stop))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |event, window, cx| {
                    if kit::is_enter(event) {
                        return;
                    }
                    if let Some(on_select) = &on_select {
                        on_select(window, cx);
                    }
                });
        }
        row
    }
}

/// One option of a [`RadioGroup`].
pub struct RadioGroupOption {
    pub value: SharedString,
    pub label: AnyElement,
    pub disabled: bool,
}

impl RadioGroupOption {
    pub fn new(value: impl Into<SharedString>, label: impl IntoElement) -> Self {
        RadioGroupOption {
            value: value.into(),
            label: label.into_any_element(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A column of radios that are one answer. The group is one tab stop; the
/// arrow keys move through it and select as they go.
#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    options: Vec<RadioGroupOption>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    size: Size,
    tint: Tint,
    disabled: bool,
    on_value_change: Option<OnValueChange>,
}

impl RadioGroup {
    pub fn new(id: impl Into<ElementId>, options: Vec<RadioGroupOption>) -> Self {
        RadioGroup {
            id: id.into(),
            options,
            value: None,
            default_value: None,
            size: Size::Medium,
            tint: Tint::Primary,
            disabled: false,
            on_value_change: None,
        }
    }

    /// Controlled: the option shown selected.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Uncontrolled: where the selection starts.
    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
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

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = window.use_keyed_state(kit::child_id(&self.id, "value"), cx, {
            let default = self.default_value.clone();
            move |_, _| default
        });
        let current = self.value.clone().or_else(|| state.read(cx).clone());
        let handles = kit::focus_handles(&self.id, self.options.len(), window, cx);
        let enabled: Vec<bool> = self
            .options
            .iter()
            .map(|option| !self.disabled && !option.disabled)
            .collect();
        let values: Vec<SharedString> = self
            .options
            .iter()
            .map(|option| option.value.clone())
            .collect();

        let controlled = self.value.is_some();
        let handler = self.on_value_change.clone();
        let select: Rc<dyn Fn(SharedString, &mut Window, &mut App)> =
            Rc::new(move |value, window, cx| {
                if !controlled {
                    let value = value.clone();
                    state.update(cx, |current, cx| {
                        *current = Some(value);
                        cx.notify();
                    });
                }
                if let Some(handler) = &handler {
                    handler(value, window, cx);
                }
            });

        let checked_index = values
            .iter()
            .position(|value| Some(value) == current.as_ref())
            .filter(|index| enabled[*index]);
        let tab_stop = checked_index.or_else(|| enabled.iter().position(|e| *e));

        let group_id = self.id.clone();
        let rows = self
            .options
            .into_iter()
            .enumerate()
            .map(|(index, option)| {
                let select = select.clone();
                let value = option.value.clone();
                Radio::new(kit::child_id(&group_id, &format!("option-{index}")))
                    .size(self.size)
                    .tint(self.tint)
                    .checked(Some(&option.value) == current.as_ref())
                    .disabled(!enabled[index])
                    .roving(handles[index].clone(), tab_stop == Some(index))
                    .on_select(move |window, cx| select(value.clone(), window, cx))
                    .child(option.label)
            })
            .collect::<Vec<_>>();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(t.spacing_2_5))
            .on_key_down(move |event, window, cx| {
                let Some(step) = kit::arrow(&event.keystroke.key, true) else {
                    return;
                };
                let Some(current) = kit::focused(&handles, window) else {
                    return;
                };
                if let Some(next) = kit::target(current, &enabled, step) {
                    window.focus(&handles[next]);
                    select(values[next].clone(), window, cx);
                }
                cx.stop_propagation();
            })
            .children(rows)
    }
}
