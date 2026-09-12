use crate::{ControlSize, Theme, Tint};
use inset_embedder::Color;
use inset_foundation::App;
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_widgets::*;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Raised,
    Sunken,
    Outlined,
    Tinted,
}
#[derive(Debug, Clone)]
pub struct Card {
    pub child: WidgetRef,
    pub tint: Tint,
    pub variant: CardVariant,
    pub size: ControlSize,
}
impl Card {
    pub fn new<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            tint: Tint::Primary,
            variant: CardVariant::Raised,
            size: ControlSize::Medium,
        }
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
}
impl StatelessWidget for Card {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let (surface, border) = match self.variant {
            CardVariant::Raised => (v.color_surface, v.color_border),
            CardVariant::Sunken => (v.color_surface_muted, v.color_border),
            CardVariant::Outlined => (Color::from_argb(0, 0, 0, 0), v.color_border),
            CardVariant::Tinted => (
                v.wash(self.tint, v.wash_surface),
                v.wash(self.tint, v.wash_edge),
            ),
        };
        let padding = match self.size {
            ControlSize::Small => EdgeInsetsGeometry::all(v.spacing_3),
            ControlSize::Large => EdgeInsetsGeometry::all(v.spacing_4),
            _ => EdgeInsetsGeometry::symmetric(v.spacing_3, v.spacing_3_5),
        };
        Container::new()
            .padding(padding)
            .decoration(
                BoxDecoration::new()
                    .color(surface)
                    .border(Border::all(
                        border,
                        v.stroke_hairline,
                        BorderStyle::Solid,
                        -1.0,
                    ))
                    .border_radius(BorderRadiusGeometry::circular(v.radius_large)),
            )
            .child(self.child.clone())
            .into_widget()
    }
}
