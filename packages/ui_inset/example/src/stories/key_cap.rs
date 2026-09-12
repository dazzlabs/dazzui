use crate::Demo;
use crate::storybook::stage::row;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    row(vec![
        ui::KeyCap::new("⌘").into_widget(),
        ui::KeyCap::new("K").into_widget(),
    ])
}
