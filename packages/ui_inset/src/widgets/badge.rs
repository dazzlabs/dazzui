use crate::{ControlSize, ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_widgets::*;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    Filled,
    #[default]
    Tinted,
    Outlined,
    Plain,
    Raised,
}
#[derive(Debug, Clone)]
pub struct Badge {
    pub child: WidgetRef,
    pub tint: Tint,
    pub variant: BadgeVariant,
    pub size: ControlSize,
}
impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            child: Text::new(label).into_widget(),
            tint: Tint::Primary,
            variant: BadgeVariant::Tinted,
            size: ControlSize::Medium,
        }
    }
    pub fn child<K>(mut self, child: impl IntoWidget<K>) -> Self {
        self.child = child.into_widget();
        self
    }
    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }
}
impl StatelessWidget for Badge {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let variant = match self.variant {
            BadgeVariant::Filled => ControlVariant::Filled,
            BadgeVariant::Tinted => ControlVariant::Tinted,
            BadgeVariant::Outlined => ControlVariant::Outlined,
            _ => ControlVariant::Plain,
        };
        let mut c = v.control_colors(variant, self.tint, ControlState::Normal);
        if self.variant == BadgeVariant::Raised {
            c.surface = v.color_surface;
            c.content = v.color_content_muted;
        }
        let large = self.size == ControlSize::Large;
        let x = if matches!(self.size, ControlSize::Tiny | ControlSize::Small) {
            v.spacing_1_5
        } else {
            v.spacing_2
        };
        let y = if large { v.spacing_1 } else { v.spacing_0_5 };
        let mut face = v.label_strong;
        face.size = if large {
            v.label_medium.size
        } else {
            v.label_small.size
        };
        face.weight = v.label_medium.weight;
        let style = DefaultTextStyle::of(app, context)
            .style
            .merge(Some(&face.text_style().height(1.4).color(c.content)));
        Container::new()
            .padding(EdgeInsetsGeometry::symmetric(
                y + v.spacing_px,
                x + v.spacing_px,
            ))
            .decoration(
                BoxDecoration::new()
                    .color(c.surface)
                    .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
            )
            .foreground_decoration(
                BoxDecoration::new()
                    .border(Border::all(
                        c.border,
                        v.spacing_px,
                        BorderStyle::Solid,
                        -1.0,
                    ))
                    .border_radius(BorderRadiusGeometry::circular(v.radius_tiny)),
            )
            .child(
                Align::new()
                    .width_factor(1.0)
                    .height_factor(1.0)
                    .child(DefaultTextStyle::new(
                        style,
                        IconTheme::merge(
                            None,
                            IconThemeData::new().size(v.spacing_3).color(c.content),
                            self.child.clone(),
                        ),
                    )),
            )
            .into_widget()
    }
}
