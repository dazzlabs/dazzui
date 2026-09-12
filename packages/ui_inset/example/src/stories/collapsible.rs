use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Collapsible::new(
        "Advanced settings",
        Text::new("Additional options are shown here."),
    )
    .into_widget()
}
