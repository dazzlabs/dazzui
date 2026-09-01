use dazzui::assets::icons;
use dazzui::{
    ActiveTheme, AlertBanner, Button, Card, CardVariant, ControlVariant, EmptyState, Icon, Menu,
    MenuItem, Tint,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, Styled, div};

use crate::gallery::{Col, PageTitle, Specimen};
use crate::{Playground, act};

/// What the app says back: banners that stay, toasts that pass, a dialog that
/// blocks, and the empty state that stands in for content.
pub fn feedback(
    _state: &Playground,
    this: &Entity<Playground>,
    cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let vars = cx.theme().vars;

    let banners = Col::new()
        .child(
            AlertBanner::new()
                .tint(Tint::Info)
                .title("A new version is available")
                .description("Restart to finish updating to 2.1.0.")
                .icon(Icon::new(icons::ALERT_16_REGULAR).size(vars.spacing_4)),
        )
        .child(
            AlertBanner::new()
                .tint(Tint::Success)
                .title("Tokens rebuilt")
                .description("Four platform packages were refreshed."),
        )
        .child(
            AlertBanner::new()
                .tint(Tint::Warning)
                .title("Ramp moves only part-way")
                .description("color.warning cannot move whole under bright-dark."),
        )
        .child(
            AlertBanner::new()
                .tint(Tint::Danger)
                .title("The build failed")
                .description("dazzui: 4 errors in sidebar.rs."),
        );

    vec![
        PageTitle::new(
            "Feedback",
            "The tint carries the meaning; the shape carries how long it stays.",
        )
        .into_any_element(),
        Specimen::new("Alert banners")
            .child(div().max_w(vars.toast_max_width * 1.5).child(banners))
            .into_any_element(),
        Specimen::new("Dialog")
            .note("the scrim is the content colour at its own alpha")
            .child(
                Button::new("open-dialog", "Show a dialog")
                    .variant(ControlVariant::Filled)
                    .on_click({
                        let open = act(this, |state, _, _| state.dialog = true);
                        move |_, window, cx| open(window, cx)
                    }),
            )
            .into_any_element(),
        Specimen::new("Toast")
            .child(Button::new("post-toast", "Post a toast").on_click({
                let post = act(this, |state, _, _| state.toast = true);
                move |_, window, cx| post(window, cx)
            }))
            .into_any_element(),
        Specimen::new("Menu")
            .note("a selection or a click outside closes it")
            .child(
                Menu::new("actions")
                    .item(
                        MenuItem::new("Rename")
                            .icon(Icon::new(icons::RENAME_16_REGULAR))
                            .shortcut("⌘R"),
                    )
                    .item(
                        MenuItem::new("Duplicate")
                            .icon(Icon::new(icons::SQUARE_MULTIPLE_16_REGULAR)),
                    )
                    .item(MenuItem::new("Show hidden files").checked(true))
                    .item(
                        MenuItem::new("Delete")
                            .icon(Icon::new(icons::DELETE_16_REGULAR))
                            .disabled(true),
                    )
                    .trigger(|_open, _| {
                        Button::new("actions-trigger", "Actions")
                            .shortcut("▾")
                            .into_any_element()
                    }),
            )
            .into_any_element(),
        Specimen::new("Empty state")
            .child(Card::new().variant(CardVariant::Sunken).child(
                EmptyState::new("No components match this filter").action(
                    Button::new("clear-filter", "Clear the filter").variant(ControlVariant::Tinted),
                ),
            ))
            .into_any_element(),
    ]
}
