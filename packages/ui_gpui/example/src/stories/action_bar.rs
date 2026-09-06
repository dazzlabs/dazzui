//! `Components/ActionBar` — the row a dialog footer, a toolbar and a form's
//! last line are all the same thing.

use dazzui::{
    ActionBar, ActionBarAlign, ActiveTheme, Badge, BadgeSize, Button, ControlVariant, Icon,
    IconButton, KeyCap, KeyCapSize, Tint, assets::icons,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::separator;
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "ActionBar",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut alignments = Specimens::new();
            for align in [
                ActionBarAlign::Start,
                ActionBarAlign::End,
                ActionBarAlign::Between,
            ] {
                alignments = alignments.child(Specimen::new(align_name(align)).child(bar(align)));
            }

            DocPage::new("ActionBar")
                .description(
                    "A row of controls kept on one rhythm. It owns no colour and no box — only \
                     the gap — so it takes the shared control gap rather than a token of its \
                     own: the small profile's, whatever the children's size, since the design \
                     draws every action row on that one step.",
                )
                .child(
                    Section::new("Alignment")
                        .description(
                            "`start` is the default. `end` is a dialog footer's usual shape, \
                             with the confirming action last. `between` hands the slack to the \
                             gaps *between* the children, which is what puts a destructive \
                             action opposite the pair that confirms.",
                        )
                        .child(alignments),
                )
                .child(
                    Section::new("What it holds")
                        .description(
                            "Anything on the control rhythm: buttons, icon buttons with a \
                             vertical rule between runs, a key hint, a badge that says what the \
                             row is about. The bar never colours or sizes what it holds.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("toolbar").child(
                                    div().w(px(240.)).child(toolbar(vars.control_tiny_size)),
                                ))
                                .child(
                                    Specimen::new("confirming").child(
                                        div().w(px(300.)).child(
                                            ActionBar::new()
                                                .align(ActionBarAlign::End)
                                                .child(Button::new("cancel", "Cancel"))
                                                .child(
                                                    Button::new("save", "Save")
                                                        .variant(ControlVariant::Filled),
                                                ),
                                        ),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Start", |_, _, _| {
            bar(ActionBarAlign::Start).into_any_element()
        }),
        Story::new("End", |_, _, _| bar(ActionBarAlign::End).into_any_element()),
        Story::new("Between", |_, _, _| {
            bar(ActionBarAlign::Between).into_any_element()
        }),
        Story::new("As a toolbar", |_, _, cx| {
            let vars = cx.theme().vars;
            div()
                .w(px(320.))
                .child(toolbar(vars.control_tiny_size))
                .into_any_element()
        }),
        Story::new("An action row", |_, _, _| {
            div()
                .w(px(380.))
                .child(
                    ActionBar::new()
                        .align(ActionBarAlign::End)
                        .child(Button::new("cancel", "Cancel"))
                        .child(Button::new("save", "Save").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
        Story::new("With a key hint", |_, _, _| {
            div()
                .w(px(380.))
                .child(
                    ActionBar::new()
                        .align(ActionBarAlign::Between)
                        .child(KeyCap::new("⌘↩").size(KeyCapSize::Small))
                        .child(
                            Button::new("send", "Send")
                                .variant(ControlVariant::Filled)
                                .shortcut("⌘↩"),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Mixed controls", |_, _, _| {
            div()
                .w(px(420.))
                .child(
                    ActionBar::new()
                        .align(ActionBarAlign::Between)
                        .child(
                            Badge::new("Draft")
                                .size(BadgeSize::Small)
                                .tint(Tint::Warning),
                        )
                        .child(Button::new("publish", "Publish").variant(ControlVariant::Filled)),
                )
                .into_any_element()
        }),
    ],
);

fn align_name(align: ActionBarAlign) -> &'static str {
    match align {
        ActionBarAlign::Start => "start",
        ActionBarAlign::End => "end",
        ActionBarAlign::Between => "between",
    }
}

/// The three distributions, over the same three controls.
fn bar(align: ActionBarAlign) -> impl IntoElement {
    let name = align_name(align);

    div().w(px(420.)).child(
        ActionBar::new()
            .align(align)
            .child(
                Button::new(SharedString::from(format!("{name}-reset")), "Reset")
                    .variant(ControlVariant::Plain),
            )
            .child(Button::new(
                SharedString::from(format!("{name}-cancel")),
                "Cancel",
            ))
            .child(
                Button::new(SharedString::from(format!("{name}-apply")), "Apply")
                    .variant(ControlVariant::Filled),
            ),
    )
}

/// A toolbar: two runs of icon buttons with a rule between them.
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
        .child(
            IconButton::new("delete", "Delete", Icon::new(icons::DELETE_16_REGULAR))
                .tint(Tint::Danger),
        )
}
