use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::ActionBar::new(vec![
        ui::Button::new("Cancel").on_pressed(|_| {}).into_widget(),
        ui::Button::new("Save")
            .variant(ui::ControlVariant::Filled)
            .on_pressed(|_| {})
            .into_widget(),
    ])
    .into_widget()
}
