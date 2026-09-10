//! `Components/ToggleGroup` — several toggles joined into one control.

use dazzui::{ControlVariant, SegmentedControl, Tint, ToggleGroup, ToggleItem};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::{Story, StoryPage};

const SHOW: [(&str, &str); 3] = [
    ("whitespace", "Whitespace"),
    ("minimap", "Minimap"),
    ("blame", "Blame"),
];

fn show_items() -> Vec<ToggleItem> {
    SHOW.iter()
        .map(|(value, label)| ToggleItem::new(*value, *label))
        .collect()
}

fn style_items() -> Vec<ToggleItem> {
    vec![
        ToggleItem::new("bold", "B"),
        ToggleItem::new("italic", "I"),
        ToggleItem::new("underline", "U"),
    ]
}

pub const PAGE: StoryPage = StoryPage::new(
    "ToggleGroup",
    &[
        Story::page("Overview", |_, _, _| {
            DocPage::new("ToggleGroup")
                .description(
                    "Several toggles that belong together, joined into one control. Joined \
                     rather than merely adjacent: the corners are rounded at the ends of the \
                     run only and the shared edges collapse to one, which is what says the \
                     group is one thing with several states rather than three buttons that \
                     happen to touch.",
                )
                .child(
                    Section::new("Beside a SegmentedControl")
                        .description(
                            "This is where the two part company. A segmented control is a \
                             choice between its segments and always has exactly one; a toggle \
                             group can have several on at once, or none. Drawing a mandatory \
                             single choice with this one would let the reader clear a choice \
                             that cannot be cleared.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("SegmentedControl — one, always").child(
                                        SegmentedControl::new("doc-segmented", "source")
                                            .item("source", "Source")
                                            .item("preview", "Preview"),
                                    ),
                                )
                                .child(
                                    Specimen::new("ToggleGroup — any number, including none")
                                        .child(
                                            ToggleGroup::new("doc-styles", style_items())
                                                .value(vec![SharedString::from("bold")]),
                                        ),
                                ),
                        ),
                )
                .child(
                    Section::new("The collapsed edge")
                        .description(
                            "The ring belongs to the group and the members draw none of their \
                             own, so the rule between a pair is a single hairline rather than \
                             two stacked. That is border-collapse done the only way a row of \
                             boxes can do it, and it is what makes the run read as one control.",
                        )
                        .child(
                            ToggleGroup::new("doc-show", show_items())
                                .value(vec![SharedString::from("whitespace")]),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Glyphs", |state, this, _| {
            let on = state.styles.clone();
            ToggleGroup::new("styles", style_items())
                .value(on.clone())
                .on_toggle(act_with(this, move |state, value: &SharedString, _, _| {
                    if state.styles.contains(value) {
                        state.styles.retain(|one| one != value);
                    } else {
                        state.styles.push(value.clone());
                    }
                }))
                .into_any_element()
        }),
        Story::new("Words", |state, this, _| {
            let on = state.shown.clone();
            ToggleGroup::new("show", show_items())
                .value(on)
                .on_toggle(act_with(this, move |state, value: &SharedString, _, _| {
                    if state.shown.contains(value) {
                        state.shown.retain(|one| one != value);
                    } else {
                        state.shown.push(value.clone());
                    }
                }))
                .into_any_element()
        }),
        Story::new("Single", |state, this, _| {
            let on = state.shown.clone();
            ToggleGroup::new("single", show_items())
                .value(on)
                .on_toggle(act_with(this, move |state, value: &SharedString, _, _| {
                    // One at a time, or none: pressing the member that is down
                    // clears the run rather than being ignored, which is the
                    // whole difference from a segmented control.
                    if state.shown.contains(value) {
                        state.shown.clear();
                    } else {
                        state.shown = vec![value.clone()];
                    }
                }))
                .into_any_element()
        }),
        Story::new("Variants", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .children(
                    [
                        ControlVariant::Recessed,
                        ControlVariant::Normal,
                        ControlVariant::Outlined,
                        ControlVariant::Plain,
                    ]
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        ToggleGroup::new(
                            SharedString::from(format!("variant-{index}")),
                            show_items(),
                        )
                        .value(vec![SharedString::from("whitespace")])
                        .variant(*variant)
                    }),
                )
                .into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .children(Tint::ALL.iter().enumerate().map(|(index, tint)| {
                    ToggleGroup::new(SharedString::from(format!("tint-{index}")), show_items())
                        .value(vec![SharedString::from("whitespace")])
                        .tint(*tint)
                }))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            ToggleGroup::new(
                "dead",
                vec![
                    ToggleItem::new("whitespace", "Whitespace"),
                    ToggleItem::new("minimap", "Minimap"),
                    ToggleItem::new("blame", "Blame").disabled(true),
                ],
            )
            .value(vec![SharedString::from("whitespace")])
            .into_any_element()
        }),
    ],
);
