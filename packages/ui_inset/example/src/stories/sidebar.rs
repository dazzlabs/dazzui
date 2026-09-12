use crate::Demo;
use crate::storybook::stage::pane;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    pane(ui::Sidebar::new(vec![
        ui::NavItem::new("Projects")
            .selected(true)
            .on_pressed(|_| {})
            .into_widget(),
        ui::NavItem::new("Settings")
            .on_pressed(|_| {})
            .into_widget(),
    ]))
}
