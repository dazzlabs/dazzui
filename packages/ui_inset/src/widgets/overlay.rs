use super::Callback;
use crate::Theme;
use inset_embedder::Offset;
use inset_foundation::{App, Handle};
use inset_painting::{
    Alignment, Border, BorderRadiusGeometry, BorderStyle, BoxDecoration, EdgeInsetsGeometry,
};
use inset_rendering::{CrossAxisAlignment, HitTestBehavior, LayerLink, MainAxisSize};
use inset_services::LogicalKeyboardKey;
use inset_widgets::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PopoverPanel {
    pub child: WidgetRef,
    pub title: Option<String>,
    pub description: Option<String>,
    pub width: Option<f64>,
}
impl PopoverPanel {
    pub fn new<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            title: None,
            description: None,
            width: None,
        }
    }
}
impl StatelessWidget for PopoverPanel {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut children = vec![];
        if let Some(title) = &self.title {
            children.push(
                Text::new(title)
                    .style(v.label_large.text_style().color(v.color_content))
                    .into_widget(),
            );
        }
        if let Some(description) = &self.description {
            children.push(
                Text::new(description)
                    .style(v.body_small.text_style().color(v.color_content_secondary))
                    .into_widget(),
            );
        }
        children.push(self.child.clone());
        let mut panel = Container::new()
            .padding(EdgeInsetsGeometry::all(v.spacing_4 + v.spacing_0_5))
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
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .spacing(v.spacing_3)
                    .children(children),
            );
        if let Some(width) = self.width {
            panel = panel.width(width);
        }
        panel.into_widget()
    }
}
#[derive(Debug, Clone, Copy, Default)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}
pub type PopoverTrigger = Rc<dyn Fn(&mut App, BuildContext, bool, Callback) -> WidgetRef>;
pub type PopoverContent = Rc<dyn Fn(&mut App, BuildContext, Callback) -> WidgetRef>;
#[derive(Clone)]
pub struct Popover {
    pub hover: bool,
    pub dismiss_on_outside: bool,
    pub focus: bool,
    pub content_builder: Option<PopoverContent>,
    pub trigger: PopoverTrigger,
    pub child: WidgetRef,
    pub side: PopoverSide,
    pub align: PopoverAlign,
}
impl std::fmt::Debug for Popover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Popover")
    }
}
impl Popover {
    pub fn new<K>(
        trigger: impl Fn(&mut App, BuildContext, bool, Callback) -> WidgetRef + 'static,
        child: impl IntoWidget<K>,
    ) -> Self {
        Self {
            hover: false,
            dismiss_on_outside: true,
            focus: true,
            content_builder: None,
            trigger: Rc::new(trigger),
            child: child.into_widget(),
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
        }
    }
}
impl StatefulWidget for Popover {
    type State = PopoverState;
    fn create_state(&self) -> Self::State {
        PopoverState {
            state: StateData::new(),
            controller: None,
            link: None,
            open: false,
            hover_timer: None,
        }
    }
}
pub struct PopoverState {
    state: StateData<Popover>,
    controller: Option<Handle<OverlayPortalController>>,
    link: Option<Handle<LayerLink>>,
    open: bool,
    hover_timer: Option<inset_foundation::Timer>,
}
impl PopoverState {
    fn hover_enter(self: Handle<Self>, app: &mut App) {
        if let Some(timer) = app.get_mut(self).hover_timer.take() {
            app.cancel_timer(timer);
        }
        if !app.get(self).open {
            self.toggle(app);
        }
    }
    fn hover_exit(self: Handle<Self>, app: &mut App, delay: std::time::Duration) {
        if let Some(timer) = app.get_mut(self).hover_timer.take() {
            app.cancel_timer(timer);
        }
        let timer = app.schedule_timer(
            delay,
            inset_foundation::Listener::new(move |app| self.close(app)),
        );
        app.get_mut(self).hover_timer = Some(timer);
    }
    fn hover_region(self: Handle<Self>, child: WidgetRef, delay: std::time::Duration) -> WidgetRef {
        MouseRegion::new()
            .on_enter(Rc::new(move |app, _| self.hover_enter(app)))
            .on_exit(Rc::new(move |app, _| self.hover_exit(app, delay)))
            .child(child)
            .into_widget()
    }

    fn close(self: Handle<Self>, app: &mut App) {
        if !self.mounted(app) {
            return;
        }
        if let Some(timer) = app.get_mut(self).hover_timer.take() {
            app.cancel_timer(timer);
        }
        let c = app.get(self).controller.unwrap();
        c.hide(app);
        self.set_state(app, |s| s.open = false);
    }
    fn toggle(self: Handle<Self>, app: &mut App) {
        if app.get(self).open {
            self.close(app);
        } else {
            let c = app.get(self).controller.unwrap();
            c.show(app);
            self.set_state(app, |s| s.open = true);
        }
    }
}
impl State for PopoverState {
    type Widget = Popover;
    inset_widgets::state_accessors!();
    fn init_state(self: Handle<Self>, app: &mut App) {
        let c = OverlayPortalController::new(app, None);
        let link = LayerLink::new(app);
        let s = app.get_mut(self);
        s.controller = Some(c);
        s.link = Some(link);
    }
    fn dispose(self: Handle<Self>, app: &mut App) {
        if let Some(timer) = app.get_mut(self).hover_timer.take() {
            app.cancel_timer(timer);
        }
    }
    fn build(self: Handle<Self>, app: &mut App, context: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let link = app.get(self).link.unwrap();
        let controller = app.get(self).controller.unwrap();
        let trigger = (w.trigger)(
            app,
            context,
            app.get(self).open,
            Rc::new(move |app| self.toggle(app)),
        );
        let delay = std::time::Duration::from_secs_f64(Theme::of(app, context).motion_duration);
        let trigger = if w.hover {
            self.hover_region(
                Focus::new(trigger)
                    .can_request_focus(false)
                    .on_focus_change(move |app, focused| {
                        if focused {
                            self.hover_enter(app);
                        } else {
                            self.hover_exit(app, delay);
                        }
                    })
                    .into_widget(),
                delay,
            )
        } else {
            trigger
        };
        CompositedTransformTarget::new(link)
            .child(
                OverlayPortal::new(controller, move |app, context| {
                    let v = Theme::of(app, context);
                    let cross = match w.align {
                        PopoverAlign::Start => -1.0,
                        PopoverAlign::Center => 0.0,
                        PopoverAlign::End => 1.0,
                    };
                    let (target, follower, offset) = match w.side {
                        PopoverSide::Bottom => (
                            Alignment::new(cross, 1.0),
                            Alignment::new(cross, -1.0),
                            Offset::new(0.0, v.spacing_1),
                        ),
                        PopoverSide::Top => (
                            Alignment::new(cross, -1.0),
                            Alignment::new(cross, 1.0),
                            Offset::new(0.0, -v.spacing_1),
                        ),
                        PopoverSide::Left => (
                            Alignment::new(-1.0, cross),
                            Alignment::new(1.0, cross),
                            Offset::new(-v.spacing_1, 0.0),
                        ),
                        PopoverSide::Right => (
                            Alignment::new(1.0, cross),
                            Alignment::new(-1.0, cross),
                            Offset::new(v.spacing_1, 0.0),
                        ),
                    };
                    let content = w
                        .content_builder
                        .as_ref()
                        .map(|builder| builder(app, context, Rc::new(move |app| self.close(app))))
                        .unwrap_or_else(|| w.child.clone());
                    let content = if w.hover {
                        self.hover_region(content, delay)
                    } else {
                        content
                    };
                    let mut layers = vec![];
                    if w.dismiss_on_outside {
                        layers.push(
                            Positioned::fill(
                                GestureDetector::new()
                                    .behavior(HitTestBehavior::Opaque)
                                    .on_tap(inset_foundation::Listener::new(move |app| {
                                        self.close(app)
                                    })),
                            )
                            .into_widget(),
                        );
                    }
                    layers.push(
                        CompositedTransformFollower::new(link)
                            .target_anchor(target)
                            .follower_anchor(follower)
                            .offset(offset)
                            .child(
                                Align::new().alignment(follower.into()).child(
                                    FocusScope::new(content).autofocus(w.focus).on_key_event(
                                        Rc::new(move |app, _, event| {
                                            if event.logical_key() == LogicalKeyboardKey::ESCAPE {
                                                self.close(app);
                                                KeyEventResult::Handled
                                            } else {
                                                KeyEventResult::Ignored
                                            }
                                        }),
                                    ),
                                ),
                            )
                            .into_widget(),
                    );
                    Stack::new().children(layers).into_widget()
                })
                .child(trigger),
            )
            .into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Dialog {
    pub title: String,
    pub description: String,
    pub child: WidgetRef,
    pub actions: Vec<WidgetRef>,
    pub dismissible: bool,
}
impl Dialog {
    pub fn new<K>(title: impl Into<String>, child: impl IntoWidget<K>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            child: child.into_widget(),
            actions: vec![],
            dismissible: true,
        }
    }
    pub fn show(self, app: &mut App, context: BuildContext) {
        let vars = Theme::of(app, context);
        let scrim = vars.dialog_scrim_color;
        let dismissible = self.dismissible;
        let route = RawDialogRoute::new(
            app,
            Rc::new(move |_, _, _, _| {
                Theme::new(&vars, Center::new().child(self.clone())).into_widget()
            }),
        )
        .barrier_dismissible(app, dismissible)
        .barrier_color(app, Some(scrim))
        .barrier_label(app, "Dismiss dialog".into());
        Navigator::push(app, context, route.as_route());
    }
    pub fn close(app: &mut App, context: BuildContext) {
        Navigator::pop(app, context, None);
    }
}
impl StatelessWidget for Dialog {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let mut body = vec![self.child.clone()];
        if !self.actions.is_empty() {
            body.push(
                Row::new()
                    .spacing(v.spacing_2)
                    .children(self.actions.clone())
                    .into_widget(),
            );
        }
        let mut panel = PopoverPanel::new(
            Column::new()
                .main_axis_size(MainAxisSize::Min)
                .spacing(v.spacing_4)
                .children(body),
        );
        panel.title = Some(self.title.clone());
        panel.description = (!self.description.is_empty()).then(|| self.description.clone());
        panel.width = Some(v.dialog_width);
        panel.into_widget()
    }
}
#[derive(Debug, Clone)]
pub struct Drawer {
    pub child: WidgetRef,
    pub left: bool,
}
impl Drawer {
    pub fn new<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            left: false,
        }
    }
    pub fn show(self, app: &mut App, context: BuildContext) {
        let vars = Theme::of(app, context);
        let scrim = vars.dialog_scrim_color;
        let route = RawDialogRoute::new(
            app,
            Rc::new(move |_, _, _, _| Theme::new(&vars, self.clone()).into_widget()),
        )
        .barrier_color(app, Some(scrim))
        .barrier_label(app, "Dismiss drawer".into());
        Navigator::push(app, context, route.as_route());
    }
}
impl StatelessWidget for Drawer {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        Align::new()
            .alignment(if self.left {
                inset_painting::AlignmentGeometry::CENTER_LEFT
            } else {
                inset_painting::AlignmentGeometry::CENTER_RIGHT
            })
            .child(
                Container::new()
                    .width(v.drawer_size)
                    .height(f64::INFINITY)
                    .padding(EdgeInsetsGeometry::all(v.spacing_4))
                    .color(v.color_surface_overlay)
                    .child(self.child.clone()),
            )
            .into_widget()
    }
}
