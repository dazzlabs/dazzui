use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    {
        let items = ["Studio", "Bright", "Frost"]
            .iter()
            .map(|s| ui::ChoiceItem::new(s.to_string(), *s))
            .collect();
        ui::Combobox::new(items).on_changed(|_, _| {}).into_widget()
    }
}
