use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, demo: &Demo) -> WidgetRef {
    ui::SearchField::new(demo.text).into_widget()
}
