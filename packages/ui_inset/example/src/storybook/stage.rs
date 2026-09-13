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
            .spacing(v.spacing_3)
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

/// Adaptive equal-width tracks; the final row retains the full grid's widths.
pub fn grid(min: f64, children: Vec<WidgetRef>) -> WidgetRef {
    LayoutBuilder::new(move |app, context, constraints| {
        let v = ui::Theme::of(app, context);
        let width = if constraints.max_width.is_finite() {
            constraints.max_width.max(min)
        } else {
            min
        };
        let columns = ((width + v.spacing_4) / (min + v.spacing_4))
            .floor()
            .max(1.0) as usize;
        let track = (width - v.spacing_4 * (columns - 1) as f64) / columns as f64;
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .spacing(v.spacing_4)
            .children(
                children
                    .chunks(columns)
                    .map(|chunk| {
                        IntrinsicHeight::new()
                            .child(
                                Row::new()
                                    .main_axis_size(MainAxisSize::Min)
                                    .cross_axis_alignment(CrossAxisAlignment::Stretch)
                                    .spacing(v.spacing_4)
                                    .children(
                                        chunk
                                            .iter()
                                            .map(|child| {
                                                SizedBox::new()
                                                    .width(track)
                                                    .child(child.clone())
                                                    .into_widget()
                                            })
                                            .collect::<Vec<_>>(),
                                    ),
                            )
                            .into_widget()
                    })
                    .collect::<Vec<_>>(),
            )
            .into_widget()
    })
    .into_widget()
}

/// The shared Specimen label column and its bounded horizontal stage.
pub fn specimen(label: &str, child: WidgetRef) -> WidgetRef {
    let label = label.to_string();
    Builder::new(move |app, context| {
        let v = ui::Theme::of(app, context);
        Row::new()
            .main_axis_size(MainAxisSize::Max)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .spacing(v.spacing_4)
            .children([
                SizedBox::new()
                    .width(v.spacing_20 + v.spacing_4)
                    .child(
                        Text::new(&label)
                            .style(v.caption_medium.text_style().font_family("monospace")),
                    )
                    .into_widget(),
                Expanded::new(child.clone()).into_widget(),
            ])
            .into_widget()
    })
    .into_widget()
}
