use std::f32::consts::TAU;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, Hsla, IntoElement, ParentElement, PathBuilder, Pixels,
    RenderOnce, Styled, Window, canvas, div, point, px,
};

use crate::Tint;
use crate::prelude::*;

/// How much of the ring the head covers, as a fraction of the turn.
const HEAD_SWEEP: f32 = 0.25;

/// The track's share of the tint, and the on-accent ring's share of the white.
///
/// Both are written in `spinner.css` as literals rather than as tokens —
/// a ring drawn out of one colour needs two weights of it, and the graph has
/// no role for the lighter one.
const TRACK_ALPHA: f32 = 0.25;
const ON_ACCENT_TRACK_ALPHA: f32 = 0.35;

/// One turn a second — fast enough to read as work, slow enough not to read
/// as alarm.
const TURN: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A ring with one segment picked out: the track is the tint at quarter
/// strength, the head the tint itself, and the whole thing turns once a
/// second.
///
/// CSS draws it as a border with one side coloured and spins the box. GPUI
/// has no transform on a box, so the ring is a path — which is the same
/// drawing by other means, and the geometry still comes off the tokens.
#[derive(IntoElement)]
pub struct Spinner {
    size: SpinnerSize,
    tint: Tint,
    /// White ring for a spinner sitting on a filled control's own tint.
    on_accent: bool,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            size: SpinnerSize::Medium,
            tint: Tint::Primary,
            on_accent: false,
        }
    }

    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let size = match self.size {
            SpinnerSize::Small => vars.spacing_3_5,
            SpinnerSize::Medium => vars.spacing_4,
            SpinnerSize::Large => vars.spacing_5,
        };
        let thickness = vars.spacing_0_5;

        // A wash rather than a ramp step, so the track sits as correctly over
        // a tinted chip or a dark canvas as over the paper. On a filled
        // control the ring goes white — a tinted spinner on its own tint's
        // fill is invisible.
        let (track, head) = if self.on_accent {
            (
                vars.color_on_accent.opacity(ON_ACCENT_TRACK_ALPHA),
                vars.color_on_accent,
            )
        } else {
            (
                vars.wash(self.tint, TRACK_ALPHA),
                vars.ramp(self.tint).shade(600),
            )
        };

        div().flex_none().size(size).with_animation(
            "spinner-turn",
            Animation::new(TURN).repeat(),
            move |el, delta| el.child(ring(size, thickness, track, head, delta)),
        )
    }
}

/// The ring itself: a full circle in the track colour, and the head over it.
fn ring(size: Pixels, thickness: Pixels, track: Hsla, head: Hsla, turn: f32) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let center = bounds.center();
            let radius = f32::from(size - thickness) / 2.0;
            let at = |angle: f32| {
                point(
                    center.x + px(radius * angle.cos()),
                    center.y + px(radius * angle.sin()),
                )
            };
            let radii = point(px(radius), px(radius));

            let mut circle = PathBuilder::stroke(thickness);
            circle.move_to(at(0.0));
            circle.arc_to(radii, px(0.), false, true, at(TAU / 2.0));
            circle.arc_to(radii, px(0.), false, true, at(TAU));
            if let Ok(path) = circle.build() {
                window.paint_path(path, track);
            }

            let start = turn * TAU;
            let mut arc = PathBuilder::stroke(thickness);
            arc.move_to(at(start));
            arc.arc_to(radii, px(0.), false, true, at(start + HEAD_SWEEP * TAU));
            if let Ok(path) = arc.build() {
                window.paint_path(path, head);
            }
        },
    )
    .size(size)
}
