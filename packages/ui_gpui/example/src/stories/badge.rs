//! `Components/Badge` — an annotation, not a control.

use dazzui::{
    Badge, BadgeSize, BadgeVariant, Card, CardVariant, Table, TableAlign, TableCell, TableHead,
    TableRow, Tint,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, capitalize, case};
use crate::storybook::stage::{Col, Row};
use crate::storybook::{Story, StoryPage};

pub const PAGE: StoryPage = StoryPage::new(
    "Badge",
    &[
        Story::page("Overview", |_, _, _| {
            let mut variants = Specimens::new();
            for variant in BadgeVariant::ALL {
                let mut row = Specimen::new(case(variant));
                for tint in Tint::ALL {
                    row = row.child(
                        Badge::new(capitalize(tint.name()))
                            .variant(variant)
                            .tint(tint),
                    );
                }
                variants = variants.child(row);
            }

            let mut sizes = Specimens::new();
            for size in BadgeSize::ALL {
                sizes = sizes.child(
                    Specimen::new(case(size))
                        .child(Badge::new("Beta").size(size))
                        .child(Badge::new("Passed").size(size).tint(Tint::Success))
                        .child(Badge::new("12").size(size).variant(BadgeVariant::Filled)),
                );
            }

            DocPage::new("Badge")
                .description(
                    "A tag: the display cut at label weight in a tight chip-cornered capsule. A \
                     badge is not interactive, so it deliberately stays out of `pressable` and \
                     reads the resting recipes directly — a label that changed colour under the \
                     pointer would read as something you can press.",
                )
                .child(
                    Section::new("Variants")
                        .description(
                            "Three of them are the control recipes read at rest. `raised` is \
                             the one a badge owns: paper fill and quiet ink, the chip that has \
                             to stay legible on a grey card where a tinted wash would disappear.",
                        )
                        .child(variants),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "A badge is an annotation rather than a control, so it has three \
                             sizes of its own and never reaches control height. The face and the \
                             weight never soften: only the box and, at `large`, one step of type.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Where it lands")
                        .description(
                            "A status in a table row, a count beside a label, a state on a \
                             card. Its tint carries the meaning, which is why the status ramps \
                             are the ones it is usually read against.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("states")
                                        .child(Badge::new("Passed").tint(Tint::Success))
                                        .child(Badge::new("Running").tint(Tint::Info))
                                        .child(Badge::new("Failed").tint(Tint::Danger)),
                                )
                                .child(
                                    Specimen::new("counts")
                                        .child(
                                            Badge::new("3")
                                                .size(BadgeSize::Small)
                                                .variant(BadgeVariant::Filled),
                                        )
                                        .child(
                                            Badge::new("99+")
                                                .size(BadgeSize::Small)
                                                .variant(BadgeVariant::Outlined),
                                        ),
                                )
                                .child(Specimen::new("on grey").child(on_a_card())),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Tinted", |_, _, _| Badge::new("Beta").into_any_element()),
        Story::new("Filled", |_, _, _| {
            Badge::new("Beta")
                .variant(BadgeVariant::Filled)
                .into_any_element()
        }),
        Story::new("Outlined", |_, _, _| {
            Badge::new("Beta")
                .variant(BadgeVariant::Outlined)
                .into_any_element()
        }),
        Story::new("Raised", |_, _, _| {
            Badge::new("Beta")
                .variant(BadgeVariant::Raised)
                .into_any_element()
        }),
        Story::new("All variants", |_, _, _| {
            let mut row = Row::new();
            for variant in BadgeVariant::ALL {
                row = row.child(Badge::new(capitalize(&case(variant))).variant(variant));
            }
            row.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new();
            for variant in BadgeVariant::ALL {
                let mut row = Row::new();
                for tint in Tint::ALL {
                    row = row.child(
                        Badge::new(capitalize(tint.name()))
                            .variant(variant)
                            .tint(tint),
                    );
                }
                col = col.child(row);
            }
            col.into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut row = Row::new();
            for size in BadgeSize::ALL {
                row = row.child(Badge::new(capitalize(&case(size))).size(size));
            }
            row.into_any_element()
        }),
        Story::new("States", |_, _, _| {
            Row::new()
                .child(Badge::new("Passed").tint(Tint::Success))
                .child(Badge::new("Running").tint(Tint::Info))
                .child(Badge::new("Queued").tint(Tint::Neutral))
                .child(Badge::new("Failed").tint(Tint::Danger))
                .child(Badge::new("Deprecated").tint(Tint::Warning))
                .into_any_element()
        }),
        Story::new("Counts", |_, _, _| {
            Row::new()
                .child(
                    Badge::new("3")
                        .size(BadgeSize::Small)
                        .variant(BadgeVariant::Filled),
                )
                .child(Badge::new("12").size(BadgeSize::Small))
                .child(
                    Badge::new("99+")
                        .size(BadgeSize::Small)
                        .variant(BadgeVariant::Outlined),
                )
                .into_any_element()
        }),
        Story::new("In a table", |_, _, _| {
            div()
                .w(px(380.))
                .child(
                    Table::new()
                        .child(
                            TableHead::new()
                                .child(TableCell::new().head(true).child("Build"))
                                .child(
                                    TableCell::new()
                                        .head(true)
                                        .align(TableAlign::End)
                                        .child("Status"),
                                ),
                        )
                        .child(
                            TableRow::new("aurora")
                                .child(TableCell::new().child("aurora"))
                                .child(
                                    TableCell::new().align(TableAlign::End).child(
                                        Badge::new("Passed")
                                            .size(BadgeSize::Small)
                                            .tint(Tint::Success),
                                    ),
                                ),
                        )
                        .child(
                            TableRow::new("cinder")
                                .child(TableCell::new().child("cinder"))
                                .child(
                                    TableCell::new().align(TableAlign::End).child(
                                        Badge::new("Failed")
                                            .size(BadgeSize::Small)
                                            .tint(Tint::Danger),
                                    ),
                                ),
                        ),
                )
                .into_any_element()
        }),
        Story::new("On a card", |_, _, _| on_a_card().into_any_element()),
    ],
);

/// The chip that has to stay legible on a grey card.
fn on_a_card() -> impl IntoElement {
    Card::new().variant(CardVariant::Sunken).child(
        div()
            .flex()
            .w(px(240.))
            .items_center()
            .justify_between()
            .child("Tokens")
            .child(
                Badge::new(SharedString::from("Generated"))
                    .size(BadgeSize::Small)
                    .variant(BadgeVariant::Raised),
            ),
    )
}
