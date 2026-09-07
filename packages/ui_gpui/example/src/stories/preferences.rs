//! `Components/Preferences` — the settings column, and the group, the section
//! and the row that run down it.

use dazzui::{
    Button, ControlVariant, Icon, KeyCap, PreferenceGroup, PreferenceRow, PreferenceSection,
    Preferences, Select, Switch, Tint, assets::icons,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

pub const PAGE: StoryPage = StoryPage::new(
    "Preferences",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("Preferences")
                .description(
                    "A settings screen is one decision per row, grouped under a label, with the \
                     control pinned to the trailing edge — where the eye lands last. A column \
                     is the root: it caps the width and spaces the groups. Under it, a group is \
                     the level above a section, a section is a run of rows under one heading, \
                     and a row is the decision.",
                )
                .child(
                    Section::new("Rows")
                        .description(
                            "A row is a title, optionally a subtitle under it, optionally a \
                             leading glyph, and optionally a trailing control — the row's own \
                             children. A row with an `on_open` is one you can click, which is \
                             what makes it a way in rather than a fact.",
                        )
                        .child(shapes()),
                )
                .child(
                    Section::new("What sits at the trailing edge")
                        .description(
                            "A switch for a decision that takes effect at once, a select for \
                             one of several, a key cap for something bound elsewhere, a button \
                             for an action. Only one of them per row.",
                        )
                        .child(controls(state, this)),
                )
                .child(
                    Section::new("Sections and groups")
                        .description(
                            "A section carries a label above its rows and an optional footer \
                             under them. A group is the level above: sections stay one size \
                             everywhere, and the group's title outranks them rather than being a \
                             section shrunk to fit underneath one.",
                        )
                        .child(grouped(state, this)),
                )
                .into_any_element()
        }),
        Story::new("A section", |state, this, _| {
            general(state, this).into_any_element()
        }),
        Story::new("Row shapes", |_, _, _| shapes().into_any_element()),
        Story::new("Controls", |state, this, _| {
            controls(state, this).into_any_element()
        }),
        Story::new("Section headings", |_, _, _| {
            Preferences::new()
                .child(
                    PreferenceSection::new()
                        .label("Appearance")
                        .footer("Automatic follows the system between light and dark.")
                        .child(PreferenceRow::new("theme", "Theme").child(KeyCap::new("⌘,"))),
                )
                .into_any_element()
        }),
        Story::new("Unlabelled", |state, this, _| {
            Preferences::new()
                .child(
                    PreferenceSection::new()
                        .child(
                            PreferenceRow::new("launch", "Launch at login").child(
                                Switch::new("launch", state.launch)
                                    .on_change(act_with(this, |state, value: &bool, _, _| {
                                        state.launch = *value
                                    })),
                            ),
                        )
                        .child(
                            PreferenceRow::new("sounds", "Play sounds").child(
                                Switch::new("sounds", state.sounds)
                                    .on_change(act_with(this, |state, value: &bool, _, _| {
                                        state.sounds = *value
                                    })),
                            ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("A group", |state, this, _| {
            grouped(state, this).into_any_element()
        }),
        Story::new("A settings page", |state, this, _| {
            Preferences::new()
                .child(grouped(state, this))
                .child(
                    PreferenceSection::new().label("Danger zone").child(
                        PreferenceRow::new("reset", "Reset every preference")
                            .subtitle(
                                "Windows, shortcuts and appearance go back to their defaults.",
                            )
                            .child(
                                Button::new("reset", "Reset")
                                    .variant(ControlVariant::Tinted)
                                    .tint(Tint::Danger),
                            ),
                    ),
                )
                .into_any_element()
        }),
    ],
);

fn general(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    Preferences::new().child(
        PreferenceSection::new()
            .label("General")
            .child(
                PreferenceRow::new("launch", "Launch at login").child(
                    Switch::new("general-launch", state.launch)
                        .on_change(act_with(this, |state, value: &bool, _, _| {
                            state.launch = *value
                        })),
                ),
            )
            .child(
                PreferenceRow::new("updates", "Check for updates")
                    .subtitle("Downloads in the background and asks before installing.")
                    .child(
                        Switch::new("general-updates", state.updates)
                            .on_change(act_with(this, |state, value: &bool, _, _| {
                                state.updates = *value
                            })),
                    ),
            ),
    )
}

fn shapes() -> impl IntoElement {
    Preferences::new().child(
        PreferenceSection::new()
            .label("Rows")
            .child(PreferenceRow::new("title", "Title only"))
            .child(
                PreferenceRow::new("subtitle", "With a subtitle")
                    .subtitle("The line that explains it."),
            )
            .child(
                PreferenceRow::new("icon", "With an icon")
                    .icon(Icon::new(icons::SETTINGS_16_REGULAR)),
            )
            .child(PreferenceRow::new("control", "With a control").child(KeyCap::new("⇧⌘B")))
            .child(
                PreferenceRow::new("open", "That opens something")
                    .subtitle("A row with an action is a row you can click.")
                    .on_open(|_, _| {}),
            ),
    )
}

fn controls(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    Preferences::new().child(
        PreferenceSection::new()
            .label("Controls")
            .child(
                PreferenceRow::new("theme", "Theme").child(
                    Select::new("appearance", state.appearance.clone())
                        .size(dazzui::TextFieldSize::Small)
                        .option("auto", "Automatic")
                        .option("light", "Light")
                        .option("dark", "Dark")
                        .on_select(act_with(this, |state, value: &SharedString, _, _| {
                            state.appearance = value.clone()
                        })),
                ),
            )
            .child(
                PreferenceRow::new("sounds", "Play sounds").child(
                    Switch::new("controls-sounds", state.sounds)
                        .on_change(act_with(this, |state, value: &bool, _, _| {
                            state.sounds = *value
                        })),
                ),
            )
            .child(PreferenceRow::new("rebuild", "Rebuild tokens").child(KeyCap::new("⇧⌘B")))
            .child(PreferenceRow::new("cache", "Cache").child(Button::new("clear", "Clear"))),
    )
}

fn grouped(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    Preferences::new().child(
        PreferenceGroup::new("Application")
            .description("Settings that apply to every window.")
            .child(general(state, this))
            .child(
                PreferenceSection::new()
                    .label("Shortcuts")
                    .child(
                        PreferenceRow::new("palette", "Toggle the palette")
                            .child(KeyCap::new("⌘K")),
                    )
                    .child(
                        PreferenceRow::new("tokens", "Rebuild tokens").child(KeyCap::new("⇧⌘B")),
                    ),
            ),
    )
}
