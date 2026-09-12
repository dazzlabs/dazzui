use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Menu::new(
        "Actions",
        vec![
            ui::MenuItem::new("New project").on_select(|_| {}),
            ui::MenuItem::new("Settings").on_select(|_| {}),
        ],
    )
    .into_widget()
}
