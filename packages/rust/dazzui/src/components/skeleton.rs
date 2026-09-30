//! The shape of something that has not arrived.

use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px, relative};

use crate::foundation::Length;
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
/// React sweeps a `surface-subtle` highlight across it; here the groove is
/// drawn still, which is what the web draws under reduced motion.
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
        let bar = div().flex_none().bg(t.color_surface_sunken.hsla());
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
                    div().w(relative(0.6)).child(bar)
                } else {
                    div().w_full().child(bar)
                }
            }))
    }
}
