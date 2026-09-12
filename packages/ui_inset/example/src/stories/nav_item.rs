use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(vec![
        ui::NavItem::new("Overview")
            .selected(true)
            .on_pressed(|_| {})
            .into_widget(),
        ui::NavItem::new("Activity")
            .on_pressed(|_| {})
            .into_widget(),
    ])
}
