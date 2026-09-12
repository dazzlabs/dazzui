use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, demo: &Demo) -> WidgetRef {
    ui::FormField::new("Project name", ui::TextField::new().controller(demo.text))
        .hint("Use a short descriptive name.")
        .into_widget()
}
