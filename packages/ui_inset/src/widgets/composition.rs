use super::{Callback, Pressable, Spinner};
use crate::{ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{
    AlignmentGeometry, Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::{BoxConstraints, CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use inset_widgets::{
    Align, BuildContext, Column, ConstrainedBox, Container, DefaultTextStyle, Expanded, IntoWidget,
    Row, StatelessWidget, Text, WidgetRef,
};
use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct ActionBar {
    pub children: Vec<WidgetRef>,
    pub vertical: bool,
    pub align: MainAxisAlignment,
    pub stretch: bool,
}
impl ActionBar {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self {
            children,
            vertical: false,
            align: MainAxisAlignment::Start,
            stretch: false,
        }
    }
}
impl ActionBar {
    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }
    pub fn align(mut self, align: MainAxisAlignment) -> Self {
        self.align = align;
        self
    }
}
impl StatelessWidget for ActionBar {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        if self.vertical {
            Column::new()
                .main_axis_size(MainAxisSize::Min)
                .spacing(v.control_small_gap)
                .children(self.children.clone())
                .into_widget()
        } else {
            Row::new()
                .main_axis_size(MainAxisSize::Max)
                .main_axis_alignment(self.align)
                .cross_axis_alignment(if self.stretch {
                    CrossAxisAlignment::Stretch
                } else {
                    CrossAxisAlignment::Center
                })
                .spacing(v.control_small_gap)
                .children(self.children.clone())
                .into_widget()
        }
    }
}
#[derive(Debug, Clone)]
pub struct Preferences {
    pub children: Vec<WidgetRef>,
}
impl Preferences {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self { children }
    }
}
impl StatelessWidget for Preferences {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        ConstrainedBox::new(BoxConstraints {
            max_width: v.preferences_width,
            ..BoxConstraints::new()
        })
        .child(
            Column::new()
                .main_axis_size(MainAxisSize::Min)
                .cross_axis_alignment(CrossAxisAlignment::Stretch)
                .spacing(v.spacing_8)
                .children(self.children.clone()),
        )
        .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct PreferenceGroup {
    pub title: String,
    pub description: String,
    pub children: Vec<WidgetRef>,
}
impl PreferenceGroup {
    pub fn new(title: impl Into<String>, children: Vec<WidgetRef>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            children,
        }
    }
}
impl StatelessWidget for PreferenceGroup {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut children = vec![
            Text::new(&self.title)
                .style(v.label_strong.text_style())
                .into_widget(),
        ];
        if !self.description.is_empty() {
            children.push(
                Text::new(&self.description)
                    .style(v.caption_small.text_style().color(v.color_content_subtle))
                    .into_widget(),
            );
        }
        children.extend(self.children.clone());
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .spacing(v.spacing_4 + v.spacing_0_5)
            .children(children)
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct PreferenceRow {
    pub title: String,
    pub description: String,
    pub control: WidgetRef,
}
impl PreferenceRow {
    pub fn new<K>(title: impl Into<String>, control: impl IntoWidget<K>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            control: control.into_widget(),
        }
    }
}
impl StatelessWidget for PreferenceRow {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut labels = vec![
            Text::new(&self.title)
                .style(v.label_medium.text_style())
                .into_widget(),
        ];
        if !self.description.is_empty() {
            labels.push(
                Text::new(&self.description)
                    .style(v.caption_small.text_style().color(v.color_content_subtle))
                    .into_widget(),
            );
        }
        ConstrainedBox::new(BoxConstraints {
            min_height: v.control_medium_size,
            ..BoxConstraints::new()
        })
        .child(
            Row::new().spacing(v.spacing_2_5).children([
                Expanded::new(
                    Column::new()
                        .main_axis_size(MainAxisSize::Min)
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .spacing(v.spacing_1)
                        .children(labels),
                )
                .into_widget(),
                self.control.clone(),
            ]),
        )
        .into_widget()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Idle,
    Active,
    Done,
}
#[derive(Debug, Clone)]
pub struct Step {
    pub label: String,
    pub meta: Option<String>,
    pub status: StepStatus,
    pub tint: Tint,
}
impl Step {
    pub fn new(label: impl Into<String>, status: StepStatus) -> Self {
        Self {
            label: label.into(),
            meta: None,
            status,
            tint: Tint::Primary,
        }
    }
}
impl StatelessWidget for Step {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let color = match self.status {
            StepStatus::Idle => v.color_content_subtle,
            StepStatus::Active => v.color_content,
            _ => v.color_content_muted,
        };
        let marker = match self.status {
            StepStatus::Active => Spinner {
                size: crate::ControlSize::Medium,
                tint: self.tint,
                on_accent: false,
            }
            .into_widget(),
            StepStatus::Done => Text::new("✓")
                .style(v.label_small.text_style().color(v.color_success.shade(600)))
                .into_widget(),
            _ => Text::new("○")
                .style(v.label_small.text_style().color(v.color_content_faint))
                .into_widget(),
        };
        let mut children = vec![
            marker,
            Expanded::new(
                Text::new(&self.label).style(if self.status == StepStatus::Active {
                    v.label_medium.text_style().color(color)
                } else {
                    v.label_quiet.text_style().color(color)
                }),
            )
            .into_widget(),
        ];
        if let Some(meta) = &self.meta {
            children.push(
                Text::new(meta)
                    .style(v.caption_small.text_style().color(
                        if self.status == StepStatus::Active {
                            v.control_colors(ControlVariant::Plain, self.tint, ControlState::Normal)
                                .content
                        } else {
                            v.color_content_faint
                        },
                    ))
                    .into_widget(),
            );
        }
        Row::new()
            .spacing(v.spacing_2_5)
            .children(children)
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct StepList {
    pub steps: Vec<Step>,
}
impl StepList {
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }
}
impl StatelessWidget for StepList {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_2)
            .children(self.steps.iter().cloned().map(IntoWidget::into_widget))
            .into_widget()
    }
}
#[derive(Clone)]
pub struct OptionCard {
    pub title: String,
    pub description: String,
    pub selected: bool,
    pub tint: Tint,
    pub on_pressed: Option<Callback>,
}
impl std::fmt::Debug for OptionCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptionCard")
            .field("title", &self.title)
            .finish()
    }
}
impl OptionCard {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            selected: false,
            tint: Tint::Primary,
            on_pressed: None,
        }
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_pressed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for OptionCard {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let radius = Theme::of(app, context).radius_large;
        let w = self.clone();
        let mut p = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            let dead = phase == ControlState::Disabled;
            let surface = if dead {
                v.color_surface_sunken
            } else if w.selected {
                v.wash(w.tint, v.wash_surface)
            } else if phase == ControlState::Hovered {
                v.color_surface_subtle
            } else {
                v.color_surface_muted
            };
            let border = if w.selected && !dead {
                v.control_colors(ControlVariant::Outlined, w.tint, phase)
                    .border
            } else {
                v.color_border
            };
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(v.spacing_3_5, v.spacing_3))
                .decoration(
                    BoxDecoration::new()
                        .color(surface)
                        .border(Border::all(
                            border,
                            if w.selected {
                                v.stroke_control
                            } else {
                                v.stroke_hairline
                            },
                            BorderStyle::Solid,
                            -1.0,
                        ))
                        .border_radius(BorderRadiusGeometry::circular(radius)),
                )
                .child(
                    Column::new()
                        .main_axis_size(MainAxisSize::Min)
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .spacing(v.spacing_1_5)
                        .children([
                            Text::new(&w.title)
                                .style(v.label_strong.text_style().color(if dead {
                                    v.color_content_faint
                                } else {
                                    v.color_content
                                }))
                                .into_widget(),
                            Text::new(&w.description)
                                .style(v.caption_small.text_style().color(if dead {
                                    v.color_content_faint
                                } else {
                                    v.color_content_subtle
                                }))
                                .into_widget(),
                        ]),
                )
                .into_widget()
        });
        p.radius = radius;
        p.on_pressed = self.on_pressed.clone();
        p.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Table {
    pub children: Vec<WidgetRef>,
}
impl Table {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self { children }
    }
}
impl StatelessWidget for Table {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .children(self.children.clone())
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct TableHead {
    pub cells: Vec<WidgetRef>,
}
impl TableHead {
    pub fn new(cells: Vec<WidgetRef>) -> Self {
        Self { cells }
    }
}
impl StatelessWidget for TableHead {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Container::new()
            .color(v.color_surface_chrome)
            .padding(EdgeInsetsGeometry::symmetric(0.0, v.spacing_5))
            .child(Row::new().children(self.cells.clone()))
            .into_widget()
    }
}
#[derive(Clone)]
pub struct TableRow {
    pub cells: Vec<WidgetRef>,
    pub selected: bool,
    pub on_pressed: Option<Callback>,
}
impl std::fmt::Debug for TableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TableRow")
    }
}
impl TableRow {
    pub fn new(cells: Vec<WidgetRef>) -> Self {
        Self {
            cells,
            selected: false,
            on_pressed: None,
        }
    }
}
impl StatelessWidget for TableRow {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.clone();
        let mut p = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(v.spacing_3, v.spacing_5))
                .color(if w.selected {
                    v.wash(Tint::Primary, v.wash_surface)
                } else if phase == ControlState::Hovered {
                    v.color_surface_subtle
                } else {
                    crate::generated::colors::TRANSPARENT
                })
                .child(Row::new().children(w.cells.clone()))
                .into_widget()
        });
        p.on_pressed = self.on_pressed.clone();
        p.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct TableCell {
    pub child: WidgetRef,
    pub flex: i32,
    pub head: bool,
    pub end: bool,
}
impl TableCell {
    pub fn new<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            flex: 1,
            head: false,
            end: false,
        }
    }
}
impl TableCell {
    pub fn head(mut self, head: bool) -> Self {
        self.head = head;
        self
    }
    pub fn end(mut self, end: bool) -> Self {
        self.end = end;
        self
    }
}
impl StatelessWidget for TableCell {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let style = if self.head {
            v.label_small.text_style().color(v.color_content_faint)
        } else {
            v.body_medium.text_style()
        };
        Expanded::new(
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(
                    if self.head { v.spacing_2 } else { 0.0 },
                    0.0,
                ))
                .child(
                    Align::new()
                        .height_factor(1.0)
                        .alignment(if self.end {
                            AlignmentGeometry::CENTER_RIGHT
                        } else {
                            AlignmentGeometry::CENTER_LEFT
                        })
                        .child(DefaultTextStyle::new(style, self.child.clone())),
                ),
        )
        .flex(self.flex)
        .into_widget()
    }
}
