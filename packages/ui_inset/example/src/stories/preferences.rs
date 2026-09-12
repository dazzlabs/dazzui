use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Preferences::new(vec![
        ui::PreferenceGroup::new(
            "General",
            vec![
                ui::PreferenceRow::new(
                    "Notifications",
                    ui::Switch::new(true).on_changed(|_, _| {}),
                )
                .into_widget(),
            ],
        )
        .into_widget(),
    ])
    .into_widget()
}
