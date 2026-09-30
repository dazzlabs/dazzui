//! A choice out of a list too long to lay out.
//!
//! The list is the same sheet a `Menu` opens (popup.rs), and it opens over
//! its trigger with the chosen row laid over the value, the way a desktop
//! pop-up button does. The trigger looks exactly like a text field: it wears
//! the field box of `text-field.css`, [`FieldBox`], the same one a
//! `TextField` and a `Combobox` wear.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, point, prelude::*, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::components::popup::{
    Placement, PopupRow, PopupState, Press, dismissable, group_label, group_label_height, measured,
    part_id, positioned, row_height, sheet, toggle_press,
};
use crate::components::text_field::{FieldBox, FieldState};
use crate::foundation::{Size, Tint, focus_visible, pointer_down};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// One choice.
#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        SelectOption {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A choice, or choices under a heading the way `<optgroup>` groups them.
#[derive(Clone, Debug)]
pub enum SelectEntry {
    Option(SelectOption),
    Group {
        label: SharedString,
        options: Vec<SelectOption>,
    },
}

impl SelectEntry {
    pub fn group(label: impl Into<SharedString>, options: Vec<SelectOption>) -> Self {
        SelectEntry::Group {
            label: label.into(),
            options,
        }
    }
}

impl From<SelectOption> for SelectEntry {
    fn from(option: SelectOption) -> Self {
        SelectEntry::Option(option)
    }
}

type OnValueChange = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

/// A select.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    options: Vec<SelectEntry>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    on_value_change: Option<OnValueChange>,
    placeholder: SharedString,
    size: Size,
    tint: Tint,
    mono: bool,
    invalid: bool,
    disabled: bool,
}

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Select {
            id: id.into(),
            options: Vec::new(),
            value: None,
            default_value: None,
            on_value_change: None,
            placeholder: "Choose…".into(),
            size: Size::Medium,
            tint: Tint::Primary,
            mono: false,
            invalid: false,
            disabled: false,
        }
    }

    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<SelectEntry>>) -> Self {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    /// Holds the value for the caller, who hears of changes through
    /// [`on_value_change`](Self::on_value_change).
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// What the trigger prints while nothing is chosen.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// `small`, `medium` or `large` — a text field's sizes.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Set the value in the code face — for a key, an ID, a URL.
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    /// React's `state="error"`: the box re-tinted to danger.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let id = self.id.clone();
        let default_value = self.default_value.clone();
        let held = window.use_keyed_state(part_id(&id, "value"), cx, |_, _| default_value);
        let focus: FocusHandle = window
            .use_keyed_state(part_id(&id, "focus"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let current = self.value.clone().or_else(|| held.read(cx).clone());

        let flat: Vec<SelectOption> = self
            .options
            .iter()
            .flat_map(|entry| match entry {
                SelectEntry::Option(option) => vec![option.clone()],
                SelectEntry::Group { options, .. } => options.clone(),
            })
            .collect();
        let selected = current
            .as_ref()
            .and_then(|value| flat.iter().position(|option| &option.value == value));

        let state = PopupState::keyed(&id, false, window, cx);
        let open = state.read(cx).open && !self.disabled;
        let anchor = state.read(cx).anchor.clone();

        // Choosing: the value moves, the list closes, the caller hears.
        let choose: Rc<dyn Fn(usize, &mut Window, &mut App)> = {
            let state = state.clone();
            let held = held.clone();
            let flat = flat.clone();
            let notify = self.on_value_change.clone();
            let controlled = self.value.is_some();
            Rc::new(move |index, window, cx| {
                let Some(option) = flat.get(index).filter(|option| !option.disabled) else {
                    return;
                };
                if !controlled {
                    held.update(cx, |held, cx| {
                        *held = Some(option.value.clone());
                        cx.notify();
                    });
                }
                state.update(cx, |state, cx| state.hide(window, cx));
                if let Some(notify) = &notify {
                    notify(option.value.clone(), window, cx);
                }
            })
        };

        // The trigger.
        let label = match selected {
            Some(index) => Some(flat[index].label.clone()),
            None => current.clone(),
        };
        let placeholder = label.is_none();
        let lifted = open || focus_visible(&focus, window, cx);
        let toggle: Press = toggle_press(&state, selected, None);
        let mut trigger = FieldBox::new(self.size)
            .tint(self.tint)
            .state(if self.invalid {
                FieldState::Error
            } else {
                FieldState::Default
            })
            .focused(lifted)
            .disabled(self.disabled)
            .mono(self.mono)
            .style(div().id("trigger"), t)
            .gap(px(t.spacing_2))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .when(placeholder, |this| {
                        this.text_color(t.color_content_faint.hsla())
                    })
                    .child(label.unwrap_or_else(|| self.placeholder.clone())),
            )
            .child(icon_slot(
                t.icon_small,
                div()
                    .flex()
                    .flex_none()
                    .text_color(t.color_content_subtle.hsla())
                    .child(Icon::new(glyphs::CHEVRON_DOWN_12_REGULAR)),
            ));
        if self.disabled {
            trigger = trigger.cursor_not_allowed();
        } else {
            let click = toggle.clone();
            let keys = toggle.clone();
            let is_open = state.clone();
            trigger = trigger
                .cursor_pointer()
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .on_click(move |_, window, cx| click(window, cx))
                .on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if matches!(event.keystroke.key.as_str(), "down" | "up")
                        && !is_open.read(cx).open
                    {
                        keys(window, cx);
                        cx.stop_propagation();
                    }
                });
        }

        // The list.
        let popup = open.then(|| {
            let highlighted = state.read(cx).highlighted;
            let enabled: Rc<Vec<bool>> = Rc::new(flat.iter().map(|o| !o.disabled).collect());
            let labels: Rc<Vec<SharedString>> =
                Rc::new(flat.iter().map(|o| o.label.clone()).collect());
            let mut index = 0;
            let mut rows: Vec<AnyElement> = Vec::new();
            let mut previous_group = false;
            // Where the chosen row sits in the sheet, to lay it over the value.
            let mut y = t.stroke_hairline + t.spacing_1;
            let mut chosen_top = None;
            let aligned = selected.unwrap_or(0);
            let mut row = |option: &SelectOption, index: &mut usize, y: &mut f32| {
                let at = *index;
                if at == aligned {
                    chosen_top = Some(*y);
                }
                *y += row_height(t);
                *index += 1;
                let hover = state.clone();
                let choose = choose.clone();
                PopupRow::new(("option", at), option.label.clone())
                    .tint(self.tint)
                    .check(selected == Some(at))
                    .highlighted(highlighted == Some(at))
                    .disabled(option.disabled)
                    .on_hover(move |hovered, _, cx| {
                        hover.update(cx, |state, cx| {
                            if hovered {
                                state.highlighted = Some(at);
                            } else if state.highlighted == Some(at) {
                                state.highlighted = None;
                            }
                            cx.notify();
                        })
                    })
                    .on_press(move |window, cx| choose(at, window, cx))
                    .into_any_element()
            };
            for entry in &self.options {
                match entry {
                    SelectEntry::Option(option) => {
                        rows.push(row(option, &mut index, &mut y));
                        previous_group = false;
                    }
                    SelectEntry::Group { label, options } => {
                        let gap = if previous_group { t.spacing_1 } else { 0. };
                        y += gap + group_label_height(t);
                        let group = div()
                            .flex()
                            .flex_col()
                            .mt(px(gap))
                            .child(group_label(t, label.clone()))
                            .children(
                                options
                                    .iter()
                                    .map(|option| row(option, &mut index, &mut y))
                                    .collect::<Vec<_>>(),
                            );
                        rows.push(group.into_any_element());
                        previous_group = true;
                    }
                }
            }

            // `alignItemWithTrigger`: the chosen row's label over the value.
            let label_inset = t.spacing_1 + t.spacing_2 + t.icon_small + t.spacing_2;
            let shift_x = t.spacing_3 - label_inset;
            let shift_y = (self.size.profile(t).size - row_height(t)) / 2.
                - chosen_top.unwrap_or(t.stroke_hairline + t.spacing_1);
            let trigger_width = anchor
                .bounds()
                .map(|b| f32::from(b.size.width))
                .unwrap_or(0.);

            let keys = state.clone();
            let choose_key = choose.clone();
            let list = sheet(t)
                .id("list")
                .min_w(px(trigger_width - shift_x))
                .children(rows)
                .on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.as_str();
                    let handled = match key {
                        "enter" | "space" => {
                            if let Some(index) = keys.read(cx).highlighted {
                                choose_key(index, window, cx);
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
            let list = dismissable(list, &state, &anchor, None, cx);
            positioned(
                &anchor,
                Placement::Over(point(px(shift_x), px(shift_y))),
                list,
            )
        });

        div()
            .id(id)
            .flex()
            .flex_col()
            .w_full()
            .child(measured(&anchor, trigger))
            .children(popup)
    }
}
