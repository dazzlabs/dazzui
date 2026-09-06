//! `Components/NavItem` — the row a sidebar and a rail are made of.

use dazzui::{Icon, NavItem, NavItemSize, Sidebar, SidebarGroup, Tint, assets::icons};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, capitalize};
use crate::storybook::stage::{Col, Frame, Pane};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "NavItem",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("NavItem")
                .description(
                    "One navigable row. Selection shifts weight as well as colour: a row rests \
                     at the quiet label face and the selected one takes the filled recipe, so a \
                     column reads as one selection rather than as a list of differently \
                     coloured rows.",
                )
                .child(
                    Section::new("States")
                        .description(
                            "Idle rows sit in the navigation ink, a step back from body copy. \
                             Hover brings the row to full ink — the hover rule outranks the \
                             row's own colour, so both kinds of row answer the pointer the same \
                             way. Active takes the filled recipe on the tint.",
                        )
                        .child(
                            Col::new()
                                .width(px(220.))
                                .gap(px(0.))
                                .child(NavItem::new("idle", "Idle"))
                                .child(NavItem::new("active", "Active").active(true))
                                .child(NavItem::new("dead", "Disabled").disabled(true)),
                        ),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "`medium` is the sidebar row — 28px, the regular control height. \
                             `small` is the rail row, a step tighter on both axes because it \
                             sits closer to the content.",
                        )
                        .child(
                            Col::new()
                                .width(px(220.))
                                .gap(px(0.))
                                .child(NavItem::new("medium", "Medium").active(true))
                                .child(NavItem::new("small", "Small").size(NavItemSize::Small)),
                        ),
                )
                .child(
                    Section::new("The glyph and the action")
                        .description(
                            "An absent icon stays absent rather than becoming an empty one: the \
                             glyph slot is a fixed width, and a row without an icon would \
                             otherwise be indented past a row with one. `NavItem::action` is the \
                             row that adds rather than navigates — accent ink and the label \
                             weight at rest.",
                        )
                        .child(
                            Col::new()
                                .width(px(220.))
                                .gap(px(0.))
                                .child(
                                    NavItem::new("projects", "Projects")
                                        .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR))
                                        .active(true),
                                )
                                .child(NavItem::new("archive", "Archive"))
                                .child(NavItem::action("new", "New project")),
                        ),
                )
                .child(Section::new("Selecting").child(selecting(state, this)))
                .into_any_element()
        }),
        Story::new("Idle", |_, _, _| {
            div()
                .w(px(200.))
                .child(NavItem::new("idle", "Projects"))
                .into_any_element()
        }),
        Story::new("Active", |_, _, _| {
            div()
                .w(px(200.))
                .child(NavItem::new("active", "Projects").active(true))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .w(px(200.))
                .child(NavItem::new("dead", "Projects").disabled(true))
                .into_any_element()
        }),
        Story::new("States", |_, _, _| {
            Col::new()
                .width(px(200.))
                .gap(px(0.))
                .child(NavItem::new("idle", "Idle"))
                .child(NavItem::new("active", "Active").active(true))
                .child(NavItem::new("dead", "Disabled").disabled(true))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            Col::new()
                .width(px(200.))
                .gap(px(0.))
                .child(NavItem::new("medium", "Medium").active(true))
                .child(NavItem::new("small", "Small").size(NavItemSize::Small))
                .into_any_element()
        }),
        Story::new("With an icon", |_, _, _| {
            Col::new()
                .width(px(200.))
                .gap(px(0.))
                .child(
                    NavItem::new("projects", "Projects")
                        .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR))
                        .active(true),
                )
                .child(
                    NavItem::new("settings", "Settings")
                        .icon(Icon::new(icons::SETTINGS_16_REGULAR)),
                )
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(200.)).gap(px(0.));
            for tint in Tint::ALL {
                col = col.child(
                    NavItem::new(tint.name(), capitalize(tint.name()))
                        .active(true)
                        .tint(tint),
                );
            }
            col.into_any_element()
        }),
        Story::new("Action", |_, _, _| {
            Col::new()
                .width(px(200.))
                .gap(px(0.))
                .child(NavItem::new("projects", "Projects"))
                .child(NavItem::action("new", "New project"))
                .into_any_element()
        }),
        Story::new("Selecting", |state, this, _| {
            selecting(state, this).into_any_element()
        }),
        Story::new("In a sidebar", |state, this, _| {
            let mut group = SidebarGroup::new().label("Library").first(true);
            for (id, label, icon) in ROWS {
                group = group.child(
                    NavItem::new(SharedString::from(format!("sidebar-{id}")), label)
                        .icon(Icon::new(icon))
                        .active(state.library == id)
                        .on_click(act(this, move |state, _, _| state.library = id.into())),
                );
            }

            Frame::new(px(520.), px(300.))
                .child(Sidebar::new("nav-sidebar").child(group))
                .child(Pane::new().child("Projects"))
                .into_any_element()
        }),
    ],
);

const ROWS: [(&str, &str, &str); 3] = [
    ("projects", "Projects", icons::SQUARE_MULTIPLE_16_REGULAR),
    ("archive", "Archive", icons::TABLE_16_REGULAR),
    ("settings", "Settings", icons::SETTINGS_16_REGULAR),
];

fn selecting(
    state: &crate::Playground,
    this: &gpui::Entity<crate::Playground>,
) -> impl IntoElement {
    let mut col = Col::new().width(px(200.)).gap(px(0.));
    for (id, label, _) in ROWS {
        col = col.child(
            NavItem::new(SharedString::from(format!("select-{id}")), label)
                .active(state.nav == id)
                .on_click(act(this, move |state, _, _| state.nav = id.into())),
        );
    }
    col
}
