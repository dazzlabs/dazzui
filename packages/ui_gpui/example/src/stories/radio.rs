//! `Components/Radio` — one of several.

use dazzui::{ActiveTheme, Checkbox, FormField, Radio, RadioGroup, RadioSize, Tint};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{
    DocPage, Section, Specimen, Specimens, TokenTable, capitalize, case,
};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [RadioSize; 3] = [RadioSize::Small, RadioSize::Medium, RadioSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "Radio",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            let mut sizes = Specimens::new();
            for size in SIZES {
                let mut row = Specimen::new(case(size));
                for tint in Tint::ALL {
                    row = row.child(
                        Radio::new(
                            SharedString::from(format!("{}-{}", case(size), tint.name())),
                            capitalize(tint.name()),
                            true,
                        )
                        .size(size)
                        .tint(tint),
                    );
                }
                sizes = sizes.child(row);
            }

            DocPage::new("Radio")
                .description(
                    "One of several. The ring draws its own mark rather than stacking a dot on \
                     a circle, so the two never disagree about where the centre is. A single \
                     radio is rarely right — `RadioGroup` is the usual form, and it owns the \
                     value.",
                )
                .child(
                    Section::new("A group")
                        .description(
                            "The group carries the options, the selection and the tint, so a \
                             row cannot disagree with its neighbours. A disabled option flattens \
                             to the groove, whether or not it is the one selected.",
                        )
                        .child(plans(state, this)),
                )
                .child(
                    Section::new("Sizes and tints")
                        .description(
                            "The box and the dot are both the radio's own tokens, sized \
                             together so the mark stays centred at every step.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Against a checkbox")
                        .description(
                            "A radio is one of several; a checkbox is one decision on its own. \
                             The shapes say which: a circle belongs to a set, a square does not.",
                        )
                        .child(
                            Col::new()
                                .child(Radio::new("one", "A radio picks one of several", true))
                                .child(Checkbox::new("two", "A checkbox is one decision", true)),
                        ),
                )
                .child(
                    Section::new("Tokens")
                        .description("The ring and the mark inside it.")
                        .child(
                            TokenTable::new()
                                .length("radio.thickness", vars.radio_thickness)
                                .length("radio.box.small", vars.radio_small_box)
                                .length("radio.dot.small", vars.radio_small_dot)
                                .length("radio.box.medium", vars.radio_medium_box)
                                .length("radio.dot.medium", vars.radio_medium_dot)
                                .length("radio.box.large", vars.radio_large_box)
                                .length("radio.dot.large", vars.radio_large_dot),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Checked", |_, _, _| {
            Radio::new("checked", "Team", true).into_any_element()
        }),
        Story::new("Unchecked", |_, _, _| {
            Radio::new("unchecked", "Team", false).into_any_element()
        }),
        Story::new("A group", |state, this, _| {
            plans(state, this).into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            Col::new()
                .child(Radio::new("off", "Unavailable", false).disabled(true))
                .child(Radio::new("on", "Fixed for this plan", true).disabled(true))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(
                    Radio::new(
                        SharedString::from(case(size)),
                        capitalize(&case(size)),
                        true,
                    )
                    .size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(Radio::new(tint.name(), capitalize(tint.name()), true).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("Against a checkbox", |_, _, _| {
            Col::new()
                .child(Radio::new("radio", "A radio picks one of several", true))
                .child(Checkbox::new(
                    "checkbox",
                    "A checkbox is one decision",
                    true,
                ))
                .into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(320.))
                .child(FormField::new("Plan").child(plans(state, this)))
                .into_any_element()
        }),
    ],
);

fn plans(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    RadioGroup::new("plan", state.plan.clone())
        .option("solo", "Solo")
        .option("team", "Team")
        .disabled_option("enterprise", "Enterprise")
        .on_select(act_with(this, |state, value: &SharedString, _, _| {
            state.plan = value.clone()
        }))
}
