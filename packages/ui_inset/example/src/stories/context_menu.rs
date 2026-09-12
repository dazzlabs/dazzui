use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::ContextMenu::new(
        ui::Card::new(Text::new("Right click here")),
        vec![ui::MenuItem::new("Inspect").on_select(|_| {})],
    )
    .into_widget()
}
