use super::{Button, Callback, Callout};
use crate::{ControlVariant, Theme, Tint};
use inset_foundation::{App, Entity, Listener};
use inset_painting::EdgeInsetsGeometry;
use inset_rendering::{CrossAxisAlignment, MainAxisSize};
use inset_widgets::*;
use std::{rc::Rc, time::Duration};
#[derive(Clone)]
pub struct Toast {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub tint: Tint,
    pub action: Option<(String, Callback)>,
}
impl std::fmt::Debug for Toast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toast").field("title", &self.title).finish()
    }
}
#[derive(Debug)]
struct ToastQueue {
    next_id: u64,
    items: Vec<Toast>,
}
#[derive(Debug, Clone)]
pub struct ToastManager {
    queue: Entity<ToastQueue>,
}
impl ToastManager {
    pub fn new(app: &mut App) -> Self {
        Self {
            queue: app.new_entity(|_| ToastQueue {
                next_id: 0,
                items: vec![],
            }),
        }
    }
    pub fn show(
        &self,
        app: &mut App,
        title: impl Into<String>,
        description: impl Into<String>,
        tint: Tint,
        duration: Duration,
    ) -> u64 {
        let title = title.into();
        let description = description.into();
        let id = app.update(&self.queue, |queue, cx| {
            queue.next_id += 1;
            let id = queue.next_id;
            queue.items.push(Toast {
                id,
                title,
                description,
                tint,
                action: None,
            });
            cx.notify();
            id
        });
        if !duration.is_zero() {
            let manager = self.clone();
            app.schedule_timer(duration, Listener::new(move |app| manager.dismiss(app, id)));
        }
        id
    }
    pub fn dismiss(&self, app: &mut App, id: u64) {
        app.update(&self.queue, |queue, cx| {
            queue.items.retain(|toast| toast.id != id);
            cx.notify();
        });
    }
    pub fn len(&self, app: &mut App) -> usize {
        app.read(&self.queue).items.len()
    }
    pub fn is_empty(&self, app: &mut App) -> bool {
        self.len(app) == 0
    }
}
#[derive(Debug, Clone)]
pub struct ToastViewport {
    pub manager: ToastManager,
}
impl ToastViewport {
    pub fn new(manager: ToastManager) -> Self {
        Self { manager }
    }
}
impl StatelessWidget for ToastViewport {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let v = Theme::of(app, context);
        let items = app.read(&self.manager.queue).items.clone();
        let children: Vec<_> = items
            .iter()
            .map(|toast| {
                let manager = self.manager.clone();
                let id = toast.id;
                let mut callout = Callout::new(&toast.title, &toast.description);
                callout.tint = toast.tint;
                callout.actions.push(
                    Button::new("Dismiss")
                        .variant(ControlVariant::Plain)
                        .on_pressed(move |app| manager.dismiss(app, id))
                        .into_widget(),
                );
                if let Some((label, callback)) = &toast.action {
                    let callback = Rc::clone(callback);
                    callout.actions.push(
                        Button::new(label)
                            .on_pressed(move |app| callback(app))
                            .into_widget(),
                    );
                }
                callout.into_widget()
            })
            .collect();
        Padding::new(EdgeInsetsGeometry::all(v.spacing_4))
            .child(
                SizedBox::new().width(v.toast_max_width).child(
                    Column::new()
                        .main_axis_size(MainAxisSize::Min)
                        .cross_axis_alignment(CrossAxisAlignment::Stretch)
                        .spacing(v.spacing_2)
                        .children(children),
                ),
            )
            .into_widget()
    }
}
