//! A column's edge. The target straddles the rule between the columns so it
//! can be taken hold of without aiming, and draws nothing until the pointer
//! is on it — the rule is already there, drawn by the column.

use dazzui::ActiveTheme;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement,
    RenderOnce, Styled, Window, div, px,
};

/// Which column this is the edge of: it grows away from that side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// A column: where it starts, and how far it may go.
#[derive(Clone, Copy, Debug)]
pub struct Column {
    pub initial: f32,
    pub min: f32,
    pub max: f32,
}

impl Column {
    pub fn clamp(&self, width: f32) -> f32 {
        width.round().clamp(self.min, self.max)
    }
}

pub const LIST: Column = Column {
    initial: 272.0,
    min: 200.0,
    max: 420.0,
};

pub const THEMES: Column = Column {
    initial: 336.0,
    min: 280.0,
    max: 560.0,
};

/// A column edge being held: where the pointer took it, and the width then.
#[derive(Clone, Copy, Debug)]
pub struct Drag {
    pub side: Side,
    pub start: f32,
    pub width: f32,
}

#[derive(IntoElement)]
pub struct Resizer {
    side: Side,
    on_down: Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App)>,
}

impl Resizer {
    pub fn new(
        side: Side,
        on_down: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Resizer {
            side,
            on_down: Box::new(on_down),
        }
    }
}

impl RenderOnce for Resizer {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let group = match self.side {
            Side::Left => "resizer-left",
            Side::Right => "resizer-right",
        };
        let accent = t.color_primary.shade(500).hsla();
        let width = t.spacing_2;
        // The left edge stands inside the window, which clips at its own
        // edge; the right one straddles the rule the panel draws.
        let (left, line) = match self.side {
            Side::Left => (0.0, 0.0),
            Side::Right => (
                -(t.spacing_1 + t.stroke_hairline),
                width / 2.0 - t.spacing_px,
            ),
        };
        div()
            .id(ElementId::Name(group.into()))
            .group(group)
            .absolute()
            .top_0()
            .bottom_0()
            .left(px(left))
            .w(px(width))
            .cursor_col_resize()
            .on_mouse_down(MouseButton::Left, self.on_down)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left(px(line))
                    .w(px(t.spacing_0_5))
                    .group_hover(group, move |style| style.bg(accent)),
            )
    }
}
