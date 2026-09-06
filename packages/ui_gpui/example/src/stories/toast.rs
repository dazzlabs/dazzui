//! `Components/Toast` — what the app says in passing.

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Button, ControlVariant, Icon, Tint, Toast, ToastPlacement, ToastViewport,
    assets::icons,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div};

use crate::on_click;
use crate::storybook::gallery::{DocPage, Section, TokenTable, capitalize};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Toast",
    &[
        Story::page("Overview", |_, _, cx| {
            let vars = cx.theme().vars;

            let mut tints = Col::new();
            for tint in Tint::ALL {
                tints = tints.child(
                    Toast::new(
                        SharedString::from(format!("tint-{}", tint.name())),
                        capitalize(tint.name()),
                    )
                    .tint(tint)
                    .icon(Icon::new(icons::ALERT_16_REGULAR))
                    .on_dismiss(|_, _| {}),
                );
            }

            DocPage::new("Toast")
                .description(
                    "A message that passes, where a callout stays. The tint carries the meaning \
                     and the shape carries how long it is around — which is the whole of the \
                     difference between the two components. Lifetime belongs to the host: a few \
                     seconds for a plain notice, longer when it carries an action, and a failure \
                     that stays until it is dismissed.",
                )
                .child(
                    Section::new("Shapes")
                        .description(
                            "The message alone; the message with a dismiss; or the full shape — \
                             a leading glyph, the message, and one action. Each is as wide as \
                             what it holds, capped at the toast's own maximum, so a long message \
                             truncates rather than pushing the box wider.",
                        )
                        .child(
                            Col::new()
                                .child(Toast::new("plain", "Message only."))
                                .child(
                                    Toast::new("dismiss", "With a dismiss.")
                                        .tint(Tint::Info)
                                        .on_dismiss(|_, _| {}),
                                )
                                .child(detailed("doc")),
                        ),
                )
                .child(
                    Section::new("Tints")
                        .description(
                            "The tint reaches the leading glyph and whatever the trailing action \
                             is tinted with, and stops there: the box stays paper, because a \
                             toast is read over whatever happens to be under it.",
                        )
                        .child(tints),
                )
                .child(
                    Section::new("Tokens")
                        .description("The one the toast owns.")
                        .child(TokenTable::new().length("toast.max-width", vars.toast_max_width)),
                )
                .into_any_element()
        }),
        Story::new("Default", |_, _, _| {
            Toast::new("default", "Tokens rebuilt — four packages refreshed.").into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    Toast::new(
                        SharedString::from(format!("story-tint-{}", tint.name())),
                        capitalize(tint.name()),
                    )
                    .tint(tint)
                    .icon(Icon::new(icons::ALERT_16_REGULAR))
                    .on_dismiss(|_, _| {}),
                );
            }
            col.into_any_element()
        }),
        Story::new("Shapes", |_, _, _| {
            Col::new()
                .child(Toast::new("plain", "Message only."))
                .child(
                    Toast::new("dismiss", "With a dismiss.")
                        .tint(Tint::Info)
                        .on_dismiss(|_, _| {}),
                )
                .child(detailed("story"))
                .into_any_element()
        }),
        Story::new("Truncation", |_, _, _| {
            Toast::new(
                "truncation",
                "A message long enough to run past the toast's own maximum width, where it \
                 truncates rather than pushing the box wider.",
            )
            .tint(Tint::Warning)
            .on_dismiss(|_, _| {})
            .into_any_element()
        }),
        Story::fullscreen("A stack", |_, _, _| {
            ToastViewport::new()
                .child(
                    Toast::new("stack-a", "Tokens rebuilt.")
                        .tint(Tint::Success)
                        .on_dismiss(|_, _| {}),
                )
                .child(
                    Toast::new("stack-b", "Two packages still building…")
                        .tint(Tint::Info)
                        .on_dismiss(|_, _| {}),
                )
                .into_any_element()
        }),
        Story::fullscreen("In a top viewport", |_, _, _| {
            ToastViewport::new()
                .placement(ToastPlacement::Top)
                .child(
                    Toast::new("top", "Tokens rebuilt.")
                        .tint(Tint::Success)
                        .on_dismiss(|_, _| {}),
                )
                .into_any_element()
        }),
        Story::fullscreen("Posting one", |state, this, _| {
            div()
                .relative()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Button::new("post", "Rebuild tokens")
                        .variant(ControlVariant::Filled)
                        .on_click(on_click(this, |state| state.toast = true)),
                )
                .when(state.toast, |el| {
                    el.child(
                        ToastViewport::new().child(
                            Toast::new("posted", "Tokens rebuilt — four packages refreshed.")
                                .tint(Tint::Success)
                                .on_dismiss({
                                    let dismiss =
                                        crate::act(this, |state, _, _| state.toast = false);
                                    move |window, cx| dismiss(window, cx)
                                }),
                        ),
                    )
                })
                .into_any_element()
        }),
    ],
);

/// The full shape: a glyph, the message, an action and the dismiss.
fn detailed(id: &'static str) -> impl IntoElement {
    Toast::new(
        SharedString::from(format!("{id}-detailed")),
        "The build failed — 4 errors in sidebar.rs.",
    )
    .tint(Tint::Danger)
    .icon(Icon::new(icons::ALERT_16_REGULAR))
    .action(
        Button::new(SharedString::from(format!("{id}-show")), "Show")
            .variant(ControlVariant::Plain)
            .tint(Tint::Danger),
    )
    .on_dismiss(|_, _| {})
}
