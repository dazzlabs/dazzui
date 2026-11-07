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
    // The line strut keeps the surrounding font's ascent/descent in the
    // baseline group even when the line contains only a replaced element.
    // Text and Code supply measured baselines; an SVG uses its bottom edge.
    let kids = node
        .children()
        .iter()
        .zip(node.kids(window, cx))
        .map(|(json, kid)| {
            if json.is_object() && (json.get("$el").is_some() || json.get("$icon").is_some()) {
                InlineSlot(dazzui::foundation::inline_context(kid).into_any_element())
                    .into_any_element()
            } else {
                // A run of words is free to narrow to the line it is given
                // and wrap there, as the web's inline text does: GPUI
                // measures text's min-content as its whole line, so a flex
                // item holding it would otherwise never shrink.
                InlineSlot(InlineText(kid).into_any_element()).into_any_element()
            }
        })
        .collect::<Vec<_>>();
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .when_some(color, |this, color| this.text_color(color))
        .when(node.flag("strong"), |this| {
            this.font_weight(t.label_strong.font_weight())
        })
        .children(kids)
        .into_any_element()
}

#[derive(gpui::IntoElement)]
struct InlineSlot(AnyElement);

impl gpui::RenderOnce for InlineSlot {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        // Keep the strut with its run: an overlong word must not wrap away
        // from a separate zero-width strut and leave an empty first line.
        div()
            .flex()
            .min_w_0()
            .items_baseline()
            .child(LineStrut)
            .child(self.0)
    }
}

#[derive(gpui::IntoElement)]
struct LineStrut;

impl gpui::RenderOnce for LineStrut {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let style = window.text_style();
        let leading = style
            .line_height
            .to_pixels(style.font_size, window.rem_size());
        div()
            .flex_none()
            .w_0()
            .h(leading)
            .baseline_offset(window.text_baseline())
    }
}

#[derive(gpui::IntoElement)]
struct InlineText(AnyElement);

impl gpui::RenderOnce for InlineText {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .min_w_0()
            .baseline_offset(window.text_baseline())
            .child(self.0)
    }
}

/// The type a story's page is set in, which a scene inherits — the React
/// page's `body`.
pub fn page_type(element: gpui::Div, cx: &App) -> gpui::Div {
    let t = cx.theme();
    element
        .type_style(t.body_medium)
        .text_color(t.color_content.hsla())
}
