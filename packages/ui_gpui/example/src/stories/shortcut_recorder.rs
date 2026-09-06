//! `Components/ShortcutRecorder` — the field you press a chord into.

use dazzui::{
    ActiveTheme, FormField, KeyCap, PreferenceRow, PreferenceSection, ShortcutRecorder,
    TextFieldState, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, TokenTable, capitalize};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

pub const PAGE: StoryPage = StoryPage::new(
    "ShortcutRecorder",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            DocPage::new("ShortcutRecorder")
                .description(
                    "A field whose value is a key chord: focus it and the next chord you press \
                     becomes the binding. It prints what it holds the way the rest of the system \
                     does — the same symbols a `KeyCap` and a menu's shortcut column use.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Bound, unbound, recording, disabled, and the error state a taken \
                             chord puts it in. Recording is not a value the host sets: it is \
                             what focus means here, so the field says so itself.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("bound").child(bound(
                                    state,
                                    this,
                                    "doc-bound",
                                )))
                                .child(
                                    Specimen::new("unbound")
                                        .child(ShortcutRecorder::new("doc-unbound", "")),
                                )
                                .child(
                                    Specimen::new("error").child(
                                        ShortcutRecorder::new("doc-error", "⌘Q")
                                            .state(TextFieldState::Error),
                                    ),
                                )
                                .child(
                                    Specimen::new("disabled").child(
                                        ShortcutRecorder::new("doc-dead", "⌘K").disabled(true),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("What it prints")
                        .description(
                            "Modifiers in the platform's order, then the key. A word key stays \
                             a word; a symbol key stays a symbol.",
                        )
                        .child(
                            Row::new()
                                .child(ShortcutRecorder::new("doc-a", "⌘K"))
                                .child(ShortcutRecorder::new("doc-b", "⇧⌘K"))
                                .child(ShortcutRecorder::new("doc-c", "⌃⌥⇧⌘K"))
                                .child(ShortcutRecorder::new("doc-d", "⌘↩")),
                        ),
                )
                .child(
                    Section::new("In settings")
                        .description(
                            "The trailing control of a preference row, beside the command it \
                             binds. A conflict is a field in the error state with the reason \
                             under it — the same shape any invalid field takes.",
                        )
                        .child(
                            Col::new()
                                .width(px(500.))
                                .child(in_settings(state, this))
                                .child(conflict()),
                        ),
                )
                .child(
                    Section::new("Tokens")
                        .description("The field's own width.")
                        .child(
                            TokenTable::new()
                                .length("shortcut-recorder.width", vars.shortcut_recorder_width),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bound", |state, this, _| {
            bound(state, this, "bound").into_any_element()
        }),
        Story::new("Unbound", |_, _, _| {
            ShortcutRecorder::new("unbound", "").into_any_element()
        }),
        Story::new("Error state", |_, _, _| {
            ShortcutRecorder::new("error", "⌘Q")
                .state(TextFieldState::Error)
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            ShortcutRecorder::new("dead", "⌘K")
                .disabled(true)
                .into_any_element()
        }),
        Story::new("States", |state, this, _| {
            Col::new()
                .child(bound(state, this, "states-bound"))
                .child(ShortcutRecorder::new("states-unbound", ""))
                .child(ShortcutRecorder::new("states-error", "⌘Q").state(TextFieldState::Error))
                .child(ShortcutRecorder::new("states-dead", "⌘K").disabled(true))
                .into_any_element()
        }),
        Story::new("Modified", |_, _, _| {
            Col::new()
                .child(ShortcutRecorder::new("m-a", "⌘K"))
                .child(ShortcutRecorder::new("m-b", "⇧⌘K"))
                .child(ShortcutRecorder::new("m-c", "⌥⌘K"))
                .child(ShortcutRecorder::new("m-d", "⌃⌥⇧⌘K"))
                .into_any_element()
        }),
        Story::new("Word keys", |_, _, _| {
            Col::new()
                .child(ShortcutRecorder::new("w-a", "⌘↩"))
                .child(ShortcutRecorder::new("w-b", "⌘⌫"))
                .child(ShortcutRecorder::new("w-c", "⇧Space"))
                .into_any_element()
        }),
        Story::new("Against a key cap", |_, _, _| {
            Row::new()
                .child(KeyCap::new("⇧⌘B"))
                .child(ShortcutRecorder::new("against", "⇧⌘B"))
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    ShortcutRecorder::new(tint.name(), "⇧⌘B")
                        .tint(tint)
                        .placeholder(capitalize(tint.name())),
                );
            }
            col.into_any_element()
        }),
        Story::new("Conflicts", |_, _, _| conflict().into_any_element()),
        Story::new("In settings", |state, this, _| {
            in_settings(state, this).into_any_element()
        }),
    ],
);

fn bound(state: &Playground, this: &Entity<Playground>, id: &'static str) -> impl IntoElement {
    ShortcutRecorder::new(SharedString::from(id), state.palette_shortcut.clone()).on_change(
        act_with(this, |state, value: &SharedString, _, _| {
            state.palette_shortcut = value.clone()
        }),
    )
}

/// The error state is what a taken chord looks like: the field turns and the
/// line under it says which command already has it.
fn conflict() -> impl IntoElement {
    div().w(px(320.)).child(
        FormField::new("Rebuild tokens")
            .hint("⌘Q is already bound to Quit.")
            .state(TextFieldState::Error)
            .child(ShortcutRecorder::new("conflict", "⌘Q").state(TextFieldState::Error)),
    )
}

fn in_settings(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    div().w(px(500.)).child(
        PreferenceSection::new()
            .label("Shortcuts")
            .footer("Press a chord in a field to rebind it.")
            .child(
                PreferenceRow::new("palette", "Toggle the palette").child(bound(
                    state,
                    this,
                    "settings-palette",
                )),
            )
            .child(
                PreferenceRow::new("tokens", "Rebuild tokens")
                    .child(ShortcutRecorder::new("settings-tokens", "⇧⌘B")),
            )
            .child(
                PreferenceRow::new("playground", "Open the playground")
                    .child(ShortcutRecorder::new("settings-playground", "")),
            ),
    )
}
