//! The conformance hook: every node codegen addressed (`"$at"`) is drawn
//! through here, so the collector can find the box drawn at the same
//! address React's was.
//!
//! Outside a collection this is the element itself. While collecting, it is
//! wrapped in an element that records the bounds its child laid out at.

use std::cell::RefCell;
use std::collections::BTreeMap;

use dazzui::{ActiveTheme, ThemeTokens};
use gpui::{
    AnyElement, App, Bounds, DefiniteLength, Display, Element, ElementId, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, ObservedPaint, ObservedSurface, Pixels, Rgba,
    Window,
};
use serde_json::{Value, json};

thread_local! {
    /// `Some` while collecting: every specimen's bounds, by its index.
    static FRAMES: RefCell<Option<BTreeMap<usize, Bounds<Pixels>>>> = const { RefCell::new(None) };
    static STYLES: RefCell<BTreeMap<usize, PaintReading>> = const { RefCell::new(BTreeMap::new()) };
}

struct PaintReading {
    root: Value,
    decorated: Option<Value>,
}

/// Starts recording, clearing what an earlier scene recorded.
pub fn start() {
    STYLES.with(|styles| styles.borrow_mut().clear());
    FRAMES.with(|frames| *frames.borrow_mut() = Some(BTreeMap::new()));
}

/// Keep addresses within one painted frame; animated boxes move between frames.
pub fn begin_frame() {
    STYLES.with(|styles| styles.borrow_mut().clear());
    FRAMES.with(|frames| {
        if let Some(frames) = frames.borrow_mut().as_mut() {
            frames.clear();
        }
    });
}

/// What has been recorded since [`start`].
pub fn frames() -> BTreeMap<usize, Bounds<Pixels>> {
    FRAMES.with(|frames| frames.borrow().clone().unwrap_or_default())
}

/// Paint properties observed in the current frame, independently from boxes.
pub fn styles(specimens: &[Value]) -> BTreeMap<usize, Value> {
    STYLES.with(|styles| {
        styles
            .borrow()
            .iter()
            .map(|(&index, reading)| {
                // These native controls have a layout-only popup/clear-button
                // wrapper. Read the field it actually paints, as React reads
                // a NumberField's group or a ShortcutRecorder's inner button.
                let inner = matches!(
                    specimens
                        .get(index)
                        .and_then(|specimen| specimen["component"].as_str()),
                    Some(
                        "select"
                            | "combobox"
                            | "autocomplete"
                            | "date-range-picker"
                            | "shortcut-recorder"
                    )
                );
                let value = if inner {
                    reading.decorated.as_ref().unwrap_or(&reading.root)
                } else {
                    &reading.root
                };
                (index, value.clone())
            })
            .collect()
    })
}

pub fn wrap(at: Option<&Value>, element: impl IntoElement, _path: &str) -> AnyElement {
    wrap_as(at, element, false)
}

/// A detached sheet's actual surface takes precedence over its in-flow trigger.
pub fn surface(at: Option<&Value>, element: impl IntoElement) -> AnyElement {
    wrap_as(at, element, true)
}

fn wrap_as(at: Option<&Value>, element: impl IntoElement, surface: bool) -> AnyElement {
    let collecting = FRAMES.with(|frames| frames.borrow().is_some());
    match at.and_then(Value::as_u64) {
        Some(index) if collecting => Specimen {
            index: index as usize,
            child: element.into_any_element(),
            surface,
        }
        .into_any_element(),
        _ => element.into_any_element(),
    }
}

/// Lays its child out as if it were not there — the child's own layout id
/// is its — and records where it landed.
struct Specimen {
    index: usize,
    child: AnyElement,
    surface: bool,
}

impl IntoElement for Specimen {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Specimen {
    type RequestLayoutState = LayoutId;
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
        cx: &mut App,
    ) -> (LayoutId, LayoutId) {
        let layout = self.child.request_layout(window, cx);
        (layout, layout)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut LayoutId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let bounds = window.painted_bounds(bounds);
        FRAMES.with(|frames| {
            if let Some(frames) = frames.borrow_mut().as_mut() {
                // A specimen drawn twice (a trigger inside an open popup's
                // twin) keeps the first box, as the walk addresses it.
                if self.surface {
                    frames.insert(self.index, bounds);
                } else {
                    frames.entry(self.index).or_insert(bounds);
                }
            }
        });
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut LayoutId,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let painted = window.painted_bounds(bounds);
        let (_, paint) =
            window.observe_element_paint(bounds, |window| self.child.paint(window, cx));
        let values = PaintReading {
            root: values(&paint, paint.surface.as_ref(), painted, cx.theme()),
            decorated: paint
                .decorated_surface
                .as_ref()
                .map(|surface| values(&paint, Some(surface), painted, cx.theme())),
        };
        STYLES.with(|styles| {
            let mut styles = styles.borrow_mut();
            if self.surface {
                styles.insert(self.index, values);
            } else {
                styles.entry(self.index).or_insert(values);
            }
        });
    }
}

fn hex(color: Hsla) -> String {
    let color = Rgba::from(color);
    let byte = |channel: f32| (channel.clamp(0., 1.) * 255.).round() as u8;
    let (r, g, b, a) = (byte(color.r), byte(color.g), byte(color.b), byte(color.a));
    match a {
        0 => "#00000000".into(),
        255 => format!("#{r:02x}{g:02x}{b:02x}"),
        _ => format!("#{r:02x}{g:02x}{b:02x}{a:02x}"),
    }
}

fn values(
    paint: &ObservedPaint,
    surface: Option<&ObservedSurface>,
    bounds: Bounds<Pixels>,
    theme: &ThemeTokens,
) -> Value {
    let mut values = serde_json::Map::new();
    // An unstyled canvas host is not the custom shape it draws. The path
    // mesh has lost its source stroke/radius, so those surface properties
    // remain unread rather than reporting the host's square, borderless box.
    let surface = surface.filter(|_| !paint.custom_paths || paint.decorated_surface.is_some());
    if let Some(surface) = surface {
        let style = &surface.style;
        let rem = surface.rem_size;
        let pixels = |length: DefiniteLength| match length {
            DefiniteLength::Absolute(length) => Some(f32::from(length.to_pixels(rem))),
            DefiniteLength::Fraction(_) => None,
        };
        let corner = f32::from(style.corner_radii.to_pixels(rem).top_left)
            .min(f32::from(bounds.size.width.min(bounds.size.height)) / 2.);
        let stroke = f32::from(style.border_widths.top.to_pixels(rem));
        let fill = match &style.background {
            None => Some("#00000000".to_string()),
            Some(fill) => fill
                .color()
                .and_then(|background| background.solid_color())
                .map(hex),
        };
        let shadow = if style.box_shadow.is_empty() {
            Some("none")
        } else {
            [
                ("2xs", theme.shadow_2xs),
                ("xs", theme.shadow_xs),
                ("sm", theme.shadow_sm),
                ("md", theme.shadow_md),
                ("lg", theme.shadow_lg),
                ("xl", theme.shadow_xl),
                ("2xl", theme.shadow_2xl),
            ]
            .into_iter()
            .find_map(|(name, layers)| {
                (style.box_shadow == dazzui::tokens::shadows(layers)).then_some(name)
            })
        };
        values.extend(json!({
            "corner": corner, "stroke": stroke,
            "strokeColor": if stroke > 0. { Some(hex(style.border_color.unwrap_or_default())) } else { None },
            "fill": fill, "paddingX": pixels(style.padding.left), "paddingY": pixels(style.padding.top),
            "gap": if matches!(style.display, Display::Flex | Display::Grid) { pixels(style.gap.width) } else { None },
            "shadow": shadow,
        }).as_object().unwrap().clone());
    }
    values.insert(
        "ink".into(),
        json!(
            paint
                .text
                .as_ref()
                .map(|text| text.color)
                .or(paint.glyph_ink)
                .map(hex)
        ),
    );
    if let Some(text) = &paint.text {
        values.insert("baseline".into(), json!(f32::from(text.baseline.y)));
        values.insert("fontSize".into(), json!(f32::from(text.font_size)));
        values.insert(
            "fontWeight".into(),
            json!(text.font_weight.map(|weight| weight.0)),
        );
        values.insert("lineHeight".into(), json!(f32::from(text.line_height)));
    }
    Value::Object(values)
}
