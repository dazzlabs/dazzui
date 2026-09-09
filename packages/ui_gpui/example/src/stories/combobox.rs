//! `Components/Combobox` — a choice out of a list too long to scroll.

use dazzui::{Combobox, ComboboxOption, FormField, TextFieldSize, TextFieldState};
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

const ZONES: [(&str, &str, &str); 8] = [
    ("lisbon", "Europe/Lisbon", "Europe"),
    ("london", "Europe/London", "Europe"),
    ("berlin", "Europe/Berlin", "Europe"),
    ("shanghai", "Asia/Shanghai", "Asia"),
    ("tokyo", "Asia/Tokyo", "Asia"),
    ("sydney", "Australia/Sydney", "Asia"),
    ("new_york", "America/New_York", "Americas"),
    ("sao_paulo", "America/Sao_Paulo", "Americas"),
];

fn zones(id: impl Into<gpui::ElementId>, state: &Playground, grouped: bool) -> Combobox {
    let mut combobox = Combobox::new(id, &state.zone_query).value(state.zone.clone());
    for (value, label, group) in ZONES {
        let option = ComboboxOption::new(value, label);
        combobox = combobox.option(if grouped { option.group(group) } else { option });
    }
    combobox
}

pub const PAGE: StoryPage = StoryPage::new(
    "Combobox",
    &[
        Story::page("Overview", |state, this, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase())).child(
                        div().w(px(224.)).child(
                            zones(SharedString::from(format!("doc-{size:?}")), state, false)
                                .size(size)
                                .on_change(pick(this)),
                        ),
                    ),
                );
            }

            DocPage::new("Combobox")
                .description(
                    "A choice out of a list too long to scroll: the reader types to narrow it, \
                     and picks one of what is left. It is a Select with a query in front of it, \
                     and that is the only question worth asking when choosing between them — a \
                     list of six is a select, a list of six hundred is this. Both end in one of \
                     the options; neither lets a reader enter something that is not on the \
                     list, which is what separates them from an Autocomplete.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The box is the TextField's and the list is the menu's own panel, \
                             so a combobox in a form is the same box as the field beside it and \
                             the same panel a Menu opens.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Grouped")
                        .description(
                            "The headings are drawn from what survived the query rather than \
                             from the list as it was given: a group the query emptied is not a \
                             heading over nothing.",
                        )
                        .child(
                            div()
                                .w(px(256.))
                                .child(zones("doc-grouped", state, true).on_change(pick(this))),
                        ),
                )
                .child(
                    Section::new("States")
                        .description(
                            "Erroring re-tints the control to danger, exactly as a TextField \
                             does. The empty line is prose in a panel of rows, so it takes the \
                             row's inset and the faint ink rather than looking like a row that \
                             cannot be picked.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("error").child(div().w(px(224.)).child(
                                    zones("doc-error", state, false).state(TextFieldState::Error),
                                )))
                                .child(
                                    Specimen::new("disabled").child(
                                        div()
                                            .w(px(224.))
                                            .child(zones("doc-off", state, false).disabled(true)),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, this, _| {
            div()
                .w(px(256.))
                .child(zones("bare", state, false).on_change(pick(this)))
                .into_any_element()
        }),
        Story::new("Grouped", |state, this, _| {
            div()
                .w(px(256.))
                .child(zones("grouped", state, true).on_change(pick(this)))
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new().width(px(256.));
            for size in SIZES {
                col = col.child(
                    zones(SharedString::from(format!("sizes-{size:?}")), state, false).size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("States", |state, _, _| {
            Col::new()
                .width(px(256.))
                .child(zones("states-default", state, false))
                .child(zones("states-error", state, false).state(TextFieldState::Error))
                .child(zones("states-disabled", state, false).disabled(true))
                .into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(288.))
                .child(
                    FormField::new("Time zone")
                        .hint("Used for every schedule")
                        .child(zones("form-zone", state, true).on_change(pick(this))),
                )
                .into_any_element()
        }),
    ],
);

fn pick(
    this: &gpui::Entity<Playground>,
) -> impl Fn(&SharedString, &mut gpui::Window, &mut gpui::App) + 'static {
    act_with(this, |state, value: &SharedString, _, cx| {
        state.zone = value.clone();
        // Picking one fills the query with what was picked, the way a select
        // shows its chosen row.
        let label = ZONES
            .iter()
            .find(|(id, _, _)| *id == value.as_ref())
            .map(|(_, label, _)| *label)
            .unwrap_or_default();
        state
            .zone_query
            .update(cx, |buffer, cx| buffer.set_value(label, cx));
    })
}
