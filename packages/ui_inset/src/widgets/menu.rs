use super::{Button, Callback, Popover, PopoverAlign, PopoverTrigger};
use crate::Theme;
use inset_foundation::App;
use inset_painting::{
    Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Clone)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub checked: Option<bool>,
    pub enabled: bool,
    pub on_select: Option<Callback>,
}
impl std::fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuItem")
            .field("label", &self.label)
            .finish()
    }
}
impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            checked: None,
            enabled: true,
            on_select: None,
        }
    }
    pub fn on_select(mut self, callback: impl Fn(&mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(callback));
        self
    }
}
#[derive(Clone)]
pub struct MenuPanel {
    pub items: Vec<MenuItem>,
    pub on_dismiss: Option<Callback>,
}
impl std::fmt::Debug for MenuPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MenuPanel")
            .field("items", &self.items)
            .finish()
    }
}
impl MenuPanel {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            on_dismiss: None,
        }
    }
}
impl StatelessWidget for MenuPanel {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let first = self.items.iter().position(|item| item.enabled);
        let children: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let label = format!(
                    "{}{}{}",
                    match item.checked {
                        Some(true) => "✓  ",
                        Some(false) => "    ",
                        None => "",
                    },
                    item.label,
                    item.shortcut
                        .as_ref()
                        .map(|s| format!("    {s}"))
                        .unwrap_or_default()
                );
                let mut button = super::Pressable::new(move |app, context, phase| {
                    let v = crate::Theme::of(app, context);
                    let colors = v.control_colors(
                        crate::ControlVariant::Normal,
                        crate::Tint::Neutral,
                        phase,
                    );
                    Container::new()
                        .padding(EdgeInsetsGeometry::symmetric(
                            v.menu_item_padding,
                            v.spacing_2,
                        ))
                        .decoration(
                            BoxDecoration::new()
                                .color(if phase == crate::ControlState::Normal {
                                    crate::generated::colors::TRANSPARENT
                                } else {
                                    colors.surface
                                })
                                .border_radius(BorderRadiusGeometry::circular(v.radius_small)),
                        )
                        .child(
                            Text::new(&label)
                                .style(v.label_quiet.text_style().color(colors.content)),
                        )
                        .into_widget()
                });
                button.autofocus = first == Some(i);
                button.radius = v.radius_small;
                if item.enabled {
                    let callback = item.on_select.clone();
                    let dismiss = self.on_dismiss.clone();
                    button = button.on_pressed(move |app| {
                        if let Some(dismiss) = &dismiss {
                            dismiss(app);
                        }
                        if let Some(callback) = &callback {
                            callback(app);
                        }
                    });
                }
                button.into_widget()
            })
            .collect();
        let panel = Container::new()
            .width(v.menu_min_width)
            .padding(EdgeInsetsGeometry::all(v.spacing_1))
            .decoration(
                BoxDecoration::new()
                    .color(v.color_surface_overlay)
                    .box_shadow(crate::shadows(v.shadow_lg))
                    .border(Border::all(
                        v.color_border_strong,
                        v.stroke_hairline,
                        BorderStyle::Solid,
                        -1.0,
                    ))
                    .border_radius(BorderRadiusGeometry::circular(v.radius_large)),
            )
            .child(
                Column::new()
                    .main_axis_size(MainAxisSize::Min)
                    .cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .children(children),
            )
            .into_widget();
        Focus::new(panel)
            .can_request_focus(false)
            .on_key_event(Rc::new(|app, _, event| {
                use inset_services::{KeyEvent, LogicalKeyboardKey};
                if matches!(event, KeyEvent::Up(_)) {
                    return KeyEventResult::Ignored;
                }
                let key = event.logical_key();
                if let Some(node) = primary_focus(app) {
                    if key == LogicalKeyboardKey::ARROW_DOWN {
                        node.next_focus(app);
                        return KeyEventResult::Handled;
                    }
                    if key == LogicalKeyboardKey::ARROW_UP {
                        node.previous_focus(app);
                        return KeyEventResult::Handled;
                    }
                }
                KeyEventResult::Ignored
            }))
            .into_widget()
    }
}
#[derive(Clone)]
pub struct Menu {
    pub trigger: PopoverTrigger,
    pub items: Vec<MenuItem>,
    pub align: PopoverAlign,
}
impl std::fmt::Debug for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Menu").field("items", &self.items).finish()
    }
}
impl Menu {
    pub fn new(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        let label = label.into();
        Self {
            trigger: Rc::new(move |_, _, _, toggle| {
                Button::new(&label)
                    .on_pressed(move |app| toggle(app))
                    .into_widget()
            }),
            items,
            align: PopoverAlign::End,
        }
    }
}
impl StatelessWidget for Menu {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let trigger = self.trigger.clone();
        let items = self.items.clone();
        let mut popup = Popover::new(
            move |app, context, open, toggle| trigger(app, context, open, toggle),
            SizedBox::shrink(),
        );
        popup.align = self.align;
        popup.content_builder = Some(Rc::new(move |_, _, close| {
            MenuPanel {
                items: items.clone(),
                on_dismiss: Some(close),
            }
            .into_widget()
        }));
        popup.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Menubar {
    pub menus: Vec<Menu>,
}
impl Menubar {
    pub fn new(menus: Vec<Menu>) -> Self {
        Self { menus }
    }
}
impl StatelessWidget for Menubar {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Row::new()
            .main_axis_size(MainAxisSize::Min)
            .spacing(v.spacing_1)
            .children(self.menus.iter().cloned().map(IntoWidget::into_widget))
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub child: WidgetRef,
    pub items: Vec<MenuItem>,
}
impl ContextMenu {
    pub fn new<K>(child: impl IntoWidget<K>, items: Vec<MenuItem>) -> Self {
        Self {
            child: child.into_widget(),
            items,
        }
    }
}
impl StatelessWidget for ContextMenu {
    fn build(&self, _: &mut App, _: BuildContext) -> WidgetRef {
        let child = self.child.clone();
        let mut menu = Menu::new("", self.items.clone());
        menu.align = PopoverAlign::Start;
        menu.trigger = Rc::new(move |_, _, _, toggle| {
            GestureDetector::new()
                .on_secondary_tap(inset_foundation::Listener::new(move |app| toggle(app)))
                .child(child.clone())
                .into_widget()
        });
        menu.into_widget()
    }
}
