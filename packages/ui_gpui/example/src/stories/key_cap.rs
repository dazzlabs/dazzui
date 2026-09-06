//! `Components/KeyCap` — a key, printed.

use dazzui::{
    Button, ControlVariant, KeyCap, KeyCapSize, KeyCapVariant, PreferenceRow, PreferenceSection,
    SearchField,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize, case};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

const SIZES: [KeyCapSize; 3] = [KeyCapSize::Small, KeyCapSize::Medium, KeyCapSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "KeyCap",
    &[
        Story::page("Overview", |state, _, _| {
            let mut variants = Specimens::new();
            for variant in KeyCapVariant::ALL {
                variants = variants.child(
                    Specimen::new(case(variant))
                        .child(KeyCap::new("⌘K").variant(variant))
                        .child(KeyCap::new("⇧⌘P").variant(variant))
                        .child(KeyCap::new("Esc").variant(variant)),
                );
            }

            DocPage::new("KeyCap")
                .description(
                    "The printed form of a key. It is not a control and never reacts to the \
                     pointer — it is a label saying which key does the thing beside it. Its \
                     colours are already in the token graph, so it takes no tint.",
                )
                .child(
                    Section::new("Variants")
                        .description(
                            "`hint` is the quiet one that sits inside a field or beside a menu \
                             row. `strong` is the one in prose, where it has to hold its own \
                             against body copy. `key` is the drawn cap, for a word key that has \
                             to read as a physical key rather than as a symbol.",
                        )
                        .child(variants),
                )
                .child(
                    Section::new("Chords")
                        .description(
                            "A chord can be one cap with the symbols joined, or a cap per key. \
                             One cap is the usual form — it is one shortcut, and three boxes \
                             read as three keys to press in turn.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("joined").child(KeyCap::new("⇧⌘P")))
                                .child(
                                    Specimen::new("separate")
                                        .child(KeyCap::new("⇧"))
                                        .child(KeyCap::new("⌘"))
                                        .child(KeyCap::new("P")),
                                ),
                        ),
                )
                .child(
                    Section::new("Where it lands")
                        .description(
                            "The trailing hint in a search field, the shortcut column of a \
                             menu, the trailing slot of a settings row. Inside a button the \
                             shortcut is *not* a cap — it is the button's own colour at a \
                             lowered opacity, because a cap inside a control reads as a second \
                             control.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("field").child(
                                        div().w(px(320.)).child(
                                            SearchField::new("search", &state.search)
                                                .hint(KeyCap::new("⌘K").size(KeyCapSize::Small)),
                                        ),
                                    ),
                                )
                                .child(
                                    Specimen::new("settings").child(
                                        div().w(px(420.)).child(
                                            PreferenceSection::new().label("Shortcuts").child(
                                                PreferenceRow::new("palette", "Toggle the palette")
                                                    .child(KeyCap::new("⌘K")),
                                            ),
                                        ),
                                    ),
                                )
                                .child(
                                    Specimen::new("button").child(
                                        Button::new("translate", "Translate")
                                            .variant(ControlVariant::Filled)
                                            .shortcut("⌘S"),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Hint", |_, _, _| KeyCap::new("⌘K").into_any_element()),
        Story::new("Strong", |_, _, _| {
            KeyCap::new("⌘K")
                .variant(KeyCapVariant::Strong)
                .into_any_element()
        }),
        Story::new("Key", |_, _, _| {
            KeyCap::new("Esc")
                .variant(KeyCapVariant::Key)
                .into_any_element()
        }),
        Story::new("All variants", |_, _, _| {
            let mut row = Row::new();
            for variant in KeyCapVariant::ALL {
                row = row.child(KeyCap::new("⌘K").variant(variant));
            }
            row.into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new();
            for size in SIZES {
                row =
                    row.child(KeyCap::new(SharedString::from(capitalize(&case(size)))).size(size));
            }
            row.into_any_element()
        }),
        Story::new("A chord", |_, _, _| {
            Row::new()
                .child(KeyCap::new("⇧"))
                .child(KeyCap::new("⌘"))
                .child(KeyCap::new("P"))
                .into_any_element()
        }),
        Story::new("A joined chord", |_, _, _| {
            KeyCap::new("⇧⌘P").into_any_element()
        }),
        Story::new("Word keys", |_, _, _| {
            Row::new()
                .child(KeyCap::new("Esc").variant(KeyCapVariant::Key))
                .child(KeyCap::new("Tab").variant(KeyCapVariant::Key))
                .child(KeyCap::new("Enter").variant(KeyCapVariant::Key))
                .child(KeyCap::new("Space").variant(KeyCapVariant::Key))
                .into_any_element()
        }),
        Story::new("As a search hint", |state, _, _| {
            div()
                .w(px(320.))
                .child(
                    SearchField::new("search", &state.search)
                        .hint(KeyCap::new("⌘K").size(KeyCapSize::Small)),
                )
                .into_any_element()
        }),
        Story::new("In a settings row", |_, _, _| {
            div()
                .w(px(460.))
                .child(
                    PreferenceSection::new()
                        .label("Shortcuts")
                        .child(
                            PreferenceRow::new("palette", "Toggle the palette")
                                .child(KeyCap::new("⌘K")),
                        )
                        .child(
                            PreferenceRow::new("rebuild", "Rebuild tokens")
                                .child(KeyCap::new("⇧⌘B")),
                        ),
                )
                .into_any_element()
        }),
    ],
);
