//! `Components/Toggle` — a button that stays down.

use dazzui::{ControlSize, ControlVariant, SegmentedControl, Switch, Tint, Toggle};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

const SIZES: [ControlSize; 4] = [
    ControlSize::Tiny,
    ControlSize::Small,
    ControlSize::Medium,
    ControlSize::Large,
];

const VARIANTS: [ControlVariant; 6] = [
    ControlVariant::Recessed,
    ControlVariant::Normal,
    ControlVariant::Filled,
    ControlVariant::Tinted,
    ControlVariant::Outlined,
    ControlVariant::Plain,
];

fn size_label(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Tiny => "tiny",
        ControlSize::Small => "small",
        ControlSize::Medium => "medium",
        ControlSize::Large => "large",
    }
}

fn variant_label(variant: ControlVariant) -> &'static str {
    match variant {
        ControlVariant::Recessed => "recessed",
        ControlVariant::Normal => "normal",
        ControlVariant::Filled => "filled",
        ControlVariant::Tinted => "tinted",
        ControlVariant::Outlined => "outlined",
        ControlVariant::Plain => "plain",
    }
}

pub const PAGE: StoryPage = StoryPage::new(
    "Toggle",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("Toggle")
                .description(
                    "A button that stays down. Three components in this package are one press \
                     away from each other: a Switch commits immediately and reads as a setting; \
                     a SegmentedControl is a choice between its segments and always has one; a \
                     Toggle is neither — a single property that is on or off, in a row where \
                     none of them may be on.",
                )
                .child(
                    Section::new("Beside its neighbours")
                        .description(
                            "Reach for a Switch where the thing is a setting in a list and the \
                             change takes effect on the spot. Reach for a SegmentedControl \
                             where the reader is choosing one of several alternatives and \
                             cannot choose none. Reach for this where the answer is yes or no \
                             and the control lives in a toolbar rather than a form.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("Switch").child(Switch::new("doc-switch", true)),
                                )
                                .child(
                                    Specimen::new("SegmentedControl").child(
                                        SegmentedControl::new("doc-segmented", "left")
                                            .item("left", "Left")
                                            .item("center", "Center"),
                                    ),
                                )
                                .child(
                                    Specimen::new("Toggle")
                                        .child(Toggle::new("doc-toggle", "Bold").pressed(true)),
                                ),
                        ),
                )
                .child(
                    Section::new("Pressed")
                        .description(
                            "Drawn the way the system draws every selection: the tinted wash, \
                             and the label a weight heavier. Colour alone would be one grade in \
                             a toolbar of glyphs, which is exactly where it is hardest to see — \
                             and the weight is the half that survives for a reader who cannot \
                             tell the two grades apart. The recipe stays tinted when it is \
                             disabled and only the weight drops, so a dead toggle still \
                             flattens to the groove.",
                        )
                        .child(
                            Specimens::new()
                                .child(Specimen::new("up").child(Toggle::new("doc-up", "Wrap")))
                                .child(
                                    Specimen::new("down")
                                        .child(Toggle::new("doc-down", "Wrap").pressed(true)),
                                )
                                .child(Specimen::new("disabled, down").child(
                                    Toggle::new("doc-dead", "Wrap").pressed(true).disabled(true),
                                )),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Pressed", |_, _, _| {
            div()
                .flex()
                .gap(px(6.))
                .child(Toggle::new("up", "Wrap"))
                .child(Toggle::new("down", "Wrap").pressed(true))
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            div()
                .flex()
                .items_center()
                .gap(px(6.))
                .children(SIZES.iter().enumerate().map(|(index, size)| {
                    Toggle::new(
                        SharedString::from(format!("size-{index}")),
                        size_label(*size),
                    )
                    .size(*size)
                    .variant(ControlVariant::Recessed)
                    .pressed(true)
                }))
                .into_any_element()
        }),
        Story::new("Variants", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .child(
                    div()
                        .flex()
                        .gap(px(6.))
                        .children(VARIANTS.iter().enumerate().map(|(index, variant)| {
                            Toggle::new(
                                SharedString::from(format!("up-{index}")),
                                variant_label(*variant),
                            )
                            .variant(*variant)
                        })),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(6.))
                        .children(VARIANTS.iter().enumerate().map(|(index, variant)| {
                            Toggle::new(
                                SharedString::from(format!("down-{index}")),
                                variant_label(*variant),
                            )
                            .variant(*variant)
                            .pressed(true)
                        })),
                )
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            div()
                .flex()
                .gap(px(6.))
                .children(Tint::ALL.iter().enumerate().map(|(index, tint)| {
                    Toggle::new(SharedString::from(format!("tint-{index}")), tint.name())
                        .tint(*tint)
                        .pressed(true)
                }))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .flex()
                .gap(px(6.))
                .child(Toggle::new("dead-up", "Wrap").disabled(true))
                .child(
                    Toggle::new("dead-down", "Wrap")
                        .pressed(true)
                        .disabled(true),
                )
                .into_any_element()
        }),
        Story::new("A toolbar", |state, this, _| {
            let bold = state.bold;
            div()
                .flex()
                .gap(px(4.))
                .child(
                    Toggle::new("bold", "B")
                        .pressed(bold)
                        .on_toggle(act(this, move |state, _, _| state.bold = !bold)),
                )
                .child(Toggle::new("italic", "I"))
                .child(Toggle::new("underline", "U"))
                .into_any_element()
        }),
    ],
);
