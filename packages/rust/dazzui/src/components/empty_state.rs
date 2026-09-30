//! What a pane shows before it has content.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use crate::foundation::StyledExt;
use crate::theme::ActiveTheme;

/// One muted line and at most one action. The surrounding chrome already
/// names the pane, so there is no heading, no explainer and no picture.
#[derive(IntoElement)]
pub struct EmptyState {
    title: AnyElement,
    action: Option<AnyElement>,
}

impl EmptyState {
    pub fn new(title: impl IntoElement) -> Self {
        EmptyState {
            title: title.into_any_element(),
            action: None,
        }
    }

    /// The way out of the empty state — usually one button.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .items_center()
            .justify_center()
            .gap(px(t.spacing_3))
            .py(px(t.spacing_14))
            .px(px(t.spacing_8))
            .child(
                div()
                    .text_center()
                    .type_style(t.body_medium)
                    .text_color(t.color_content_subtle.hsla())
                    .child(self.title),
            )
            .when_some(self.action, |this, action| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(t.control_medium_gap))
                        .child(action),
                )
            })
    }
}
