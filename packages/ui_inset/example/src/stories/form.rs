use crate::Demo;
use dazzui_inset as ui;
use inset::*;
pub fn overview(_app: &mut App, demo: &Demo) -> WidgetRef {
    {
        let mut form = ui::Form::new(vec![
            ui::FormField::new("Project name", ui::TextField::new().controller(demo.text))
                .name("name")
                .into_widget(),
        ]);
        form.errors
            .insert("name".into(), "This name is already taken.".into());
        form.into_widget()
    }
}
