use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    Builder::new(|_, context| {
        ui::Button::new("Open dialog")
            .on_pressed(move |app| {
                let mut dialog = ui::Dialog::new(
                    "Project settings",
                    Text::new("Choose your project settings."),
                );
                dialog.actions = vec![
                    Builder::new(|_, context| {
                        ui::Button::new("Close")
                            .on_pressed(move |app| ui::Dialog::close(app, context))
                            .into_widget()
                    })
                    .into_widget(),
                ];
                dialog.show(app, context);
            })
            .into_widget()
    })
    .into_widget()
}
