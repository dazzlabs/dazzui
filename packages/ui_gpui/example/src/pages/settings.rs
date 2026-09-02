use dazzui::{
    ActiveTheme, KeyCap, PreferenceGroup, PreferenceRow, PreferenceSection, Select,
    ShortcutRecorder, Switch, TextFieldSize,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, SharedString};

use crate::gallery::PageTitle;
use crate::{Playground, act_with};

/// A preferences screen, which is the composition most of these components
/// were drawn for: rows of one decision each, grouped under a label, with the
/// control pinned to the trailing edge.
pub fn settings(
    state: &Playground,
    this: &Entity<Playground>,
    cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let _vars = cx.theme().vars;

    vec![
        PageTitle::new(
            "Settings",
            "One decision per row, and the control sits where the eye lands last.",
        )
        .into_any_element(),
        // A group is the level above a section: sections stay one size
        // everywhere, and the title outranks them rather than being a section
        // shrunk to fit underneath one.
        PreferenceGroup::new("Application")
            .description("Settings that apply to every window.")
            .child(
                PreferenceSection::new()
                    .label("General")
                    .child(
                        PreferenceRow::new("launch", "Launch at login").child(
                            Switch::new("launch-switch", state.launch)
                                .on_change(act_with(this, |state, checked: &bool, _, _| {
                                    state.launch = *checked
                                })),
                        ),
                    )
                    .child(
                        PreferenceRow::new("updates", "Check for updates")
                            .subtitle("Downloads in the background and asks before installing.")
                            .child(Switch::new("updates-switch", state.updates).on_change(
                                act_with(this, |state, checked: &bool, _, _| {
                                    state.updates = *checked
                                }),
                            )),
                    )
                    .child(
                        PreferenceRow::new("sounds", "Play sounds").child(
                            Switch::new("sounds-switch", state.sounds)
                                .on_change(act_with(this, |state, checked: &bool, _, _| {
                                    state.sounds = *checked
                                })),
                        ),
                    ),
            )
            .child(
                PreferenceSection::new()
                    .label("Appearance")
                    .footer("Automatic follows the system between light and dark.")
                    .child(
                        PreferenceRow::new("theme", "Theme").child(
                            Select::new("appearance", state.appearance.clone())
                                .size(TextFieldSize::Small)
                                .option("auto", "Automatic")
                                .option("light", "Light")
                                .option("dark", "Dark")
                                .on_select(act_with(this, |state, value: &SharedString, _, _| {
                                    state.appearance = value.clone()
                                })),
                        ),
                    ),
            )
            .into_any_element(),
        // A section on its own, with no group over it.
        PreferenceSection::new()
            .label("Shortcuts")
            .child(
                PreferenceRow::new("palette", "Toggle the palette").child(
                    ShortcutRecorder::new("palette-shortcut", state.palette_shortcut.clone())
                        .on_change(act_with(this, |state, value: &SharedString, _, _| {
                            state.palette_shortcut = value.clone()
                        })),
                ),
            )
            .child(PreferenceRow::new("rebuild", "Rebuild tokens").child(KeyCap::new("⇧⌘B")))
            .into_any_element(),
    ]
}
