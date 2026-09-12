use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Link::new("View documentation")
        .on_pressed(|_| {})
        .into_widget()
}
