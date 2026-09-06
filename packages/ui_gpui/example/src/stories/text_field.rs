//! `Components/TextField` — the box a value is typed into.

use dazzui::{
    Button, ControlVariant, FieldValue, FormField, TextField, TextFieldSize, TextFieldState, Tint,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "TextField",
    &[
        Story::page("Overview", |state, _, _| {
            let mut sizes = Col::new().width(px(420.));
            for size in SIZES {
                sizes = sizes.child(
                    div()
                        .w(px(240.))
                        .child(TextField::new(&state.name).size(size)),
                );
            }

            DocPage::new("TextField")
                .description(
                    "The box a value is typed into, on the control heights everything else \
                     uses. The focus ring is the system accent in every tint — it never follows \
                     the control's own colour, because a ring that changed colour per field \
                     would stop meaning focus.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Default, empty with a placeholder, invalid, and disabled. The \
                             error state turns the field's own edge; the message under it is the \
                             `FormField`'s hint, not a second line the field draws.",
                        )
                        .child(
                            Col::new()
                                .width(px(420.))
                                .child(div().w(px(240.)).child(TextField::new(&state.name)))
                                .child(div().w(px(240.)).child(TextField::new(&state.notes)))
                                .child(div().w(px(240.)).child(
                                    TextField::new(&state.email).state(TextFieldState::Error),
                                ))
                                .child(
                                    div()
                                        .w(px(240.))
                                        .child(TextField::new(&state.name).disabled(true)),
                                ),
                        ),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The three field sizes are the control profile's, so a field, a \
                             select and a button on the same row are the same height.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Mono and read-only")
                        .description(
                            "`mono` is for a value that is code — a hash, a path, a command; it \
                             is the one place a component reaches a `base.*` primitive, because \
                             `base.font.code` has no semantic token in front of it. \
                             `FieldValue` is the other end: a value that cannot be edited, in \
                             the field's own shape.",
                        )
                        .child(
                            Col::new()
                                .width(px(420.))
                                .child(
                                    div()
                                        .w(px(240.))
                                        .child(TextField::new(&state.name).mono(true)),
                                )
                                .child(FieldValue::new("785f949").mono(true))
                                .child(FieldValue::new("DazzUI")),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Default", |state, _, _| {
            div()
                .w(px(240.))
                .child(TextField::new(&state.name))
                .into_any_element()
        }),
        Story::new("Placeholder", |state, _, _| {
            div()
                .w(px(240.))
                .child(TextField::new(&state.notes))
                .into_any_element()
        }),
        Story::new("Disabled", |state, _, _| {
            div()
                .w(px(240.))
                .child(TextField::new(&state.name).disabled(true))
                .into_any_element()
        }),
        Story::new("Error state", |state, _, _| {
            div()
                .w(px(240.))
                .child(TextField::new(&state.email).state(TextFieldState::Error))
                .into_any_element()
        }),
        Story::new("States", |state, _, _| {
            Col::new()
                .width(px(240.))
                .child(TextField::new(&state.name))
                .child(TextField::new(&state.email).state(TextFieldState::Error))
                .child(TextField::new(&state.name).disabled(true))
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new().width(px(240.));
            for size in SIZES {
                col = col.child(TextField::new(&state.name).size(size));
            }
            col.into_any_element()
        }),
        Story::new("Tints", |state, _, _| {
            let mut col = Col::new().width(px(240.));
            for tint in Tint::ALL {
                col = col.child(TextField::new(&state.name).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("Mono", |state, _, _| {
            div()
                .w(px(240.))
                .child(TextField::new(&state.name).mono(true))
                .into_any_element()
        }),
        Story::new("Beside a button", |state, _, _| {
            Row::new()
                .child(div().w(px(240.)).child(TextField::new(&state.name)))
                .child(Button::new("add", "Add").variant(ControlVariant::Filled))
                .into_any_element()
        }),
        Story::new("A read-only value", |_, _, _| {
            Col::new()
                .width(px(320.))
                .child(FieldValue::new("DazzUI"))
                .child(FieldValue::new("785f949").mono(true))
                .into_any_element()
        }),
        Story::new("Multiline", |state, _, _| {
            div()
                .w(px(320.))
                .child(TextField::new(&state.notes).multiline(true))
                .into_any_element()
        }),
        Story::new("Multiline in a form", |state, _, _| {
            div()
                .w(px(380.))
                .child(
                    FormField::new("Notes")
                        .hint("Markdown is supported.")
                        .child(TextField::new(&state.notes).multiline(true)),
                )
                .into_any_element()
        }),
    ],
);
