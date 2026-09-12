use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(vec![
        Text::new("Above").into_widget(),
        ui::Divider::new().into_widget(),
        Text::new("Below").into_widget(),
    ])
}
