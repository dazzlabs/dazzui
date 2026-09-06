//! `Components/EmptyState` — what stands in for content that is not there.

use dazzui::{ActionBar, Button, Card, CardVariant, ControlVariant, EmptyState, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Frame, Pane, Row};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "EmptyState",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("EmptyState")
                .description(
                    "A line saying what is missing, and at most the one action that would fix \
                     it. It is the pane's content rather than a notice about the pane, so it \
                     centres in the space it was given instead of hanging from the top of it.",
                )
                .child(
                    Section::new("Shapes")
                        .description(
                            "The title alone is enough where nothing can be done — a filter \
                             that matched nothing is fixed by changing the filter, not by a \
                             button. Where there is something to do, the action slot takes it; \
                             two actions go in an `ActionBar` so the gap stays on the control \
                             scale.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(EmptyState::new("Nothing matched “tokens”"))
                                .child(with_an_action("overview"))
                                .child(with_two_actions("overview")),
                        ),
                )
                .child(
                    Section::new("Where it lands")
                        .description(
                            "A well, when the empty thing is a section of a page; the pane \
                             itself, when the empty thing is the whole view.",
                        )
                        .child(
                            Frame::new(px(620.), px(260.))
                                .child(Pane::new().child(with_an_action("pane"))),
                        ),
                )
                .into_any_element()
        }),
        Story::new("On its own", |_, _, _| {
            EmptyState::new("No components match this filter").into_any_element()
        }),
        Story::new("With an action", |_, _, _| {
            with_an_action("story").into_any_element()
        }),
        Story::new("With two actions", |_, _, _| {
            with_two_actions("story").into_any_element()
        }),
        Story::new("Nothing matched", |_, _, _| {
            EmptyState::new("Nothing matched “tokens”").into_any_element()
        }),
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

fn with_two_actions(id: &'static str) -> impl IntoElement {
    EmptyState::new("This project has no glossary yet").action(
        ActionBar::new()
            .child(Button::new(
                gpui::SharedString::from(format!("{id}-import")),
                "Import",
            ))
            .child(
                Button::new(
                    gpui::SharedString::from(format!("{id}-create")),
                    "Create one",
                )
                .variant(ControlVariant::Filled)
                .tint(Tint::Primary),
            ),
    )
}
