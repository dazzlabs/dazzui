//! `Tokens/Color` — the ramps, and the roles that alias them.

use dazzui::{ActiveTheme, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Ramp, Section, Swatch, Tiles};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Color",
    &[
        Story::page("Palette", |_, _, cx| {
            let vars = cx.theme().vars;
            let mut ramps = div().flex().flex_col().gap(vars.spacing_5);
            for tint in Tint::ALL {
                ramps = ramps.child(Ramp::new(tint.name(), vars.ramp(tint)));
            }

            DocPage::new("Palette")
                .description(
                    "Six ramps, eleven steps each, in the theme selected above. A ramp is a \
                     **role ladder, not a lightness ramp** — read Bright's primary before \
                     assuming otherwise: its 700 is *lighter* than its 600, because that theme's \
                     primary action is ink navy whose hover lifts. A step is only ever reached \
                     by a recipe, so what a step means is the role that recipe gives it.",
                )
                .child(
                    Section::new("Ramps")
                        .description(
                            "A component never names a step. This page is the one place they \
                             are printed, and it is a reference rather than a menu: a shade \
                             written into a component would be a lie rather than merely a \
                             duplicate, because the next theme re-points the ladder underneath \
                             it.",
                        )
                        .child(ramps),
                )
                .into_any_element()
        }),
        Story::page("Roles", |_, _, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Roles")
                .description(
                    "The named colours a component actually reads. Each theme decides its own \
                     paper, ink and edges — those are the one kind of raw value the semantic \
                     layer holds — and everything else aliases a ramp.",
                )
                .child(
                    Section::new("Surfaces")
                        .description(
                            "The canvas the window sits on, the paper a card is, and the \
                             grooves under it. Disabled controls flatten to `surface-sunken`, \
                             which is why it has to read as a hole rather than as a quieter card.",
                        )
                        .child(
                            Tiles::new(px(200.))
                                .child(
                                    Swatch::new("canvas", vars.color_canvas)
                                        .note("The window's ground"),
                                )
                                .child(Swatch::new("surface", vars.color_surface).note("Paper"))
                                .child(
                                    Swatch::new("surface-muted", vars.color_surface_muted)
                                        .note("Sidebar, titlebar"),
                                )
                                .child(
                                    Swatch::new("surface-sunken", vars.color_surface_sunken)
                                        .note("The groove"),
                                )
                                .child(Swatch::new("surface-subtle", vars.color_surface_subtle))
                                .child(Swatch::new("surface-inset", vars.color_surface_inset))
                                .child(
                                    Swatch::new("surface-raised", vars.color_surface_raised)
                                        .note("Popups"),
                                )
                                .child(Swatch::new("surface-chrome", vars.color_surface_chrome)),
                        ),
                )
                .child(
                    Section::new("Ink")
                        .description(
                            "The ladder body copy, labels and hints are set in. Every step is \
                             held to a WCAG contrast floor in every theme by a check in the \
                             token build, which fails rather than shipping a role that landed \
                             under it.",
                        )
                        .child(
                            Tiles::new(px(200.))
                                .child(Swatch::new("content", vars.color_content).note("Body"))
                                .child(Swatch::new(
                                    "content-secondary",
                                    vars.color_content_secondary,
                                ))
                                .child(
                                    Swatch::new("content-nav", vars.color_content_nav)
                                        .note("Idle nav rows"),
                                )
                                .child(
                                    Swatch::new("content-muted", vars.color_content_muted)
                                        .note("Hints"),
                                )
                                .child(Swatch::new("content-subtle", vars.color_content_subtle))
                                .child(
                                    Swatch::new("content-faint", vars.color_content_faint)
                                        .note("Disabled"),
                                )
                                .child(
                                    Swatch::new("on-accent", vars.color_on_accent)
                                        .note("On a filled surface"),
                                ),
                        ),
                )
                .child(
                    Section::new("Edges")
                        .description(
                            "One hairline, one stronger, one quieter. A separator draws \
                             `stroke.hairline` and halves on Retina; a control outline draws \
                             `stroke.control` and stays put.",
                        )
                        .child(
                            Tiles::new(px(200.))
                                .child(Swatch::new("border", vars.color_border))
                                .child(Swatch::new("border-strong", vars.color_border_strong))
                                .child(Swatch::new("border-muted", vars.color_border_muted)),
                        ),
                )
                .into_any_element()
        }),
    ],
);
