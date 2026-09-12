use crate::Theme;
use inset_foundation::App;
use inset_painting::{Border, BorderRadiusGeometry, BorderStyle, BoxDecoration};
use inset_rendering::DecorationPosition;
use inset_widgets::{BuildContext, DecoratedBox, IntoWidget, StatelessWidget, WidgetRef};
/// A foreground focus halo. It does not change the child's layout.
#[derive(Debug, Clone)]
pub struct FocusRing {
    pub child: WidgetRef,
    pub visible: bool,
    pub radius: Option<f64>,
}
impl FocusRing {
    pub fn new<K>(visible: bool, child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            visible,
            radius: None,
        }
    }
}
impl StatelessWidget for FocusRing {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        if !self.visible {
            return self.child.clone();
        }
        let v = Theme::of(app, context);
        DecoratedBox::new(
            BoxDecoration::new()
                .border(Border::all(
                    v.focus_ring(),
                    v.focus_width,
                    BorderStyle::Solid,
                    1.0,
                ))
                .border_radius(BorderRadiusGeometry::circular(
                    self.radius.unwrap_or(v.radius_small),
                )),
        )
        .position(DecorationPosition::Foreground)
        .child(self.child.clone())
        .into_widget()
    }
}
