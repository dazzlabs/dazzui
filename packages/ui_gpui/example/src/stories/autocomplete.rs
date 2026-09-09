//! `Components/Autocomplete` — a text field that suggests as you type.

use dazzui::{Autocomplete, FormField, TextFieldSize, TextFieldState};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

const BRANCHES: [&str; 7] = [
    "main",
    "release/4.2",
    "release/4.1",
    "fix/token-drift",
    "fix/popup-layer",
    "feat/slider",
    "feat/drawer",
];

fn branches(id: impl Into<gpui::ElementId>, state: &Playground) -> Autocomplete {
    let mut field = Autocomplete::new(id, &state.branch).mono(true);
    for branch in BRANCHES {
        field = field.item(branch);
    }
    field
}

pub const PAGE: StoryPage = StoryPage::new(
    "Autocomplete",
    &[
        Story::page("Overview", |state, this, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase())).child(
                        div().w(px(224.)).child(
                            branches(SharedString::from(format!("doc-{size:?}")), state)
                                .size(size)
                                .on_change(fill(this)),
                        ),
                    ),
                );
            }

            DocPage::new("Autocomplete")
                .description(
                    "A text field that suggests as you type. The difference from a Combobox is \
                     what the value is: a combobox ends in one of its options and nothing else; \
                     an autocomplete's value is whatever was typed, and the list is a shortcut \
                     to a likely answer rather than the set of allowed ones. A branch name, a \
                     search query, a tag that may not exist yet.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "It is a TextField wearing the shared popup sheet, and it is \
                             literally that: the same box, the same panel, the same rows. There \
                             is no chevron and no check column — nothing here is opened or \
                             chosen, and the field is emptied the way any text field is.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Against a Combobox")
                        .description(
                            "Ask what happens when the reader types something that is not on \
                             the list. If that is an answer, this is the component. If it is a \
                             mistake, it is a Combobox.",
                        )
                        .child(
                            div()
                                .w(px(256.))
                                .child(branches("doc-against", state).on_change(fill(this))),
                        ),
                )
                .child(
                    Section::new("States")
                        .description(
                            "Erroring re-tints to danger like every other text control, so the \
                             border, the wash and the value's ink move together.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("error").child(div().w(px(224.)).child(
                                    branches("doc-error", state).state(TextFieldState::Error),
                                )))
                                .child(
                                    Specimen::new("disabled").child(
                                        div()
                                            .w(px(224.))
                                            .child(branches("doc-off", state).disabled(true)),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, this, _| {
            div()
                .w(px(256.))
                .child(branches("bare", state).mono(false).on_change(fill(this)))
                .into_any_element()
        }),
        Story::new("Mono", |state, this, _| {
            div()
                .w(px(256.))
                .child(branches("mono", state).on_change(fill(this)))
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new().width(px(256.));
            for size in SIZES {
                col = col.child(
                    branches(SharedString::from(format!("sizes-{size:?}")), state).size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("States", |state, _, _| {
            Col::new()
                .width(px(256.))
                .child(branches("states-default", state))
                .child(branches("states-error", state).state(TextFieldState::Error))
                .child(branches("states-disabled", state).disabled(true))
                .into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(320.))
                .child(
                    FormField::new("Branch")
                        .hint("Creates a new branch if it does not exist")
                        .child(branches("form-branch", state).on_change(fill(this))),
                )
                .into_any_element()
        }),
    ],
);

fn fill(
    this: &gpui::Entity<Playground>,
) -> impl Fn(&SharedString, &mut gpui::Window, &mut gpui::App) + 'static {
    act_with(this, |state, value: &SharedString, _, cx| {
        let value = value.clone();
        state
            .branch
            .update(cx, |buffer, cx| buffer.set_value(value, cx));
    })
}
