use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Accordion::new(vec![
        ui::AccordionItem::new("General", Text::new("General options")),
        ui::AccordionItem::new("Advanced", Text::new("Advanced options")),
    ])
    .into_widget()
}
