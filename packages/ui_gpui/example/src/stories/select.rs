//! `Components/Select` — one of many, in a field's box.

use dazzui::{
    Button, ControlVariant, FormField, PreferenceRow, PreferenceSection, Select, TextFieldSize,
    TextFieldState, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, capitalize, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "Select",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("Select")
                .description(
                    "One value out of many. It wears the text field's box — the same heights, \
                     the same states, the same focus ring — so a select and an input in the \
                     same form line up. Its list is the shared floating sheet, the one a menu \
                     and a tooltip also wear.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Default, error and disabled, exactly as the field has them. An \
                             option can be disabled on its own without the control being.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(region(state, this, "doc", None, false))
                                .child(region(
                                    state,
                                    this,
                                    "doc-error",
                                    Some(TextFieldState::Error),
                                    false,
                                ))
                                .child(region(state, this, "doc-dead", None, true)),
                        ),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The three field sizes, which are the control profile's under \
                             another name. `small` is the one a settings row wants at its \
                             trailing edge.",
                        )
                        .child({
                            let mut col = Col::new().width(px(260.));
                            for size in SIZES {
                                col = col.child(
                                    Select::new(
                                        SharedString::from(format!("doc-size-{}", case(size))),
                                        state.region.clone(),
                                    )
                                    .size(size)
                                    .option("us", "United States")
                                    .option("eu", "Europe")
                                    .on_select(act_with(
                                        this,
                                        |state, value: &SharedString, _, _| {
                                            state.region = value.clone()
                                        },
                                    )),
                                );
                            }
                            col
                        }),
                )
                .child(
                    Section::new("In a form")
                        .description(
                            "Under a `FormField` label like any other control, or pinned to the \
                             trailing edge of a settings row at the small step.",
                        )
                        .child(
                            Col::new()
                                .width(px(460.))
                                .child(
                                    FormField::new("Region")
                                        .hint("Where the documents are stored.")
                                        .child(region(state, this, "doc-form", None, false)),
                                )
                                .child(
                                    PreferenceSection::new().label("Appearance").child(
                                        PreferenceRow::new("theme", "Theme")
                                            .child(appearance(state, this, "doc-row")),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Default", |state, this, _| {
            region(state, this, "default", None, false).into_any_element()
        }),
        Story::new("Disabled", |state, this, _| {
            region(state, this, "disabled", None, true).into_any_element()
        }),
        Story::new("Error state", |state, this, _| {
            region(state, this, "error", Some(TextFieldState::Error), false).into_any_element()
        }),
        Story::new("Sizes", |state, this, _| {
            let mut col = Col::new().width(px(260.));
            for size in SIZES {
                col = col.child(
                    Select::new(
                        SharedString::from(format!("size-{}", case(size))),
                        state.region.clone(),
                    )
                    .size(size)
                    .option("us", "United States")
                    .option("eu", "Europe")
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.region = value.clone()
                    })),
                );
            }
            col.into_any_element()
        }),
        Story::new("Tints", |state, this, _| {
            let mut col = Col::new().width(px(260.));
            for tint in Tint::ALL {
                col = col.child(
                    Select::new(
                        SharedString::from(format!("tint-{}", tint.name())),
                        state.region.clone(),
                    )
                    .tint(tint)
                    .option("us", capitalize(tint.name()))
                    .option("eu", "Europe")
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.region = value.clone()
                    })),
                );
            }
            col.into_any_element()
        }),
        Story::new("Beside a button", |state, this, _| {
            Row::new()
                .child(region(state, this, "beside", None, false))
                .child(Button::new("apply", "Apply").variant(ControlVariant::Filled))
                .into_any_element()
        }),
        Story::new("In a form field", |state, this, _| {
            div()
                .w(px(320.))
                .child(
                    FormField::new("Region")
                        .hint("Where the documents are stored.")
                        .child(region(state, this, "field", None, false)),
                )
                .into_any_element()
        }),
        Story::new("In a settings row", |state, this, _| {
            div()
                .w(px(460.))
                .child(PreferenceSection::new().label("Appearance").child(
                    PreferenceRow::new("theme", "Theme").child(appearance(state, this, "row")),
                ))
                .into_any_element()
        }),
    ],
);

fn region(
    state: &Playground,
    this: &Entity<Playground>,
    id: &'static str,
    error: Option<TextFieldState>,
    disabled: bool,
) -> impl IntoElement {
    let mut select = Select::new(SharedString::from(id), state.region.clone())
        .option("us", "United States")
        .option("eu", "Europe")
        .option("ap", "Asia Pacific")
        .disabled(disabled)
        .on_select(act_with(this, |state, value: &SharedString, _, _| {
            state.region = value.clone()
        }));

    if let Some(state) = error {
        select = select.state(state);
    }

    div().w(px(220.)).child(select)
}

fn appearance(state: &Playground, this: &Entity<Playground>, id: &'static str) -> impl IntoElement {
    Select::new(SharedString::from(id), state.appearance.clone())
        .size(TextFieldSize::Small)
        .option("auto", "Automatic")
        .option("light", "Light")
        .option("dark", "Dark")
        .on_select(act_with(this, |state, value: &SharedString, _, _| {
            state.appearance = value.clone()
        }))
}
