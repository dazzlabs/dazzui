//! `Components/EmptyState` — what stands in for content that is not there.

use dazzui::{Button, Card, CardVariant, ControlVariant, EmptyState};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::stage::{Frame, Pane};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "EmptyState",
    &[
        Story::new("In a well", |_, _, _| {
            div()
                .w(px(460.))
                .child(
                    Card::new()
                        .variant(CardVariant::Sunken)
                        .child(with_an_action("well")),
                )
                .into_any_element()
        }),
        Story::new("In a pane", |_, _, _| {
            Frame::new(px(560.), px(300.))
                .child(Pane::new().child(with_an_action("pane")))
                .into_any_element()
        }),
    ],
);

fn with_an_action(id: &'static str) -> impl IntoElement {
    EmptyState::new("No components match this filter").action(
        Button::new(
            gpui::SharedString::from(format!("{id}-clear")),
            "Clear the filter",
        )
        .variant(ControlVariant::Tinted),
    )
}
