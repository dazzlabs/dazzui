//! `Components/ContextMenu` — the menu a right-click opens.

use dazzui::{Card, ContextMenu, MenuItem};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

fn target(label: &'static str) -> impl IntoElement {
    Card::new().child(
        div()
            .w(px(352.))
            .h(px(144.))
            .flex()
            .items_center()
            .justify_center()
            .child(label),
    )
}

fn actions(id: &'static str) -> ContextMenu {
    ContextMenu::new(id)
        .item(MenuItem::new("Open").shortcut("↩"))
        .item(MenuItem::new("Rename").shortcut("F2"))
        .item(MenuItem::new("Duplicate").shortcut("⌘D"))
        .item(MenuItem::new("Reveal in Finder"))
        .item(MenuItem::new("Delete").disabled(true))
}

pub const PAGE: StoryPage = StoryPage::new(
    "ContextMenu",
    &[
        Story::page("Overview", |state, this, _| {
            let sort = state.sort.clone();
            let mut sorted = ContextMenu::new("doc-sorted");
            for key in ["name", "date", "size"] {
                sorted = sorted.item(
                    MenuItem::new(SharedString::from(format!("Sort by {key}")))
                        .checked(sort == key)
                        .on_select(act(this, move |state, _, _| state.sort = key.into())),
                );
            }

            DocPage::new("ContextMenu")
                .description(
                    "The menu a right-click opens, over the thing that was clicked. It is the \
                     same menu a button opens — the panel and the rows are the menu's own, so \
                     this shares the drawing rather than resembling it. What is its own is \
                     where it opens: at the pointer rather than against a trigger's edge.",
                )
                .child(
                    Section::new("A region that has a menu")
                        .description(
                            "The trigger is a listener on what you give it, so a row, a card or \
                             a canvas keeps its own layout. Every affordance in a context menu \
                             must also be reachable somewhere a pointer can see — a right-click \
                             is a shortcut, never the only door.",
                        )
                        .child(
                            actions("doc-actions").child(target("Right-click anywhere in here")),
                        ),
                )
                .child(
                    Section::new("Checked rows")
                        .description(
                            "Set `checked` on every row of a group, including the unchecked \
                             ones, so the leading column holds its width and the labels stay on \
                             one left edge.",
                        )
                        .child(sorted.child(target("Sorted"))),
                )
                .into_any_element()
        }),
        Story::new("Bare", |_, _, _| {
            actions("bare")
                .child(target("Right-click anywhere in here"))
                .into_any_element()
        }),
        Story::new("Checked", |state, this, _| {
            let sort = state.sort.clone();
            let mut menu = ContextMenu::new("checked");
            for key in ["name", "date", "size"] {
                menu = menu.item(
                    MenuItem::new(SharedString::from(format!("Sort by {key}")))
                        .checked(sort == key)
                        .on_select(act(this, move |state, _, _| state.sort = key.into())),
                );
            }
            menu.child(target("Right-click to choose a sort"))
                .into_any_element()
        }),
        Story::new("On a row", |_, _, _| {
            let mut col = Col::new().width(px(352.));
            for branch in ["main", "release/4.2", "fix/token-drift"] {
                col = col.child(
                    ContextMenu::new(SharedString::from(format!("row-{branch}")))
                        .item(MenuItem::new("Check out"))
                        .item(MenuItem::new("Rename").shortcut("F2"))
                        .item(MenuItem::new("Delete").disabled(branch == "main"))
                        .child(Card::new().child(div().w_full().child(branch))),
                );
            }
            col.into_any_element()
        }),
    ],
);
