use crate::Demo;
use crate::storybook::stage::pane;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    pane(ui::Splitter::new(vec![
        ui::SplitterPanel::new(ui::Card::new(Text::new("Explorer"))),
        ui::SplitterPanel::new(ui::Card::new(Text::new("Editor"))),
    ]))
}
