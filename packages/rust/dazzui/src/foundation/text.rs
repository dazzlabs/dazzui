//! Plain labels use GPUI's text shaper and the display's pixel grid.

use gpui::{
    AnyElement, App, AvailableSpace, Bounds, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, SharedString, Size, Styled,
    TextAlign, TextOverflow, WhiteSpace, Window, WrappedLine, div, px,
};
use std::{cell::RefCell, rc::Rc};

/// Prepare shaping runs with the authored DazzUI font stack, preserving
/// grapheme clusters and the style's weight, features and decorations.
pub fn text_runs(
    text: &str,
    style: &gpui::TextStyle,
    size: Pixels,
    window: &Window,
) -> Vec<gpui::TextRun> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    return crate::platform_fonts::runs(text, style, size, window);
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = (size, window);
        vec![style.to_run(text.len())]
    }
}

/// Prepare a plain string with the kit's font stack and display-pixel
/// measurement. Rich elements pass through unchanged. Controls do this for
/// their own children; use it for surrounding text drawn directly with GPUI.
pub fn text_child(child: impl IntoElement) -> AnyElement {
    precise_text(child.into_any_element())
}

/// Preserve rich children. The SDK rounds a string's intrinsic advance up to
/// whole logical pixels; here measurement keeps the display's finer precision.
pub(crate) fn precise_text(mut element: AnyElement) -> AnyElement {
    if let Some(child) = element.downcast_mut::<AnyElement>() {
        return precise_text(std::mem::replace(child, gpui::Empty.into_any_element()));
    }
    let text = element.downcast_mut::<SharedString>().cloned().or_else(|| {
        element
            .downcast_mut::<&'static str>()
            .map(|text| SharedString::from(*text))
    });
    match text {
        Some(text) => LabelText(text, false).into_any_element(),
        None => element,
    }
}

struct LabelText(SharedString, bool);

/// An inline font-content box retains its inherited leading for painting.
/// Rich children keep their own layout model.
pub(crate) fn inline_text(element: AnyElement) -> AnyElement {
    let mut element = precise_text(element);
    if let Some(text) = element.downcast_mut::<LabelText>() {
        text.1 = true;
    }
    element
}

/// A note's leading space shares its text run rather than rounding alone.
pub(crate) fn text_with_prefix(
    prefix: &'static str,
    mut element: AnyElement,
) -> Option<AnyElement> {
    if let Some(child) = element.downcast_mut::<AnyElement>() {
        return text_with_prefix(
            prefix,
            std::mem::replace(child, gpui::Empty.into_any_element()),
        );
    }
    let text = element
        .downcast_mut::<LabelText>()
        .map(|label| label.0.clone())
        .or_else(|| element.downcast_mut::<SharedString>().cloned())
        .or_else(|| {
            element
                .downcast_mut::<&'static str>()
                .map(|text| SharedString::from(*text))
        });
    match text {
        Some(text) if text.is_empty() => None,
        Some(text) => Some(text_child(format!("{prefix}{text}"))),
        None => Some(
            div()
                .flex()
                .child(text_child(prefix))
                .child(element)
                .into_any_element(),
        ),
    }
}

impl IntoElement for LabelText {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

pub(crate) struct LabelLayout {
    lines: Rc<RefCell<Vec<WrappedLine>>>,
    line_height: Pixels,
    box_line_height: Pixels,
    baseline: Pixels,
    align: TextAlign,
}

impl Element for LabelText {
    type RequestLayoutState = LabelLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        _: &mut App,
    ) -> (LayoutId, LabelLayout) {
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = style
            .line_height
            .to_pixels(font_size.into(), window.rem_size());
        let (box_line_height, baseline) = if self.1 {
            let font = window.text_system().resolve_font(&style.font());
            let ascent = window.text_system().ascent(font, font_size).abs().round();
            let descent = window.text_system().descent(font, font_size).abs().round();
            (ascent + descent, ascent)
        } else {
            (line_height, window.text_baseline())
        };
        let runs = text_runs(&self.0, &style, font_size, window);
        let text = self.0.clone();
        let lines = Rc::new(RefCell::new(Vec::new()));
        let layout = LabelLayout {
            lines: lines.clone(),
            line_height,
            box_line_height,
            baseline,
            align: style.text_align,
        };
        let id = window.request_measured_layout(
            gpui::Style {
                baseline_offset: Some(baseline.into()),
                ..Default::default()
            },
            move |known, available, window, cx| {
                let available_width = known.width.or(match available.width {
                    AvailableSpace::Definite(width) => Some(width),
                    _ => None,
                });
                let wrap_width = (style.white_space == WhiteSpace::Normal)
                    .then_some(available_width)
                    .flatten();
                let mut runs = runs.clone();
                let text = match &style.text_overflow {
                    Some(TextOverflow::Truncate(suffix)) => {
                        let width = known.width.or_else(|| {
                            available_width.map(|width| width * style.line_clamp.unwrap_or(1))
                        });
                        match width {
                            Some(width) => cx
                                .text_system()
                                .line_wrapper(style.font(), font_size)
                                .truncate_line(text.clone(), width, suffix, &mut runs),
                            None => text.clone(),
                        }
                    }
                    None => text.clone(),
                };
                match window.text_system().shape_text(
                    text,
                    font_size,
                    &runs,
                    wrap_width,
                    style.line_clamp,
                ) {
                    Ok(shaped) => {
                        let mut size = Size {
                            width: px(0.),
                            height: px(0.),
                        };
                        for line in &shaped {
                            let line_size = line.size(box_line_height);
                            size.width = size.width.max(line_size.width);
                            size.height += line_size.height;
                        }
                        // Keep CSS layout precision after shaping. Rounding
                        // each run to display pixels accumulates across a row
                        // and shifts centred or trailing controls. Taffy still
                        // aligns the final boxes to the display pixel grid.
                        let scale = 64.;
                        size.width = if style.white_space == WhiteSpace::Normal {
                            (size.width * scale).ceil() / scale
                        } else {
                            (size.width * scale).round() / scale
                        };
                        *lines.borrow_mut() = shaped.into_vec();
                        size
                    }
                    Err(error) => {
                        eprintln!("label layout: {error}");
                        lines.borrow_mut().clear();
                        Size::default()
                    }
                }
            },
        );
        (id, layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut LabelLayout,
        _: &mut Window,
        _: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut LabelLayout,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut origin = bounds.origin;
        let color = window.text_style().color;
        for line in layout.lines.borrow_mut().iter_mut() {
            line.set_color(color);
            let mut paint_origin = origin;
            paint_origin.y += layout.baseline - line.baseline_offset(layout.line_height);
            if let Err(error) = line.paint_background(
                paint_origin,
                layout.line_height,
                layout.align,
                Some(bounds),
                window,
                cx,
            ) {
                eprintln!("label background: {error}");
            }
            if let Err(error) = line.paint(
                paint_origin,
                layout.line_height,
                layout.align,
                Some(bounds),
                window,
                cx,
            ) {
                eprintln!("label paint: {error}");
            }
            origin.y += line.size(layout.box_line_height).height;
        }
    }
}
