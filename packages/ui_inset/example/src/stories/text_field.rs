use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, demo: &Demo) -> WidgetRef {
    ui::TextField::new()
        .controller(demo.text)
        .placeholder("Project name")
        .into_widget()
}
