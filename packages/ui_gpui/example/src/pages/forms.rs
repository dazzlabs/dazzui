use dazzui::{
    ActiveTheme, Checkbox, FormField, KeyCap, OptionCard, RadioGroup, SearchField, Select, Switch,
    SwitchSize, TextField, TextFieldState,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, SharedString, Styled, div};

use crate::gallery::{Col, PageTitle, Row, Specimen};
use crate::{Playground, act_with};

/// The form vocabulary, shown as a form rather than as a shelf of specimens —
/// the field chrome, the label and hint above and below it, and the error
/// state are only really legible next to each other.
pub fn forms(
    state: &Playground,
    this: &Entity<Playground>,
    cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let vars = cx.theme().vars;

    vec![
        PageTitle::new(
            "Forms",
            "Every field is pinned to the same control height, so a row of them lines up.",
        )
        .into_any_element(),
        Specimen::new("Fields")
            .child(
                div().max_w(vars.dialog_width).child(
                    Col::new()
                        .gap(vars.spacing_4)
                        .child(FormField::new("Workspace name").child(TextField::new(&state.name)))
                        .child(
                            FormField::new("Email")
                                .hint("We could not reach this address.")
                                .state(TextFieldState::Error)
                                .child(TextField::new(&state.email).state(TextFieldState::Error)),
                        )
                        .child(
                            FormField::new("Region").child(
                                Select::new("region", state.region.clone())
                                    .option("us", "United States")
                                    .option("eu", "Europe")
                                    .option("ap", "Asia Pacific")
                                    .on_select(act_with(
                                        this,
                                        |state, value: &SharedString, _, _| {
                                            state.region = value.clone();
                                        },
                                    )),
                            ),
                        )
                        .child(
                            FormField::new("Notes")
                                .hint("Markdown is supported.")
                                .child(TextField::new(&state.notes).multiline(true)),
                        ),
                ),
            )
            .into_any_element(),
        Specimen::new("Search")
            .child(
                div()
                    .max_w(vars.dialog_width)
                    .child(SearchField::new("search", &state.search).hint(KeyCap::new("⌘F"))),
            )
            .into_any_element(),
        Specimen::new("Checkboxes")
            .note("a note rides on the label, not on the row")
            .child(
                Col::new()
                    .child(
                        Checkbox::new(
                            "newsletter",
                            "Send me the monthly newsletter",
                            state.newsletter,
                        )
                        .on_change(act_with(
                            this,
                            |state, checked: &bool, _, _| {
                                state.newsletter = *checked;
                            },
                        )),
                    )
                    .child(
                        Checkbox::new("telemetry", "Share anonymous usage data", state.telemetry)
                            .note("Crash reports only.")
                            .on_change(act_with(this, |state, checked: &bool, _, _| {
                                state.telemetry = *checked;
                            })),
                    )
                    .child(
                        Checkbox::new("unavailable", "Unavailable on this plan", false)
                            .disabled(true),
                    ),
            )
            .into_any_element(),
        Specimen::new("Radio group")
            .child(
                RadioGroup::new("plan", state.plan.clone())
                    .option("solo", "Solo")
                    .option("team", "Team")
                    .disabled_option("enterprise", "Enterprise")
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.plan = value.clone();
                    })),
            )
            .into_any_element(),
        Specimen::new("Switches")
            .child(
                Row::new()
                    .gap(vars.spacing_4)
                    .child(
                        Switch::new("notify-small", state.notify)
                            .size(SwitchSize::Small)
                            .on_change(act_with(this, |state, checked: &bool, _, _| {
                                state.notify = *checked;
                            })),
                    )
                    .child(
                        Switch::new("notify", state.notify)
                            .on_change(act_with(this, |state, checked: &bool, _, _| {
                                state.notify = *checked
                            })),
                    )
                    .child(
                        Switch::new("notify-large", state.notify)
                            .size(SwitchSize::Large)
                            .on_change(act_with(this, |state, checked: &bool, _, _| {
                                state.notify = *checked;
                            })),
                    )
                    .child(Switch::new("notify-off", false).disabled(true)),
            )
            .into_any_element(),
        Specimen::new("Option cards")
            .note("selection thickens the edge as well as colouring it")
            .child(
                Row::new()
                    .gap(vars.spacing_3)
                    .child(
                        OptionCard::new("auto", "Automatic")
                            .description("Follow the system appearance.")
                            .selected(state.appearance_card == "auto")
                            .on_select({
                                let pick = act_with(this, |state, value: &SharedString, _, _| {
                                    state.appearance_card = value.clone();
                                });
                                move |window, cx| pick(&SharedString::from("auto"), window, cx)
                            }),
                    )
                    .child(
                        OptionCard::new("manual", "Manual")
                            .description("Pick a theme and keep it.")
                            .selected(state.appearance_card == "manual")
                            .on_select({
                                let pick = act_with(this, |state, value: &SharedString, _, _| {
                                    state.appearance_card = value.clone();
                                });
                                move |window, cx| pick(&SharedString::from("manual"), window, cx)
                            }),
                    ),
            )
            .into_any_element(),
    ]
}
