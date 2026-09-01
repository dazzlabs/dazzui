use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    linear_color_stop, linear_gradient, relative,
};

use crate::Tint;
use crate::half;
use crate::prelude::*;

/// How wide the indeterminate bar's sweep is, and how long one pass takes.
///
/// Both are `progress.css`'s own: an indeterminate bar has no width to
/// animate, so it sweeps a fixed share of the track instead.
const SWEEP_WIDTH: f32 = 0.4;
const SWEEP: Duration = Duration::from_millis(1200);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgressSize {
    #[default]
    Small,
    Medium,
    Large,
}

/// A rule cut into the surface, not a capsule: the corner is a soft square,
/// and the fill's leading edge is a hard clip against it — a pill's rounded
/// head would read as a thumb.
#[derive(IntoElement)]
pub struct Progress {
    /// 0–100. Omit for an indeterminate bar.
    value: Option<f32>,
    size: ProgressSize,
    tint: Tint,
    /// Paint the fill with the theme's two-stop accent ramp instead of a flat
    /// tint.
    gradient: bool,
}

impl Progress {
    /// A bar reporting how far along something is.
    pub fn new(value: f32) -> Self {
        Progress {
            value: Some(value),
            size: ProgressSize::Small,
            tint: Tint::Primary,
            gradient: false,
        }
    }

    /// A bar for work whose length is not known.
    pub fn indeterminate() -> Self {
        Progress {
            value: None,
            size: ProgressSize::Small,
            tint: Tint::Primary,
            gradient: false,
        }
    }

    pub fn size(mut self, size: ProgressSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn gradient(mut self, gradient: bool) -> Self {
        self.gradient = gradient;
        self
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let height = match self.size {
            ProgressSize::Small => vars.spacing_1,
            ProgressSize::Medium => vars.spacing_1_5,
            ProgressSize::Large => vars.spacing_2,
        };
        let flat = vars.ramp(self.tint).shade(600);

        let fill = move || {
            div().h_full().map(|el| {
                if self.gradient {
                    // The document-progress fill: the theme's two-stop accent
                    // ramp.
                    el.bg(linear_gradient(
                        90.,
                        linear_color_stop(vars.progress_gradient_from, 0.),
                        linear_color_stop(vars.progress_gradient_to, 1.),
                    ))
                } else {
                    el.bg(flat)
                }
            })
        };

        div()
            .relative()
            .w_full()
            .h(height)
            .overflow_hidden()
            // The corner is half the bar's own height: a soft square rather
            // than a capsule.
            .rounded(half(height))
            .bg(vars.color_surface_sunken)
            .map(|track| match self.value {
                Some(value) => track.child(fill().w(relative(value.clamp(0., 100.) / 100.))),
                // An indeterminate bar has no width to animate, so it sweeps
                // instead.
                None => track.child(div().size_full().with_animation(
                    "progress-sweep",
                    Animation::new(SWEEP).repeat(),
                    move |el, delta| {
                        el.child(
                            fill()
                                .absolute()
                                .top_0()
                                .w(relative(SWEEP_WIDTH))
                                .left(relative(delta * (1.0 + SWEEP_WIDTH) - SWEEP_WIDTH)),
                        )
                    },
                )),
            })
    }
}
