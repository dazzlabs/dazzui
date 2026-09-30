//! What a story arranges its components with — the React example's
//! `components/stage.tsx`: `Row`, `Col`, `Frame` and `Text`, and nothing
//! else. Lengths are read as the web writes them: a spacing step for a gap,
//! rem or a percentage for a width.

use dazzui::{ActiveTheme, Length, StyledExt, Tint};
use gpui::{AnyElement, App, IntoElement, ParentElement, Styled, Window, div, prelude::*, px};

use crate::lib::scene::Node;

fn gap(node: &Node, cx: &App) -> f32 {
    let step = node.string("gap").unwrap_or_else(|| "3".into());
    cx.theme().spacing(&step).unwrap_or(cx.theme().spacing_3)
}

fn width(node: &Node) -> Option<Length> {
    node.string("width").and_then(|width| Length::parse(&width))
}

/// Instances side by side, wrapping when they run out of room. `between`
/// pushes the first and the last to the two edges instead.
pub fn row(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let between = node.string("justify").as_deref() == Some("between");
    let align = node.string("align").unwrap_or_else(|| "center".into());
    div()
        .flex()
        .flex_row()
        .when(!between, |this| this.flex_wrap())
        .when(between, |this| this.justify_between())
        .map(|this| align_items(this, &align))
        .gap(px(gap(node, cx)))
        .max_w_full()
        .when_some(width(node), |this, width| this.w(width.definite()))
        .children(node.kids(window, cx))
        .into_any_element()
}

/// Instances stacked, at the width the component needs.
pub fn col(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let align = node.string("align").unwrap_or_else(|| "stretch".into());
    div()
        .flex()
        .flex_col()
        .map(|this| align_items(this, &align))
        .gap(px(gap(node, cx)))
        .max_w_full()
        .when_some(width(node), |this, width| this.w(width.definite()))
        .children(node.kids(window, cx))
        .into_any_element()
}

fn align_items(element: gpui::Div, align: &str) -> gpui::Div {
    match align {
        "start" => element.items_start(),
        "end" => element.items_end(),
        "center" => element.items_center(),
        _ => element,
    }
}

/// A window, for what only makes sense inside one: a sidebar, a splitter.
pub fn frame(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let t = cx.theme();
    let width = node
        .string("width")
        .and_then(|w| Length::parse(&w))
        .unwrap_or(Length::Px(36.0 * 16.0));
    let height = node
        .string("height")
        .and_then(|h| Length::parse(&h))
        .unwrap_or(Length::Px(16.0 * 16.0));
    div()
        .flex()
        .overflow_hidden()
        .w(width.definite())
        .max_w_full()
        .h(height.definite())
        .border(px(t.stroke_hairline))
        .border_color(t.color_border.hsla())
        .rounded(px(t.radius_large))
        .bg(t.color_surface.hsla())
        .children(node.kids(window, cx))
        .into_any_element()
}

/// A run of text, which may hold components inline. `secondary` is the
/// quieter ink; `strong` the weight a name is; a tint colours it, and a
/// glyph inside takes the colour.
pub fn text(node: &Node, window: &mut Window, cx: &mut App) -> AnyElement {
    let t = cx.theme();
    let color = match node.option("tint", Tint::parse) {
        Some(tint) => Some(tint.ramp(t).shade(500).hsla()),
        None if node.string("tone").as_deref() == Some("secondary") => {
            Some(t.color_content_secondary.hsla())
        }
        None => None,
    };
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .when_some(color, |this, color| this.text_color(color))
        .when(node.flag("strong"), |this| {
            this.font_weight(t.label_strong.font_weight())
        })
        .children(node.kids(window, cx))
        .into_any_element()
}

/// The type a story's page is set in, which a scene inherits — the React
/// page's `body`.
pub fn page_type(element: gpui::Div, cx: &App) -> gpui::Div {
    let t = cx.theme();
    element
        .type_style(t.body_medium)
        .text_color(t.color_content.hsla())
}
