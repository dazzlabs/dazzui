//! `Components/Code` — a run of code inside a sentence.

use dazzui::Code;
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

/// GPUI lays a paragraph out as a run of text, and an element cannot sit inside
/// one — so a sentence with code in it is a row of runs rather than one
/// paragraph. The React page reads as a sentence; this one reads as its parts.
fn sentence() -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(4.))
        .child("Pass")
        .child(Code::new("--features runtime-shaders"))
        .child("then run")
        .child(Code::new("cargo test"))
        .child("from")
        .child(Code::new("packages/ui_gpui"))
}

pub const PAGE: StoryPage = StoryPage::new(
    "Code",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Code")
                .description(
                    "A run of code inside a sentence — a flag, a path, an identifier. It is a \
                     wash and a face, not a box: the chip corner and the half-step inset keep \
                     it separable from the prose around it without breaking the line's rhythm, \
                     which is what a border at this size always does.",
                )
                .child(
                    Section::new("In a sentence")
                        .description(
                            "The face is the code face read straight off base.font.code — the \
                             one place in this package a base primitive is named, because the \
                             code face is not a role. The size steps down one from the prose it \
                             sits in: a monospace face at the same nominal size always reads \
                             larger, and the run would ride above the line.",
                        )
                        .child(sentence()),
                )
                .child(
                    Section::new("What it is not")
                        .description(
                            "A run of code is not a KeyCap and not a Badge. A KeyCap is a key \
                             you press and is drawn as one; a Badge is an annotation about \
                             something else. This is the thing itself, quoted.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("an identifier")
                                        .child(Code::new("ThemeVariables")),
                                )
                                .child(
                                    Specimen::new("a path")
                                        .child(Code::new("packages/ds/tokens/global/_base.yaml")),
                                )
                                .child(Specimen::new("a flag").child(Code::new("--no-verify"))),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Inline", |_, _, _| {
            Code::new("pnpm parity").into_any_element()
        }),
        Story::new("In prose", |_, _, _| sentence().into_any_element()),
    ],
);
