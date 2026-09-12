use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(
        [
            ui::CardVariant::Raised,
            ui::CardVariant::Sunken,
            ui::CardVariant::Outlined,
            ui::CardVariant::Tinted,
        ]
        .iter()
        .map(|variant| {
            ui::Card::new(Text::new(format!("{variant:?}")))
                .variant(*variant)
                .into_widget()
        })
        .collect(),
    )
}
