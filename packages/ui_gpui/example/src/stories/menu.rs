//! `Components/Menu` — the floating sheet, and the rows in it.

use dazzui::{
    ActiveTheme, Button, Icon, IconButton, Menu, MenuAlign, MenuItem, assets::icons,
};
use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize, case};
use crate::storybook::stage::Row;
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Menu",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            DocPage::new("Menu")
                .description(
                    "A trigger and a floating sheet of rows. The sheet is one drawing shared \
                     with the select's list and the tooltip; the menu adds only what is \
                     genuinely its own — the icon slot, the shortcut column, and the tick that \
                     appears once any row is checkable.",
                )
                .child(
                    Section::new("Rows")
                        .description(
                            "A row can carry a leading glyph, a trailing shortcut, a check, and \
                             a disabled state. The shortcut is a faint hint in the display cut \
                             rather than a boxed cap: a cap in a menu row would be furniture.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("plain").child(plain("overview")))
                                .child(Specimen::new("icons").child(with_icons("overview")))
                                .child(Specimen::new("checked").child(checked(state, this))),
                        ),
                )
                .child(
                    Section::new("Alignment")
                        .description(
                            "The sheet hangs from the trigger's trailing edge by default, which \
                             is where a `▾` button wants it. `start` hangs it from the leading \
                             edge instead. The gap between the trigger and the sheet is a token \
                             rather than a number passed in code.",
                        )
                        .child(
                            Row::new()
                                .gap(vars.spacing_20)
                                .child(aligned(MenuAlign::Start))
                                .child(aligned(MenuAlign::End)),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Default", |_, _, _| plain("story").into_any_element()),
        Story::new("With shortcuts", |_, _, _| {
            Menu::new("shortcuts")
                .item(MenuItem::new("Rename").shortcut("⌘R"))
                .item(MenuItem::new("Duplicate").shortcut("⇧⌘D"))
                .item(MenuItem::new("Delete").shortcut("⌫"))
                .trigger(trigger("Actions"))
                .into_any_element()
        }),
        Story::new("With icons", |_, _, _| {
            with_icons("story").into_any_element()
        }),
        Story::new("Checked", |state, this, _| {
            checked(state, this).into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            Menu::new("disabled-rows")
                .item(MenuItem::new("Rename").icon(Icon::new(icons::RENAME_16_REGULAR)))
                .item(
                    MenuItem::new("Delete")
                        .icon(Icon::new(icons::DELETE_16_REGULAR))
                        .disabled(true),
                )
                .trigger(trigger("Actions"))
                .into_any_element()
        }),
        Story::new("Alignment", |_, _, cx| {
            let vars = cx.theme().vars;
            Row::new()
                .gap(vars.spacing_20)
                .child(aligned(MenuAlign::Start))
                .child(aligned(MenuAlign::End))
                .into_any_element()
        }),
        Story::new("From an icon button", |_, _, _| {
            Menu::new("from-icon")
                .item(MenuItem::new("Rename").icon(Icon::new(icons::RENAME_16_REGULAR)))
                .item(MenuItem::new("Duplicate").icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)))
                .trigger(|_open, _| {
                    IconButton::new(
                        "from-icon-trigger",
                        "Actions",
                        Icon::new(icons::SETTINGS_16_REGULAR),
                    )
                    .into_any_element()
                })
                .into_any_element()
        }),
        Story::new("A disabled menu", |_, _, _| {
            Menu::new("disabled-menu")
                .item(MenuItem::new("Rename"))
                .disabled(true)
                .trigger(trigger("Actions"))
                .into_any_element()
        }),
    ],
);

/// The `▾` button a menu hangs from — the trigger every plain story wears.
fn trigger(label: &'static str) -> impl FnOnce(bool, &mut gpui::App) -> AnyElement + 'static {
    move |_open, _cx| {
        Button::new(SharedString::from(format!("trigger-{label}")), label)
            .shortcut("▾")
            .into_any_element()
    }
}

fn plain(id: &'static str) -> impl IntoElement {
    Menu::new(SharedString::from(format!("{id}-plain")))
        .item(MenuItem::new("Rename"))
        .item(MenuItem::new("Duplicate"))
        .item(MenuItem::new("Delete"))
        .trigger(trigger("Actions"))
}

fn with_icons(id: &'static str) -> impl IntoElement {
    Menu::new(SharedString::from(format!("{id}-icons")))
        .item(
            MenuItem::new("Rename")
                .icon(Icon::new(icons::RENAME_16_REGULAR))
                .shortcut("⌘R"),
        )
        .item(MenuItem::new("Duplicate").icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)))
        .item(MenuItem::new("Delete").icon(Icon::new(icons::DELETE_16_REGULAR)))
        .trigger(trigger("Actions"))
}

/// The tick column: a menu that carries any checked item reserves the column
/// for all of them, so the labels stay on one edge.
fn checked(state: &crate::Playground, this: &gpui::Entity<crate::Playground>) -> impl IntoElement {
    Menu::new("checked")
        .item(
            MenuItem::new("Show hidden files")
                .checked(state.hidden_files)
                .on_select(act(this, |state, _, _| {
                    state.hidden_files = !state.hidden_files
                })),
        )
        .item(
            MenuItem::new("Show the sidebar")
                .checked(state.show_sidebar)
                .on_select(act(this, |state, _, _| {
                    state.show_sidebar = !state.show_sidebar
                })),
        )
        .trigger(trigger("View"))
}

fn aligned(align: MenuAlign) -> impl IntoElement {
    let name: &'static str = match align {
        MenuAlign::Start => "start",
        MenuAlign::End => "end",
    };

    div().w(px(120.)).child(
        Menu::new(SharedString::from(format!("align-{name}")))
            .align(align)
            .item(MenuItem::new("Rename"))
            .item(MenuItem::new("Duplicate"))
            .trigger(move |_open, _cx| {
                Button::new(
                    SharedString::from(format!("align-trigger-{name}")),
                    capitalize(&case(align)),
                )
                .shortcut("▾")
                .into_any_element()
            }),
    )
}
