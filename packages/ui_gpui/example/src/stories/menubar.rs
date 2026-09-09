//! `Components/Menubar` — several menus that behave as one.

use dazzui::{MenuItem, Menubar, MenubarMenu};
use gpui::{IntoElement, ParentElement};

use crate::act;
use crate::storybook::gallery::{DocPage, Section};
use crate::storybook::{Story, StoryPage};

fn bar(id: &'static str) -> Menubar {
    Menubar::new(id)
        .menu(
            MenubarMenu::new("File")
                .item(MenuItem::new("New project").shortcut("⌘N"))
                .item(MenuItem::new("Open…").shortcut("⌘O"))
                .item(MenuItem::new("Save").shortcut("⌘S"))
                .item(MenuItem::new("Close window").shortcut("⌘W")),
        )
        .menu(
            MenubarMenu::new("Edit")
                .item(MenuItem::new("Undo").shortcut("⌘Z"))
                .item(MenuItem::new("Redo").shortcut("⇧⌘Z").disabled(true))
                .item(MenuItem::new("Find…").shortcut("⌘F")),
        )
        .menu(
            MenubarMenu::new("View")
                .item(MenuItem::new("Sidebar").shortcut("⌘1"))
                .item(MenuItem::new("Dock").shortcut("⌘2"))
                .item(MenuItem::new("Full screen").shortcut("⌃⌘F")),
        )
}

pub const PAGE: StoryPage = StoryPage::new(
    "Menubar",
    &[
        Story::page("Overview", |state, this, _| {
            let density = state.density.clone();
            let mut view = MenubarMenu::new("View");
            for name in ["Compact", "Comfortable"] {
                view = view.item(
                    MenuItem::new(name)
                        .checked(density == name)
                        .on_select(act(this, move |state, _, _| state.density = name.into())),
                );
            }

            DocPage::new("Menubar")
                .description(
                    "The application menu bar: several menus that behave as one. That is the \
                     whole reason it exists rather than a row of Menus — once one of them is \
                     open, moving the pointer along the bar opens the next without a second \
                     click. It is the behaviour every desktop menu bar has and none of them \
                     documents.",
                )
                .child(
                    Section::new("The bar")
                        .description(
                            "Each panel is the menu's own panel of rows, so the drawing is the \
                             same everywhere; the bar adds only its triggers. They are chrome \
                             rather than controls — no box at rest, and a wash only under the \
                             pointer or while their own menu is open.",
                        )
                        .child(bar("doc-bar")),
                )
                .child(
                    Section::new("Checked rows")
                        .description(
                            "A menu that reports a choice reveals the leading column on every \
                             row of the group. Set `checked` on all of them, including the \
                             unchecked ones, so the labels stay on one edge.",
                        )
                        .child(Menubar::new("doc-checks").menu(view)),
                )
                .into_any_element()
        }),
        Story::new("Bare", |_, _, _| bar("bare").into_any_element()),
        Story::new("With checks", |state, this, _| {
            let density = state.density.clone();
            let mut view = MenubarMenu::new("View");
            for name in ["Compact", "Comfortable"] {
                view = view.item(
                    MenuItem::new(name)
                        .checked(density == name)
                        .on_select(act(this, move |state, _, _| state.density = name.into())),
                );
            }
            Menubar::new("with-checks").menu(view).into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            bar("disabled")
                .menu(
                    MenubarMenu::new("Window")
                        .disabled(true)
                        .item(MenuItem::new("Minimise")),
                )
                .into_any_element()
        }),
    ],
);
