//! `Tokens/Themes` — the four, and the argument for how they are carried.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Badge, BadgeSize, Button, ControlVariant, Divider, KeyCap, Progress, ThemeName,
    Tint, themed,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Ramp, Section};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Themes",
    &[
        Story::page("Four at once", |_, _, cx| {
            let vars = cx.theme().vars;
            let mut cards = div().flex().flex_wrap().gap(vars.spacing_4);
            for name in ThemeName::ALL {
                cards = cards.child(themed(name, card(name)));
            }

            DocPage::new("Themes")
                .description(
                    "Four themes, one component tree. A theme here is a value scoped to a \
                     subtree with `themed(...)` rather than a process-wide mode, so the four \
                     cards below are the same view rendered four times, each resolving against \
                     its own variables — and each theme is a `static` a component borrows rather \
                     than a value it rebuilds.",
                )
                .child(
                    Section::new("The four")
                        .description(
                            "Studio Light is the baseline the others are modes over: Studio \
                             Dark is the same skeleton on a near-black canvas, Bright is warm \
                             paper with ink navy and an acid marker, and Bright Dark takes that \
                             acid green as the accent itself.",
                        )
                        .child(cards),
                )
                .into_any_element()
        }),
        Story::page("Ramps", |_, _, cx| {
            let outer = cx.theme().vars;
            let mut page = DocPage::new("Ramps in four themes").description(
                "The same six ramps, under each theme in turn. This is the page that says why a \
                 component must never name a shade: the ladder under the recipe is a different \
                 ladder in every block below, and the recipe table above them is untouched.",
            );

            for name in ThemeName::ALL {
                let vars = name.variables();
                let mut ramps = div()
                    .flex()
                    .flex_col()
                    .gap(outer.spacing_4)
                    .p(outer.spacing_4)
                    .rounded(outer.radius_medium)
                    .bg(vars.color_surface);
                for tint in Tint::ALL {
                    ramps = ramps.child(Ramp::new(tint.name(), vars.ramp(tint)));
                }
                page = page.child(Section::new(name.label()).child(themed(name, ramps)));
            }

            page.into_any_element()
        }),
    ],
);

/// One theme's own little window: the surfaces stacked, a control on each, and
/// the accent that separates the families.
fn card(name: ThemeName) -> impl IntoElement {
    let vars = name.variables();

    div()
        .flex()
        .flex_col()
        .w(px(320.))
        .gap(vars.spacing_3)
        .p(vars.spacing_4)
        .rounded(vars.frame_window_radius)
        .bg(vars.color_surface)
        .border(vars.spacing_px)
        .border_color(vars.color_border)
        .token_shadow(vars.shadow_sm)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .type_style(vars.label_large)
                        .text_color(vars.color_content)
                        .child(name.label()),
                )
                .child(Badge::new(name.attribute()).size(BadgeSize::Small)),
        )
        .child(
            div()
                .type_style(vars.body_small)
                .text_color(vars.color_content_muted)
                .child("The quick brown fox jumps over the lazy dog."),
        )
        .child(
            Row::new()
                .child(
                    Button::new(
                        SharedString::from(format!("{}-primary", name.attribute())),
                        "Primary",
                    )
                    .variant(ControlVariant::Filled),
                )
                .child(
                    Button::new(
                        SharedString::from(format!("{}-quiet", name.attribute())),
                        "Quiet",
                    )
                    .variant(ControlVariant::Plain),
                ),
        )
        .child(Divider::horizontal())
        .child(
            Row::new()
                .child(
                    Badge::new("Passed")
                        .size(BadgeSize::Small)
                        .tint(Tint::Success),
                )
                .child(
                    Badge::new("Failed")
                        .size(BadgeSize::Small)
                        .tint(Tint::Danger),
                )
                .child(KeyCap::new("⌘S")),
        )
        .child(Progress::new(60.).gradient(true))
}
