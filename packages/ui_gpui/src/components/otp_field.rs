use gpui::{
    App, ElementId, Entity, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, relative,
};

use crate::components::text_field::{
    TextBuffer, TextFieldSize, TextFieldState, editable, field_box,
};
use crate::prelude::*;
use crate::{Tint, TypeStyle, colors};

/// The one-time code a reader is copying out of a message.
///
/// It is drawn as a run of cells but it is one field underneath, and that is
/// deliberate: a paste of the whole code fills every cell at once, and the
/// caret only ever has one place to be. A run of separate inputs gets both
/// wrong and has to reinvent the moves between them.
///
/// Each cell is the text controls' box at one character wide, so a code sits
/// on the same rhythm as the fields around it — a code entered in a box of its
/// own design would be the only control on the page that was.
#[derive(IntoElement)]
pub struct OtpField {
    id: ElementId,
    field: Entity<TextBuffer>,
    length: usize,
    /// Breaks the run where a printed code breaks: `3` on a six-digit code
    /// makes it two groups of three, which is how it is read aloud.
    group_after: Option<usize>,
    /// For a code that is a secret rather than a one-time token — a PIN.
    mask: bool,
    size: TextFieldSize,
    state: TextFieldState,
    tint: Tint,
    disabled: bool,
}

impl OtpField {
    pub fn new(id: impl Into<ElementId>, field: &Entity<TextBuffer>, length: usize) -> Self {
        OtpField {
            id: id.into(),
            field: field.clone(),
            length,
            group_after: None,
            mask: false,
            size: TextFieldSize::Medium,
            state: TextFieldState::Default,
            tint: Tint::Primary,
            disabled: false,
        }
    }

    pub fn group_after(mut self, group_after: usize) -> Self {
        self.group_after = Some(group_after);
        self
    }

    pub fn mask(mut self, mask: bool) -> Self {
        self.mask = mask;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for OtpField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size.control());
        let focused = self.field.read(cx).is_focused(window);
        let value: Vec<char> = self.field.read(cx).value().chars().collect();
        let caret = value.len().min(self.length.saturating_sub(1));
        let tinted = self.state.tint(self.tint);

        let cells = (0..self.length).map(|index| {
            let filled = index < value.len();
            let here = focused && !self.disabled && index == caret;

            div()
                .flex()
                .flex_none()
                // The break between two groups of digits is a step of air
                // rather than a rule, which is what a printed code does.
                .when(
                    self.group_after
                        .is_some_and(|after| index > 0 && index % after == 0),
                    |el| el.ml(vars.spacing_3 - vars.spacing_2),
                )
                .child(
                    // A filled cell lifts to the paper and keeps it, so the
                    // run says how far through the code the reader is without
                    // the caret having to be the only thing that does.
                    field_box(
                        self.size,
                        self.state,
                        tinted,
                        self.disabled,
                        here || filled,
                        false,
                        cx,
                    )
                    .w(metrics.size)
                    .px(gpui::px(0.))
                    .justify_center()
                    // The mono face and tabular figures: a code is digits
                    // being compared, and a 1 narrower than a 7 makes the run
                    // look mis-typed. It is the one place a component reaches
                    // a `base.*` primitive, which `otp-field.css` does too.
                    .type_style(TypeStyle {
                        family: &["ui-monospace"],
                        weight: vars.body_small.weight,
                        ..metrics.content
                    })
                    .child(if !filled {
                        SharedString::from("")
                    } else if self.mask {
                        SharedString::from("•")
                    } else {
                        SharedString::from(value[index].to_string())
                    }),
                )
        });

        div()
            .id(self.id)
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .gap(vars.spacing_2)
            .on_mouse_down(MouseButton::Left, {
                let field = self.field.clone();
                move |_, window, cx| field.read(cx).focus(window)
            })
            .children(cells)
            // One field under the run, drawn in nothing: it holds the value
            // and takes the paste, and is what the press focuses. The cells
            // above are a picture of it.
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .line_height(relative(1.0))
                    .child(editable(
                        &self.field,
                        metrics.content,
                        metrics.content.line_height,
                        colors::TRANSPARENT,
                        colors::TRANSPARENT,
                        colors::TRANSPARENT,
                        colors::TRANSPARENT,
                        false,
                        cx,
                    )),
            )
    }
}
