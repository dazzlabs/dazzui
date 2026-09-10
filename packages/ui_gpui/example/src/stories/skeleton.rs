//! `Components/Skeleton` — the shape of something that has not arrived.

use dazzui::{Card, Skeleton, SkeletonShape, SkeletonText};
use gpui::{IntoElement, ParentElement, Styled, div, px, relative};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

/// An avatar and two lines: the shape a list row will land in.
fn placeholder() -> impl IntoElement {
    div()
        .flex()
        .gap(px(12.))
        .w(px(320.))
        .child(Skeleton::new().shape(SkeletonShape::Circle).width(px(40.)))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .gap(px(8.))
                .child(div().w(relative(0.7)).child(Skeleton::new()))
                .child(div().w(relative(0.45)).child(Skeleton::new())),
        )
}

fn in_a_card() -> impl IntoElement {
    div().w(px(352.)).child(
        Card::new().child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(placeholder())
                .child(SkeletonText::new().lines(2)),
        ),
    )
}

pub const PAGE: StoryPage =
    StoryPage::new(
        "Skeleton",
        &[
            Story::page("Overview", |_, _, _| {
                DocPage::new("Skeleton")
                .description(
                    "The shape of something that has not arrived. A Spinner says the app is \
                     busy; a skeleton says what is coming, and where it will be — which is why \
                     it belongs on the first load of a view whose layout is already known, and \
                     a spinner belongs on an action whose result is not. Drawing one for \
                     content that then lands somewhere else is worse than drawing nothing.",
                )
                .child(
                    Section::new("Shapes")
                        .description(
                            "text is a bar on the line box of the type it stands in for, so it \
                             needs no height. circle is an avatar or a glyph and squares itself \
                             from its width alone. rect is everything with a box of its own — a \
                             thumbnail, a chart, a card.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("text").child(
                                    div().w(px(80.)).child(Skeleton::new()),
                                ))
                                .child(Specimen::new("circle").child(
                                    Skeleton::new().shape(SkeletonShape::Circle).width(px(40.)),
                                ))
                                .child(Specimen::new("rect").child(
                                    Skeleton::new()
                                        .shape(SkeletonShape::Rect)
                                        .width(px(80.))
                                        .height(px(48.)),
                                )),
                        ),
                )
                .child(
                    Section::new("Lines")
                        .description(
                            "A paragraph's worth of bars, with the last one short. The ragged \
                             edge is most of what makes a block of bars legible as text at all \
                             — a full-width final line reads as a table.",
                        )
                        .child(div().w(px(320.)).child(SkeletonText::new())),
                )
                .child(
                    Section::new("Materials")
                        .description(
                            "The fill is surface-sunken — the same dead step a disabled control \
                             flattens to, which is exactly what a skeleton is. React runs a \
                             wash of surface-subtle across it; this port draws the groove \
                             alone, because a repeating gradient is a shader and every other \
                             moving part here is a shape GPUI can turn.",
                        )
                        .child(in_a_card()),
                )
                .into_any_element()
            }),
            Story::new("Shapes", |_, _, _| {
                div()
                    .flex()
                    .gap(px(12.))
                    .child(Skeleton::new().shape(SkeletonShape::Circle).width(px(40.)))
                    .child(
                        Skeleton::new()
                            .shape(SkeletonShape::Rect)
                            .width(px(80.))
                            .height(px(48.)),
                    )
                    .into_any_element()
            }),
            Story::new("Text", |_, _, _| {
                div()
                    .w(px(320.))
                    .child(SkeletonText::new())
                    .into_any_element()
            }),
            Story::new("Lines", |_, _, _| {
                div()
                    .w(px(320.))
                    .child(SkeletonText::new().lines(5))
                    .into_any_element()
            }),
            Story::new("A list row", |_, _, _| placeholder().into_any_element()),
            Story::new("In a card", |_, _, _| in_a_card().into_any_element()),
        ],
    );
