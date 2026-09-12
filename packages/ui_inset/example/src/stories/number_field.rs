use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let state = demo.state.clone();
        let value = app.read(&state).value;
        ui::NumberField::new(value)
            .range(0.0, 100.0)
            .on_changed(move |app, value| {
                app.update(&state, |s, cx| {
                    s.value = value;
                    cx.notify();
                })
            })
            .into_widget()
    }
}
