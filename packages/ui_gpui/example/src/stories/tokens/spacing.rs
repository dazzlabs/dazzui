//! `Tokens/Spacing` — the grid every gap and pad comes off.

use dazzui::ActiveTheme;
use gpui::{IntoElement, ParentElement, Styled, div};

use crate::storybook::gallery::{Bar, DocPage, Section};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Spacing",
    &[Story::page("Scale", |_, _, cx| {
        let vars = cx.theme().vars;
        let steps = [
            ("spacing.px", vars.spacing_px),
            ("spacing.0", vars.spacing_0),
            ("spacing.0-5", vars.spacing_0_5),
            ("spacing.1", vars.spacing_1),
            ("spacing.1-5", vars.spacing_1_5),
            ("spacing.2", vars.spacing_2),
            ("spacing.2-5", vars.spacing_2_5),
            ("spacing.3", vars.spacing_3),
            ("spacing.3-5", vars.spacing_3_5),
            ("spacing.4", vars.spacing_4),
            ("spacing.5", vars.spacing_5),
            ("spacing.6", vars.spacing_6),
            ("spacing.7", vars.spacing_7),
            ("spacing.8", vars.spacing_8),
            ("spacing.9", vars.spacing_9),
            ("spacing.10", vars.spacing_10),
            ("spacing.11", vars.spacing_11),
            ("spacing.12", vars.spacing_12),
            ("spacing.14", vars.spacing_14),
            ("spacing.16", vars.spacing_16),
            ("spacing.20", vars.spacing_20),
        ];

        let mut bars = div().flex().flex_col().gap(vars.spacing_2);
        for (name, length) in steps {
            bars = bars.child(Bar::new(name, length));
        }

        DocPage::new("Spacing")
            .description(
                "The one scale, and the one part of the semantic layer that holds values \
                 directly rather than aliasing a primitive: these names *are* the grid steps, \
                 and a base twin under the same names would only be duplication.",
            )
            .child(
                Section::new("Steps")
                    .description(
                        "A value the design lands between two steps is composed out of them \
                         rather than written down — the 18px a dialog body pads by is \
                         `spacing.4 + spacing.0-5`, which this port spells `plus(...)`. A step \
                         and a half says what it is, where `18px` only says what it measures.",
                    )
                    .child(bars),
            )
            .into_any_element()
    })],
);
