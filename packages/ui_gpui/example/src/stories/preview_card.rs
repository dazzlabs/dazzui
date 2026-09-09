//! `Components/PreviewCard` — what is behind a link, without following it.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Avatar, AvatarShape, AvatarSize, Badge, BadgeSize, PopoverSide, PreviewCard, Tint,
};
use gpui::{IntoElement, ParentElement, SharedString, div};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

const SIDES: [(PopoverSide, &str); 4] = [
    (PopoverSide::Top, "top"),
    (PopoverSide::Bottom, "bottom"),
    (PopoverSide::Left, "left"),
    (PopoverSide::Right, "right"),
];

/// A link, drawn the way the `plain` recipe draws one: the accent one grade
/// darker than the fill.
fn link(label: &'static str, cx: &gpui::App) -> impl IntoElement {
    let vars = cx.theme().vars;

    div()
        .type_style(vars.label_quiet)
        .text_color(vars.control_color(
            dazzui::ControlVariant::Plain,
            dazzui::ControlPart::Content,
            Tint::Primary,
            dazzui::ControlState::Normal,
        ))
        .child(label)
}

fn person(cx: &gpui::App) -> impl IntoElement {
    let vars = cx.theme().vars;

    Col::new()
        .child(
            Row::new()
                .child(Avatar::new("Ada Lovelace").size(AvatarSize::Large))
                .child(
                    Col::new()
                        .gap(vars.spacing_0_5)
                        .child(
                            div()
                                .type_style(vars.label_strong)
                                .text_color(vars.color_content)
                                .child("Ada Lovelace"),
                        )
                        .child(
                            div()
                                .type_style(vars.label_quiet)
                                .text_color(vars.color_content_subtle)
                                .child("@ada"),
                        ),
                ),
        )
        .child(
            div()
                .type_style(vars.body_small)
                .text_color(vars.color_content_secondary)
                .child("Writes the notes that turn a machine into a program."),
        )
}

pub const PAGE: StoryPage = StoryPage::new(
    "PreviewCard",
    &[
        Story::page("Overview", |_, _, cx| {
            DocPage::new("PreviewCard")
                .description(
                    "What is behind a link, shown without following it — a repository, a \
                     person, an issue. It is a Tooltip that holds a card rather than a label, \
                     and the difference is what may go in it: a tooltip must carry nothing a \
                     reader needs, because a pointer never reaches it; a preview card stays \
                     open while the pointer travels, so it can hold a link of its own. It is \
                     never the only way to reach what it shows.",
                )
                .child(
                    Section::new("A link, previewed")
                        .description(
                            "It opens on hover — never on a press, which belongs to the link. \
                             The sheet is the Popover's, with a ceiling on its width: a popover \
                             is as wide as what a caller put in it, a preview card is filled \
                             from whatever it previews.",
                        )
                        .child(
                            PreviewCard::new("doc-link")
                                .trigger(link("@ada", cx))
                                .child(person(cx)),
                        ),
                )
                .child(
                    Section::new("Against a Tooltip")
                        .description(
                            "If the panel has anything in it a reader might want to reach — a \
                             link, a button, a value — it is a preview card. If it is a phrase \
                             naming what an icon does, it is a tooltip, and it must stay one.",
                        )
                        .child(
                            PreviewCard::new("doc-repo")
                                .trigger(link("dazzlabs/dazzui", cx))
                                .child(
                                    Col::new()
                                        .child(
                                            Row::new()
                                                .child(
                                                    Avatar::new("DazzUI")
                                                        .shape(AvatarShape::Square)
                                                        .size(AvatarSize::Small),
                                                )
                                                .child("dazzlabs/dazzui")
                                                .child(
                                                    Badge::new("public")
                                                        .size(BadgeSize::Small)
                                                        .tint(Tint::Neutral),
                                                ),
                                        )
                                        .child(
                                            "The design system, its tokens, and four platform \
                                             implementations of it.",
                                        ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("On a link", |_, _, cx| {
            Row::new()
                .child("Reviewed by")
                .child(
                    PreviewCard::new("on-a-link")
                        .trigger(link("@ada", cx))
                        .child(person(cx)),
                )
                .child("three days ago.")
                .into_any_element()
        }),
        Story::new("A repository", |_, _, cx| {
            PreviewCard::new("a-repository")
                .trigger(link("dazzlabs/dazzui", cx))
                .child(
                    Col::new()
                        .child(
                            Row::new()
                                .child(
                                    Avatar::new("DazzUI")
                                        .shape(AvatarShape::Square)
                                        .size(AvatarSize::Small),
                                )
                                .child("dazzlabs/dazzui")
                                .child(
                                    Badge::new("public")
                                        .size(BadgeSize::Small)
                                        .tint(Tint::Neutral),
                                ),
                        )
                        .child(
                            "The design system, its tokens, and four platform implementations \
                             of it.",
                        ),
                )
                .into_any_element()
        }),
        Story::new("Sides", |_, _, cx| {
            let mut row = Row::new();
            for (side, name) in SIDES {
                row = row.child(
                    PreviewCard::new(SharedString::from(format!("side-{name}")))
                        .side(side)
                        .trigger(link(name, cx))
                        .child(person(cx)),
                );
            }
            row.into_any_element()
        }),
        Story::new("Instant", |_, _, cx| {
            // The pointer opens it the moment it arrives: GPUI's hover is the
            // frame's, so there is no rest to wait out.
            PreviewCard::new("instant")
                .trigger(link("@ada", cx))
                .child(person(cx))
                .into_any_element()
        }),
    ],
);
