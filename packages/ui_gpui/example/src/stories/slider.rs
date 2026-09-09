//! `Components/Slider` — a value picked along a range.

use dazzui::{FormField, PreferenceRow, PreferenceSection, Slider, SliderSize, Tint};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::act_with;
use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};

const SIZES: [SliderSize; 3] = [SliderSize::Small, SliderSize::Medium, SliderSize::Large];

pub const PAGE: StoryPage = StoryPage::new(
    "Slider",
    &[
        Story::page("Overview", |state, this, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase())).child(
                        div().w(px(240.)).child(
                            Slider::new(
                                SharedString::from(format!("doc-{size:?}")),
                                state.font_size,
                            )
                            .bounds(8., 24.)
                            .size(size)
                            .on_change(act_with(
                                this,
                                |state, values: &Vec<f32>, _, _| {
                                    state.font_size = values[0];
                                },
                            )),
                        ),
                    ),
                );
            }

            DocPage::new("Slider")
                .description(
                    "A value picked along a range. It owns the parts that make one usable \
                     rather than merely draggable: the press on the track that jumps the \
                     nearest thumb, the drag, and — on a range — which thumb the pointer \
                     actually grabbed when the two are on top of each other. Reach for it where \
                     the reader is feeling for a value rather than stating one; a number they \
                     know is a NumberField.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The row's height comes off the control profile, so a slider in a \
                             form sits on the same rhythm as the fields around it. The groove \
                             and the thumb are the component's own tokens, and both are even on \
                             every size for the reason the radio's ring and dot are.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Range")
                        .description(
                            "A range is the same component: pass more than one value and it \
                             draws a thumb per value. There is no second component, because \
                             everything but the number of thumbs is identical.",
                        )
                        .child(
                            div().w(px(320.)).child(
                                Slider::range("doc-range", state.band.clone())
                                    .value_label(SharedString::from(format!(
                                        "{:.0} – {:.0}",
                                        state.band[0], state.band[1]
                                    )))
                                    .on_change(act_with(this, |state, values: &Vec<f32>, _, _| {
                                        state.band = values.clone();
                                    })),
                            ),
                        ),
                )
                .child(
                    Section::new("Disabled")
                        .description(
                            "It flattens to the groove like every other control — no lift, no \
                             accent, one dead look — rather than becoming a translucent ghost \
                             of the live one.",
                        )
                        .child(
                            div()
                                .w(px(288.))
                                .child(Slider::new("doc-disabled", 40.).disabled(true)),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, this, _| {
            div()
                .w(px(288.))
                .child(
                    Slider::new("bare", state.font_size)
                        .bounds(8., 24.)
                        .on_change(act_with(this, |state, values: &Vec<f32>, _, _| {
                            state.font_size = values[0];
                        })),
                )
                .into_any_element()
        }),
        Story::new("With value", |state, this, _| {
            div()
                .w(px(320.))
                .child(
                    Slider::new("with-value", state.font_size)
                        .bounds(8., 24.)
                        .value_label(SharedString::from(format!("{:.0}", state.font_size)))
                        .on_change(act_with(this, |state, values: &Vec<f32>, _, _| {
                            state.font_size = values[0];
                        })),
                )
                .into_any_element()
        }),
        Story::new("Range", |state, this, _| {
            div()
                .w(px(320.))
                .child(
                    Slider::range("range", state.band.clone())
                        .value_label(SharedString::from(format!(
                            "{:.0} – {:.0}",
                            state.band[0], state.band[1]
                        )))
                        .on_change(act_with(this, |state, values: &Vec<f32>, _, _| {
                            state.band = values.clone();
                        })),
                )
                .into_any_element()
        }),
        Story::new("Sizes", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for size in SIZES {
                col = col.child(
                    Slider::new(SharedString::from(format!("sizes-{size:?}")), 40.).size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("Tints", |_, _, _| {
            let mut col = Col::new().width(px(320.));
            for tint in Tint::ALL {
                col = col.child(Slider::new(tint.name(), 60.).tint(tint));
            }
            col.into_any_element()
        }),
        Story::new("Disabled", |_, _, _| {
            div()
                .w(px(320.))
                .child(
                    Slider::new("disabled", 40.)
                        .disabled(true)
                        .value_label("40"),
                )
                .into_any_element()
        }),
        Story::new("Interactive", |state, this, _| {
            div()
                .w(px(352.))
                .child(
                    FormField::new("Editor font size")
                        .hint(SharedString::from(format!("{:.0}px", state.font_size)))
                        .child(
                            Slider::new("interactive", state.font_size)
                                .bounds(8., 24.)
                                .value_label(SharedString::from(format!(
                                    "{:.0}px",
                                    state.font_size
                                )))
                                .on_change(act_with(this, |state, values: &Vec<f32>, _, _| {
                                    state.font_size = values[0];
                                })),
                        ),
                )
                .into_any_element()
        }),
        Story::new("In settings", |state, this, _| {
            div()
                .w(px(480.))
                .child(
                    PreferenceSection::new()
                        .label("Appearance")
                        .child(
                            PreferenceRow::new("font", "Editor font size")
                                .subtitle("Applies to every open project.")
                                .child(
                                    div().w(px(160.)).child(
                                        Slider::new("settings-font", state.font_size)
                                            .bounds(8., 24.)
                                            .size(SliderSize::Small)
                                            .on_change(act_with(
                                                this,
                                                |state, values: &Vec<f32>, _, _| {
                                                    state.font_size = values[0];
                                                },
                                            )),
                                    ),
                                ),
                        )
                        .child(
                            PreferenceRow::new("width", "Sidebar width").child(
                                div().w(px(160.)).child(
                                    Slider::new("settings-width", 240.)
                                        .bounds(180., 360.)
                                        .size(SliderSize::Small),
                                ),
                            ),
                        ),
                )
                .into_any_element()
        }),
    ],
);
