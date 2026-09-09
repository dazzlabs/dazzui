use gpui::{
    App, ElementId, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::TypeStyle;
use crate::assets::icons;
use crate::components::Icon;
use crate::components::text_field::{
    TextBuffer, TextFieldSize, TextFieldState, editable, field_box,
};
use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, Tint, ValueListener, colors, hairline, pressable,
};

/// A number, with the two things a plain text control never gets right: a
/// stepper you can hold down, and a value that stays a number.
///
/// The box is the [`TextField`](crate::TextField)'s — [`field_box`] draws it
/// for both — so a form mixing the two does not step. What is this
/// component's own is the stepper at the trailing edge, and the clamp that
/// keeps the value inside its bounds.
#[derive(IntoElement)]
pub struct NumberField {
    id: ElementId,
    field: Entity<TextBuffer>,
    min: Option<f32>,
    max: Option<f32>,
    /// What one press of the stepper moves.
    step: f32,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    /// A trailing unit inside the box — `px`, `%`, `ms`. It is a hint rather
    /// than part of the value, so it never enters what the field holds.
    unit: Option<SharedString>,
    disabled: bool,
    on_change: Option<ValueListener<f32>>,
}

impl NumberField {
    pub fn new(id: impl Into<ElementId>, field: &Entity<TextBuffer>) -> Self {
        NumberField {
            id: id.into(),
            field: field.clone(),
            min: None,
            max: None,
            step: 1.,
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            unit: None,
            disabled: false,
            on_change: None,
        }
    }

    pub fn bounds(mut self, min: f32, max: f32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn state(mut self, state: TextFieldState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn unit(mut self, unit: impl Into<SharedString>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, listener: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for NumberField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let NumberField {
            id,
            field,
            min,
            max,
            step,
            size,
            state,
            tint,
            unit,
            disabled,
            on_change,
        } = self;

        let metrics = vars.metrics(size.control());
        let focused = field.read(cx).is_focused(window);
        let tinted = state.tint(tint);
        let accent = vars.ramp(tinted).shade(600);
        let text = if disabled {
            vars.control_color(
                ControlVariant::Normal,
                ControlPart::Content,
                tinted,
                ControlState::Disabled,
            )
        } else if state == TextFieldState::Error {
            vars.ramp(tinted).shade(800)
        } else {
            vars.color_content
        };

        let current: f32 = field.read(cx).value().parse().unwrap_or(0.);
        let clamp = move |value: f32| {
            let value = min.map_or(value, |min| value.max(min));
            max.map_or(value, |max| value.min(max))
        };
        let on_change: Option<crate::SharedValueListener<f32>> = on_change.map(std::rc::Rc::from);
        let nudge = {
            let on_change = on_change.clone();
            move |delta: f32, window: &mut Window, cx: &mut App| {
                if let Some(listener) = on_change.as_ref() {
                    listener(&clamp(current + delta), window, cx);
                }
            }
        };

        let can_increase = !disabled && max.is_none_or(|max| current < max);
        let can_decrease = !disabled && min.is_none_or(|min| current > min);

        field_box(size, state, tinted, disabled, focused, false, cx)
            .id(id)
            // The trailing step is the stepper's own inset: it is a control
            // rather than text, so it sits closer to the edge than a value.
            .pr(vars.spacing_1)
            .gap(vars.spacing_1)
            .on_mouse_down(MouseButton::Left, {
                let field = field.clone();
                move |_, window, cx| field.read(cx).focus(window)
            })
            .child(editable(
                &field,
                TypeStyle {
                    weight: vars.body_small.weight,
                    ..metrics.content
                },
                metrics.content.line_height,
                text,
                vars.color_content_faint,
                accent,
                accent.opacity(vars.focus_glow_alpha),
                false,
                cx,
            ))
            .when_some(unit, |el, unit| {
                el.child(
                    div()
                        .flex_none()
                        .type_style(vars.label_quiet)
                        .text_size(vars.label_small.size)
                        .line_height(relative(1.0))
                        .text_color(vars.color_content_faint)
                        .child(unit),
                )
            })
            // Stacked rather than a minus and a plus on either side: the field
            // is read left to right, and a value with a control on both sides
            // of it reads as three things. The pair is one AppKit stepper.
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .justify_center()
                    .h_full()
                    .my(vars.spacing_1)
                    .pl(vars.spacing_1)
                    .border_l(hairline(vars))
                    .border_color(vars.color_border)
                    .child(step_button(
                        "increase",
                        icons::CHEVRON_UP_12_REGULAR,
                        can_increase,
                        {
                            let nudge = nudge.clone();
                            move |window, cx| nudge(step, window, cx)
                        },
                        vars,
                    ))
                    .child(step_button(
                        "decrease",
                        icons::CHEVRON_DOWN_12_REGULAR,
                        can_decrease,
                        move |window, cx| nudge(-step, window, cx),
                        vars,
                    )),
            )
    }
}

/// One half of the stepper.
fn step_button(
    id: &'static str,
    icon: &'static str,
    enabled: bool,
    on_press: impl Fn(&mut Window, &mut App) + 'static,
    vars: &'static crate::ThemeVariables,
) -> impl IntoElement {
    let on_press = std::rc::Rc::new(on_press);

    pressable(id, enabled, move |state, _, _| {
        let washed = enabled && (state.hovered || state.pressed);

        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .w(vars.spacing_4)
            .h(vars.spacing_3)
            .rounded(vars.radius_tiny)
            .bg(if washed {
                vars.control_color(
                    ControlVariant::Normal,
                    ControlPart::Surface,
                    Tint::Neutral,
                    ControlState::Hovered,
                )
            } else {
                colors::TRANSPARENT
            })
            .child(
                Icon::new(icon)
                    .size(vars.caption_small.size)
                    .color(if !enabled {
                        vars.control_color(
                            ControlVariant::Normal,
                            ControlPart::Content,
                            Tint::Neutral,
                            ControlState::Disabled,
                        )
                    } else if washed {
                        vars.color_content
                    } else {
                        vars.color_content_subtle
                    }),
            )
            .when(enabled, |el| {
                let on_press = on_press.clone();
                el.on_click(move |_, window, cx| on_press(window, cx))
            })
            .into_any_element()
    })
}
