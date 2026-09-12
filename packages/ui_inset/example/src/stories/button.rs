use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(
        ui::ControlVariant::ALL
            .iter()
            .map(|variant| {
                ui::Button::new(variant.name())
                    .variant(*variant)
                    .on_pressed(|_| {})
                    .into_widget()
            })
            .collect(),
    )
}
