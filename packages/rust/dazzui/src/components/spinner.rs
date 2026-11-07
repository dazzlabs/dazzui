//! A ring with one segment picked out, turning once a second.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use gpui::{
    App, Bounds, Hsla, IntoElement, ParentElement, PathBuilder, Pixels, RenderOnce, Styled, Window,
    canvas, div, point, px,
};

use crate::foundation::motion::Motion;
use crate::foundation::{Size, Tint};
use crate::theme::ActiveTheme;

/// A spinner: the track is the tint at quarter strength, the head the tint
/// itself. On a filled control the ring goes white, since a tinted spinner
/// on its own tint's fill is invisible.
#[derive(IntoElement)]
pub struct Spinner {
    size: Size,
    tint: Tint,
    on_accent: bool,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            size: Size::Medium,
            tint: Tint::Primary,
            on_accent: false,
        }
    }

    /// `small`, `medium` or `large`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// White ring for a spinner sitting on a filled control's own tint.
    pub fn on_accent(mut self, on_accent: bool) -> Self {
        self.on_accent = on_accent;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Spinner::new()
    }
}

impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let edge = match self.size {
            Size::Tiny | Size::Small => t.spacing_3_5,
            Size::Medium => t.spacing_4,
            Size::Large => t.spacing_5,
        };
        let (track, head) = if self.on_accent {
            (
                t.color_on_accent.alpha(t.spinner_on_accent_track_alpha),
                t.color_on_accent.hsla(),
            )
        } else {
            let tint = self.tint.ramp(t).shade(600);
            (tint.alpha(t.spinner_track_alpha), tint.hsla())
        };
        let width = t.spacing_0_5;
        div().flex_none().size(px(edge)).child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, cx| {
                    let frame = Motion::frame(cx);
                    let turn = frame.cycle(t.spinner_duration, t.spinner_reduced_duration);
                    paint_ring(bounds, width, track, head, turn * TAU, window);
                    if !frame.frozen {
                        window.request_animation_frame();
                    }
                },
            )
            .size_full(),
        )
    }
}

/// The ring, and its head — the top quarter a CSS `border-top-color` picks
/// out — turned by `angle`.
fn paint_ring(
    bounds: Bounds<Pixels>,
    width: f32,
    track: Hsla,
    head: Hsla,
    angle: f32,
    window: &mut Window,
) {
    let center = bounds.center();
    let radius = (f32::from(bounds.size.width) - width) / 2.0;
    let at = |theta: f32| {
        point(
            center.x + px(radius * theta.cos()),
            center.y + px(radius * theta.sin()),
        )
    };
    let radii = point(px(radius), px(radius));

    let mut ring = PathBuilder::stroke(px(width));
    ring.move_to(at(0.0));
    ring.arc_to(radii, px(0.), false, true, at(PI));
    ring.arc_to(radii, px(0.), false, true, at(TAU));
    if let Ok(path) = ring.build() {
        window.paint_path(path, track);
    }

    let start = -FRAC_PI_2 - PI / 4.0 + angle;
    let mut arc = PathBuilder::stroke(px(width));
    arc.move_to(at(start));
    arc.arc_to(radii, px(0.), false, true, at(start + FRAC_PI_2));
    if let Ok(path) = arc.build() {
        window.paint_path(path, head);
    }
}
