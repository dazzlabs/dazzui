use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::Tag::new("Rust").on_remove(|_| {}).into_widget()
}
