use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
};

use crate::prelude::*;

/// What a pane shows before it has content: one muted line and at most one
/// action.
///
/// The surrounding chrome already names the pane, so there is no heading, no
/// explainer copy, and no illustration.
#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    /// The way out of the empty state — usually one button.
    actions: Vec<AnyElement>,
}

impl EmptyState {
    pub fn new(title: impl Into<SharedString>) -> Self {
        EmptyState {
            title: title.into(),
            actions: Vec::new(),
        }
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .flex_1()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(vars.spacing_3)
            .py(vars.spacing_14)
            .px(vars.spacing_8)
            .child(
                div()
                    .type_style(vars.body_medium)
                    .text_color(vars.color_content_subtle)
                    .child(self.title),
            )
            .when(!self.actions.is_empty(), |el| {
                // A row of controls, so it takes the control gap rather than
                // the block gap.
                el.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(vars.control_medium_gap)
                        .children(self.actions),
                )
            })
    }
}
