use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Menubar::new(vec![
        ui::Menu::new("File", vec![ui::MenuItem::new("New").on_select(|_| {})]),
        ui::Menu::new("Edit", vec![ui::MenuItem::new("Copy").on_select(|_| {})]),
    ])
    .into_widget()
}
