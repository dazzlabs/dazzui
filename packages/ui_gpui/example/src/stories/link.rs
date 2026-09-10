//! `Components/Link` — text that goes somewhere.

use dazzui::{Link, LinkUnderline, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Link",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Link")
                .description(
                    "Text that goes somewhere. It is the one interactive thing in this package \
                     that is genuinely inline, which is why it exists beside Button(Plain) \
                     rather than inside it: a button keeps its box at every variant, and a box \
                     is what a link in a sentence must not have. It takes the shared state \
                     machine for its colour — the plain recipe, a grade darker on hover — and \
                     none of its geometry.",
                )
                .child(
                    Section::new("Underline")
                        .description(
                            "hover is the default, because a paragraph with a rule under every \
                             link reads as struck through. always is for a link that has to be \
                             findable without a pointer — a list of references, a footer. none \
                             is for one whose surroundings already say it is a link.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("hover")
                                        .child(Link::new("doc-hover", "Open the changelog")),
                                )
                                .child(
                                    Specimen::new("always").child(
                                        Link::new("doc-always", "Open the changelog")
                                            .underline(LinkUnderline::Always),
                                    ),
                                )
                                .child(
                                    Specimen::new("none").child(
                                        Link::new("doc-none", "Open the changelog")
                                            .underline(LinkUnderline::None),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("External")
                        .description(
                            "Goes somewhere outside this window, and draws the arrow that says \
                             so before the reader commits.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("same window")
                                        .child(Link::new("doc-same", "Release notes")),
                                )
                                .child(
                                    Specimen::new("external")
                                        .child(Link::new("doc-external", "Base UI").external(true)),
                                ),
                        ),
                )
                .child(
                    Section::new("Tints")
                        .description(
                            "A link is accent text, so it reads the plain recipe's content \
                             grade off whichever ramp it is tinted with. Danger is the one \
                             worth reaching for — a link that deletes something.",
                        )
                        .child(div().flex().flex_col().gap(px(8.)).children(
                            Tint::ALL.iter().enumerate().map(|(index, tint)| {
                                div().child(
                                    Link::new(
                                        gpui::SharedString::from(format!("doc-tint-{index}")),
                                        "Continue",
                                    )
                                    .tint(*tint),
                                )
                            }),
                        )),
                )
                .into_any_element()
        }),
        Story::new("Underlines", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(Link::new("hover", "Open the changelog"))
                .child(Link::new("always", "Open the changelog").underline(LinkUnderline::Always))
                .child(Link::new("none", "Open the changelog").underline(LinkUnderline::None))
                .into_any_element()
        }),
        Story::new("External", |_, _, _| {
            Link::new("external", "Base UI")
                .external(true)
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .children(Tint::ALL.iter().enumerate().map(|(index, tint)| {
                    div().child(
                        Link::new(
                            gpui::SharedString::from(format!("tint-{index}")),
                            "Continue",
                        )
                        .tint(*tint),
                    )
                }))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            Link::new("disabled", "Not available yet")
                .disabled(true)
                .into_any_element()
        }),
        Story::new("In prose", |_, _, _| {
            // GPUI lays a paragraph out as a run of text, and an element cannot
            // sit inside one — so prose with links in it is a row of runs. The
            // React page reads as one sentence; this one reads as its parts.
            div()
                .flex()
                .items_center()
                .gap(px(4.))
                .child("Behaviour comes from")
                .child(Link::new("prose-base", "Base UI"))
                .child("and the drawing is ours — or open the")
                .child(Link::new("prose-docs", "upstream docs").external(true))
                .into_any_element()
        }),
    ],
);
