//! `Components/Card` — a surface, and nothing that answers the pointer.

use dazzui::prelude::*;
use dazzui::{
    ActionBar, ActionBarAlign, ActiveTheme, Button, Card, CardSize, CardVariant, ControlVariant,
    ThemeVariables, Tint,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, capitalize, case};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Card",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut variants = Row::new().align_top(true);
            for variant in CardVariant::ALL {
                variants = variants.child(sample(vars, variant));
            }

            let mut tints = Row::new();
            for tint in Tint::ALL {
                tints = tints.child(
                    Card::new()
                        .variant(CardVariant::Tinted)
                        .tint(tint)
                        .child(div().w(px(88.)).child(capitalize(tint.name()))),
                );
            }

            DocPage::new("Card")
                .description(
                    "A surface with a corner and a pad. It is not interactive and stays out of \
                     `pressable`: a panel that lit up under the pointer would read as something \
                     you can press. Where a card has to be selectable, that is an `OptionCard`.",
                )
                .child(
                    Section::new("Variants")
                        .description(
                            "`raised` is the paper card on the canvas; `sunken` is the groove a \
                             well or a code block sits in; `outlined` is the drawn edge with no \
                             fill; `tinted` is the wash that carries a status.",
                        )
                        .child(variants),
                )
                .child(
                    Section::new("Tints")
                        .description(
                            "Only `tinted` reads a ramp — the others are the theme's own surface \
                             roles, so a tint leaves them where they are. The wash is an alpha of \
                             the tint rather than an opaque step, which is what lets the same \
                             card sit correctly on paper and on a near-black canvas.",
                        )
                        .child(tints),
                )
                .child(
                    Section::new("Compositions")
                        .description(
                            "A card holds a heading, its detail and at most one action row. A \
                             sunken card inside a raised one is the well pattern — the groove \
                             reads as inside the panel rather than as a second panel.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(with_an_action(vars))
                                .child(nested(vars)),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Raised", |_, _, cx| {
            sample(cx.theme().vars, CardVariant::Raised).into_any_element()
        }),
        Story::new("Sunken", |_, _, cx| {
            sample(cx.theme().vars, CardVariant::Sunken).into_any_element()
        }),
        Story::new("Outlined", |_, _, cx| {
            sample(cx.theme().vars, CardVariant::Outlined).into_any_element()
        }),
        Story::new("Tinted", |_, _, cx| {
            sample(cx.theme().vars, CardVariant::Tinted).into_any_element()
        }),
        Story::new("Variants", |_, _, cx| {
            let vars = cx.theme().vars;
            let mut row = Row::new().align_top(true);
            for variant in CardVariant::ALL {
                row = row.child(sample(vars, variant));
            }
            row.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut row = Row::new();
            for tint in Tint::ALL {
                row = row.child(
                    Card::new()
                        .variant(CardVariant::Tinted)
                        .tint(tint)
                        .child(div().w(px(96.)).child(capitalize(tint.name()))),
                );
            }
            row.into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new().align_top(true);
            for size in CardSize::ALL {
                row = row.child(
                    Card::new()
                        .size(size)
                        .child(div().w(px(96.)).child(capitalize(&case(size)))),
                );
            }
            row.into_any_element()
        }),
        Story::new("With an action", |_, _, cx| {
            with_an_action(cx.theme().vars).into_any_element()
        }),
        Story::new("As a well", |_, _, cx| {
            let vars = cx.theme().vars;
            div()
                .w(px(320.))
                .child(
                    Card::new().variant(CardVariant::Sunken).child(
                        div()
                            .type_style(crate::storybook::gallery::code(vars.body_small))
                            .child("cargo run -p dazzui_playground"),
                    ),
                )
                .into_any_element()
        }),
        Story::new("Nested", |_, _, cx| {
            nested(cx.theme().vars).into_any_element()
        }),
    ],
);

/// One card of each variant, with the same thing inside it.
fn sample(vars: &'static ThemeVariables, variant: CardVariant) -> impl IntoElement {
    div()
        .w(px(180.))
        .child(Card::new().variant(variant).child(blurb(
            vars,
            capitalize(&case(variant)),
            "A surface.",
        )))
}

fn with_an_action(vars: &'static ThemeVariables) -> impl IntoElement {
    div().w(px(320.)).child(
        Card::new()
            .child(blurb(
                vars,
                "Tokens are stale",
                "packages/ds changed after the last build.",
            ))
            .child(
                ActionBar::new()
                    .align(ActionBarAlign::End)
                    .child(Button::new("card-rebuild", "Rebuild").variant(ControlVariant::Filled)),
            ),
    )
}

fn nested(vars: &'static ThemeVariables) -> impl IntoElement {
    div().w(px(340.)).child(
        Card::new()
            .variant(CardVariant::Sunken)
            .child(blurb(vars, "Queue", "Two jobs waiting."))
            .child(Card::new().child(blurb(vars, "aurora", "main · 2m 14s")))
            .child(Card::new().child(blurb(vars, "beacon", "release/2.1 · queued"))),
    )
}

fn blurb(
    vars: &'static ThemeVariables,
    title: impl Into<SharedString>,
    note: impl Into<SharedString>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(vars.spacing_1)
        .child(
            div()
                .type_style(vars.label_medium)
                .text_color(vars.color_content)
                .child(title.into()),
        )
        .child(
            div()
                .type_style(vars.caption_small)
                .text_color(vars.color_content_muted)
                .child(note.into()),
        )
}
