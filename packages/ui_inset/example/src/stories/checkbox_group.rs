use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let state = demo.state.clone();
        let value = app.read(&state).checks.clone();
        let mut group = ui::CheckboxGroup::new(
            ["Source", "Tests", "Examples"]
                .iter()
                .map(|s| ui::ChoiceItem::new(s.to_string(), *s))
                .collect(),
            value,
        )
        .on_changed(move |app, value| {
            app.update(&state, |s, cx| {
                s.checks = value;
                cx.notify();
            })
        });
        group.parent_label = Some("Select all".into());
        group.into_widget()
    }
}
