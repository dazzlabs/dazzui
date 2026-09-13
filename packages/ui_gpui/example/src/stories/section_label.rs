//! `Components/SectionLabel` — the heading over a run of rows.

use dazzui::{Card, Divider, NavItem, SectionLabel, Sidebar, SidebarGroup, Tint};
use gpui::{IntoElement, ParentElement, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize};
use crate::storybook::stage::{Frame, Pane};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "SectionLabel",
    &[
        Story::page("Overview", |_, _, _| {
            let mut tinted = Specimen::new("tinted");
            for tint in Tint::ALL {
                tinted = tinted.child(SectionLabel::new(capitalize(tint.name())).tint(tint));
            }

            DocPage::new("SectionLabel")
                .description(
                    "The quiet heading that names a run of rows. A sidebar group already wears \
                     one; this is the same label reached directly, for a panel or a column that \
                     is not a sidebar.",
                )
                .child(
                    Section::new("Tone")
                        .description(
                            "It takes the subtle ink by default and the faint one where the \
                             label is only there to break a long run. A tint colours it where a \
                             section carries a meaning of its own — a danger zone in a settings \
                             page.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("default").child(SectionLabel::new("Library")))
                                .child(
                                    Specimen::new("faint")
                                        .child(SectionLabel::new("Library").faint(true)),
                                )
                                .child(tinted),
                        ),
                )
                .child(
                    Section::new("Where it lands")
                        .description(
                            "Over a run of rows in a panel or an aside. In a sidebar the group \
                             draws it, so a label written by hand there would be a second \
                             heading over the same rows.",
                        )
                        .child(in_a_panel()),
                )
                .into_any_element()
        }),
        Story::new("In a panel", |_, _, _| in_a_panel().into_any_element()),
        Story::new("In a column", |_, _, _| {
            Frame::new(px(520.), px(280.))
                .child(
                    Sidebar::new("labels")
                        .child(
                            SidebarGroup::new()
                                .label("Library")
                                .first(true)
                                .child(NavItem::new("projects", "Projects").active(true))
                                .child(NavItem::new("archive", "Archive")),
                        )
                        .child(
                            SidebarGroup::new()
                                .label("Recent")
                                .child(NavItem::new("aurora", "aurora")),
                        ),
                )
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
    ],
);

fn in_a_panel() -> impl IntoElement {
    div().w(px(260.)).child(
        Card::new()
            .child(SectionLabel::new("Details"))
            .child("aurora")
            .child(Divider::horizontal())
            .child(SectionLabel::new("Timing"))
            .child("2m 14s"),
    )
}
