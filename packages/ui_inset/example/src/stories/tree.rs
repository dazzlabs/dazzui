use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    {
        let mut root = ui::TreeItem::new("src", "src");
        root.children = vec![
            ui::TreeItem::new("main", "main.rs"),
            ui::TreeItem::new("lib", "lib.rs"),
        ];
        let mut tree = ui::Tree::new(vec![root]);
        tree.on_selected = Some(std::rc::Rc::new(|_, _| {}));
        tree.into_widget()
    }
}
