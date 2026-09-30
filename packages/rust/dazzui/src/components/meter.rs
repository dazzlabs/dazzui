//! A reading inside a known range — disk used of disk there, an API quota.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::components::progress::{fill_bar, groove_height};
use crate::foundation::{Size, Tint, baseline};
use crate::theme::ActiveTheme;
use crate::tokens::Face;

/// The same cut a [`Progress`](crate::Progress) bar is, with what is the
/// meter's own above it: a label and its number. A meter is a measurement
/// rather than something running, so it never animates and always has a
/// value.
#[derive(IntoElement)]
pub struct Meter {
    value: f32,
    min: f32,
    max: f32,
    size: Size,
    tint: Tint,
    label: Option<AnyElement>,
    show_value: bool,
}

impl Meter {
    pub fn new(value: f32) -> Self {
        Meter {
            value,
            min: 0.0,
            max: 100.0,
            size: Size::Medium,
            tint: Tint::Primary,
            label: None,
            show_value: false,
        }
    }

    /// The range the value is read against; 0–100 by default.
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Names the reading.
    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.label = Some(label.into_any_element());
        self
    }

    /// Prints the value, as a percentage of the range, beside the label.
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }
}

impl RenderOnce for Meter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let span = (self.max - self.min).max(f32::EPSILON);
        let fraction = ((self.value - self.min) / span).clamp(0.0, 1.0);
        let height = groove_height(self.size, t);
        let label_size = t.label_small.size;

        let head = (self.label.is_some() || self.show_value).then(|| {
            let label = self.label.map(|label| {
                div()
                    .text_color(t.color_content.hsla())
                    .font_family(t.label_quiet.face.family())
                    .text_size(px(label_size))
                    .font_weight(t.label_medium.font_weight())
                    .line_height(px(label_size))
                    .child(label)
            });
            // The two runs share a baseline, and two faces set solid put it
            // at different heights: the one whose baseline sits higher is
            // lowered to meet the other, as `align-items: baseline` does.
            let label_base = baseline(t.label_quiet.face, label_size, label_size, window);
            let value_base = baseline(Face::Mono, label_size, label_size, window);
            let label = label.map(|label| label.mt(px((value_base - label_base).max(0.0))));
            // A reading: the mono face, so a column of meters keeps its
            // numbers on one line.
            let value = self.show_value.then(|| {
                div()
                    .text_color(t.color_content_muted.hsla())
                    .font_family(Face::Mono.family())
                    .text_size(px(label_size))
                    .line_height(px(label_size))
                    .mt(px((label_base - value_base).max(0.0)))
                    .child(SharedString::from(format!(
                        "{}%",
                        (fraction * 100.0).round() as i64
                    )))
            });
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap(px(t.spacing_3))
                .children(label)
                .children(value)
        });

        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(t.spacing_2))
            .children(head)
            .child(
                div()
                    .w_full()
                    .h(px(height))
                    .overflow_hidden()
                    .bg(t.color_surface_sunken.hsla())
                    .rounded(px(height / 2.0))
                    .child(
                        fill_bar(fraction, height / 2.0).bg(self.tint.ramp(t).shade(600).hsla()),
                    ),
            )
    }
}
