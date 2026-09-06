//! `Components/SegmentedControl` — one choice, on a track.

use dazzui::{
    ActiveTheme, FormField, SegmentedActiveStyle, SegmentedControl, SegmentedControlSize, Tint,
};
use gpui::{Entity, IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens, TokenTable, case};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [SegmentedControlSize; 3] = [
    SegmentedControlSize::Small,
    SegmentedControlSize::Medium,
    SegmentedControlSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "SegmentedControl",
    &[
        Story::page("Overview", |state, this, cx| {
            let vars = cx.theme().vars;

            let mut styles = Specimens::new();
            for style in [SegmentedActiveStyle::Raised, SegmentedActiveStyle::Filled] {
                styles = styles.child(
                    Specimen::new(case(style)).child(modes(state, this, style, None, false, "doc")),
                );
            }

            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(Specimen::new(case(size)).child(modes(
                    state,
                    this,
                    SegmentedActiveStyle::Raised,
                    Some(size),
                    false,
                    "doc-size",
                )));
            }

            DocPage::new("SegmentedControl")
                .description(
                    "One choice out of a few, on a sunken track. Where pill tabs filter a list \
                     under them, this switches what a view *is* — a mode, an appearance, a unit \
                     — which is why it keeps the track: the unchosen options stay visible as \
                     part of one control.",
                )
                .child(
                    Section::new("Active style")
                        .description(
                            "`raised` is the AppKit shape: the chosen segment is a paper chip \
                             lifted off the track, and the label goes to the strong face. \
                             `filled` takes the tint instead, for a control that has to say \
                             which mode it is in from across the window.",
                        )
                        .child(styles),
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "Three heights off the control profile. The track itself is a \
                             surface role and never moves.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Stretch")
                        .description(
                            "By default a segment is as wide as its label. `stretch` divides the \
                             width evenly instead, which is what a control inside a form field \
                             wants so it lines up with the fields above it.",
                        )
                        .child(
                            Col::new()
                                .width(px(420.))
                                .child(modes(
                                    state,
                                    this,
                                    SegmentedActiveStyle::Raised,
                                    None,
                                    false,
                                    "doc-a",
                                ))
                                .child(modes(
                                    state,
                                    this,
                                    SegmentedActiveStyle::Raised,
                                    None,
                                    true,
                                    "doc-b",
                                )),
                        ),
                )
                .child(
                    Section::new("Tokens")
                        .description("The track's own inset.")
                        .child(
                            TokenTable::new()
                                .length("segmented-control.inset", vars.segmented_control_inset),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Raised", |state, this, _| {
            modes(
                state,
                this,
                SegmentedActiveStyle::Raised,
                None,
                false,
                "raised",
            )
            .into_any_element()
        }),
        Story::new("Filled", |state, this, _| {
            modes(
                state,
                this,
                SegmentedActiveStyle::Filled,
                None,
                false,
                "filled",
            )
            .into_any_element()
        }),
        Story::new("Sizes", |state, this, _| {
            let mut col = Col::new();
            for size in SIZES {
                col = col.child(modes(
                    state,
                    this,
                    SegmentedActiveStyle::Raised,
                    Some(size),
                    false,
                    "size",
                ));
            }
            col.into_any_element()
        }),
        Story::new("Tints", |state, this, _| {
            let mut col = Col::new();
            for tint in Tint::ALL {
                col = col.child(
                    SegmentedControl::new(
                        SharedString::from(format!("tint-{}", tint.name())),
                        state.mode.clone(),
                    )
                    .item("design", "Design")
                    .item("code", "Code")
                    .item("preview", "Preview")
                    .active_style(SegmentedActiveStyle::Filled)
                    .tint(tint)
                    .on_select(act_with(this, |state, value: &SharedString, _, _| {
                        state.mode = value.clone()
                    })),
                );
            }
            col.into_any_element()
        }),
        Story::new("Stretch", |state, this, _| {
            div()
                .w(px(380.))
                .child(modes(
                    state,
                    this,
                    SegmentedActiveStyle::Raised,
                    None,
                    true,
                    "stretch",
                ))
                .into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            SegmentedControl::new("disabled", "design")
                .item("design", "Design")
                .item("code", "Code")
                .disabled_item("preview", "Preview")
                .into_any_element()
        }),
        Story::new("Two segments", |_, _, _| {
            SegmentedControl::new("two", "light")
                .item("light", "Light")
                .item("dark", "Dark")
                .into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(380.))
                .child(FormField::new("Editor").child(modes(
                    state,
                    this,
                    SegmentedActiveStyle::Raised,
                    None,
                    true,
                    "form",
                )))
                .into_any_element()
        }),
    ],
);

fn modes(
    state: &Playground,
    this: &Entity<Playground>,
    style: SegmentedActiveStyle,
    size: Option<SegmentedControlSize>,
    stretch: bool,
    id: &'static str,
) -> impl IntoElement {
    let mut control = SegmentedControl::new(
        SharedString::from(format!("{id}-{}", case(style))),
        state.mode.clone(),
    )
    .item("design", "Design")
    .item("code", "Code")
    .item("preview", "Preview")
    .active_style(style)
    .stretch(stretch)
    .on_select(act_with(this, |state, value: &SharedString, _, _| {
        state.mode = value.clone()
    }));

    if let Some(size) = size {
        control = control.size(size);
    }

    control
}
