use dazzui_inset as ui;
use inset::*;
pub fn column(children: Vec<WidgetRef>) -> WidgetRef {
    Builder::new(move |app, context| {
        let v = ui::Theme::of(app, context);
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .spacing(v.spacing_3)
            .children(children.clone())
            .into_widget()
    })
    .into_widget()
}
pub fn row(children: Vec<WidgetRef>) -> WidgetRef {
    Builder::new(move |app, context| {
        let v = ui::Theme::of(app, context);
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_2)
            .children(children.clone())
            .into_widget()
    })
    .into_widget()
}
pub fn pane<K>(child: impl IntoWidget<K>) -> WidgetRef {
    let child = child.into_widget();
    Builder::new(move |app, context| {
        let v = ui::Theme::of(app, context);
        SizedBox::new()
            .height(v.spacing_20 * 2.0)
            .child(child.clone())
            .into_widget()
    })
    .into_widget()
}
