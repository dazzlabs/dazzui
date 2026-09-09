//! `Components/Collapsible` — one thing that opens.

use dazzui::{Badge, Collapsible, Icon, Tint, assets::icons};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

const LOG: &str = "$ pnpm test --filter core\n  ✓ resolves a relative import  (4ms)\n  \
                   ✓ resolves a bare specifier   (2ms)\n  ✓ falls back to the index     (3ms)";

fn environment(id: &'static str, open: bool) -> Collapsible {
    Collapsible::new(id, "Environment")
        .initially_open(open)
        .child("Node 24.20, pnpm 10.4, macOS 15.5")
}

fn tests(id: &'static str) -> Collapsible {
    Collapsible::new(id, "Ran 3 tests")
        .initially_open(true)
        .icon(Icon::new(icons::TABLE_16_REGULAR))
        .meta(Badge::new("passed").tint(Tint::Success))
        .child(LOG)
}

pub const PAGE: StoryPage =
    StoryPage::new(
        "Collapsible",
        &[
            Story::page("Overview", |_, _, _| {
                DocPage::new("Collapsible")
                    .description(
                        "One thing that opens. The header row is the affordance and the panel is \
                     what it discloses — a tool call's output, a stack trace, the rest of a \
                     long description. It is the smallest of the three ways this system hides \
                     something: a collapsible keeps its subject in place and only folds it, \
                     where a Popover moves it off the page and a Dialog stops the page.",
                    )
                    .child(
                        Section::new("Shut and open")
                            .description(
                                "The chevron points along the row when shut and down the page when \
                             open. It is the only moving part: a header that also slid or faded \
                             would read as two things happening.",
                            )
                            .child(
                                Specimens::new()
                                    .child(Specimen::new("shut").child(
                                        div().w(px(352.)).child(environment("doc-shut", false)),
                                    ))
                                    .child(Specimen::new("open").child(
                                        div().w(px(352.)).child(environment("doc-open", true)),
                                    )),
                            ),
                    )
                    .child(
                        Section::new("The header row")
                            .description(
                                "A glyph before the title and an aside after it. The aside is what \
                             makes a shut row worth reading — a count, a duration, a status — \
                             so the reader can decide whether to open it at all.",
                            )
                            .child(div().w(px(416.)).child(tests("doc-tests"))),
                    )
                    .child(
                        Section::new("Against an Accordion")
                            .description(
                                "Several of these that know about each other are an Accordion, \
                             where opening one shuts the last. A run of separate collapsibles \
                             does not, which is right when the sections are unrelated and wrong \
                             when they are one list.",
                            )
                            .child(div().w(px(416.)).child(environment("doc-alone", false))),
                    )
                    .into_any_element()
            }),
            Story::new("Shut", |_, _, _| {
                div()
                    .w(px(352.))
                    .child(environment("shut", false))
                    .into_any_element()
            }),
            Story::new("Open", |_, _, _| {
                div()
                    .w(px(352.))
                    .child(environment("open", true))
                    .into_any_element()
            }),
            Story::new("With meta", |_, _, _| {
                div().w(px(416.)).child(tests("meta")).into_any_element()
            }),
            Story::new("Disabled", |_, _, _| {
                div()
                    .w(px(352.))
                    .child(environment("disabled", false).disabled(true))
                    .into_any_element()
            }),
            Story::new("Interactive", |state, this, _| {
                let open = state.disclosed;

                div()
                    .w(px(384.))
                    .child(
                        Collapsible::new("live", "Request headers")
                            .open(open)
                            .meta(if open { "hide" } else { "show" })
                            .on_toggle(act(this, move |state, _, _| state.disclosed = !open))
                            .child("accept: application/json\nauthorization: Bearer ****"),
                    )
                    .into_any_element()
            }),
        ],
    );
