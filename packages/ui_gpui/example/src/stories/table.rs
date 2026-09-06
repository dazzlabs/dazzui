//! `Components/Table` — rows of one thing.

use dazzui::{
    Badge, BadgeSize, Card, CardVariant, EmptyState, Switch, Table, TableAlign, TableCell,
    TableHead, TableRow, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, capitalize};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act, act_with};

const BUILDS: [(&str, &str, &str, Tint); 3] = [
    ("aurora", "main", "Passed", Tint::Success),
    ("beacon", "release/2.1", "Running", Tint::Info),
    ("cinder", "fix/tokens", "Failed", Tint::Danger),
];

pub const PAGE: StoryPage = StoryPage::new(
    "Table",
    &[
        Story::page("Overview", |state, this, _| {
            DocPage::new("Table")
                .description(
                    "A header row and a run of rows under it. The parts are separate elements \
                     rather than a data source, so a cell holds whatever a cell should hold — a \
                     badge, a switch, a number set to the trailing edge.",
                )
                .child(
                    Section::new("Rows")
                        .description(
                            "A row is inert unless it is told otherwise. `interactive` gives it \
                             the hover wash and a click; `active` marks the selected one. Hover \
                             outranks selection, so moving over a selected row still says which \
                             row the pointer is on.",
                        )
                        .child(
                            Row::new()
                                .align_top(true)
                                .child(builds(state, this, true, "doc-live"))
                                .child(builds(state, this, false, "doc-inert")),
                        ),
                )
                .child(
                    Section::new("Cells")
                        .description(
                            "A cell aligns to the start by default and to the end where it \
                             holds a number or a control — which is what keeps a column of \
                             figures readable and a column of switches on one edge.",
                        )
                        .child(with_controls(state, this)),
                )
                .child(
                    Section::new("Empty")
                        .description(
                            "A table with no rows is not a table with a message in it: the \
                             empty state replaces the table, in the well the table would have \
                             sat in.",
                        )
                        .child(empty()),
                )
                .into_any_element()
        }),
        Story::new("Selectable", |state, this, _| {
            builds(state, this, true, "selectable").into_any_element()
        }),
        Story::new("Inert", |state, this, _| {
            builds(state, this, false, "inert").into_any_element()
        }),
        Story::new("Alignment", |_, _, _| {
            div()
                .w(px(380.))
                .child(
                    Table::new()
                        .child(
                            TableHead::new()
                                .child(TableCell::new().head(true).child("Package"))
                                .child(
                                    TableCell::new()
                                        .head(true)
                                        .align(TableAlign::End)
                                        .child("Tokens"),
                                ),
                        )
                        .child(
                            TableRow::new("react")
                                .child(TableCell::new().child("ui_react"))
                                .child(TableCell::new().align(TableAlign::End).child("312")),
                        )
                        .child(
                            TableRow::new("gpui")
                                .child(TableCell::new().child("ui_gpui"))
                                .child(TableCell::new().align(TableAlign::End).child("312")),
                        ),
                )
                .into_any_element()
        }),
        Story::new("With controls", |state, this, _| {
            with_controls(state, this).into_any_element()
        }),
        Story::new("Tints", |state, this, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    div().w(px(320.)).child(
                        Table::new().tint(tint).child(
                            TableRow::new(SharedString::from(format!("tint-{}", tint.name())))
                                .active(true)
                                .interactive(true)
                                .on_click(act(this, |_, _, _| {}))
                                .child(TableCell::new().child(capitalize(tint.name())))
                                .child(TableCell::new().align(TableAlign::End).child("selected")),
                        ),
                    ),
                );
            }
            let _ = state;
            col.into_any_element()
        }),
        Story::new("Empty", |_, _, _| empty().into_any_element()),
    ],
);

fn builds(
    state: &Playground,
    this: &Entity<Playground>,
    interactive: bool,
    id: &'static str,
) -> impl IntoElement {
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

    for (name, branch, status, tint) in BUILDS {
        let mut row = TableRow::new(SharedString::from(format!("{id}-{name}")))
            .active(interactive && state.build == name)
            .interactive(interactive);
        if interactive {
            row = row.on_click(act(this, move |state, _, _| state.build = name.into()));
        }
        table = table.child(
            row.child(TableCell::new().child(name))
                .child(TableCell::new().child(branch))
                .child(
                    TableCell::new()
                        .align(TableAlign::End)
                        .child(Badge::new(status).size(BadgeSize::Small).tint(tint)),
                ),
        );
    }

    div().w(px(420.)).child(table)
}

fn with_controls(state: &Playground, this: &Entity<Playground>) -> impl IntoElement {
    let mut table = Table::new().child(
        TableHead::new()
            .child(TableCell::new().head(true).child("Build"))
            .child(
                TableCell::new()
                    .head(true)
                    .align(TableAlign::End)
                    .child("Notify"),
            ),
    );

    for (name, _, _, _) in BUILDS {
        let on = state.notified.iter().any(|value| value.as_ref() == name);
        table = table.child(
            TableRow::new(SharedString::from(format!("notify-{name}")))
                .child(TableCell::new().child(name))
                .child(
                    TableCell::new().align(TableAlign::End).child(
                        Switch::new(SharedString::from(format!("switch-{name}")), on)
                            .size(dazzui::SwitchSize::Small)
                            .on_change(act_with(this, move |state, value: &bool, _, _| {
                                state.notified.retain(|entry| entry.as_ref() != name);
                                if *value {
                                    state.notified.push(name.into());
                                }
                            })),
                    ),
                ),
        );
    }

    div().w(px(380.)).child(table)
}

fn empty() -> impl IntoElement {
    div().w(px(420.)).child(
        Card::new()
            .variant(CardVariant::Sunken)
            .child(EmptyState::new("No builds have run on this branch")),
    )
}
