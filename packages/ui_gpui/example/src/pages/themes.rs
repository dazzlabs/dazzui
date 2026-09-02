use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Badge, BadgeSize, Button, ControlVariant, Divider, KeyCap, Progress, ThemeName,
    Tint, themed,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, Styled, div};

use crate::Playground;
use crate::gallery::{Col, PageTitle, Row, Specimen};
use crate::pages::controls::capitalize;

/// The four themes at once.
///
/// This is the page that argues for the design of the thing: a theme is a
/// value scoped to a subtree with [`themed`], not a process-wide mode, so
/// four of them can sit side by side and every component inside each one
/// resolves against its own variables. The cards below are the same view
/// rendered four times.
pub fn themes(
    _state: &Playground,
    _this: &Entity<Playground>,
    cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let vars = cx.theme().vars;

    let cards = div()
        .flex()
        .flex_wrap()
        .gap(vars.spacing_4)
        .children(ThemeName::ALL.map(|name| themed(name, theme_card(name))));

    let ramps = Col::new()
        .gap(vars.spacing_2)
        .children(Tint::ALL.map(|tint| {
            let ramp = vars.ramp(tint);
            Row::new()
                .gap(vars.spacing_1)
                .child(
                    div()
                        .w(vars.spacing_16)
                        .type_style(vars.caption_small)
                        .text_color(vars.color_content_muted)
                        .child(tint.name()),
                )
                .children(ramp.steps().iter().map(|(_, color)| {
                    div()
                        .w(vars.spacing_8)
                        .h(vars.spacing_6)
                        .rounded(vars.radius_tiny)
                        .bg(*color)
                        .border(vars.spacing_px)
                        .border_color(vars.color_border)
                }))
        }));

    vec![
        PageTitle::new(
            "Themes",
            "Four themes, one component tree — each card scopes its own with themed().",
        )
        .into_any_element(),
        cards.into_any_element(),
        Specimen::new("Ramps")
            .note("a role ladder, not a lightness ramp — read the ink ladder before assuming")
            .child(ramps)
            .into_any_element(),
    ]
}

/// One theme's own little window: the surfaces stacked, a control on each,
/// and the accent that separates the families.
fn theme_card(name: ThemeName) -> impl IntoElement {
    let vars = name.variables();

    div()
        .flex()
        .flex_col()
        .w(vars.spacing_20 * 4.0)
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
                        gpui::SharedString::from(format!("{}-primary", name.attribute())),
                        "Primary",
                    )
                    .variant(ControlVariant::Filled),
                )
                .child(
                    Button::new(
                        gpui::SharedString::from(format!("{}-quiet", name.attribute())),
                        "Quiet",
                    )
                    .variant(ControlVariant::Plain),
                ),
        )
        .child(Divider::horizontal())
        .child(
            Row::new()
                .child(
                    Badge::new(capitalize("passed"))
                        .size(BadgeSize::Small)
                        .tint(Tint::Success),
                )
                .child(
                    Badge::new(capitalize("failed"))
                        .size(BadgeSize::Small)
                        .tint(Tint::Danger),
                )
                .child(KeyCap::new("⌘S")),
        )
        .child(Progress::new(60.).gradient(true))
}
