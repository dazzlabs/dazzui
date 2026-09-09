//! `Components/Accordion` — several disclosures that know about each other.

use dazzui::{Accordion, AccordionItem};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

const FAQ: [(&str, &str, &str); 3] = [
    (
        "where",
        "Where do the values come from?",
        "Every colour, length and face is a token in packages/ds. A component reads the \
         variable; it never writes a value of its own.",
    ),
    (
        "themes",
        "How many themes are there?",
        "Twelve, as six families over one default. The modes compose, so a value shared along \
         the light/dark axis is written once.",
    ),
    (
        "ports",
        "What keeps the platforms in line?",
        "The parity pipeline measures the React build in a browser and holds every port to \
         those numbers.",
    ),
];

fn faq(id: &'static str) -> Accordion {
    let mut accordion = Accordion::new(id);
    for (value, title, body) in FAQ {
        accordion = accordion.item(AccordionItem::new(value, title).child(body));
    }
    accordion
}

pub const PAGE: StoryPage = StoryPage::new(
    "Accordion",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Accordion")
                .description(
                    "Several disclosures that know about each other. That knowledge is the \
                     whole difference from a run of Collapsibles: by default opening one shuts \
                     the last, so a reader is always looking at one section rather than at a \
                     page that grew while they were reading it.",
                )
                .child(
                    Section::new("One at a time")
                        .description(
                            "The default, and the one to reach for when the sections are \
                             alternatives — a FAQ, a set of steps, anything where two open at \
                             once would push the third off the screen.",
                        )
                        .child(div().w(px(448.)).child(faq("doc-single"))),
                )
                .child(
                    Section::new("Divided")
                        .description(
                            "Undivided, the sections sit on one run and the rows are separated \
                             by their own hover wash alone. `divided` draws the hairline, for \
                             the case where they are genuinely a list and the reader is \
                             counting them.",
                        )
                        .child(div().w(px(448.)).child(faq("doc-divided").divided(true))),
                )
                .into_any_element()
        }),
        Story::new("One at a time", |_, _, _| {
            div().w(px(448.)).child(faq("single")).into_any_element()
        }),
        Story::new("Multiple", |_, _, _| {
            div()
                .w(px(448.))
                .child(
                    faq("multiple")
                        .multiple(true)
                        .open(vec!["where".into(), "themes".into()]),
                )
                .into_any_element()
        }),
        Story::new("Divided", |_, _, _| {
            div()
                .w(px(448.))
                .child(faq("divided").divided(true))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .w(px(448.))
                .child(
                    Accordion::new("disabled")
                        .divided(true)
                        .item(AccordionItem::new("where", FAQ[0].1).child(FAQ[0].2))
                        .item(
                            AccordionItem::new("locked", "Managed by your admin")
                                .disabled(true)
                                .child("Nothing here can be changed on this device."),
                        ),
                )
                .into_any_element()
        }),
    ],
);
