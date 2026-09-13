//! `Components/Progress` — the bar, determinate and not.

use dazzui::{Progress, ProgressSize, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, case};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const SIZES: [ProgressSize; 3] = [
    ProgressSize::Small,
    ProgressSize::Medium,
    ProgressSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "Progress",
    &[
        Story::page("Overview", |_, _, _| {
            let mut values = Col::new().width(px(420.));
            for value in [0., 25., 50., 75., 100.] {
                values = values.child(Progress::new(value));
            }

            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(case(size))
                        .child(div().w(px(160.)).child(Progress::new(60.).size(size)))
                        .child(
                            div()
                                .w(px(160.))
                                .child(Progress::new(60.).size(size).tint(Tint::Success)),
                        )
                        .child(
                            div()
                                .w(px(160.))
                                .child(Progress::new(60.).size(size).gradient(true)),
                        ),
                );
            }

            DocPage::new("Progress")
                .description(
                    "A bar. With a value it fills; without one it sweeps, which is the honest \
                     drawing for work whose end is not known. The track is a token rather than \
                     the tint washed down, so a bar reads the same on every surface.",
                )
                .child(
                    Section::new("Values")
                        .description("Zero draws the track alone; a hundred fills it.")
                        .child(values),
                )
                .child(
                    Section::new("Sizes and tints")
                        .description(
                            "Three heights on the same corner. The tint is the fill's ramp; the \
                             gradient is the two-stop version of it, for the one bar a view \
                             points at.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Indeterminate")
                        .description(
                            "No value at all is the sweep. It is the same bar: nothing about \
                             the box changes, only what is drawn inside it.",
                        )
                        .child(
                            div()
                                .w(px(420.))
                                .child(Progress::indeterminate().size(ProgressSize::Medium)),
                        ),
                )
                .into_any_element()
        }),
    ],
);
