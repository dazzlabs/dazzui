//! `Components/FormField` — the label over a control and the hint under it.

use dazzui::{
    FieldValue, FormField, KeyCap, Select, Switch, TextField, TextFieldState, Tint,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section, capitalize};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "FormField",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("FormField")
                .description(
                    "The chrome around a control: a label above it, a hint below, and the error \
                     state that turns both. It takes any control as its child rather than \
                     knowing about text fields, so a switch or a key cap gets the same label \
                     treatment as an input.",
                )
                .child(
                    Section::new("Label and hint")
                        .description(
                            "The hint is the field's ordinary explanation. In the error state it \
                             becomes the message, in the tint's ink — one line rather than a hint \
                             and an error stacked, because a field that explains itself twice is \
                             a field the reader stops reading.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(
                                    div().w(px(300.)).child(
                                        FormField::new("Workspace name")
                                            .hint("Shown to everyone in the workspace.")
                                            .child(TextField::new(&state.name)),
                                    ),
                                )
                                .child(
                                    div().w(px(300.)).child(
                                        FormField::new("Workspace name")
                                            .child(TextField::new(&state.name)),
                                    ),
                                )
                                .child(div().w(px(300.)).child(invalid(state))),
                        ),
                )
                .child(
                    Section::new("Over any control")
                        .description(
                            "A select, a multiline field, a read-only `FieldValue`, or a control \
                             that is not a field at all. The label is a label for whatever is \
                             underneath it.",
                        )
                        .child(
                            Col::new()
                                .width(px(380.))
                                .child(over_a_select(state, this))
                                .child(
                                    FormField::new("Notes")
                                        .hint("Markdown is supported.")
                                        .child(TextField::new(&state.notes).multiline(true)),
                                )
                                .child(
                                    FormField::new("Commit")
                                        .hint("Read from the last build.")
                                        .child(FieldValue::new("785f949").mono(true)),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("With a hint", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    FormField::new("Workspace name")
                        .hint("Shown to everyone in the workspace.")
                        .child(TextField::new(&state.name)),
                )
                .into_any_element()
        }),
        Story::new("Without a hint", |state, _, _| {
            div()
                .w(px(320.))
                .child(FormField::new("Workspace name").child(TextField::new(&state.name)))
                .into_any_element()
        }),
        Story::new("Error state", |state, _, _| {
            div().w(px(320.)).child(invalid(state)).into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for tint in Tint::ALL {
                col = col.child(
                    FormField::new(capitalize(tint.name()))
                        .tint(tint)
                        .child(FieldValue::new("Focus the field to see the ring")),
                );
            }
            col.into_any_element()
        }),
        Story::new("Over a select", |state, this, _| {
            div()
                .w(px(320.))
                .child(over_a_select(state, this))
                .into_any_element()
        }),
        Story::new("Over a multiline field", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    FormField::new("Notes")
                        .hint("Markdown is supported.")
                        .child(TextField::new(&state.notes).multiline(true)),
                )
                .into_any_element()
        }),
        Story::new("With a field value", |_, _, _| {
            div()
                .w(px(320.))
                .child(
                    FormField::new("Commit")
                        .hint("Read from the last build.")
                        .child(FieldValue::new("785f949").mono(true)),
                )
                .into_any_element()
        }),
        Story::new("Over any control", |state, this, _| {
            Col::new()
                .width(px(320.))
                .child(
                    FormField::new("Notifications").child(
                        Switch::new("notify", state.notify)
                            .on_change(act_with(this, |state, value: &bool, _, _| {
                                state.notify = *value
                            })),
                    ),
                )
                .child(FormField::new("Shortcut").child(KeyCap::new("⇧⌘B")))
                .into_any_element()
        }),
        Story::new("A full form", |state, this, _| {
            Col::new()
                .width(px(380.))
                .child(FormField::new("Workspace name").child(TextField::new(&state.name)))
                .child(invalid(state))
                .child(over_a_select(state, this))
                .child(
                    FormField::new("Notes")
                        .hint("Markdown is supported.")
                        .child(TextField::new(&state.notes).multiline(true)),
                )
                .into_any_element()
        }),
    ],
);

fn invalid(state: &crate::Playground) -> impl IntoElement {
    FormField::new("Email")
        .hint("We could not reach this address.")
        .state(TextFieldState::Error)
        .child(TextField::new(&state.email).state(TextFieldState::Error))
}

fn over_a_select(
    state: &crate::Playground,
    this: &gpui::Entity<crate::Playground>,
) -> impl IntoElement {
    FormField::new("Region")
        .hint("Where the documents are stored.")
        .child(
            Select::new("region", state.region.clone())
                .option("us", "United States")
                .option("eu", "Europe")
                .option("ap", "Asia Pacific")
                .on_select(act_with(this, |state, value: &gpui::SharedString, _, _| {
                    state.region = value.clone()
                })),
        )
}
