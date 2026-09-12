use crate::Demo;
use crate::storybook::stage::row;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    row(ui::Tint::ALL
        .iter()
        .map(|tint| ui::Badge::new(tint.name()).tint(*tint).into_widget())
        .collect())
}
