//! `Components/Progress` — the bar, determinate and not.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Button, Card, Progress, ProgressSize, Step, StepList, StepStatus, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act};

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
        Story::new("Default", |_, _, _| {
            div()
                .w(px(320.))
                .child(Progress::new(35.))
                .into_any_element()
        }),
        Story::new("Values", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for value in [0., 25., 50., 75., 100.] {
                col = col.child(Progress::new(value));
            }
            col.into_any_element()
        }),
        Story::new("Indeterminate", |_, _, _| {
            div()
                .w(px(320.))
                .child(Progress::indeterminate())
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for size in SIZES {
                col = col.child(Progress::new(60.).size(size));
            }
            col.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for tint in Tint::ALL {
                col = col.child(Progress::new(60.).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("Gradient", |_, _, _| {
            Col::new()
                .width(px(320.))
                .child(Progress::new(60.))
                .child(Progress::new(60.).gradient(true))
                .into_any_element()
        }),
        Story::new("Live", |state, this, _| {
            live(state, this).into_any_element()
        }),
        Story::new("With a count", |_, _, cx| counted(cx).into_any_element()),
        Story::new("Beside stages", |_, _, cx| {
            div()
                .w(px(360.))
                .child(
                    Card::new().child(counted(cx)).child(
                        StepList::new()
                            .child(Step::new("Resolve the token graph", StepStatus::Done))
                            .child(Step::new("Write the platform sources", StepStatus::Active))
                            .child(Step::new("Publish the package", StepStatus::Idle)),
                    ),
                )
                .into_any_element()
        }),
    ],
);

fn live(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    Col::new()
        .width(px(320.))
        .child(
            Progress::new(state.progress)
                .size(ProgressSize::Medium)
                .gradient(true),
        )
        .child(
            Row::new().child(Button::new("advance", "Advance").on_click({
                let advance = act(this, |state, _, _| {
                    state.progress = if state.progress >= 100. {
                        0.
                    } else {
                        (state.progress + 20.).min(100.)
                    };
                });
                move |_, window, cx| advance(window, cx)
            })),
        )
}

fn counted(cx: &mut gpui::App) -> impl IntoElement {
    let vars = cx.theme().vars;

    div()
        .w(px(320.))
        .flex()
        .flex_col()
        .gap(vars.spacing_2)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .type_style(vars.label_medium)
                        .text_color(vars.color_content)
                        .child("Rebuilding tokens"),
                )
                .child(
                    div()
                        .type_style(vars.caption_small)
                        .text_color(vars.color_content_muted)
                        .child("3 of 4"),
                ),
        )
        .child(Progress::new(75.))
}
