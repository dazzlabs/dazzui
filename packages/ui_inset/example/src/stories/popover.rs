use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Popover::new(
        |_, _, _, toggle| {
            ui::Button::new("Open popover")
                .on_pressed(move |app| toggle(app))
                .into_widget()
        },
        ui::PopoverPanel::new(Text::new("Anchored content")),
    )
    .into_widget()
}
