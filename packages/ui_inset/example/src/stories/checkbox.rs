use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let state = demo.state.clone();
        let value = app.read(&state).checked;
        ui::Checkbox::new(value)
            .label("Include archived projects")
            .on_changed(move |app, value| {
                app.update(&state, |s, cx| {
                    s.checked = value == Some(true);
                    cx.notify();
                })
            })
            .into_widget()
    }
}
