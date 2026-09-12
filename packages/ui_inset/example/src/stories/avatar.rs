use crate::Demo;
use crate::storybook::stage::row;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    row(vec![
        ui::Avatar::new("Ada Lovelace").into_widget(),
        ui::Avatar::new("Grace Hopper").into_widget(),
    ])
}
