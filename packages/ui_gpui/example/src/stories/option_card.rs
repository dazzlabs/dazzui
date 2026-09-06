//! `Components/OptionCard` — a card you can choose.

use dazzui::{OptionCard, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Tiles, capitalize};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

const PLATFORMS: [(&str, &str, &str); 4] = [
    ("react", "React", "Base UI under the drawing."),
    ("flutter", "Flutter", "One theme value, no cascade."),
    ("swiftui", "SwiftUI", "Per-subtree themes."),
    ("gpui", "GPUI", "A seed and a const fn."),
];

pub const PAGE: StoryPage = StoryPage::new(
    "OptionCard",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("OptionCard")
                .description(
                    "The choosable card: a radio's job in a card's shape, for a decision that \
                     needs a sentence of explanation rather than a word. It is interactive, so \
                     unlike `Card` it goes through `pressable` and answers the pointer.",
                )
                .child(
                    Section::new("Selection")
                        .description(
                            "Selection shifts weight as well as colour — the title goes to the \
                             strong label face, the edge takes the tint, and the fill takes the \
                             tint's wash. That is what makes a chosen card readable at a glance \
                             in a grid of four.",
                        )
                        .child(appearance(state, this)),
                )
                .child(
                    Section::new("Shapes")
                        .description(
                            "The description is optional: without one the card is a wide radio \
                             row, which is the right shape for two short answers. Disabled \
                             flattens to the groove, like every other control.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(
                                    div()
                                        .w(px(140.))
                                        .child(OptionCard::new("light", "Light").selected(true)),
                                )
                                .child(div().w(px(140.)).child(OptionCard::new("dark", "Dark")))
                                .child(
                                    div().w(px(220.)).child(
                                        OptionCard::new("enterprise", "Enterprise")
                                            .description("Not on this plan.")
                                            .disabled(true),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("In a grid")
                        .description(
                            "Where the options are more than two, they go in a grid that fills \
                             the width rather than in a row that runs off it.",
                        )
                        .child(grid()),
                )
                .into_any_element()
        }),
        Story::new("Selected", |_, _, _| {
            div()
                .w(px(240.))
                .child(
                    OptionCard::new("automatic", "Automatic")
                        .description("Follow the system appearance.")
                        .selected(true),
                )
                .into_any_element()
        }),
        Story::new("Unselected", |_, _, _| {
            div()
                .w(px(240.))
                .child(OptionCard::new("manual", "Manual").description("Pick a theme and keep it."))
                .into_any_element()
        }),
        Story::new("A group", |state, this, _| {
            appearance(state, this).into_any_element()
        }),
        Story::new("Without a description", |_, _, _| {
            Row::new()
                .child(
                    div()
                        .w(px(140.))
                        .child(OptionCard::new("light", "Light").selected(true)),
                )
                .child(div().w(px(140.)).child(OptionCard::new("dark", "Dark")))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .w(px(220.))
                .child(
                    OptionCard::new("enterprise", "Enterprise")
                        .description("Not on this plan.")
                        .disabled(true),
                )
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut row = Row::new().align_top(true);
            for tint in Tint::ALL {
                row = row.child(
                    div().w(px(130.)).child(
                        OptionCard::new(tint.name(), capitalize(tint.name()))
                            .selected(true)
                            .tint(tint),
                    ),
                );
            }
            row.into_any_element()
        }),
        Story::new("In a grid", |_, _, _| grid().into_any_element()),
    ],
);

fn appearance(
    state: &crate::Playground,
    this: &gpui::Entity<crate::Playground>,
) -> impl IntoElement {
    Row::new()
        .align_top(true)
        .child(
            div().w(px(220.)).child(
                OptionCard::new("auto", "Automatic")
                    .description("Follow the system appearance.")
                    .selected(state.appearance_card == "auto")
                    .on_select(act(this, |state, _, _| {
                        state.appearance_card = "auto".into()
                    })),
            ),
        )
        .child(
            div().w(px(220.)).child(
                OptionCard::new("manual", "Manual")
                    .description("Pick a theme and keep it.")
                    .selected(state.appearance_card == "manual")
                    .on_select(act(this, |state, _, _| {
                        state.appearance_card = "manual".into()
                    })),
            ),
        )
}

fn grid() -> impl IntoElement {
    let mut tiles = Tiles::new(px(200.));
    for (id, name, note) in PLATFORMS {
        tiles = tiles.child(
            OptionCard::new(SharedString::from(format!("grid-{id}")), name)
                .description(note)
                .selected(id == "gpui"),
        );
    }
    div().w(px(640.)).child(tiles)
}
