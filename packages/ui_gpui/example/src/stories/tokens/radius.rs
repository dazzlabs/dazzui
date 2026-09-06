//! `Tokens/Radius` — one corner scale, and where each step lands.

use dazzui::{ActiveTheme, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Tile, Tiles};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Radius",
    &[Story::page("Scale", |_, _, cx| {
        let vars = cx.theme().vars;
        let steps = [
            ("radius.none", vars.radius_none, "Square"),
            ("radius.tiny", vars.radius_tiny, "A badge, a swatch"),
            ("radius.small", vars.radius_small, "Controls, nav rows"),
            ("radius.medium", vars.radius_medium, "Cards, popups"),
            ("radius.large", vars.radius_large, "The large control"),
            ("radius.big", vars.radius_big, "A window"),
            ("radius.full", vars.radius_full, "A pill"),
        ];

        let mut tiles = Tiles::new(px(180.));
        for (name, radius, note) in steps {
            tiles = tiles.child(
                Tile::new(name).note(note).child(
                    div()
                        .w_full()
                        .h(vars.spacing_16)
                        .rounded(radius)
                        .bg(vars.ramp(Tint::Primary).shade(500)),
                ),
            );
        }

        DocPage::new("Radius")
            .description(
                "One scale per axis, so there is no second radius to alias by accident. The \
                 corners are the AppKit metrics the design is drawn to — which is why Bright's \
                 pill controls are a theme re-pointing this scale rather than a component \
                 choosing a different one.",
            )
            .child(
                Section::new("Steps")
                    .description("Each drawn on the surface a component of that corner sits on.")
                    .child(tiles),
            )
            .into_any_element()
    })],
);
