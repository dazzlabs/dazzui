use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Tooltip::new(
        "Create a new project",
        ui::Button::new("Hover here").on_pressed(|_| {}),
    )
    .into_widget()
}
