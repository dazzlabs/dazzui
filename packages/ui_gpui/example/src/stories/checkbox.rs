//! `Components/Checkbox` — a box, its label, and the note under it.

use dazzui::{Checkbox, CheckboxGroup, CheckboxSize, FormField, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize, case};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const PLATFORMS: [&str; 4] = ["react", "flutter", "swiftui", "gpui"];

const SIZES: [CheckboxSize; 3] = [
    CheckboxSize::Small,
    CheckboxSize::Medium,
    CheckboxSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "Checkbox",
    &[
        Story::page("Overview", |state, this, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                let mut row = Specimen::new(case(size));
                for tint in Tint::ALL {
                    row = row.child(
                        Checkbox::new(
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

            DocPage::new("Checkbox")
                .description(
                    "A box, a label beside it and an optional note under that. The whole row is \
                     the target, and the box carries the tint — so a checklist reads as one \
                     column of decisions rather than as a column of squares.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Checked, unchecked and disabled. Disabled flattens to the groove — \
                             one dead look whatever the state under it — rather than a \
                             translucent ghost of the live control.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("checked").child(Checkbox::new(
                                    "on",
                                    "Newsletter",
                                    true,
                                )))
                                .child(Specimen::new("unchecked").child(Checkbox::new(
                                    "off",
                                    "Newsletter",
                                    false,
                                )))
                                .child(
                                    Specimen::new("disabled")
                                        .child(
                                            Checkbox::new("dead-off", "Newsletter", false)
                                                .disabled(true),
                                        )
                                        .child(
                                            Checkbox::new("dead-on", "Newsletter", true)
                                                .disabled(true),
                                        ),
                                ),
                        ),
                )
                .child(
                    Section::new("Sizes and tints")
                        .description(
                            "The box is a component token of its own — a checkbox is squarer \
                             than the control radius — and its three sizes are the checkbox's, \
                             not the control profile's.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("The note")
                        .description(
                            "A note sits under the label rather than beside it, indented to the \
                             label's own edge — so the second line reads as the same decision \
                             explained, not as a second decision.",
                        )
                        .child(
                            div().w(px(420.)).child(
                                Checkbox::new(
                                    "telemetry",
                                    "Share anonymous usage data",
                                    state.telemetry,
                                )
                                .note("Crash reports only. Never the contents of a document.")
                                .on_change(act_with(this, |state, value: &bool, _, _| {
                                    state.telemetry = *value
                                })),
                            ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Checked", |_, _, _| {
            Checkbox::new("checked", "Send me the newsletter", true).into_any_element()
        }),
        Story::new("Unchecked", |_, _, _| {
            Checkbox::new("unchecked", "Send me the newsletter", false).into_any_element()
        }),
        Story::new("Interactive", |state, this, _| {
            Checkbox::new("live", "Send me the monthly newsletter", state.newsletter)
                .on_change(act_with(this, |state, value: &bool, _, _| {
                    state.newsletter = *value
                }))
                .into_any_element()
        }),
        Story::new("A group", |state, this, _| {
            // The parent is what a lone checkbox can never be: mixed. It reads
            // the group rather than a value of its own, and pressing it sets
            // or clears the lot.
            let chosen = state.platforms.clone();
            let all = PLATFORMS.len();
            let mut group = CheckboxGroup::new().parent(
                Checkbox::new("every", "Every platform", chosen.len() == all)
                    .indeterminate(!chosen.is_empty() && chosen.len() < all)
                    .note(SharedString::from(format!("{} of {all}", chosen.len())))
                    .on_change(act_with(this, move |state, _: &bool, _, _| {
                        state.platforms = if state.platforms.len() == all {
                            Vec::new()
                        } else {
                            PLATFORMS.iter().map(|name| (*name).into()).collect()
                        };
                    })),
            );
            for name in PLATFORMS {
                let on = chosen.iter().any(|chosen| chosen == name);
                group = group.child(Checkbox::new(SharedString::from(name), name, on).on_change(
                    act_with(this, move |state, _: &bool, _, _| {
                        if on {
                            state.platforms.retain(|chosen| chosen != name);
                        } else {
                            state.platforms.push(name.into());
                        }
                    }),
                ));
            }
            gpui::div()
                .w(gpui::px(288.))
                .child(group)
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            Col::new()
                .child(Checkbox::new("dead-off", "Unavailable on this plan", false).disabled(true))
                .child(Checkbox::new("dead-on", "Always on for this plan", true).disabled(true))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(
                    Checkbox::new(
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
                col =
                    col.child(Checkbox::new(tint.name(), capitalize(tint.name()), true).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("With a note", |state, this, _| {
            div()
                .w(px(380.))
                .child(
                    Checkbox::new("note", "Share anonymous usage data", state.telemetry)
                        .note("Crash reports only. Never the contents of a document.")
                        .on_change(act_with(this, |state, value: &bool, _, _| {
                            state.telemetry = *value
                        })),
                )
                .into_any_element()
        }),
        Story::new("A checklist", |state, this, _| {
            checklist(state, this).into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(380.))
                .child(
                    FormField::new("Platforms")
                        .hint("The packages this change reaches.")
                        .child(checklist(state, this)),
                )
                .into_any_element()
        }),
    ],
);

/// A run of decisions that share a column, which is what a checkbox is for.
fn checklist(
    state: &crate::Playground,
    this: &gpui::Entity<crate::Playground>,
) -> impl IntoElement {
    Col::new()
        .child(
            Checkbox::new("react", "React", state.newsletter)
                .on_change(act_with(this, |state, value: &bool, _, _| {
                    state.newsletter = *value
                })),
        )
        .child(
            Checkbox::new("flutter", "Flutter", state.telemetry)
                .on_change(act_with(this, |state, value: &bool, _, _| {
                    state.telemetry = *value
                })),
        )
        .child(Checkbox::new("swiftui", "SwiftUI", false))
        .child(Checkbox::new("gpui", "GPUI", true))
}
