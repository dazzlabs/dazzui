use super::{Callback, Pressable, ValueChanged};
use crate::{ControlState, ControlVariant, Theme, Tint};
use inset_foundation::App;
use inset_painting::{BorderRadiusGeometry, BoxDecoration, EdgeInsetsGeometry};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct NavItem {
    pub label: String,
    pub selected: bool,
    pub icon: Option<WidgetRef>,
    pub on_pressed: Option<Callback>,
}
impl std::fmt::Debug for NavItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NavItem")
            .field("label", &self.label)
            .finish()
    }
}
impl NavItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            selected: false,
            icon: None,
            on_pressed: None,
        }
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    pub fn on_pressed(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_pressed = Some(Rc::new(callback));
        self
    }
}
impl StatelessWidget for NavItem {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let radius = Theme::of(app, context).radius_small;
        let w = self.clone();
        let mut p = Pressable::new(move |app, context, phase| {
            let v = Theme::of(app, context);
            let filled =
                v.control_colors(ControlVariant::Filled, Tint::Primary, ControlState::Normal);
            let (surface, content) = if phase == ControlState::Disabled {
                (crate::generated::colors::TRANSPARENT, v.color_content_faint)
            } else if w.selected {
                (filled.surface, filled.content)
            } else if phase == ControlState::Hovered {
                (
                    v.control_colors(ControlVariant::Plain, Tint::Primary, ControlState::Hovered)
                        .surface,
                    v.color_content,
                )
            } else {
                (crate::generated::colors::TRANSPARENT, v.color_content_nav)
            };
            let mut children = vec![];
            if let Some(icon) = &w.icon {
                children.push(icon.clone());
            }
            children.push(
                Text::new(&w.label)
                    .style(v.label_quiet.text_style().color(content))
                    .into_widget(),
            );
            Container::new()
                .padding(EdgeInsetsGeometry::symmetric(v.spacing_2, v.spacing_2_5))
                .decoration(
                    BoxDecoration::new()
                        .color(surface)
                        .border_radius(BorderRadiusGeometry::circular(radius)),
                )
                .child(Row::new().spacing(v.spacing_1_5).children(children))
                .into_widget()
        });
        p.radius = radius;
        p.on_pressed = self.on_pressed.clone();
        p.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Sidebar {
    pub children: Vec<WidgetRef>,
    pub collapsed: bool,
}
impl Sidebar {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self {
            children,
            collapsed: false,
        }
    }
}
impl StatelessWidget for Sidebar {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Container::new()
            .width(if self.collapsed {
                v.frame_sidebar_icon_width
            } else {
                v.frame_sidebar_width
            })
            .color(v.color_surface_column)
            .padding(EdgeInsetsGeometry::all(v.spacing_2))
            .child(
                SingleChildScrollView::new().child(
                    Column::new()
                        .cross_axis_alignment(CrossAxisAlignment::Stretch)
                        .spacing(v.frame_nav_gap)
                        .children(self.children.clone()),
                ),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Rail {
    pub children: Vec<WidgetRef>,
}
impl Rail {
    pub fn new(children: Vec<WidgetRef>) -> Self {
        Self { children }
    }
}
impl StatelessWidget for Rail {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Container::new()
            .width(v.frame_rail_width)
            .color(v.color_surface_column)
            .child(
                Column::new()
                    .spacing(v.frame_nav_gap)
                    .children(self.children.clone()),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Aside {
    pub title: String,
    pub child: WidgetRef,
}
impl Aside {
    pub fn new<K>(title: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            title: title.into(),
            child: child.into_widget(),
        }
    }
}
impl StatelessWidget for Aside {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Container::new()
            .width(v.frame_aside_width)
            .color(v.color_surface_column)
            .padding(EdgeInsetsGeometry::all(v.spacing_3))
            .child(
                Column::new()
                    .cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .spacing(v.spacing_3)
                    .children([
                        Text::new(&self.title)
                            .style(v.label_strong.text_style())
                            .into_widget(),
                        self.child.clone(),
                    ]),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct TreeItem {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeItem>,
}
impl TreeItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: vec![],
        }
    }
}
#[derive(Clone)]
pub struct Tree {
    pub items: Vec<TreeItem>,
    pub selected: Option<String>,
    pub on_selected: Option<ValueChanged<String>>,
}
impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree").field("items", &self.items).finish()
    }
}
impl Tree {
    pub fn new(items: Vec<TreeItem>) -> Self {
        Self {
            items,
            selected: None,
            on_selected: None,
        }
    }
}
impl StatelessWidget for Tree {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let children: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                if item.children.is_empty() {
                    let mut nav = NavItem::new(&item.label)
                        .selected(self.selected.as_ref() == Some(&item.id));
                    if let Some(callback) = self.on_selected.clone() {
                        let id = item.id.clone();
                        nav = nav.on_pressed(move |app| callback(app, id.clone()));
                    }
                    nav.into_widget()
                } else {
                    let subtree = Tree {
                        items: item.children.clone(),
                        selected: self.selected.clone(),
                        on_selected: self.on_selected.clone(),
                    };
                    super::Collapsible::new(
                        &item.label,
                        Padding::new(EdgeInsetsGeometry::only(0.0, 0.0, 0.0, v.spacing_4))
                            .child(subtree),
                    )
                    .into_widget()
                }
            })
            .collect();
        Column::new()
            .main_axis_size(MainAxisSize::Min)
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .children(children)
            .into_widget()
    }
}
