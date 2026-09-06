//! `Components/Divider` — one device pixel, in two orientations and two tones.

use dazzui::prelude::*;
use dazzui::{
    ActionBar, ActiveTheme, Card, Divider, DividerTone, Icon, IconButton, assets::icons,
};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Col, Row, separator};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Divider",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Divider")
                .description(
                    "A rule. It is one *device* pixel rather than one logical one: the hairline \
                     halves on a Retina display, the way `base.css` does with its \
                     `min-resolution` query, so a separator stays as thin as the screen can \
                     draw it. A drawn control outline is a different thing and reads \
                     `stroke.control` instead.",
                )
                .child(
                    Section::new("Tones")
                        .description(
                            "`default` is the ordinary border; `muted` is the stronger one, for \
                             the rule that has to survive on a busier surface. Both are semantic \
                             roles rather than steps off a ramp, so a theme moves them together.",
                        )
                        .child(
                            Col::new()
                                .width(px(420.))
                                .child(Divider::horizontal())
                                .child(Divider::horizontal().tone(DividerTone::Muted)),
                        ),
                )
                .child(
                    Section::new("Orientation")
                        .description(
                            "A horizontal rule fills its parent. A vertical one stretches to the \
                             row it sits in under CSS; here a row of intrinsically sized \
                             children has to give the rule the height of the controls beside it.",
                        )
                        .child(div().w(px(220.)).child(toolbar(vars.control_tiny_size))),
                )
                .child(
                    Section::new("Between rows")
                        .description(
                            "A list separates its rows with a rule between them rather than \
                             under each, so the run does not end in a line hanging off the last \
                             entry.",
                        )
                        .child(rows(cx)),
                )
                .into_any_element()
        }),
        Story::new("Horizontal", |_, _, _| {
            div()
                .w(px(320.))
                .child(Divider::horizontal())
                .into_any_element()
        }),
        Story::new("Muted", |_, _, _| {
            div()
                .w(px(320.))
                .child(Divider::horizontal().tone(DividerTone::Muted))
                .into_any_element()
        }),
        Story::new("Tones", |_, _, _| {
            Col::new()
                .width(px(320.))
                .child(Divider::horizontal())
                .child(Divider::horizontal().tone(DividerTone::Muted))
                .into_any_element()
        }),
        Story::new("Vertical", |_, _, cx| {
            let vars = cx.theme().vars;
            Row::new()
                .child(div().child("Left"))
                .child(separator(vars.control_small_size))
                .child(div().child("Right"))
                .into_any_element()
        }),
        Story::new("In a toolbar", |_, _, cx| {
            let vars = cx.theme().vars;
            div()
                .w(px(260.))
                .child(toolbar(vars.control_tiny_size))
                .into_any_element()
        }),
        Story::new("In a card", |_, _, _| {
            div()
                .w(px(260.))
                .child(
                    Card::new()
                        .child("Tokens")
                        .child(Divider::horizontal())
                        .child("Components"),
                )
                .into_any_element()
        }),
        Story::new("Between rows", |_, _, cx| rows(cx).into_any_element()),
    ],
);

fn toolbar(height: gpui::Pixels) -> impl IntoElement {
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
        .child(separator(height))
        .child(IconButton::new(
            "rename",
            "Rename",
            Icon::new(icons::RENAME_16_REGULAR),
        ))
}

/// The rule doing its job: a list that separates its rows with one *between*
/// them rather than with a border under each.
fn rows(cx: &mut gpui::App) -> impl IntoElement {
    let vars = cx.theme().vars;
    let entries = ["aurora", "beacon", "cinder"];
    let mut column = div()
        .w(px(260.))
        .flex()
        .flex_col()
        .rounded(vars.radius_medium)
        .bg(vars.color_surface)
        .border(vars.spacing_px)
        .border_color(vars.color_border);

    for (index, name) in entries.iter().enumerate() {
        if index > 0 {
            column = column.child(Divider::horizontal());
        }
        column = column.child(
            div()
                .py(vars.spacing_2)
                .px(vars.spacing_3)
                .type_style(vars.label_quiet)
                .text_color(vars.color_content)
                .child(*name),
        );
    }

    column
}
