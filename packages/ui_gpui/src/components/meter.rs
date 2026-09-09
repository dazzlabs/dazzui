use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{Tint, half};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MeterSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A reading inside a known range — disk used of disk there, spend against a
/// budget, an API quota.
///
/// It is not a [`Progress`](crate::Progress), and the difference is not how
/// they look. Progress reports on something running: it moves on its own, it
/// can be indeterminate, and it ends. A meter measures something that already
/// has a value, so it never animates and never has none — which is why it
/// prints its number and progress does not.
#[derive(IntoElement)]
pub struct Meter {
    /// The reading, between `min` and `max`.
    value: f32,
    min: f32,
    max: f32,
    size: MeterSize,
    tint: Tint,
    /// Names the reading.
    label: Option<SharedString>,
    /// What is printed beside the label. The number is the meter's whole
    /// point, so most readings want one.
    value_label: Option<SharedString>,
}

impl Meter {
    pub fn new(value: f32) -> Self {
        Meter {
            value,
            min: 0.,
            max: 100.,
            size: MeterSize::Medium,
            tint: Tint::Primary,
            label: None,
            value_label: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn size(mut self, size: MeterSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value_label(mut self, value_label: impl Into<SharedString>) -> Self {
        self.value_label = Some(value_label.into());
        self
    }
}

impl RenderOnce for Meter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let thickness = match self.size {
            MeterSize::Small => vars.spacing_1,
            MeterSize::Medium => vars.spacing_1_5,
            MeterSize::Large => vars.spacing_2,
        };
        let span = self.max - self.min;
        let fraction = if span <= 0. {
            0.
        } else {
            ((self.value - self.min) / span).clamp(0., 1.)
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(vars.spacing_2)
            .when(self.label.is_some() || self.value_label.is_some(), |el| {
                el.child(
                    div()
                        .flex()
                        .items_baseline()
                        .justify_between()
                        .gap(vars.spacing_3)
                        .when_some(self.label, |el, label| {
                            el.child(
                                div()
                                    .flex_1()
                                    .type_style(vars.label_quiet)
                                    .text_size(vars.label_small.size)
                                    .font_weight(vars.label_medium.weight)
                                    .line_height(relative(1.0))
                                    .text_color(vars.color_content)
                                    .child(label),
                            )
                        })
                        .when_some(self.value_label, |el, value| {
                            el.child(
                                div()
                                    .flex_none()
                                    // A reading rather than a label: the mono
                                    // face, so a column of meters keeps its
                                    // numbers on one edge. It is the one place
                                    // a component reaches a `base.*`
                                    // primitive, which `meter.css` does too.
                                    .font_family("ui-monospace")
                                    .text_size(vars.label_small.size)
                                    .line_height(relative(1.0))
                                    .text_color(vars.color_content_muted)
                                    .child(value),
                            )
                        }),
                )
            })
            // The same cut a Progress bar is — the sunken groove, the soft
            // square corner, the hard-clipped fill. The fill has no
            // transition on purpose: a progress bar interpolates because the
            // thing behind it is running; a meter that eased into place would
            // be animating a fact.
            .child(
                div()
                    .w_full()
                    .h(thickness)
                    .overflow_hidden()
                    .rounded(half(thickness))
                    .bg(vars.color_surface_sunken)
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .bg(vars.ramp(self.tint).shade(600)),
                    ),
            )
    }
}
