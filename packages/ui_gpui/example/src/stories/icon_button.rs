//! `Components/IconButton` — the square button a toolbar is made of.

use dazzui::{
    ActionBar, ActiveTheme, Card, ControlSize, ControlVariant, Icon, IconButton, Tint,
    assets::icons,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize};
use crate::storybook::stage::{Row, separator};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "IconButton",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            let mut variants = Specimens::new();
            for variant in ControlVariant::ALL {
                let mut row = Specimen::new(variant.name());
                for tint in Tint::ALL {
                    row = row.child(
                        IconButton::new(
                            SharedString::from(format!("{}-{}", variant.name(), tint.name())),
                            capitalize(tint.name()),
                            Icon::new(icons::STAR_16_REGULAR),
                        )
                        .variant(variant)
                        .tint(tint),
                    );
                }
                variants = variants.child(row);
            }

            let mut sizes = Row::new();
            for size in ControlSize::ALL {
                sizes = sizes.child(
                    IconButton::new(
                        SharedString::from(format!("size-{}", size.name())),
                        capitalize(size.name()),
                        Icon::new(icons::STAR_16_REGULAR),
                    )
                    .size(size),
                );
            }

            DocPage::new("IconButton")
                .description(
                    "A button whose label is a glyph. It is the same control as `Button` — the \
                     same recipes, the same states — squared to the control height, with the \
                     label moved into the tooltip and the accessible name. It defaults to \
                     `plain` at the `tiny` step, because a toolbar of these should read as \
                     chrome rather than as a row of accented actions.",
                )
                .child(
                    Section::new("Variants")
                        .description(
                            "Every control recipe, on any ramp. Pass a tint where one button \
                             genuinely leads — a destructive action in a toolbar of neutral ones.",
                        )
                        .child(variants),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The control profile again, and `tiny` is the one this component \
                             was drawn for: a square on the toolbar step.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Active")
                        .description(
                            "`active` is the on-state of a button that holds a decision — a \
                             pin, a filter, a panel toggle. It takes the filled recipe, so an on \
                             button reads the way a selected row does rather than as a pressed \
                             one.",
                        )
                        .child(pin(state, this)),
                )
                .child(
                    Section::new("In a toolbar")
                        .description(
                            "Runs of buttons separated by a vertical rule, and at most one \
                             tinted button.",
                        )
                        .child(div().w(px(300.)).child(toolbar(vars.control_tiny_size))),
                )
                .into_any_element()
        }),
        Story::new("Plain", |_, _, _| {
            IconButton::new("pin", "Pin", Icon::new(icons::PIN_16_REGULAR)).into_any_element()
        }),
        Story::new("Filled", |_, _, _| {
            IconButton::new("star", "Star", Icon::new(icons::STAR_16_REGULAR))
                .variant(ControlVariant::Filled)
                .into_any_element()
        }),
        Story::new("Variants", |_, _, _| {
            let mut row = Row::new();
            for variant in ControlVariant::ALL {
                row = row.child(
                    IconButton::new(
                        variant.name(),
                        capitalize(variant.name()),
                        Icon::new(icons::STAR_16_REGULAR),
                    )
                    .variant(variant),
                );
            }
            row.into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            let mut row = Row::new();
            for variant in ControlVariant::ALL {
                row = row.child(
                    IconButton::new(
                        SharedString::from(format!("dead-{}", variant.name())),
                        capitalize(variant.name()),
                        Icon::new(icons::STAR_16_REGULAR),
                    )
                    .variant(variant)
                    .disabled(true),
                );
            }
            row.into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new();
            for size in ControlSize::ALL {
                row = row.child(
                    IconButton::new(
                        size.name(),
                        capitalize(size.name()),
                        Icon::new(icons::STAR_16_REGULAR),
                    )
                    .size(size),
                );
            }
            row.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut row = Row::new();
            for tint in Tint::ALL {
                row = row.child(
                    IconButton::new(
                        tint.name(),
                        capitalize(tint.name()),
                        Icon::new(icons::STAR_16_REGULAR),
                    )
                    .variant(ControlVariant::Tinted)
                    .tint(tint),
                );
            }
            row.into_any_element()
        }),
        Story::new("Active", |state, this, _| {
            pin(state, this).into_any_element()
        }),
        Story::new("Active tints", |_, _, _| {
            let mut row = Row::new();
            for tint in Tint::ALL {
                row = row.child(
                    IconButton::new(
                        SharedString::from(format!("on-{}", tint.name())),
                        capitalize(tint.name()),
                        Icon::new(icons::PIN_16_REGULAR),
                    )
                    .tint(tint)
                    .active(true),
                );
            }
            row.into_any_element()
        }),
        Story::new("A toolbar", |_, _, cx| {
            let vars = cx.theme().vars;
            div()
                .w(px(300.))
                .child(toolbar(vars.control_tiny_size))
                .into_any_element()
        }),
        Story::new("In a list header", |_, _, _| {
            div()
                .w(px(300.))
                .child(
                    Card::new().child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child("Builds")
                            .child(
                                ActionBar::new()
                                    .child(IconButton::new(
                                        "filter",
                                        "Filter",
                                        Icon::new(icons::SEARCH_16_REGULAR),
                                    ))
                                    .child(IconButton::new(
                                        "settings",
                                        "Settings",
                                        Icon::new(icons::SETTINGS_16_REGULAR),
                                    )),
                            ),
                    ),
                )
                .into_any_element()
        }),
    ],
);

/// The on-state: an icon button that holds a decision rather than firing one.
fn pin(state: &crate::Playground, this: &gpui::Entity<crate::Playground>) -> impl IntoElement {
    Row::new()
        .child(
            IconButton::new("pinned", "Pin", Icon::new(icons::PIN_16_REGULAR))
                .active(state.pinned)
                .on_click({
                    let toggle = act(this, |state, _, _| state.pinned = !state.pinned);
                    move |_, window, cx| toggle(window, cx)
                }),
        )
        .child(IconButton::new(
            "unpinned",
            "Pin",
            Icon::new(icons::PIN_16_REGULAR),
        ))
}

fn toolbar(height: gpui::Pixels) -> impl IntoElement {
    ActionBar::new()
        .child(IconButton::new(
            "back",
            "Back",
            Icon::new(icons::CHEVRON_LEFT_16_REGULAR),
        ))
        .child(IconButton::new(
            "forward",
            "Forward",
            Icon::new(icons::CHEVRON_RIGHT_16_REGULAR),
        ))
        .child(separator(height))
        .child(IconButton::new(
            "rename",
            "Rename",
            Icon::new(icons::RENAME_16_REGULAR),
        ))
        .child(separator(height))
        .child(
            IconButton::new("delete", "Delete", Icon::new(icons::DELETE_16_REGULAR))
                .tint(Tint::Danger),
        )
}
