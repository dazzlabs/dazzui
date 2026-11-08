//! A code, a character to a box.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    App, ElementId, Entity, Focusable, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::components::text_field::{FieldBox, FieldState, part_id, selection_color};
use crate::foundation::text_input::{self, InputState, OnText, TextInput, input_state};
use crate::foundation::{Size, StyledExt, Tint};
use crate::theme::ActiveTheme;
use crate::tokens::Face;

/// The one-time code a reader is copying out of a message.
///
/// A run of cells rather than one field, because that is what makes it fast:
/// typing moves to the next cell, Backspace in an empty cell to the one
/// before, the arrows walk the run, and a paste fills every cell from the
/// one it lands in.
///
/// Each cell is the text controls' box at one character wide, in the mono
/// face — a code is digits being compared.
#[derive(IntoElement)]
pub struct OtpField {
    id: ElementId,
    length: usize,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    on_value_change: Option<OnText>,
    group_after: Option<usize>,
    size: Size,
    state: FieldState,
    tint: Tint,
    disabled: bool,
}

impl OtpField {
    pub fn new(id: impl Into<ElementId>, length: usize) -> Self {
        OtpField {
            id: id.into(),
            length,
            value: None,
            default_value: None,
            on_value_change: None,
            group_after: None,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            disabled: false,
        }
    }

    /// The code, controlled.
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
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    /// Air between two runs of cells — `3` on a six-digit code makes it two
    /// groups of three, which is how a code is read aloud.
    pub fn group_after(mut self, group_after: usize) -> Self {
        self.group_after = Some(group_after);
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: FieldState) -> Self {
        self.state = state;
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

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

/// The run's characters, one per cell, `None` where a cell is empty.
#[derive(Default)]
struct Cells {
    chars: Vec<Option<char>>,
    initialized: bool,
}

impl Cells {
    fn code(&self) -> String {
        self.chars.iter().flatten().collect()
    }

    fn set_code(&mut self, code: &str, length: usize) {
        let mut chars: Vec<Option<char>> = code.chars().take(length).map(Some).collect();
        chars.resize(length, None);
        self.chars = chars;
    }
}

impl RenderOnce for OtpField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let length = self.length;
        let cells = window
            .use_keyed_state(part_id(&self.id, "cells"), cx, |_, _| {
                Rc::new(RefCell::new(Cells::default()))
            })
            .read(cx)
            .clone();
        {
            let mut cells = cells.borrow_mut();
            if let Some(value) = &self.value {
                cells.set_code(value, length);
            } else if !cells.initialized {
                let code = self.default_value.clone().unwrap_or_default();
                cells.set_code(&code, length);
            } else if cells.chars.len() != length {
                let code = cells.code();
                cells.set_code(&code, length);
            }
            cells.initialized = true;
        }
        let inputs: Rc<Vec<Entity<InputState>>> = Rc::new(
            (0..length)
                .map(|index| {
                    input_state(
                        ElementId::NamedChild(
                            Box::new(self.id.clone()),
                            format!("cell-{index}").into(),
                        ),
                        window,
                        cx,
                    )
                })
                .collect(),
        );
        let controlled = self.value.is_some();
        let on_change = self.on_value_change.clone();

        // Writes cells from `index` on, reports the code, and moves the focus
        // to where typing goes next.
        let write: Rc<dyn Fn(usize, &str, &mut Window, &mut App)> = {
            let cells = cells.clone();
            let inputs = inputs.clone();
            Rc::new(move |index, text, window, cx| {
                let (code, changed, next) = {
                    let mut held = cells.borrow_mut();
                    let before = held.code();
                    let mut chars = held.chars.clone();
                    let typed: Vec<char> = text.chars().filter(|ch| !ch.is_whitespace()).collect();
                    let next = if typed.is_empty() {
                        chars[index] = None;
                        index
                    } else {
                        for (offset, ch) in typed.iter().enumerate() {
                            if index + offset < length {
                                chars[index + offset] = Some(*ch);
                            }
                        }
                        (index + typed.len()).min(length - 1)
                    };
                    let code: String = chars.iter().flatten().collect();
                    if !controlled {
                        held.chars = chars;
                    }
                    let changed = code != before;
                    (code, changed, next)
                };
                if next != index {
                    let handle = inputs[next].read(cx).focus_handle(cx);
                    window.focus(&handle);
                }
                if changed && let Some(on_change) = &on_change {
                    on_change(&code, window, cx);
                }
                window.refresh();
            })
        };
        let focus_cell = {
            let inputs = inputs.clone();
            move |index: usize, window: &mut Window, cx: &mut App| {
                let handle = inputs[index].read(cx).focus_handle(cx);
                window.focus(&handle);
            }
        };

        let profile = self.size.profile(t);
        let content = profile.content.in_face(Face::Mono);
        let held = cells.borrow();
        let cells_row = (0..length).map(|index| {
            let input = inputs[index].clone();
            let focused = input.read(cx).is_focused(window);
            let filled = held.chars.get(index).copied().flatten();
            let group_end = self.group_after.is_some_and(|after| {
                after > 0 && index > 0 && index % after == 0 && index != length - 1
            });
            let chrome = FieldBox::new(self.size)
                .tint(self.tint)
                .state(self.state)
                .focused(focused)
                .disabled(self.disabled);
            let mut colors = chrome.colors(t);
            // A filled cell lifts to the paper and keeps it, so the run reads
            // as how far through the code the reader is. An error repaints it
            // and disabled greys it, over the lift.
            if filled.is_some() {
                colors.surface = t.color_surface.hsla();
                colors.border = t.color_border_strong.hsla();
            }
            let invalid = FieldBox::new(self.size)
                .tint(self.tint)
                .state(self.state)
                .disabled(self.disabled)
                .colors(t);
            if self.state == FieldState::Error {
                colors.border = invalid.border;
                colors.surface = invalid.surface;
                colors.content = invalid.content;
            }
            if self.disabled {
                colors.surface = invalid.surface;
                colors.content = invalid.content;
            }

            let write = write.clone();
            let back = focus_cell.clone();
            let back_write = write.clone();
            let forward = focus_cell.clone();
            let empty = filled.is_none();
            let text = TextInput::new(ElementId::NamedChild(
                Box::new(self.id.clone()),
                format!("cell-input-{index}").into(),
            ))
            .state(input.clone())
            .value(filled.map(String::from).unwrap_or_default())
            .centered(true)
            .select_on_focus(true)
            .disabled(self.disabled)
            .selection_color(selection_color(t))
            .on_change(move |text, window, cx| write(index, text, window, cx));

            div()
                .flex()
                .flex_none()
                .items_center()
                .size(px(profile.size))
                .border(px(t.stroke_hairline))
                .border_color(colors.border)
                .rounded(px(t.control_field_radius))
                .bg(colors.surface)
                .text_color(colors.content)
                .shadow(colors.glow.into_iter().collect())
                .font_family(content.face.family())
                .tabular_nums()
                .text_size(px(content.size))
                .line_height(px(content.line_height))
                .when(group_end, |this| this.ml(px(t.spacing_3)))
                .map(|this| {
                    if self.disabled {
                        this.cursor_not_allowed()
                    } else {
                        this.cursor_text()
                    }
                })
                .when(!self.disabled, |this| {
                    let pressed = input.downgrade();
                    this.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                        pressed
                            .update(cx, |input, cx| input.mouse_down(event, window, cx))
                            .ok();
                    })
                    // Backspace in an empty cell empties the one before and
                    // goes there; the arrows walk the run.
                    .capture_action(move |_: &text_input::Backspace, window, cx| {
                        if empty && index > 0 {
                            cx.stop_propagation();
                            back_write(index - 1, "", window, cx);
                            back(index - 1, window, cx);
                        }
                    })
                    .capture_action({
                        let back = focus_cell.clone();
                        move |_: &text_input::Left, window, cx| {
                            cx.stop_propagation();
                            back(index.saturating_sub(1), window, cx);
                        }
                    })
                    .capture_action(
                        move |_: &text_input::Right, window, cx| {
                            cx.stop_propagation();
                            forward((index + 1).min(length - 1), window, cx);
                        },
                    )
                })
                .child(text)
                .map(|cell| {
                    let cell_id = ElementId::NamedChild(
                        Box::new(self.id.clone()),
                        format!("cell-{index}").into(),
                    );
                    chrome.animate(cell.id(cell_id.clone()), cell_id, t, window, cx)
                })
        });
        let row = div()
            .flex()
            .items_center()
            .gap(px(t.spacing_2))
            .children(cells_row.collect::<Vec<_>>());
        drop(held);
        row
    }
}
