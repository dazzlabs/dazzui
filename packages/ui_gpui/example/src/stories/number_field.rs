//! `Components/NumberField` — a number, with a stepper.

use dazzui::{
    FormField, NumberField, PreferenceRow, PreferenceSection, TextFieldSize, TextFieldState,
};
use gpui::{IntoElement, ParentElement, SharedString, Styled, div, px};

use crate::storybook::gallery::{DocPage, Section, Specimen, Specimens};
use crate::storybook::stage::Col;
use crate::storybook::{Story, StoryPage};
use crate::{Playground, act_with};

const SIZES: [TextFieldSize; 3] = [
    TextFieldSize::Small,
    TextFieldSize::Medium,
    TextFieldSize::Large,
];

pub const PAGE: StoryPage = StoryPage::new(
    "NumberField",
    &[
        Story::page("Overview", |state, this, _| {
            let mut sizes = Specimens::new();
            for size in SIZES {
                sizes = sizes.child(
                    Specimen::new(SharedString::from(format!("{size:?}").to_lowercase())).child(
                        div().w(px(128.)).child(
                            NumberField::new(
                                SharedString::from(format!("doc-{size:?}")),
                                &state.port,
                            )
                            .size(size)
                            .bounds(8., 24.)
                            .on_change(step(this)),
                        ),
                    ),
                );
            }

            DocPage::new("NumberField")
                .description(
                    "A number, with the two things a plain text control never gets right: a \
                     stepper you can hold down, and a value that stays a number. The box is the \
                     TextField's — one function draws it for both — so a form mixing the two \
                     does not step. What is this component's own is the stepper at the trailing \
                     edge, and the clamp that keeps the value inside its bounds.",
                )
                .child(
                    Section::new("Sizes")
                        .description(
                            "The stepper takes its room out of the trailing padding rather than \
                             out of the value, and it is stacked rather than a minus and a plus \
                             on either side: a value with a control on both sides of it reads \
                             as three things.",
                        )
                        .child(sizes),
                )
                .child(
                    Section::new("Bounds and units")
                        .description(
                            "A unit inside the box is a hint rather than part of the value — it \
                             is the faint ink, and it never enters what the field holds.",
                        )
                        .child(
                            div().w(px(192.)).child(
                                NumberField::new("doc-unit", &state.port)
                                    .unit("px")
                                    .bounds(8., 24.)
                                    .on_change(step(this)),
                            ),
                        ),
                )
                .child(
                    Section::new("States")
                        .description(
                            "Erroring re-tints the control to danger rather than bolting on a \
                             second palette, exactly as a TextField does — the border, the wash \
                             and the value's ink all move together.",
                        )
                        .child(
                            Specimens::new()
                                .child(
                                    Specimen::new("default").child(
                                        div().w(px(128.)).child(
                                            NumberField::new("doc-default", &state.port)
                                                .on_change(step(this)),
                                        ),
                                    ),
                                )
                                .child(
                                    Specimen::new("error").child(
                                        div().w(px(128.)).child(
                                            NumberField::new("doc-error", &state.port)
                                                .state(TextFieldState::Error),
                                        ),
                                    ),
                                )
                                .child(Specimen::new("disabled").child(div().w(px(128.)).child(
                                    NumberField::new("doc-disabled", &state.port).disabled(true),
                                ))),
                        ),
                )
                .into_any_element()
        }),
        Story::new("Bare", |state, this, _| {
            div()
                .w(px(160.))
                .child(NumberField::new("bare", &state.port).on_change(step(this)))
                .into_any_element()
        }),
        Story::new("With unit", |state, this, _| {
            div()
                .w(px(160.))
                .child(
                    NumberField::new("with-unit", &state.port)
                        .unit("px")
                        .bounds(8., 24.)
                        .on_change(step(this)),
                )
                .into_any_element()
        }),
        Story::new("Sizes", |state, _, _| {
            let mut col = Col::new().width(px(160.));
            for size in SIZES {
                col = col.child(
                    NumberField::new(SharedString::from(format!("sizes-{size:?}")), &state.port)
                        .size(size),
                );
            }
            col.into_any_element()
        }),
        Story::new("States", |state, _, _| {
            Col::new()
                .width(px(160.))
                .child(NumberField::new("states-default", &state.port))
                .child(NumberField::new("states-error", &state.port).state(TextFieldState::Error))
                .child(NumberField::new("states-disabled", &state.port).disabled(true))
                .into_any_element()
        }),
        Story::new("In a form", |state, this, _| {
            div()
                .w(px(288.))
                .child(
                    FormField::new("Dev server port")
                        .hint("Restart to apply")
                        .child(
                            NumberField::new("form-port", &state.port)
                                .bounds(1024., 65535.)
                                .on_change(step(this)),
                        ),
                )
                .into_any_element()
        }),
        Story::new("In settings", |state, this, _| {
            div()
                .w(px(480.))
                .child(
                    PreferenceSection::new()
                        .label("Editor")
                        .child(
                            PreferenceRow::new("size", "Font size")
                                .subtitle("Applies to every project.")
                                .child(
                                    div().w(px(112.)).child(
                                        NumberField::new("settings-size", &state.port)
                                            .size(TextFieldSize::Small)
                                            .unit("px")
                                            .bounds(8., 24.)
                                            .on_change(step(this)),
                                    ),
                                ),
                        )
                        .child(
                            PreferenceRow::new("tab", "Tab width").child(
                                div().w(px(112.)).child(
                                    NumberField::new("settings-tab", &state.port)
                                        .size(TextFieldSize::Small)
                                        .bounds(1., 8.),
                                ),
                            ),
                        ),
                )
                .into_any_element()
        }),
    ],
);

/// The stepper hands back a number; the buffer is what holds it, so the
/// playground writes it straight back.
fn step(
    this: &gpui::Entity<Playground>,
) -> impl Fn(&f32, &mut gpui::Window, &mut gpui::App) + 'static {
    act_with(this, |state, value: &f32, _, cx| {
        let text = format!("{value:.0}");
        state
            .port
            .update(cx, |buffer, cx| buffer.set_value(text, cx));
    })
}
