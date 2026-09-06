//! `Components/StepList` — a job, reported.

use dazzui::{Button, Card, Step, StepList, StepStatus, Tint};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, capitalize, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act};

const STEPS: [&str; 4] = [
    "Resolve the token graph",
    "Write the platform sources",
    "Compile dazzui",
    "Publish the package",
];

pub const PAGE: StoryPage = StoryPage::new(
    "StepList",
    &[
        Story::page("Overview", |state, this, _| {
            let mut statuses = StepList::new();
            for status in [StepStatus::Done, StepStatus::Active, StepStatus::Idle] {
                statuses = statuses.child(Step::new(capitalize(&case(status)), status));
            }

            DocPage::new("StepList")
                .description(
                    "What a long job says while it runs: a line per stage, a mark saying where \
                     it got to, and an optional trailing detail. It reports rather than \
                     navigates — nothing in it is clickable.",
                )
                .child(
                    Section::new("Statuses")
                        .description(
                            "`done` is the tick; `active` is the stage in flight and the only \
                             one that carries the tint; `idle` is the stage not reached yet, in \
                             faint ink. Exactly one step should be active.",
                        )
                        .child(div().w(px(360.)).child(statuses)),
                )
                .child(
                    Section::new("Detail")
                        .description(
                            "The trailing slot is for the number the stage produced — a \
                             duration, a count, a hyphen where there is nothing yet. It is not a \
                             second label: a step whose detail explains what it does is a step \
                             whose label is wrong.",
                        )
                        .child(div().w(px(400.)).child(timed())),
                )
                .child(
                    Section::new("Advancing")
                        .description("The list as a job actually moves through it.")
                        .child(advancing(state, this)),
                )
                .into_any_element()
        }),
        Story::new("In flight", |_, _, _| {
            div()
                .w(px(320.))
                .child(
                    StepList::new()
                        .child(Step::new(STEPS[0], StepStatus::Done))
                        .child(Step::new(STEPS[1], StepStatus::Done))
                        .child(Step::new(STEPS[2], StepStatus::Active))
                        .child(Step::new(STEPS[3], StepStatus::Idle)),
                )
                .into_any_element()
        }),
        Story::new("Statuses", |_, _, _| {
            let mut list = StepList::new();
            for status in [StepStatus::Done, StepStatus::Active, StepStatus::Idle] {
                list = list.child(Step::new(capitalize(&case(status)), status));
            }
            div().w(px(320.)).child(list).into_any_element()
        }),
        Story::new("Finished", |_, _, _| {
            div()
                .w(px(320.))
                .child(
                    StepList::new()
                        .child(Step::new(STEPS[0], StepStatus::Done))
                        .child(Step::new(STEPS[1], StepStatus::Done))
                        .child(Step::new(STEPS[2], StepStatus::Done)),
                )
                .into_any_element()
        }),
        Story::new("Timed detail", |_, _, _| {
            div().w(px(400.)).child(timed()).into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    StepList::new()
                        .child(Step::new(capitalize(tint.name()), StepStatus::Active).tint(tint)),
                );
            }
            col.into_any_element()
        }),
        Story::new("Advancing", |state, this, _| {
            advancing(state, this).into_any_element()
        }),
        Story::new("In a card", |_, _, _| {
            div()
                .w(px(360.))
                .child(Card::new().child("Rebuilding tokens").child(timed()))
                .into_any_element()
        }),
    ],
);

fn timed() -> impl IntoElement {
    StepList::new()
        .child(Step::new(STEPS[0], StepStatus::Done).meta("0.4s"))
        .child(Step::new(STEPS[1], StepStatus::Done).meta("1.2s"))
        .child(Step::new(STEPS[2], StepStatus::Active).meta("running"))
        .child(Step::new(STEPS[3], StepStatus::Idle).meta("—"))
}

fn advancing(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    let mut list = StepList::new();
    for (index, label) in STEPS.iter().enumerate() {
        let status = if index < state.reached {
            StepStatus::Done
        } else if index == state.reached {
            StepStatus::Active
        } else {
            StepStatus::Idle
        };
        list = list.child(Step::new(SharedString::from(*label), status));
    }

    Col::new()
        .width(px(360.))
        .child(list)
        .child(
            Row::new().child(Button::new("advance", "Advance").on_click({
                let advance = act(this, |state, _, _| {
                    state.reached = if state.reached >= STEPS.len() {
                        0
                    } else {
                        state.reached + 1
                    };
                });
                move |_, window, cx| advance(window, cx)
            })),
        )
}
