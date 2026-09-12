use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let manager = demo.toast.clone();
        ui::Button::new("Show notification")
            .on_pressed(move |app| {
                manager.show(
                    app,
                    "Saved",
                    "Project settings updated.",
                    ui::Tint::Success,
                    std::time::Duration::from_secs(4),
                );
            })
            .into_widget()
    }
}
