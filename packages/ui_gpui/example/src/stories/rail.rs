//! `Components/Rail` — the second navigation column.

use dazzui::{
    Icon, NavItem, NavItemSize, Rail, RailGroup, Sidebar, SidebarGroup, assets::icons,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, px};

use crate::Playground;
use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::{Frame, Pane};
use crate::storybook::{Story, StoryPage};

const THEMES: [(&str, &str); 4] = [
    ("studio-light", "Studio Light"),
    ("studio-dark", "Studio Dark"),
    ("bright-light", "Bright"),
    ("bright-dark", "Bright Dark"),
];

pub const PAGE: StoryPage = StoryPage::new(
    "Rail",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("Rail")
                .description(
                    "The column between the sidebar and the content: the groups, books or pages \
                     inside whatever the sidebar selected. It is lighter than the sidebar \
                     because it sits closer to the content — the two columns are not the same \
                     grey, and which is which is what tells you where you are.",
                )
                .child(
                    Section::new("Rows")
                        .description(
                            "A rail's rows are `NavItem`s at the `small` step, a size tighter \
                             than the sidebar's on both axes. The first group usually goes \
                             unlabelled: it is what the pane is already called, and naming it \
                             twice says nothing.",
                        )
                        .child(
                            Frame::new(px(620.), px(300.))
                                .child(
                                    Rail::new()
                                        .child(
                                            RailGroup::new().first(true).child(
                                                NavItem::new("overview", "Overview")
                                                    .size(NavItemSize::Small)
                                                    .active(true),
                                            ),
                                        )
                                        .child(
                                            RailGroup::new()
                                                .label("Themes")
                                                .child(rows(state, this)),
                                        ),
                                )
                                .child(Pane::new().child("Overview")),
                        ),
                )
                .child(
                    Section::new("Three columns")
                        .description(
                            "Sidebar, rail, content — and the aside on the far edge when the \
                             selection has details worth carrying. Each column's width is a \
                             `frame.*` metric rather than a value the component decides.",
                        )
                        .child(
                            Frame::new(px(820.), px(320.))
                                .child(
                                    Sidebar::new("rail-sidebar").child(
                                        SidebarGroup::new().label("Library").first(true).child(
                                            NavItem::new("design", "Design system")
                                                .icon(Icon::new(icons::PAINT_BRUSH_16_REGULAR))
                                                .active(true),
                                        ),
                                    ),
                                )
                                .child(
                                    Rail::new().child(
                                        RailGroup::new()
                                            .label("Themes")
                                            .first(true)
                                            .child(rows(state, this)),
                                    ),
                                )
                                .child(Pane::new().child("Studio Light")),
                        ),
                )
                .into_any_element()
        }),
        Story::new("A column", |state, this, _| {
            Frame::new(px(520.), px(300.))
                .child(Rail::new().child(RailGroup::new().first(true).child(rows(state, this))))
                .child(Pane::new().child("Studio Light"))
                .into_any_element()
        }),
        Story::new("Groups", |state, this, _| {
            Frame::new(px(520.), px(300.))
                .child(
                    Rail::new()
                        .child(
                            RailGroup::new().first(true).child(
                                NavItem::new("overview", "Overview")
                                    .size(NavItemSize::Small)
                                    .active(true),
                            ),
                        )
                        .child(RailGroup::new().label("Themes").child(rows(state, this))),
                )
                .child(Pane::new().child("Overview"))
                .into_any_element()
        }),
        Story::new("With an action", |state, this, _| {
            Frame::new(px(520.), px(300.))
                .child(
                    Rail::new()
                        .child(RailGroup::new().first(true).child(rows(state, this)))
                        .child(NavItem::action("new", "New theme").size(NavItemSize::Small)),
                )
                .child(Pane::new().child("Studio Light"))
                .into_any_element()
        }),
        Story::new("Scrolling", |_, _, _| {
            let mut group = RailGroup::new().label("Components").first(true);
            for name in crate::storybook::component_names() {
                group = group.child(
                    NavItem::new(SharedString::from(format!("rail-{name}")), name)
                        .size(NavItemSize::Small)
                        .active(name == "Rail"),
                );
            }

            Frame::new(px(420.), px(220.))
                .child(Rail::new().child(group))
                .child(Pane::new().child("Rail"))
                .into_any_element()
        }),
    ],
);

fn rows(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    let mut group = RailGroup::new().first(true);
    for (id, label) in THEMES {
        group = group.child(
            NavItem::new(SharedString::from(format!("rail-row-{id}")), label)
                .size(NavItemSize::Small)
                .active(state.rail == id)
                .on_click(crate::act(this, move |state, _, _| state.rail = id.into())),
        );
    }
    group
}
