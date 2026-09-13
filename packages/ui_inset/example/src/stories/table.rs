use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Table::new(vec![
        ui::TableHead::new(vec![
            ui::TableCell::new(Text::new("Name"))
                .head(true)
                .into_widget(),
            ui::TableCell::new(Text::new("Status"))
                .head(true)
                .into_widget(),
        ])
        .into_widget(),
        ui::TableRow::new(vec![
            ui::TableCell::new(Text::new("Playground")).into_widget(),
            ui::TableCell::new(ui::Badge::new("Ready")).into_widget(),
        ])
        .into_widget(),
    ])
    .into_widget()
}
