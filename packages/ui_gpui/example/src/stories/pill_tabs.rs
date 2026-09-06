//! `Components/PillTabs` — the filter row over a list.

use dazzui::{Divider, PillTabs, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize};
use crate::storybook::stage::{Col, Frame, Pane};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "PillTabs",
    &[
        Story::page("Overview", |state, this, _| {
            let mut tints = Col::new();
            for tint in Tint::ALL {
                tints = tints.child(
                    PillTabs::new(SharedString::from(format!("tint-{}", tint.name())), "open")
                        .item("all", "All")
                        .item("open", capitalize(tint.name()))
                        .tint(tint),
                );
            }

            DocPage::new("PillTabs")
                .description(
                    "A run of pills that filters the list under it. Unlike a segmented control \
                     it has no track: the pills sit on the page, so the row can wrap and can \
                     carry counts without turning into a toolbar.",
                )
                .child(
                    Section::new("Selection")
                        .description(
                            "The active pill takes the strong label face as well as the tint's \
                             wash — the same rule the nav row follows, and the reason the row \
                             still reads when the theme's accent is quiet.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("plain").child(tabs(state, this, false, "doc")),
                                )
                                .child(Specimen::new("counts").child(tabs(
                                    state,
                                    this,
                                    true,
                                    "doc-counts",
                                )))
                                .child(
                                    Specimen::new("disabled").child(
                                        PillTabs::new("disabled", "all")
                                            .counted_item("all", "All", 24)
                                            .item("archived", "Archived"),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("Tints")
                        .description(
                            "One recipe table, six ramps — the pill's wash is the tint's, so a \
                             filter row can carry the meaning of what it filters.",
                        )
                        .child(tints),
                )
                .into_any_element()
        }),
        Story::new("Default", |state, this, _| {
            tabs(state, this, false, "default").into_any_element()
        }),
        Story::new("With counts", |state, this, _| {
            tabs(state, this, true, "counts").into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    PillTabs::new(
                        SharedString::from(format!("story-tint-{}", tint.name())),
                        "open",
                    )
                    .item("all", "All")
                    .item("open", capitalize(tint.name()))
                    .tint(tint),
                );
            }
            col.into_any_element()
        }),
        Story::new("Wrapping", |_, _, _| {
            div()
                .w(px(380.))
                .child(
                    PillTabs::new("wrapping", "all")
                        .counted_item("all", "All", 124)
                        .counted_item("open", "Open", 12)
                        .counted_item("review", "In review", 4)
                        .counted_item("blocked", "Blocked", 2)
                        .counted_item("done", "Done", 106)
                        .item("archived", "Archived"),
                )
                .into_any_element()
        }),
        Story::new("In a pane header", |state, this, _| {
            Frame::new(px(560.), px(260.))
                .child(
                    Pane::new()
                        .child("Builds")
                        .child(tabs(state, this, true, "pane"))
                        .child(Divider::horizontal()),
                )
                .into_any_element()
        }),
    ],
);

fn tabs(
    state: &crate::Playground,
    this: &gpui::Entity<crate::Playground>,
    counts: bool,
    id: &'static str,
) -> impl IntoElement {
    let mut tabs = PillTabs::new(SharedString::from(id), state.tab.clone())
        .on_select(act_with(this, |state, value: &SharedString, _, _| {
            state.tab = value.clone()
        }));

    for (value, label, count) in [
        ("all", "All", 24),
        ("open", "Open", 3),
        ("done", "Done", 21),
    ] {
        tabs = if counts {
            tabs.counted_item(value, label, count)
        } else {
            tabs.item(value, label)
        };
    }

    tabs
}
