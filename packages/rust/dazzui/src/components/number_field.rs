//! A number, stepped or typed, with the unit it is counted in.

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::components::icon::Icon;
use crate::components::text_field::{
    FieldBox, FieldState, part_id, placeholder_color, selection_color,
};
use crate::foundation::text_input::{self, TextInput, input_state};
use crate::foundation::{ControlColors, Size, State, Tint, Variant, pointer_down};
use crate::glyphs;
use crate::theme::ActiveTheme;

type OnNumber = Rc<dyn Fn(Option<f64>, &mut Window, &mut App)>;

/// What the field holds between frames: the number, and the text being
/// typed while it is not yet one.
#[derive(Default)]
struct Held {
    value: Option<f64>,
    draft: Option<String>,
    initialized: bool,
}

/// A number, with the two things a plain text field never gets right: a
/// stepper, and a value that stays a number. What is typed is parsed and
/// clamped to `min`/`max` when it is committed — Return or leaving the
/// field — and the arrow keys step it.
///
/// The box is the [`TextField`](crate::TextField)'s, so a form's rows do not
/// step; what is the number field's own is the stepper at its trailing edge.
#[derive(IntoElement)]
pub struct NumberField {
    id: ElementId,
    value: Option<Option<f64>>,
    default_value: Option<f64>,
    on_value_change: Option<OnNumber>,
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
    placeholder: SharedString,
    unit: Option<AnyElement>,
    size: Size,
    state: FieldState,
    tint: Tint,
    disabled: bool,
    read_only: bool,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        NumberField {
            id: id.into(),
            value: None,
            default_value: None,
            on_value_change: None,
            min: None,
            max: None,
            step: 1.0,
            placeholder: SharedString::default(),
            unit: None,
            size: Size::Medium,
            state: FieldState::Default,
            tint: Tint::Primary,
            disabled: false,
            read_only: false,
        }
    }

    /// The number, controlled. `None` is an empty field.
    pub fn value(mut self, value: Option<f64>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, value: f64) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(Option<f64>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// What the field prints while it is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// A trailing unit inside the box — `px`, `%`, `ms`.
    pub fn unit(mut self, unit: impl IntoElement) -> Self {
        self.unit = Some(unit.into_any_element());
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

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Accepted for React's shape; GPUI 0.2 has no accessibility tree.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

/// A number as the field prints it: no trailing zeros, no exponent.
fn format(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        let text = format!("{value:.6}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[derive(Clone)]
struct Rules {
    min: Option<f64>,
    max: Option<f64>,
    step: f64,
}

impl Rules {
    fn clamp(&self, value: f64) -> f64 {
        let value = self.min.map_or(value, |min| value.max(min));
        self.max.map_or(value, |max| value.min(max))
    }

    /// One step from `value`, snapped to the step grid from `min`, as Base
    /// UI steps. An empty field steps from `min`, or from zero.
    fn stepped(&self, value: Option<f64>, direction: f64) -> f64 {
        let base = self.min.unwrap_or(0.0);
        let Some(value) = value else {
            return self.clamp(base);
        };
        let offset = (value - base) / self.step;
        let snapped = if direction > 0.0 {
            (offset + 1e-9).floor() + 1.0
        } else {
            (offset - 1e-9).ceil() - 1.0
        };
        self.clamp(base + snapped * self.step)
    }
}

impl RenderOnce for NumberField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let input = input_state(self.id.clone(), window, cx);
        let focused = input.read(cx).is_focused(window);
        let held = window
            .use_keyed_state(part_id(&self.id, "held"), cx, |_, _| {
                Rc::new(RefCell::new(Held::default()))
            })
            .read(cx)
            .clone();
        {
            let mut held = held.borrow_mut();
            if !held.initialized {
                held.value = self.default_value;
                held.initialized = true;
            }
            if let Some(value) = self.value {
                held.value = value;
            }
            // A draft is only kept while it is being typed.
            if !focused {
                held.draft = None;
            }
        }
        let rules = Rules {
            min: self.min,
            max: self.max,
            step: self.step,
        };
        let current = held.borrow().value;
        let text = held
            .borrow()
            .draft
            .clone()
            .unwrap_or_else(|| current.map(format).unwrap_or_default());
        let controlled = self.value.is_some();
        let on_change = self.on_value_change.clone();
        let editable = !self.disabled && !self.read_only;

        // Setting the number: held here unless a parent holds it, reported
        // either way, and the draft dropped.
        let set: Rc<dyn Fn(Option<f64>, &mut Window, &mut App)> = {
            let held = held.clone();
            let on_change = on_change.clone();
            Rc::new(move |value, window, cx| {
                let previous = {
                    let mut held = held.borrow_mut();
                    held.draft = None;
                    let previous = held.value;
                    if !controlled {
                        held.value = value;
                    }
                    previous
                };
                if previous != value
                    && let Some(on_change) = &on_change
                {
                    on_change(value, window, cx);
                }
                window.refresh();
            })
        };
        let commit = {
            let held = held.clone();
            let set = set.clone();
            let rules = rules.clone();
            move |window: &mut Window, cx: &mut App| {
                let draft = held.borrow().draft.clone();
                let Some(draft) = draft else { return };
                let trimmed = draft.trim();
                let value = if trimmed.is_empty() {
                    None
                } else {
                    match trimmed.parse::<f64>() {
                        Ok(value) => Some(rules.clamp(value)),
                        // What does not parse goes back to what it was.
                        Err(_) => held.borrow().value,
                    }
                };
                set(value, window, cx);
            }
        };
        let step_by = {
            let held = held.clone();
            let set = set.clone();
            let rules = rules.clone();
            let commit = commit.clone();
            move |direction: f64, window: &mut Window, cx: &mut App| {
                commit(window, cx);
                let value = held.borrow().value;
                set(Some(rules.stepped(value, direction)), window, cx);
            }
        };

        let can_increment = editable
            && match (current, self.max) {
                (Some(value), Some(max)) => value < max,
                _ => true,
            };
        let can_decrement = editable
            && match (current, self.min) {
                (Some(value), Some(min)) => value > min,
                _ => true,
            };

        let text_input = TextInput::new(self.id.clone())
            .state(input.clone())
            .value(text)
            .placeholder(self.placeholder)
            .placeholder_color(placeholder_color(t))
            .selection_color(selection_color(t))
            .disabled(self.disabled)
            .read_only(self.read_only)
            .on_change({
                let held = held.clone();
                move |text, _, _| held.borrow_mut().draft = Some(text.to_string())
            })
            .on_submit({
                let commit = commit.clone();
                move |_, window, cx| commit(window, cx)
            })
            .on_blur({
                let commit = commit.clone();
                move |window, cx| commit(window, cx)
            });

        let tint = self.state.tint(self.tint);
        let step_button = |id: &'static str, glyph: &'static str, enabled: bool, direction: f64| {
            let step_by = step_by.clone();
            let disabled = ControlColors::resolve(t, Variant::Normal, tint, State::Disabled);
            let hovered = ControlColors::resolve(t, Variant::Normal, tint, State::Hovered);
            // Two stacked actions share the field's height, so the glyph
            // must fit its half even in the most compact theme.
            let glyph_size = t.icon_small.min(t.spacing_3);
            div()
                .id(part_id(&self.id, id))
                .flex()
                .items_center()
                .justify_center()
                .w(px(t.spacing_4))
                .h(px(t.spacing_3))
                .rounded(px(t.radius_tiny))
                .map(|this| {
                    if enabled {
                        this.text_color(t.color_content_subtle.hsla())
                            .cursor_pointer()
                            .hover(move |style| {
                                style.bg(hovered.surface).text_color(t.color_content.hsla())
                            })
                            .on_mouse_down(MouseButton::Left, |_, window, cx| {
                                pointer_down(window, cx);
                                cx.stop_propagation();
                            })
                            .on_click(move |_, window, cx| step_by(direction, window, cx))
                    } else {
                        this.text_color(disabled.content).cursor_not_allowed()
                    }
                })
                .child(Icon::new(glyph).size(glyph_size))
        };

        // The stepper: two half-height buttons in one column, hairlined off
        // the value. Stacked rather than a minus and a plus on either side —
        // a value with a control on both sides reads as three things.
        let stepper = div()
            .flex()
            .flex_col()
            .flex_none()
            .map(|mut this| {
                this.style().align_self = Some(gpui::AlignSelf::Stretch);
                this
            })
            .justify_center()
            .my(px(t.spacing_1))
            .border_l(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .pl(px(t.spacing_1))
            .child(step_button(
                "increment",
                glyphs::CHEVRON_UP_12_REGULAR,
                can_increment,
                1.0,
            ))
            .child(step_button(
                "decrement",
                glyphs::CHEVRON_DOWN_12_REGULAR,
                can_decrement,
                -1.0,
            ));

        let label = t.label_quiet;
        let unit = self.unit.map(|unit| {
            div()
                .flex_none()
                .text_color(t.color_content_faint.hsla())
                .font_family(label.face.family())
                .text_size(px(t.label_small.size))
                .line_height(px(t.label_small.size))
                .child(unit)
        });

        let chrome = FieldBox::new(self.size)
            .tint(self.tint)
            .state(self.state)
            .focused(focused)
            .disabled(self.disabled);
        let up = step_by.clone();
        let down = step_by;
        chrome
            .style(div(), t)
            .gap(px(t.spacing_1))
            // The trailing inset is the stepper's own: it is a control rather
            // than text, so it sits closer to the edge than a value would.
            .pr(px(t.spacing_1))
            .map(|this| {
                if self.disabled {
                    this.cursor_not_allowed()
                } else {
                    this.cursor_text()
                }
            })
            .when(!self.disabled, |this| {
                let input = input.downgrade();
                this.on_mouse_down(MouseButton::Left, move |event, window, cx| {
                    input
                        .update(cx, |input, cx| input.mouse_down(event, window, cx))
                        .ok();
                })
            })
            .when(editable, |this| {
                this.capture_action(move |_: &text_input::Up, window, cx| {
                    cx.stop_propagation();
                    up(1.0, window, cx);
                })
                .capture_action(move |_: &text_input::Down, window, cx| {
                    cx.stop_propagation();
                    down(-1.0, window, cx);
                })
            })
            .child(text_input)
            .children(unit)
            .child(stepper)
    }
}
