use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let state = demo.state.clone();
        let value = app.read(&state).otp.clone();
        ui::OtpField::new(6, value)
            .on_changed(move |app, value| {
                app.update(&state, |s, cx| {
                    s.otp = value;
                    cx.notify();
                })
            })
            .into_widget()
    }
}
