use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Badge, BadgeSize, Card, CardVariant, Progress, ProgressSize, Spinner, SpinnerSize,
    Step, StepList, StepStatus, Table, TableAlign, TableCell, TableHead, TableRow, Tint,
};
use gpui::{AnyElement, Context, Entity, IntoElement, ParentElement, SharedString, Styled, div};

use crate::gallery::{Col, PageTitle, Row, Specimen};
use crate::pages::controls::capitalize;
use crate::{Playground, act_with};

/// Surfaces and the things that sit on them: cards, a table, progress, and
/// the step list that reports on a long-running job.
pub fn data(
    state: &Playground,
    this: &Entity<Playground>,
    cx: &mut Context<Playground>,
) -> Vec<AnyElement> {
    let vars = cx.theme().vars;

    let builds = [
        ("aurora", "main", "Passed", Tint::Success),
        ("beacon", "release/2.1", "Running", Tint::Info),
        ("cinder", "fix/tokens", "Failed", Tint::Danger),
    ];

    let mut cards = Row::new().gap(vars.spacing_3);
    for (name, variant) in [
        ("Raised", CardVariant::Raised),
        ("Sunken", CardVariant::Sunken),
        ("Outlined", CardVariant::Outlined),
        ("Tinted", CardVariant::Tinted),
    ] {
        cards = cards.child(
            Card::new()
                .variant(variant)
                .child(
                    div()
                        .type_style(vars.label_medium)
                        .text_color(vars.color_content)
                        .child(name),
                )
                .child(
                    div()
                        .type_style(vars.caption_small)
                        .text_color(vars.color_content_muted)
                        .child("A surface."),
                ),
        );
    }

    let mut table = Table::new().child(
        TableHead::new()
            .child(TableCell::new().head(true).child("Build"))
            .child(TableCell::new().head(true).child("Branch"))
            .child(
                TableCell::new()
                    .head(true)
                    .align(TableAlign::End)
                    .child("Status"),
            ),
    );
    for (name, branch, status, tint) in builds {
        table = table.child(
            TableRow::new(name)
                .active(state.build == name)
                .on_click({
                    let pick = act_with(this, |state, value: &SharedString, _, _| {
                        state.build = value.clone();
                    });
                    move |window, cx| pick(&SharedString::from(name), window, cx)
                })
                .child(TableCell::new().child(name))
                .child(TableCell::new().child(branch))
                .child(
                    TableCell::new()
                        .align(TableAlign::End)
                        .child(Badge::new(status).size(BadgeSize::Small).tint(tint)),
                ),
        );
    }

    vec![
        PageTitle::new(
            "Data",
            "A card is a surface; a row is a surface that answers the pointer.",
        )
        .into_any_element(),
        Specimen::new("Cards")
            .note("raised · sunken · outlined · tinted")
            .child(cards)
            .into_any_element(),
        Specimen::new("Table")
            .note("hover outranks selection, exactly as the stylesheet has it")
            .child(table)
            .into_any_element(),
        Specimen::new("Progress")
            .child(
                div().max_w(vars.dialog_width).child(
                    Col::new()
                        .gap(vars.spacing_3)
                        .child(Progress::new(35.))
                        .child(Progress::new(70.).size(ProgressSize::Medium).gradient(true))
                        // No value at all is the indeterminate sweep.
                        .child(
                            Progress::indeterminate()
                                .size(ProgressSize::Large)
                                .tint(Tint::Info),
                        ),
                ),
            )
            .into_any_element(),
        Specimen::new("Spinners")
            .child(
                Row::new()
                    .gap(vars.spacing_4)
                    .child(Spinner::new().size(SpinnerSize::Small))
                    .child(Spinner::new())
                    .child(Spinner::new().size(SpinnerSize::Large).tint(Tint::Neutral)),
            )
            .into_any_element(),
        Specimen::new("Steps")
            .child(
                StepList::new()
                    .child(Step::new("Resolve the token graph", StepStatus::Done))
                    .child(Step::new("Write the platform sources", StepStatus::Done))
                    .child(Step::new("Compile dazzui", StepStatus::Active).meta("14s"))
                    .child(Step::new("Publish the crate", StepStatus::Idle)),
            )
            .into_any_element(),
        Specimen::new("Tints")
            .note(SharedString::from(capitalize("one ramp per role")))
            .child(
                Row::new()
                    .children(Tint::ALL.map(|tint| Badge::new(capitalize(tint.name())).tint(tint))),
            )
            .into_any_element(),
    ]
}
