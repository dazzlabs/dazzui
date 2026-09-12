use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Callout::new("Update available", "Restart the app to finish installing.").into_widget()
}
