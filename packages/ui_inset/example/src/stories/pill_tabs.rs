use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let state = demo.state.clone();
        let value = app.read(&state).selected;
        ui::PillTabs::new(
            (0..3)
                .map(|i| ui::ChoiceItem::new(i, format!("Tab {}", i + 1)))
                .collect(),
            Some(value),
        )
        .on_changed(move |app, value| {
            app.update(&state, |s, cx| {
                s.selected = value;
                cx.notify();
            })
        })
        .into_widget()
    }
}
