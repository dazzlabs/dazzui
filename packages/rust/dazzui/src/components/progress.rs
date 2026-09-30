//! How far something running has got.

use std::sync::OnceLock;
use std::time::Instant;

use gpui::{
    App, Background, Bounds, IntoElement, ParentElement, RenderOnce, Styled, Window, canvas, div,
    fill, linear_color_stop, linear_gradient, point, px, relative, size,
};

use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

/// A rule cut into the surface, not a capsule: a soft square corner, and a
/// fill whose leading edge is a hard clip against it.
#[derive(IntoElement)]
pub struct Progress {
    value: Option<f32>,
    size: Size,
    tint: Tint,
    gradient: bool,
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            value: None,
            size: Size::Small,
            tint: Tint::Primary,
            gradient: false,
        }
    }

    /// 0–100. Leave unset for an indeterminate bar.
    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value);
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

    /// Paint the fill with the theme's two-stop accent ramp instead of a
    /// flat tint.
    pub fn gradient(mut self, gradient: bool) -> Self {
        self.gradient = gradient;
        self
    }
}

impl Default for Progress {
    fn default() -> Self {
        Progress::new()
    }
}

/// The groove's height for a size — shared with [`Meter`](crate::Meter),
/// which is cut the same way.
pub(crate) fn groove_height(size: Size, t: &crate::ThemeTokens) -> f32 {
    match size {
        Size::Tiny | Size::Small => t.spacing_1,
        Size::Medium => t.spacing_1_5,
        Size::Large => t.spacing_2,
    }
}

/// The fill, hard-clipped at its leading edge. GPUI's `overflow_hidden`
/// clips to the groove's box but not its corner, so the fill carries the
/// corner itself: on its start, and on its end once it reaches it.
pub(crate) fn fill_bar(fraction: f32, radius: f32) -> gpui::Div {
    let bar = div().h_full().w(relative(fraction)).rounded_l(px(radius));
    if fraction >= 1.0 {
        bar.rounded_r(px(radius))
    } else {
        bar
    }
}

fn epoch() -> Instant {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    *EPOCH.get_or_init(Instant::now)
}

impl RenderOnce for Progress {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let height = groove_height(self.size, t);
        let paint: Background = if self.gradient {
            linear_gradient(
                90.,
                linear_color_stop(t.progress_gradient_from.hsla(), 0.),
                linear_color_stop(t.progress_gradient_to.hsla(), 1.),
            )
        } else {
            self.tint.ramp(t).shade(600).hsla().into()
        };
        let groove = div()
            .w_full()
            .h(px(height))
            .overflow_hidden()
            .bg(t.color_surface_sunken.hsla())
            .rounded(px(height / 2.0));
        match self.value {
            Some(value) => {
                let fraction = value.clamp(0.0, 100.0) / 100.0;
                groove.child(fill_bar(fraction, height / 2.0).bg(paint))
            }
            // No width to animate, so it sweeps: 40% of the groove, from
            // off the leading edge to past the trailing one, in 1.2s.
            None => groove.child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        let phase = (epoch().elapsed().as_secs_f32() / 1.2).fract();
                        let eased = if phase < 0.5 {
                            2.0 * phase * phase
                        } else {
                            1.0 - (-2.0 * phase + 2.0).powi(2) / 2.0
                        };
                        let width = bounds.size.width * 0.4;
                        let x = width * (-1.0 + 3.5 * eased);
                        window.paint_quad(fill(
                            Bounds::new(
                                point(bounds.origin.x + x, bounds.origin.y),
                                size(width, bounds.size.height),
                            ),
                            paint,
                        ));
                        window.request_animation_frame();
                    },
                )
                .size_full(),
            ),
        }
    }
}
