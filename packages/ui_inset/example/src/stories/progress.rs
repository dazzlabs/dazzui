use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(vec![
        ui::Progress::new(Some(0.65)).into_widget(),
        ui::Progress::new(None).into_widget(),
    ])
}
