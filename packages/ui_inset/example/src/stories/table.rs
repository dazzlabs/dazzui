use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Table::new(vec![
        ui::TableHead::new(vec![
            ui::TableCell::new(Text::new("Name")).into_widget(),
            ui::TableCell::new(Text::new("Status")).into_widget(),
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
