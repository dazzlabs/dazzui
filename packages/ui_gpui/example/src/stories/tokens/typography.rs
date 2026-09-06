//! `Tokens/Typography` — the faces, and what each is for.

use dazzui::prelude::*;
use dazzui::{ActiveTheme, TypeStyle};
use gpui::{IntoElement, ParentElement, Styled, div};

use crate::storybook::gallery::{DocPage, Section, code, px_label};
use crate::storybook::{Story, StoryPage};

const SAMPLE: &str = "Sharp, considered interfaces";

pub const PAGE: StoryPage = StoryPage::new(
    "Typography",
    &[Story::page("Scale", |_, _, cx| {
        let vars = cx.theme().vars;

        DocPage::new("Typography")
            .description(
                "Four families of face — headline, title, body, label — plus the caption. \
                 Leading is stated absolutely rather than as a multiple, because that is how the \
                 design states it: a label face sets its line height to its own size so the text \
                 sits on the centre of a 26px box, and a multiple would round that away.",
            )
            .child(
                Section::new("Headline")
                    .description("The page's own name, used once.")
                    .child(faces(&[
                        ("headline.small", vars.headline_small),
                        ("headline.medium", vars.headline_medium),
                        ("headline.large", vars.headline_large),
                    ])),
            )
            .child(
                Section::new("Title")
                    .description("A section's heading.")
                    .child(faces(&[
                        ("title.small", vars.title_small),
                        ("title.medium", vars.title_medium),
                        ("title.large", vars.title_large),
                    ])),
            )
            .child(
                Section::new("Body")
                    .description("Prose, and anything a reader reads.")
                    .child(faces(&[
                        ("body.small", vars.body_small),
                        ("body.medium", vars.body_medium),
                        ("body.large", vars.body_large),
                    ])),
            )
            .child(
                Section::new("Label")
                    .description(
                        "The display cut a control is set in. `quiet` and `strong` are the two \
                         weights selection moves between: a row rests at `quiet` and the \
                         selected entry takes the label weight, while a pill tab's active label \
                         and an option card's title go to `strong`.",
                    )
                    .child(faces(&[
                        ("label.quiet", vars.label_quiet),
                        ("label.small", vars.label_small),
                        ("label.medium", vars.label_medium),
                        ("label.large", vars.label_large),
                        ("label.strong", vars.label_strong),
                    ])),
            )
            .child(
                Section::new("Caption")
                    .description("The line under something else.")
                    .child(faces(&[
                        ("caption.small", vars.caption_small),
                        ("caption.medium", vars.caption_medium),
                        ("caption.large", vars.caption_large),
                    ])),
            )
            .into_any_element()
    })],
);

fn faces(entries: &[(&'static str, TypeStyle)]) -> impl IntoElement {
    let entries = entries.to_vec();

    div()
        .flex()
        .flex_col()
        .children(entries.into_iter().map(|(name, style)| {
            div()
                .flex()
                .flex_col()
                .child(div().type_style(style).child(SAMPLE))
                .child(
                    div()
                        .type_style(code(style))
                        .child(format!("{name} · {}", px_label(style.size))),
                )
        }))
}
