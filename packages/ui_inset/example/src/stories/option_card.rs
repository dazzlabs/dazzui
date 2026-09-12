use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::OptionCard::new("Local workspace", "Work with files on this computer.")
        .on_pressed(|_| {})
        .into_widget()
}
