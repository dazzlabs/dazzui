//! The label, the hint, and whatever control sits between them.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px, relative,
};

use crate::components::text_field::FieldState;
use crate::foundation::{StyledExt, Tint, provide};
use crate::theme::ActiveTheme;

/// What a `FormField` tells the control inside it: whether it is marked
/// invalid, as Base UI's `Field` marks its control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormFieldScope {
    pub invalid: bool,
}

/// A label over a control, and a hint under it. The wrapper draws no box —
/// the control does — so it owns only the space between the three and the
/// type of the two it prints.
///
/// It is `FormField` rather than `Field` because the `*Field` suffix belongs
/// to controls here, as in React.
#[derive(IntoElement)]
pub struct FormField {
    label: AnyElement,
    hint: Option<AnyElement>,
    state: FieldState,
    tint: Tint,
    children: Vec<AnyElement>,
}

impl FormField {
    pub fn new(label: impl IntoElement) -> Self {
        FormField {
            label: crate::foundation::text_child(label),
            hint: None,
            state: FieldState::Default,
            tint: Tint::Primary,
            children: Vec::new(),
        }
    }

    /// Help text under the control; it carries the error tone when erroring.
    pub fn hint(mut self, hint: impl IntoElement) -> Self {
        self.hint = Some(crate::foundation::text_child(hint));
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
}

impl ParentElement for FormField {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for FormField {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let invalid = self.state == FieldState::Error;
        // Erroring, the label and the hint join the control on the ramp the
        // wrapper is re-tinted to.
        let ink = if invalid {
            self.state.tint(self.tint).ramp(t).shade(700).hsla()
        } else {
            t.color_content_subtle.hsla()
        };
        let quiet = t.label_quiet;

        let field = div()
            .flex()
            .flex_col()
            .gap(px(t.spacing_1_5))
            // `width: fit-content`: the label is as wide as its words, in a
            // row of its own.
            .child(
                div().flex().child(
                    div()
                        .text_color(ink)
                        .type_style(t.label_small)
                        .child(crate::foundation::text_child(self.label)),
                ),
            )
            .children(self.children)
            .children(self.hint.map(|hint| {
                div()
                    .text_color(ink)
                    .font_family(quiet.face.family())
                    .text_size(px(quiet.size))
                    .font_weight(quiet.font_weight())
                    .line_height(relative(t.form_field_hint_leading))
                    .child(hint)
            }));
        provide(FormFieldScope { invalid }, field)
    }
}
