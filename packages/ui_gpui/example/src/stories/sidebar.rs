//! `Components/Sidebar` — the first navigation column.

use dazzui::{
    ActiveTheme, Badge, BadgeSize, Icon, NavItem, Progress, Sidebar, SidebarCard, SidebarGroup,
    assets::icons,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, px};

use crate::storybook::gallery::{DocPage, Section, TokenTable};
use crate::storybook::stage::{Frame, Pane, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act};

const ROWS: [(&str, &str, &str); 3] = [
    ("projects", "Projects", icons::SQUARE_MULTIPLE_16_REGULAR),
    ("archive", "Archive", icons::TABLE_16_REGULAR),
    ("settings", "Settings", icons::SETTINGS_16_REGULAR),
];

pub const PAGE: StoryPage = StoryPage::new(
    "Sidebar",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Sidebar")
                .description(
                    "The first navigation column: what the window is *about*, in groups of \
                     rows. It carries the muted surface — the same one the aside wears — so a \
                     window reads as content between two quieter edges.",
                )
                .child(
                    Section::new("Groups")
                        .description(
                            "Rows are broken into groups rather than stacked. The gap does most \
                             of the work and the label only names it, which is why a first group \
                             can go unlabelled where the window's title already says what it is.",
                        )
                        .child(
                            Frame::new(px(620.), px(300.))
                                .child(
                                    Sidebar::new("doc-groups")
                                        .child(library(state, this, "doc"))
                                        .child(
                                            SidebarGroup::new()
                                                .label("Recent")
                                                .child(NavItem::new("aurora", "aurora"))
                                                .child(NavItem::new("beacon", "beacon")),
                                        ),
                                )
                                .child(Pane::new().child("Projects")),
                        ),
                )
                .child(
                    Section::new("Header, card and action")
                        .description(
                            "The header sits on the titlebar's own height, so it lines up with \
                             whatever is across the top of the window. A pinned `SidebarCard` \
                             stays put while the rows scroll, and a trailing `NavItem::action` \
                             is the row that adds rather than navigates.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(
                                    Frame::new(px(300.), px(280.)).child(
                                        Sidebar::new("doc-header")
                                            .header(Badge::new("DazzUI").size(BadgeSize::Small))
                                            .footer(
                                                SidebarCard::new()
                                                    .label("Build")
                                                    .child(Progress::new(75.))
                                                    .child("3 of 4 packages"),
                                            )
                                            .child(library(state, this, "doc-header")),
                                    ),
                                )
                                .child(
                                    Frame::new(px(300.), px(280.)).child(
                                        Sidebar::new("doc-action")
                                            .child(library(state, this, "doc-action"))
                                            .child(NavItem::action("new", "New project")),
                                    ),
                                ),
                        ),
                )
                .child(
                    Section::new("Tokens")
                        .description("The column widths a shell is built on.")
                        .child(
                            TokenTable::new()
                                .length("frame.sidebar-width", vars.frame_sidebar_width)
                                .length("frame.rail-width", vars.frame_rail_width)
                                .length("frame.aside-width", vars.frame_aside_width)
                                .length("frame.titlebar-size", vars.frame_titlebar_size)
                                .length("frame.nav-gap", vars.frame_nav_gap),
                        ),
                )
                .into_any_element()
        }),
        Story::new("A column", |state, this, _| {
            Frame::new(px(560.), px(320.))
                .child(Sidebar::new("column").child(library(state, this, "column")))
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
        Story::new("Groups", |state, this, _| {
            Frame::new(px(560.), px(320.))
                .child(
                    Sidebar::new("groups")
                        .child(library(state, this, "groups"))
                        .child(
                            SidebarGroup::new()
                                .label("Recent")
                                .child(NavItem::new("aurora", "aurora"))
                                .child(NavItem::new("beacon", "beacon")),
                        ),
                )
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
        Story::new("With a header", |state, this, _| {
            Frame::new(px(560.), px(320.))
                .child(
                    Sidebar::new("header")
                        .header(Badge::new("DazzUI").size(BadgeSize::Small))
                        .child(library(state, this, "header")),
                )
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
        Story::new("With a card", |state, this, _| {
            Frame::new(px(560.), px(320.))
                .child(
                    Sidebar::new("card")
                        .footer(
                            SidebarCard::new()
                                .label("Build")
                                .child(Progress::new(75.))
                                .child("3 of 4 packages"),
                        )
                        .child(library(state, this, "card")),
                )
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
        Story::new("With an action", |state, this, _| {
            Frame::new(px(560.), px(320.))
                .child(
                    Sidebar::new("action")
                        .child(library(state, this, "action"))
                        .child(NavItem::action("new", "New project")),
                )
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
        Story::new("Resizable", |state, this, _| {
            Frame::new(px(620.), px(320.))
                .child(Sidebar::new("resizable").resizable(true).child(library(
                    state,
                    this,
                    "resizable",
                )))
                .child(Pane::new().child("Drag the trailing edge."))
                .into_any_element()
        }),
    ],
);

fn library(state: &Playground, this: &Entity<Playground>, id: &'static str) -> impl IntoElement {
    let mut group = SidebarGroup::new().label("Library").first(true);
    for (row, label, icon) in ROWS {
        group = group.child(
            NavItem::new(SharedString::from(format!("{id}-{row}")), label)
                .icon(Icon::new(icon))
                .active(state.library == row)
                .on_click(act(this, move |state, _, _| state.library = row.into())),
        );
    }
    group
}
