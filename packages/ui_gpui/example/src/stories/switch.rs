//! `Components/Switch` — a decision that takes effect at once.

use dazzui::{
    ActiveTheme, Checkbox, PreferenceRow, PreferenceSection, Switch, SwitchSize, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, TokenTable, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [SwitchSize; 3] = [SwitchSize::Small, SwitchSize::Medium, SwitchSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "Switch",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            let mut sizes = Specimens::new();
            for size in SIZES {
                let mut row = Specimen::new(case(size));
                for tint in Tint::ALL {
                    row = row.child(
                        Switch::new(
                            SharedString::from(format!("{}-{}", case(size), tint.name())),
                            true,
                        )
                        .size(size)
                        .tint(tint),
                    );
                }
                sizes = sizes.child(row);
            }

            DocPage::new("Switch")
                .description(
                    "A track and a thumb. It is the control for a setting that applies the \
                     moment it moves — which is what separates it from a checkbox, and why it \
                     belongs at the trailing edge of a preference row rather than in a form you \
                     submit.",
                )
                .child(
                    Section::new("Sizes and tints")
                        .description(
                            "The track, the height and the thumb are three tokens moved \
                             together, so the thumb keeps its inset at every size. The tint is \
                             the on-state's fill; off is a surface role and never follows it.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Against a checkbox")
                        .description(
                            "A switch says the change is already made; a checkbox says it will \
                             be made when the form is. Choosing by which one looks better is how \
                             a settings screen ends up with both.",
                        )
                        .child(
                            Col::new()
                                .child(
                                    Row::new()
                                        .child(Switch::new("against", true))
                                        .child(div().child("Takes effect the moment it moves")),
                                )
                                .child(Checkbox::new(
                                    "against-checkbox",
                                    "Takes effect when the form is submitted",
                                    true,
                                )),
                        ),
                )
                .child(
                    Section::new("In settings")
                        .description("Where nearly every switch in an app actually lives.")
                        .child(in_settings(state, this)),
                )
                .child(
                    Section::new("Tokens")
                        .description("The three the switch owns, per size.")
                        .child(
                            TokenTable::new()
                                .length("switch.small.width", vars.switch_small_width)
                                .length("switch.small.height", vars.switch_small_height)
                                .length("switch.small.thumb", vars.switch_small_thumb)
                                .length("switch.medium.width", vars.switch_medium_width)
                                .length("switch.medium.height", vars.switch_medium_height)
                                .length("switch.medium.thumb", vars.switch_medium_thumb)
                                .length("switch.large.width", vars.switch_large_width)
                                .length("switch.large.height", vars.switch_large_height)
                                .length("switch.large.thumb", vars.switch_large_thumb),
                        ),
                )
                .into_any_element()
        }),
        Story::new("On", |_, _, _| Switch::new("on", true).into_any_element()),
        Story::new("Off", |_, _, _| {
            Switch::new("off", false).into_any_element()
        }),
        Story::new("Interactive", |state, this, _| {
            Switch::new("live", state.notify)
                .on_change(act_with(this, |state, value: &bool, _, _| {
                    state.notify = *value
                }))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            Row::new()
                .child(Switch::new("dead-off", false).disabled(true))
                .child(Switch::new("dead-on", true).disabled(true))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new();
            for size in SIZES {
                row = row.child(Switch::new(SharedString::from(case(size)), true).size(size));
            }
            row.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut row = Row::new();
            for tint in Tint::ALL {
                row = row.child(Switch::new(tint.name(), true).tint(tint));
            }
            row.into_any_element()
        }),
        Story::new("Against a checkbox", |_, _, _| {
            Col::new()
                .child(
                    Row::new()
                        .child(Switch::new("switch", true))
                        .child(div().child("Takes effect the moment it moves")),
                )
                .child(Checkbox::new(
                    "checkbox",
                    "Takes effect when the form is submitted",
                    true,
                ))
                .into_any_element()
        }),
        Story::new("In settings", |state, this, _| {
            in_settings(state, this).into_any_element()
        }),
    ],
);

fn in_settings(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    div().w(px(460.)).child(
        PreferenceSection::new()
            .label("General")
            .child(
                PreferenceRow::new("launch", "Launch at login").child(
                    Switch::new("settings-launch", state.launch)
                        .on_change(act_with(this, |state, value: &bool, _, _| {
                            state.launch = *value
                        })),
                ),
            )
            .child(
                PreferenceRow::new("updates", "Check for updates")
                    .subtitle("Downloads in the background and asks before installing.")
                    .child(
                        Switch::new("settings-updates", state.updates)
                            .on_change(act_with(this, |state, value: &bool, _, _| {
                                state.updates = *value
                            })),
                    ),
            )
            .child(
                PreferenceRow::new("sounds", "Play sounds").child(
                    Switch::new("settings-sounds", state.sounds)
                        .on_change(act_with(this, |state, value: &bool, _, _| {
                            state.sounds = *value
                        })),
                ),
            ),
    )
}
