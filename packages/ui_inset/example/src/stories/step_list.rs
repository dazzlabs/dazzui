use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, _demo: &Demo) -> WidgetRef {
    ui::StepList::new(vec![
        ui::Step::new("Download", ui::StepStatus::Done),
        ui::Step::new("Install", ui::StepStatus::Active),
        ui::Step::new("Restart", ui::StepStatus::Idle),
    ])
    .into_widget()
}
