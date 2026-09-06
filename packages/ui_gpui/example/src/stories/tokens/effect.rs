//! `Tokens/Effect` — shadow, and the motion curve.

use dazzui::ActiveTheme;
use dazzui::prelude::*;
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Tile, Tiles, TokenTable};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Effect",
    &[Story::page("Shadow", |_, _, cx| {
        let vars = cx.theme().vars;
        let steps = [
            ("shadow.2xs", vars.shadow_2xs),
            ("shadow.xs", vars.shadow_xs),
            ("shadow.sm", vars.shadow_sm),
            ("shadow.md", vars.shadow_md),
            ("shadow.lg", vars.shadow_lg),
            ("shadow.xl", vars.shadow_xl),
            ("shadow.2xl", vars.shadow_2xl),
        ];

        let mut tiles = Tiles::new(px(200.));
        for (name, shadow) in steps {
            tiles = tiles.child(
                Tile::new(name).child(
                    div()
                        .w_full()
                        .h(vars.spacing_12)
                        .m(vars.spacing_2)
                        .rounded(vars.radius_medium)
                        .bg(vars.color_surface)
                        .border(vars.spacing_px)
                        .border_color(vars.color_border)
                        .token_shadow(shadow),
                ),
            );
        }

        DocPage::new("Effect")
            .description(
                "Seven shadow steps and one motion curve. A shadow is stored as its layers \
                 rather than as a finished blur, because a token describes a CSS shadow — blur \
                 *diameter* and all — and each platform draws it in its own units.",
            )
            .child(
                Section::new("Shadow")
                    .description(
                        "The hairline lift a filled button carries is the smallest step; a \
                         popup takes the middle of the scale; a dialog the top of it. Nothing in \
                         the system uses a shadow to say something is interactive — that is the \
                         recipe's job.",
                    )
                    .child(tiles),
            )
            .child(
                Section::new("Motion")
                    .description(
                        "One duration and one curve, which every state change animates on. A \
                         component that wanted its own timing would be saying its states mean \
                         something different from every other component's.",
                    )
                    .child(
                        TokenTable::new()
                            .row(
                                "motion.duration",
                                format!("{}ms", vars.motion_duration * 1000.),
                            )
                            .row("motion.easing", format!("{:?}", vars.motion_easing)),
                    ),
            )
            .into_any_element()
    })],
);
