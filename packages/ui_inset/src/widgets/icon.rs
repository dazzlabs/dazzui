//! Fluent glyphs shared with React; SVG curves are retained rather than redrawn.
use crate::Theme;
use inset_embedder::{Canvas, Color, FillRule, Offset, Paint, PathBuilder, Size};
use inset_foundation::App;
use inset_rendering::CustomPainter;
use inset_widgets::*;
use kurbo::{BezPath, PathEl};
#[derive(Debug, Clone, Copy)]
pub enum FluentGlyph {
    ArrowDownload20Regular,
    Circle20Filled,
}
impl FluentGlyph {
    fn svg(self) -> &'static str {
        match self {
            Self::ArrowDownload20Regular => {
                include_str!("../../assets/icons/arrow_download_20_regular.svg")
            }
            Self::Circle20Filled => include_str!("../../assets/icons/circle_20_filled.svg"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FluentIcon(pub FluentGlyph);
impl StatelessWidget for FluentIcon {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let theme = IconTheme::of(app, context);
        let color = theme
            .color
            .as_ref()
            .map(|color| color.color())
            .unwrap_or(v.color_content);
        let size = theme.size.unwrap_or(v.spacing_5);
        let data = self
            .0
            .svg()
            .split("d=\"")
            .nth(1)
            .unwrap()
            .split('"')
            .next()
            .unwrap();
        let path = BezPath::from_svg(data).expect("bundled Fluent SVG path");
        SizedBox::square(Some(size))
            .child(CustomPaint::new().painter(GlyphPaint { path, color }))
            .into_widget()
    }
}
struct GlyphPaint {
    path: BezPath,
    color: Color,
}
impl CustomPainter for GlyphPaint {
    fn paint(&self, _: &mut App, canvas: &mut Canvas, size: Size) {
        let point =
            |p: kurbo::Point| Offset::new(p.x * size.width() / 20.0, p.y * size.height() / 20.0);
        let mut path = PathBuilder::new();
        for element in self.path.elements() {
            match *element {
                PathEl::MoveTo(p) => {
                    path.move_to(point(p));
                }
                PathEl::LineTo(p) => {
                    path.line_to(point(p));
                }
                PathEl::QuadTo(c, p) => {
                    path.quad_to(point(c), point(p));
                }
                PathEl::CurveTo(a, b, p) => {
                    path.cubic_to(point(a), point(b), point(p));
                }
                PathEl::ClosePath => {
                    path.close();
                }
            }
        }
        canvas.draw_path(
            &path.build(),
            FillRule::NonZero,
            &Paint {
                color: self.color.into(),
                ..Default::default()
            },
        );
    }
    fn should_repaint(&self, _: &App, _: &dyn CustomPainter) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
