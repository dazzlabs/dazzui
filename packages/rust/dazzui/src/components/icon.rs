//! A glyph drawn at a size, in the colour of the text around it.

use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, Hsla, InspectorElementId, IntoElement,
    LayoutId, Pixels, SharedString, Style, TransformationMatrix, Window, px, size,
};

use crate::foundation::scoped;
use crate::theme::ActiveTheme;

/// The size an enclosing slot binds for the icons inside it — React's
/// `--dz-icon-size`. A sized control binds its profile's icon size; an icon
/// outside every slot is `icon.medium`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IconSize(pub f32);

/// `child`, with every icon inside it sized `size` unless it says otherwise.
pub fn icon_slot(size: f32, child: impl IntoElement) -> crate::foundation::scope::Provide {
    crate::foundation::provide(IconSize(size), child)
}

/// Draws a glyph, inheriting the surrounding text colour — CSS's
/// `fill: currentColor`, which is why no component passes a colour down.
///
/// A glyph is an asset path: the kit's own are in [`crate::glyphs`], served
/// by [`crate::Assets`]; an app serves its own.
pub struct Icon {
    path: SharedString,
    size: Option<f32>,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(path: impl Into<SharedString>) -> Self {
        Icon {
            path: path.into(),
            size: None,
            color: None,
        }
    }

    /// The rendered edge length, overriding the slot's.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// A colour of its own rather than the text's.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl IntoElement for Icon {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for Icon {
    type RequestLayoutState = ();
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
    ) -> (LayoutId, ()) {
        let edge = self
            .size
            .or_else(|| scoped::<IconSize>(cx).map(|slot| slot.0))
            .unwrap_or(cx.theme().icon_medium);
        self.size = Some(edge);
        let mut style = Style::default();
        style.size = size(px(edge).into(), px(edge).into());
        style.flex_shrink = 0.;
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut Window,
        _: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let color = self.color.unwrap_or_else(|| window.text_style().color);
        let _ = window.paint_svg(
            bounds,
            self.path.clone(),
            TransformationMatrix::unit(),
            color,
            cx,
        );
    }
}
