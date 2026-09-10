use gpui::{App, IntoElement, Pixels, RenderOnce, Styled, Window, div, px, relative};

use crate::prelude::*;

/// What a bar is standing in for.
///
/// `Text` is a bar on the line box of the type it replaces; `Circle` is an
/// avatar or a glyph; `Rect` is everything with a box of its own — a
/// thumbnail, a chart, a card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SkeletonShape {
    #[default]
    Text,
    Circle,
    Rect,
}

impl SkeletonShape {
    pub const ALL: [SkeletonShape; 3] = [
        SkeletonShape::Text,
        SkeletonShape::Circle,
        SkeletonShape::Rect,
    ];
}

/// The shape of something that has not arrived.
///
/// A [`Spinner`](crate::Spinner) says the app is busy; a skeleton says *what*
/// is coming, and where it will be — which is why it belongs on the first load
/// of a view whose layout is already known, and a spinner belongs on an action
/// whose result is not. Drawing one for content that then lands somewhere else
/// is worse than drawing nothing.
///
/// The fill is `surface-sunken`, the same dead step a disabled control flattens
/// to, which is exactly what this is: a place where something will be, drawn as
/// though it were not there yet.
///
/// It does not sweep. React runs a highlight across the groove, and this port
/// draws the groove alone — a repeating gradient is a shader, and every other
/// moving part here (the spinner's ring) is a shape GPUI can rotate. The
/// still frame is the same colour either way, which is what the reference
/// measures.
#[derive(IntoElement)]
pub struct Skeleton {
    shape: SkeletonShape,
    width: Option<Pixels>,
    height: Option<Pixels>,
}

impl Skeleton {
    pub fn new() -> Self {
        Skeleton {
            shape: SkeletonShape::Text,
            width: None,
            height: None,
        }
    }

    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Skeleton::new()
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let radius = match self.shape {
            SkeletonShape::Text => vars.radius_tiny,
            SkeletonShape::Circle => vars.radius_full,
            SkeletonShape::Rect => vars.radius_small,
        };

        div()
            .flex_none()
            .bg(vars.color_surface_sunken)
            .rounded(radius)
            .map(|el| match self.shape {
                // A bar on the line box of body copy, so it occupies the run
                // the sentence it replaces would have.
                SkeletonShape::Text => {
                    el.h(vars.body_small.line_height)
                        .map(|el| match self.width {
                            Some(width) => el.w(width),
                            None => el.w_full(),
                        })
                }
                // Square from the width alone, so one measurement sizes it.
                SkeletonShape::Circle => {
                    let side = self.width.or(self.height).unwrap_or(px(40.));
                    el.w(side).h(side)
                }
                SkeletonShape::Rect => el
                    .map(|el| match self.width {
                        Some(width) => el.w(width),
                        None => el.w_full(),
                    })
                    .map(|el| match self.height {
                        Some(height) => el.h(height),
                        None => el.h(relative(1.)),
                    }),
            })
    }
}

/// A paragraph's worth of bars.
///
/// The last one stops short because a full-width final line reads as a table
/// rather than as prose, and that ragged edge is most of what makes a block of
/// bars legible as text at all.
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        /// Three fifths: short enough to read as the end of a paragraph at any
        /// measure, long enough not to read as a heading. A proportion rather
        /// than a length, so there is no step of the scale to compose it from.
        const LAST_LINE: f32 = 0.6;

        div()
            .flex()
            .flex_col()
            // The leading between two lines of body copy rather than a step of
            // the spacing scale, so a stack of bars occupies the run the
            // paragraph would have.
            .gap(vars.spacing_1_5)
            .w_full()
            .children((0..self.lines).map(|line| {
                div()
                    .map(|el| {
                        if line + 1 == self.lines {
                            el.w(relative(LAST_LINE))
                        } else {
                            el.w_full()
                        }
                    })
                    .child(Skeleton::new())
            }))
    }
}
