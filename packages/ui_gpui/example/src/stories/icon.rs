//! `Components/Icon` — the glyph box.

use dazzui::{
    ActionBar, ActiveTheme, Button, ControlVariant, Icon, IconButton, NavItem, Tint, assets::icons,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

/// The set the playground ships as assets — an icon is an SVG the app loads,
/// so a story can only name one that is in the bundle.
const SET: [(&str, &str); 6] = [
    ("star", icons::STAR_16_REGULAR),
    ("pin", icons::PIN_16_REGULAR),
    ("search", icons::SEARCH_16_REGULAR),
    ("settings", icons::SETTINGS_16_REGULAR),
    ("table", icons::TABLE_16_REGULAR),
    ("delete", icons::DELETE_16_REGULAR),
];

pub const PAGE: StoryPage = StoryPage::new(
    "Icon",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Icon")
                .description(
                    "A glyph in a box. The set is Fluent — the same set `packages/ui_react` \
                     pulls from `@fluentui/react-icons` — because the React components name \
                     specific glyphs, and a port drawing its own checkmark would be a different \
                     mark at a different weight in the same box. GPUI loads them as SVG assets, \
                     so the crate ships the ones its components name.",
                )
                .child(
                    Section::new("Size")
                        .description(
                            "The size is a length rather than a scale, because a glyph is sized \
                             to the type beside it: a face's own size is the value to pass, so \
                             the mark and the label share a line.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("lengths").child(
                                        Row::new()
                                            .child(Icon::new(icons::STAR_16_REGULAR).size(px(12.)))
                                            .child(Icon::new(icons::STAR_16_REGULAR).size(px(16.)))
                                            .child(Icon::new(icons::STAR_16_REGULAR).size(px(20.)))
                                            .child(Icon::new(icons::STAR_16_REGULAR).size(px(24.))),
                                    ),
                                )
                                .child(
                                    Specimen::new("faces").child(
                                        Row::new()
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .size(vars.caption_small.size),
                                            )
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .size(vars.label_medium.size),
                                            )
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .size(vars.title_small.size),
                                            ),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("Colour")
                        .description(
                            "The glyph takes the colour it is given, and the components that \
                             hold one hand it their own — a button's label colour, a nav row's \
                             ink. It never carries a tint of its own.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("given").child(
                                        Row::new()
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .color(vars.color_content),
                                            )
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .color(vars.color_content_muted),
                                            )
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .color(vars.color_content_faint),
                                            )
                                            .child(
                                                Icon::new(icons::STAR_16_REGULAR)
                                                    .color(vars.ramp(Tint::Danger).shade(600)),
                                            ),
                                    ),
                                )
                                .child(
                                    Specimen::new("in a control")
                                        .child(
                                            Button::new("filled", "Export")
                                                .variant(ControlVariant::Filled)
                                                .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)),
                                        )
                                        .child(
                                            Button::new("plain", "Export")
                                                .variant(ControlVariant::Plain)
                                                .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)),
                                        ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Default", |_, _, _| {
            Icon::new(icons::STAR_16_REGULAR).into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            Row::new()
                .child(Icon::new(icons::STAR_16_REGULAR).size(px(12.)))
                .child(Icon::new(icons::STAR_16_REGULAR).size(px(16.)))
                .child(Icon::new(icons::STAR_16_REGULAR).size(px(20.)))
                .child(Icon::new(icons::STAR_16_REGULAR).size(px(24.)))
                .into_any_element()
        }),
        Story::new("Sized by a token", |_, _, cx| {
            let vars = cx.theme().vars;
            Row::new()
                .child(Icon::new(icons::STAR_16_REGULAR).size(vars.caption_small.size))
                .child(Icon::new(icons::STAR_16_REGULAR).size(vars.label_medium.size))
                .child(Icon::new(icons::STAR_16_REGULAR).size(vars.title_small.size))
                .into_any_element()
        }),
        Story::new("Given a colour", |_, _, cx| {
            let vars = cx.theme().vars;
            Row::new()
                .child(Icon::new(icons::STAR_16_REGULAR).color(vars.color_content))
                .child(Icon::new(icons::STAR_16_REGULAR).color(vars.color_content_muted))
                .child(Icon::new(icons::STAR_16_REGULAR).color(vars.color_content_faint))
                .child(Icon::new(icons::STAR_16_REGULAR).color(vars.ramp(Tint::Danger).shade(600)))
                .into_any_element()
        }),
        Story::new("The set", |_, _, _| {
            let mut row = Row::new();
            for (_, path) in SET {
                row = row.child(Icon::new(path));
            }
            row.into_any_element()
        }),
        Story::new("In a button", |_, _, _| {
            Button::new("export", "Export")
                .variant(ControlVariant::Filled)
                .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR))
                .into_any_element()
        }),
        Story::new("In a toolbar", |_, _, _| {
            div()
                .w(px(200.))
                .child(
                    ActionBar::new()
                        .child(IconButton::new(
                            "back",
                            "Back",
                            Icon::new(icons::CHEVRON_LEFT_16_REGULAR),
                        ))
                        .child(IconButton::new(
                            "forward",
                            "Forward",
                            Icon::new(icons::CHEVRON_RIGHT_16_REGULAR),
                        ))
                        .child(IconButton::new(
                            "settings",
                            "Settings",
                            Icon::new(icons::SETTINGS_16_REGULAR),
                        )),
                )
                .into_any_element()
        }),
        Story::new("In a nav row", |_, _, _| {
            Col::new()
                .width(px(200.))
                .gap(px(0.))
                .child(
                    NavItem::new("projects", "Projects")
                        .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR))
                        .active(true),
                )
                .child(
                    NavItem::new("settings", "Settings")
                        .icon(Icon::new(icons::SETTINGS_16_REGULAR)),
                )
                .into_any_element()
        }),
    ],
);
