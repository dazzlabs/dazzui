use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    Builder::new(|_, context| {
        ui::Button::new("Open drawer")
            .on_pressed(move |app| ui::Drawer::new(Text::new("Inspector")).show(app, context))
            .into_widget()
    })
    .into_widget()
}
