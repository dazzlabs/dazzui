use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::PreviewCard::new(
        ui::Button::new("Preview").on_pressed(|_| {}),
        Text::new("A preview of the linked resource."),
    )
    .into_widget()
}
