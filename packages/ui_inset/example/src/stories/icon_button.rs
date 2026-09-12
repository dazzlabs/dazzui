use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::IconButton::new(Text::new("+"))
        .on_pressed(|_| {})
        .into_widget()
}
