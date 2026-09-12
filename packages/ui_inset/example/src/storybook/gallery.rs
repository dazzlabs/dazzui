use dazzui_inset as ui;
use inset::*;
#[derive(Debug, Clone)]
pub struct Overview {
    title: String,
    description: String,
    child: WidgetRef,
}
impl Overview {
    pub fn new<K>(
        title: impl Into<String>,
        description: impl Into<String>,
        child: impl IntoWidget<K>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            child: child.into_widget(),
        }
    }
}
impl StatelessWidget for Overview {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = ui::Theme::of(app, context);
        Padding::new(EdgeInsetsGeometry::all(v.spacing_8))
            .child(
                Column::new()
                    .main_axis_size(MainAxisSize::Min)
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .spacing(v.spacing_5)
                    .children([
                        Text::new(&self.title)
                            .style(v.headline_medium.text_style())
                            .into_widget(),
                        Text::new(&self.description)
                            .style(v.body_small.text_style().color(v.color_content_muted))
                            .into_widget(),
                        self.child.clone(),
                    ]),
            )
            .into_widget()
    }
}
