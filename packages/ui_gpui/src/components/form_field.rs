use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    relative,
};

use crate::Tint;
use crate::components::InputState;
use crate::prelude::*;

/// Label, control, hint.
///
/// The wrapper draws no box of its own — the control it holds does that — so
/// it owns only the space between the three and the type of the two it prints
/// itself.
///
/// It is `FormField` rather than `Field` because in this library the `*Field`
/// suffix already belongs to controls — `SearchField` is one — and a wrapper
/// sharing that suffix reads like their base rather than their container.
#[derive(IntoElement)]
pub struct FormField {
    label: SharedString,
    /// Help text under the control; carries the error tone when erroring.
    hint: Option<SharedString>,
    state: InputState,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl FormField {
    pub fn new(label: impl Into<SharedString>) -> Self {
        FormField {
            label: label.into(),
            hint: None,
            state: InputState::Default,
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
}

impl ParentElement for FormField {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FormField {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        // Erroring, the label and the hint join the control on the danger
        // ramp: the wrapper is re-tinted rather than given error colours.
        let tint = self.state.tint(self.tint);
        let erroring = self.state == InputState::Error;
        let color = if erroring {
            vars.ramp(tint).shade(700)
        } else {
            vars.color_content_subtle
        };

        div()
            .flex()
            .flex_col()
            .gap(vars.spacing_1_5)
            .child(
                div()
                    .type_style(vars.label_small)
                    .text_color(color)
                    .child(self.label),
            )
            .children(self.children)
            .when_some(self.hint, |el, hint| {
                el.child(
                    div()
                        .type_style(vars.label_quiet)
                        .line_height(relative(1.6))
                        .text_color(color)
                        .child(hint),
                )
            })
    }
}
