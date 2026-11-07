//! How far something running has got.

use gpui::{
    App, Background, Bounds, ContentMask, ElementId, Hsla, IntoElement, ParentElement, RenderOnce,
    Styled, Window, canvas, div, fill, linear_color_stop, linear_gradient, point, px, relative,
    size,
};

use crate::foundation::motion::{Motion, Transition, cubic_bezier};
use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

/// A rule cut into the surface, not a capsule: a soft square corner, and a
/// fill whose leading edge is a hard clip against it.
#[derive(IntoElement)]
pub struct Progress {
    id: ElementId,
    value: Option<f32>,
    size: Size,
    tint: Tint,
    gradient: bool,
}

impl Progress {
    #[track_caller]
    pub fn new() -> Self {
        Progress {
            id: ElementId::CodeLocation(*std::panic::Location::caller()),
            value: None,
            size: Size::Small,
            tint: Tint::Primary,
            gradient: false,
        }
    }

    /// Supply a stable identity for bars built from the same call site in a loop.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
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
    #[track_caller]
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

impl RenderOnce for Progress {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let height = groove_height(self.size, t);
        let ink = self.tint.ramp(t).shade(600).hsla();
        let gradient = self.gradient;
        let frame = Motion::frame(cx);
        let duration = t.motion_duration * 2.;
        let value = self.value.map(|value| value.clamp(0., 100.) / 100.);
        let target = value.unwrap_or(t.progress_sweep_width);
        let transition = window.use_keyed_state(
            ElementId::NamedChild(Box::new(self.id), "width".into()),
            cx,
            |_, _| Transition::new(target, frame.elapsed),
        );
        transition.update(cx, |transition, _| {
            if value.is_none() {
                *transition = Transition::new(target, frame.elapsed);
            } else {
                transition.target(target, frame, duration, t.motion_easing);
            }
        });
        let groove = div()
            .w_full()
            .h(px(height))
            .overflow_hidden()
            .bg(t.color_surface_sunken.hsla())
            .rounded(px(height / 2.0));
        match value {
            Some(_) => groove.child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, cx| {
                        if bounds.size.width <= px(0.) {
                            return;
                        }
                        let frame = Motion::frame(cx);
                        let transition = transition.read(cx);
                        let fraction = transition.value(frame.elapsed, duration, t.motion_easing);
                        paint_fill(
                            bounds,
                            0.,
                            fraction,
                            background(0., fraction, gradient, ink, t),
                            window,
                        );
                        if !frame.frozen
                            && !frame.reduced
                            && transition.running(frame.elapsed, duration)
                        {
                            window.request_animation_frame();
                        }
                    },
                )
                .size_full(),
            ),
            // No width to animate, so it sweeps: 40% of the groove, from
            // off the leading edge to past the trailing one, in 1.2s.
            None => groove.child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, cx| {
                        if bounds.size.width <= px(0.) {
                            return;
                        }
                        let frame = Motion::frame(cx);
                        let phase = frame
                            .cycle(t.progress_sweep_duration, t.progress_reduced_sweep_duration);
                        let eased = cubic_bezier(t.motion_loop_easing, phase);
                        let width = bounds.size.width * t.progress_sweep_width;
                        let x = -width + (bounds.size.width + width) * eased;
                        let start = f32::from(x / bounds.size.width);
                        let end = start + t.progress_sweep_width;
                        paint_fill(
                            bounds,
                            start,
                            end,
                            background(start, end, gradient, ink, t),
                            window,
                        );
                        if !frame.frozen {
                            window.request_animation_frame();
                        }
                    },
                )
                .size_full(),
            ),
        }
    }
}

fn background(
    start: f32,
    end: f32,
    gradient: bool,
    ink: Hsla,
    t: &crate::ThemeTokens,
) -> Background {
    if gradient {
        linear_gradient(
            90.,
            linear_color_stop(t.progress_gradient_from.hsla(), start),
            linear_color_stop(t.progress_gradient_to.hsla(), end),
        )
    } else {
        ink.into()
    }
}

fn paint_fill(
    bounds: Bounds<gpui::Pixels>,
    start: f32,
    end: f32,
    paint: Background,
    window: &mut Window,
) {
    let start = start.clamp(0., 1.);
    let end = end.clamp(0., 1.);
    if end <= start {
        return;
    }
    let mask = Bounds::new(
        point(bounds.origin.x + bounds.size.width * start, bounds.origin.y),
        size(bounds.size.width * (end - start), bounds.size.height),
    );
    window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
        window.paint_quad(fill(bounds, paint).corner_radii(bounds.size.height / 2.));
    });
}
