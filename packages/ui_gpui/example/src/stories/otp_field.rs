//! `Components/OtpField` — the one-time code a reader is copying.

use dazzui::{Button, Callout, ControlVariant, FormField, OtpField, TextFieldSize, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "OtpField",
    &[
        Story::page("Overview", |state, _, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase())).child(
                        OtpField::new(SharedString::from(format!("doc-{size:?}")), &state.code, 6)
                            .size(size),
                    ),
                );
            }

            DocPage::new("OtpField")
                .description(
                    "The one-time code a reader is copying out of a message. It is drawn as a \
                     run of cells but it is one field underneath, and that is deliberate: a \
                     paste of the whole code fills every cell at once, and the caret only ever \
                     has one place to be. A run of separate inputs gets both wrong and has to \
                     reinvent the moves between them.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "Each cell is the text controls' box at one character wide, so a \
                             code sits on the same rhythm as the fields around it — a code \
                             entered in a box of its own design would be the only control on \
                             the page that was.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Grouping")
                        .description(
                            "`group_after` breaks the run where a printed code breaks — three \
                             and three is how a six-digit code is read aloud. It is a step of \
                             air rather than a rule, because a rule would read as two fields.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("ungrouped").child(OtpField::new(
                                    "doc-flat",
                                    &state.code,
                                    6,
                                )))
                                .child(Specimen::new("group_after 3").child(
                                    OtpField::new("doc-grouped", &state.code, 6).group_after(3),
                                )),
                        ),
                )
                .child(
                    Section::new("Masked and disabled")
                        .description(
                            "`mask` is for a code that is a secret rather than a one-time token \
                             — a PIN. A filled cell lifts to the paper and keeps it, so the run \
                             says how far through the code the reader is.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("masked").child(
                                        OtpField::new("doc-masked", &state.code, 4).mask(true),
                                    ),
                                )
                                .child(Specimen::new("disabled").child(
                                    OtpField::new("doc-off", &state.code, 6).disabled(true),
                                )),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, _, _| {
            OtpField::new("bare", &state.code, 6).into_any_element()
        }),
        Story::new("Filled", |state, _, _| {
            OtpField::new("filled", &state.code, 6).into_any_element()
        }),
        Story::new("Grouped", |state, _, _| {
            OtpField::new("grouped", &state.code, 6)
                .group_after(3)
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(
                    OtpField::new(
                        SharedString::from(format!("sizes-{size:?}")),
                        &state.code,
                        6,
                    )
                    .size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("Masked", |state, _, _| {
            OtpField::new("masked", &state.code, 4)
                .mask(true)
                .into_any_element()
        }),
        Story::new("Disabled", |state, _, _| {
            OtpField::new("disabled", &state.code, 6)
                .disabled(true)
                .into_any_element()
        }),
        Story::new("In a form", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    Col::new()
                        .child(
                            Callout::new()
                                .tint(Tint::Info)
                                .description("We sent a six-digit code to ada@example.com."),
                        )
                        .child(
                            FormField::new("Verification code")
                                .hint("It expires in 10 minutes")
                                .child(OtpField::new("form-code", &state.code, 6).group_after(3)),
                        )
                        .child(Button::new("verify", "Verify").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
    ],
);
