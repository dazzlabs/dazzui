//! `Components/Splitter` — regions the reader can re-proportion.

use dazzui::{SectionLabel, Splitter, SplitterOrientation, SplitterPanel};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

/// A region with a name in it, so a seam can be seen to move something.
fn region(label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .p(px(12.))
        .child(SectionLabel::new(label))
}

/// The window the specimens sit in — a bordered box, since a splitter paints
/// nothing of its own and would otherwise be seams in mid-air.
fn window(width: f32, height: f32, child: impl IntoElement) -> impl IntoElement {
    div()
        .w(px(width))
        .h(px(height))
        .overflow_hidden()
        .rounded(px(8.))
        .child(child)
}

fn pair(id: &'static str) -> Splitter {
    Splitter::new(
        id,
        vec![
            SplitterPanel::new(region("Before")).default_size(50.),
            SplitterPanel::new(region("After")).default_size(50.),
        ],
    )
}

fn stack(id: &'static str) -> Splitter {
    Splitter::new(
        id,
        vec![
            SplitterPanel::new(region("Editor"))
                .default_size(40.)
                .min_size(20.),
            SplitterPanel::new(region("Output"))
                .default_size(60.)
                .min_size(20.),
        ],
    )
    .orientation(SplitterOrientation::Vertical)
}

fn three(id: &'static str) -> Splitter {
    Splitter::new(
        id,
        vec![
            SplitterPanel::new(region("Files"))
                .default_size(24.)
                .min_size(15.)
                .max_size(40.),
            SplitterPanel::new(region("Document"))
                .default_size(52.)
                .min_size(30.),
            SplitterPanel::new(region("Inspector"))
                .default_size(24.)
                .min_size(15.)
                .max_size(40.),
        ],
    )
}

pub const PAGE: StoryPage = StoryPage::new(
    "Splitter",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Splitter")
                .description(
                    "Two or more regions the reader can re-proportion. A desktop window is \
                     mostly this — a column beside a document beside an inspector — and which \
                     of the three deserves the room is the reader's decision rather than the \
                     designer's.",
                )
                .child(
                    Section::new("Beside a Sidebar")
                        .description(
                            "Where a column has one right width and the reader only opens or \
                             shuts it, that is Sidebar instead: it carries its own width token \
                             and its own rail, and drawing it with a splitter would offer a \
                             choice the design has already made.",
                        )
                        .child(window(608., 208., three("doc-three"))),
                )
                .child(
                    Section::new("The seam")
                        .description(
                            "The handle is drawn from the space between two panels rather than \
                             added to it: the hit target is a whole step of the scale, so \
                             grabbing a seam does not demand pixel accuracy, while the rule \
                             itself stays the hairline every other separator in the system is. \
                             Under the pointer it takes the accent and swells — without \
                             reflowing, since a seam that pushed its neighbours aside on the \
                             way to a drag would be a layout jump.",
                        )
                        .child(window(544., 160., pair("doc-pair"))),
                )
                .child(
                    Section::new("Where the shares live")
                        .description(
                            "In GPUI's element state, through the same latch a menu keeps its \
                             open flag in. A host holds nothing: a drag that had to round-trip \
                             through someone else's state would lag the pointer, and a seam \
                             that lags the pointer feels broken.",
                        )
                        .child(window(480., 176., stack("doc-stack"))),
                )
                .into_any_element()
        }),
        Story::new("Horizontal", |_, _, _| {
            window(544., 192., pair("pair")).into_any_element()
        }),
        Story::new("Vertical", |_, _, _| {
            window(480., 224., stack("stack")).into_any_element()
        }),
        Story::new("Three panels", |_, _, _| {
            window(608., 224., three("three")).into_any_element()
        }),
        Story::new("Constrained", |_, _, _| {
            window(
                544.,
                192.,
                Splitter::new(
                    "constrained",
                    vec![
                        SplitterPanel::new(region("25–45%"))
                            .default_size(30.)
                            .min_size(25.)
                            .max_size(45.),
                        SplitterPanel::new(region("the rest")).default_size(70.),
                    ],
                ),
            )
            .into_any_element()
        }),
        Story::new("Nested", |_, _, _| {
            window(
                608.,
                256.,
                Splitter::new(
                    "nested",
                    vec![
                        SplitterPanel::new(region("Files"))
                            .default_size(30.)
                            .min_size(20.),
                        SplitterPanel::new(
                            Splitter::new(
                                "nested-inner",
                                vec![
                                    SplitterPanel::new(region("Editor")).default_size(65.),
                                    SplitterPanel::new(region("Terminal")).default_size(35.),
                                ],
                            )
                            .orientation(SplitterOrientation::Vertical),
                        )
                        .default_size(70.),
                    ],
                ),
            )
            .into_any_element()
        }),
    ],
);
