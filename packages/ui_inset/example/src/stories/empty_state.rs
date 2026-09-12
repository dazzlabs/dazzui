use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::EmptyState::new(
        "No projects yet",
        "Create your first project to get started.",
    )
    .into_widget()
}
