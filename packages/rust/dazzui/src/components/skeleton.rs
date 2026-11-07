//! The shape of something that has not arrived.

use gpui::{
    App, Bounds, ContentMask, IntoElement, ParentElement, RenderOnce, Styled, Window, canvas, div,
    fill, linear_color_stop, linear_gradient, point, px, relative, size,
};

use crate::foundation::Length;
use crate::foundation::motion::{Motion, cubic_bezier};
use crate::theme::ActiveTheme;

/// `text` is a bar on the line box of the type it stands in for; `circle` is
/// an avatar or a glyph; `rect` is everything with a box of its own.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SkeletonShape {
    #[default]
    Text,
    Circle,
    Rect,
}

impl SkeletonShape {
    pub fn parse(name: &str) -> Option<SkeletonShape> {
        Some(match name {
            "text" => SkeletonShape::Text,
            "circle" => SkeletonShape::Circle,
            "rect" => SkeletonShape::Rect,
            _ => return None,
        })
    }
}

/// A place where a control or a line of type will be, drawn in the dead
/// `surface-sunken` step a disabled control flattens to.
///
/// A `surface-subtle` highlight sweeps across it, unless reduced motion
/// stops the decorative animation.
#[derive(IntoElement, Default)]
pub struct Skeleton {
    shape: SkeletonShape,
    width: Option<Length>,
    height: Option<Length>,
}

impl Skeleton {
    pub fn new() -> Self {
        Skeleton::default()
    }

    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    /// A `text` bar with no width takes the whole line; a `circle` is
    /// squared from this alone.
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// `text` and `circle` derive theirs and ignore it.
    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let radius = match self.shape {
            SkeletonShape::Text => t.radius_tiny,
            SkeletonShape::Circle => t.radius_full,
            SkeletonShape::Rect => t.radius_small,
        };
        let bar = div()
            .relative()
            .flex_none()
            .bg(t.color_surface_sunken.hsla());
        let bar = bar.child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, cx| {
                    let frame = Motion::frame(cx);
                    let phase = frame.cycle(t.skeleton_sweep_duration, t.skeleton_sweep_duration);
                    let progress = if frame.reduced {
                        // The clamped CSS animation has no fill mode: once it
                        // finishes, background-position returns to its 0% base.
                        if frame.frozen
                            && frame.elapsed.as_secs_f64() * 1000.
                                < t.motion_reduced_duration as f64
                        {
                            0.
                        } else {
                            0.5
                        }
                    } else {
                        cubic_bezier(t.motion_loop_easing, phase)
                    };
                    let peak = -1. + 4. * progress;
                    let highlight = t.color_surface_subtle.hsla();
                    let invisible = gpui::Hsla { a: 0., ..highlight };
                    let corner =
                        radius.min(f32::from(bounds.size.width.min(bounds.size.height)) / 2.);
                    // CSS's 200%-wide, three-stop background is two gradients.
                    // Paint each through a rectangular mask, retaining the bar's
                    // rounded silhouette on both halves of the highlight.
                    for (start, end, from, to) in [
                        (0., peak.clamp(0., 1.), invisible, highlight),
                        (peak.clamp(0., 1.), 1., highlight, invisible),
                    ] {
                        if end <= start {
                            continue;
                        }
                        let (a, b) = if from.a == 0. {
                            (peak - 1., peak)
                        } else {
                            (peak, peak + 1.)
                        };
                        let mask = Bounds::new(
                            point(bounds.origin.x + bounds.size.width * start, bounds.origin.y),
                            size(bounds.size.width * (end - start), bounds.size.height),
                        );
                        window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                            window.paint_quad(
                                fill(
                                    bounds,
                                    linear_gradient(
                                        90.,
                                        linear_color_stop(from, a),
                                        linear_color_stop(to, b),
                                    ),
                                )
                                .corner_radii(px(corner)),
                            );
                        });
                    }
                    if !frame.frozen && !frame.reduced {
                        window.request_animation_frame();
                    }
                },
            )
            .absolute()
            .inset_0(),
        );
        match self.shape {
            SkeletonShape::Text => bar
                .w(self.width.unwrap_or(Length::Fraction(1.0)).definite())
                .h(px(t.body_small.line_height))
                .rounded(px(t.radius_tiny)),
            SkeletonShape::Circle => {
                let bar = bar.rounded(px(t.radius_full));
                match self.width {
                    Some(width) => bar.w(width.definite()).h(width.definite()),
                    None => bar,
                }
            }
            SkeletonShape::Rect => {
                let bar = bar.rounded(px(t.radius_small));
                let bar = match self.width {
                    Some(width) => bar.w(width.definite()),
                    None => bar,
                };
                match self.height {
                    Some(height) => bar.h(height.definite()),
                    None => bar,
                }
            }
        }
    }
}

/// A paragraph's worth of bars. The last stops short at three fifths, the
/// way a paragraph ends.
#[derive(IntoElement)]
pub struct SkeletonText {
    lines: usize,
}

impl SkeletonText {
    pub fn new() -> Self {
        SkeletonText { lines: 3 }
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = lines;
        self
    }
}

impl Default for SkeletonText {
    fn default() -> Self {
        SkeletonText::new()
    }
}

impl RenderOnce for SkeletonText {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let lines = self.lines;
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(t.spacing_1_5))
            .children((0..lines).map(move |index| {
                let bar = Skeleton::new();
                if index + 1 == lines {
                    // A proportion rather than a length: there is no step of
                    // the scale to compose three fifths out of.
                    div().w(relative(t.skeleton_last_line_width)).child(bar)
                } else {
                    div().w_full().child(bar)
                }
            }))
    }
}
