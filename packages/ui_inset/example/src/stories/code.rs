use crate::Demo;
use crate::storybook::stage::column;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    column(vec![
        ui::Code::new("cargo test").into_widget(),
        ui::Code::block("fn main() {\n    println!(\"Hello, Inset\");\n}").into_widget(),
    ])
}
