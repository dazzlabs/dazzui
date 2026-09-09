//! `Components/Meter` — a reading inside a known range.

use dazzui::{Meter, MeterSize, Progress, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const SIZES: [MeterSize; 3] = [MeterSize::Small, MeterSize::Medium, MeterSize::Large];

const QUOTAS: [(&str, f32, Tint); 4] = [
    ("Storage", 62., Tint::Primary),
    ("Bandwidth", 41., Tint::Info),
    ("Build minutes", 88., Tint::Warning),
    ("Seats", 100., Tint::Danger),
];

fn quotas() -> Col {
    let mut col = Col::new().width(px(352.)).gap(px(16.));
    for (label, value, tint) in QUOTAS {
        col = col.child(
            Meter::new(value)
                .label(label)
                .value_label(SharedString::from(format!("{value:.0}%")))
                .tint(tint),
        );
    }
    col
}

pub const PAGE: StoryPage = StoryPage::new(
    "Meter",
    &[
        Story::page("Overview", |_, _, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase()))
                        .child(gpui::div().w(px(240.)).child(Meter::new(62.).size(size))),
                );
            }

            DocPage::new("Meter")
                .description(
                    "A reading inside a known range — disk used of disk there, spend against a \
                     budget, an API quota. It is not a Progress, and the difference is not how \
                     they look: progress reports on something running, so it moves on its own, \
                     it can be indeterminate, and it ends. A meter measures something that \
                     already has a value, so it never animates and never has none — which is \
                     why it prints its number and progress does not.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The same groove a Progress bar is cut into — the sunken surface, \
                             the soft square corner — because they are the same hole in the \
                             page. What differs is everything above it.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Label and value")
                        .description(
                            "The number takes the mono face, so a column of meters keeps its \
                             numbers on one edge. The label is the meter's heading and its name.",
                        )
                        .child(quotas()),
                )
                .child(
                    Section::new("Against a Progress")
                        .description(
                            "Set side by side the rule is easy to keep: if the number would \
                             change on its own while the reader watched, it is progress. If it \
                             is a fact about right now, it is a meter.",
                        )
                        .child(
                            Col::new()
                                .width(px(320.))
                                .child(Meter::new(62.).label("Disk used").value_label("62%"))
                                .child(Progress::new(62.)),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |_, _, _| {
            gpui::div()
                .w(px(288.))
                .child(Meter::new(62.))
                .into_any_element()
        }),
        Story::new("With value", |_, _, _| {
            gpui::div()
                .w(px(320.))
                .child(Meter::new(62.).label("Storage").value_label("62%"))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for size in SIZES {
                col = col.child(Meter::new(62.).size(size));
            }
            col.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for tint in Tint::ALL {
                col = col.child(Meter::new(62.).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("In a quota list", |_, _, _| quotas().into_any_element()),
    ],
);
