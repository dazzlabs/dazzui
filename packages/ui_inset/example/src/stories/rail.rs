use crate::Demo;
use crate::storybook::stage::pane;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    pane(ui::Rail::new(vec![
        ui::IconButton::new(Text::new("+"))
            .on_pressed(|_| {})
            .into_widget(),
    ]))
}
