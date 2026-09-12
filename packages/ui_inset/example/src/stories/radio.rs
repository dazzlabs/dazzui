use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let selected = app.read(&demo.state).selected;
        column(
            (0..3)
                .map(|i| {
                    let state = demo.state.clone();
                    ui::Radio::new(i == selected)
                        .label(format!("Option {}", i + 1))
                        .on_selected(move |app| {
                            app.update(&state, |s, cx| {
                                s.selected = i;
                                cx.notify();
                            })
                        })
                        .into_widget()
                })
                .collect(),
        )
    }
}
