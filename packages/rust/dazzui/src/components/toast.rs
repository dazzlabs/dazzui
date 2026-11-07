//! A transient receipt for something that happened out of view.
//!
//! A [`Toaster`] is the queue: create one, hand it to a [`ToastProvider`],
//! and call [`Toaster::add`] from anywhere that holds it. The
//! [`ToastViewport`] under the provider draws what is in it, along one edge
//! of the window. A plain notice goes after five seconds, one with an action
//! after eight, and a failure stays until it is dismissed.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnchoredPositionMode, AnyElement, App, AppContext, Corner, Entity, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, anchored, div, point, prelude::FluentBuilder, px,
};

use crate::components::button::Button;
use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{Size, StyledExt, Tint, Variant, provide, scoped};
use crate::glyphs;
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

/// A toast's trailing affordance — undo, retry. Pressing it closes the toast.
#[derive(Clone)]
pub struct ToastAction {
    pub label: SharedString,
    pub on_click: Rc<dyn Fn(&mut Window, &mut App)>,
}

#[derive(Clone)]
pub struct ToastOptions {
    /// The message: one line, no wrapping.
    pub title: SharedString,
    pub tint: Tint,
    /// A leading glyph, by asset path, drawn in the tint's 600.
    pub icon: Option<SharedString>,
    pub action: Option<ToastAction>,
    /// Whether to draw the close button. On by default.
    pub dismissable: bool,
    /// How long before it goes on its own; `Some(ZERO)` keeps it until
    /// dismissed, `None` takes the default for its kind.
    pub timeout: Option<Duration>,
}

impl ToastOptions {
    pub fn new(title: impl Into<SharedString>) -> Self {
        ToastOptions {
            title: title.into(),
            tint: Tint::Neutral,
            icon: None,
            action: None,
            dismissable: true,
            timeout: None,
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some(ToastAction {
            label: label.into(),
            on_click: Rc::new(on_click),
        });
        self
    }

    pub fn dismissable(mut self, dismissable: bool) -> Self {
        self.dismissable = dismissable;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

struct ToastEntry {
    id: u64,
    options: ToastOptions,
}

#[derive(Default)]
pub struct ToastQueue {
    toasts: Vec<ToastEntry>,
    next: u64,
}

/// The queue, as a handle anything can hold and add to.
#[derive(Clone)]
pub struct Toaster(Entity<ToastQueue>);

impl Toaster {
    pub fn new(cx: &mut App) -> Toaster {
        Toaster(cx.new(|_| ToastQueue::default()))
    }

    /// Raises a toast and returns its id.
    pub fn add(&self, options: ToastOptions, cx: &mut App) -> u64 {
        let t = cx.theme();
        let timeout = options.timeout.unwrap_or(if options.tint == Tint::Danger {
            Duration::ZERO
        } else if options.action.is_some() {
            Duration::from_secs_f32(t.toast_action_duration / 1000.)
        } else {
            Duration::from_secs_f32(t.toast_notice_duration / 1000.)
        });
        let id = self.0.update(cx, |queue, cx| {
            let id = queue.next;
            queue.next += 1;
            queue.toasts.push(ToastEntry { id, options });
            cx.notify();
            id
        });
        cx.refresh_windows();
        if !timeout.is_zero() {
            let toaster = self.clone();
            cx.spawn(async move |cx| {
                cx.background_executor().timer(timeout).await;
                let _ = cx.update(|cx| toaster.close(id, cx));
            })
            .detach();
        }
        id
    }

    pub fn close(&self, id: u64, cx: &mut App) {
        self.0.update(cx, |queue, cx| {
            queue.toasts.retain(|toast| toast.id != id);
            cx.notify();
        });
        cx.refresh_windows();
    }

    /// How many toasts are showing.
    pub fn len(&self, cx: &App) -> usize {
        self.0.read(cx).toasts.len()
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.len(cx) == 0
    }
}

/// Holds the queue for the tree beneath it. One per window.
#[derive(IntoElement)]
pub struct ToastProvider {
    toaster: Toaster,
    children: Vec<AnyElement>,
}

impl ToastProvider {
    pub fn new(toaster: Toaster) -> Self {
        ToastProvider {
            toaster,
            children: Vec::new(),
        }
    }
}

impl ParentElement for ToastProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(crate::foundation::precise_text));
    }
}

impl RenderOnce for ToastProvider {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        // React's provider draws nothing; its children sit in the flow as
        // they would without it.
        provide(
            self.toaster,
            div()
                .flex()
                .flex_row()
                .items_center()
                .children(self.children),
        )
    }
}

/// Which edge the stack hangs from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPlacement {
    #[default]
    Bottom,
    Top,
}

impl ToastPlacement {
    pub fn parse(name: &str) -> Option<ToastPlacement> {
        match name {
            "bottom" => Some(ToastPlacement::Bottom),
            "top" => Some(ToastPlacement::Top),
            _ => None,
        }
    }
}

/// Where a window's toasts land: centred along one edge, above everything,
/// and transparent to the pointer between them.
#[derive(IntoElement, Default)]
pub struct ToastViewport {
    placement: ToastPlacement,
}

impl ToastViewport {
    pub fn new() -> Self {
        ToastViewport::default()
    }

    pub fn placement(mut self, placement: ToastPlacement) -> Self {
        self.placement = placement;
        self
    }
}

impl RenderOnce for ToastViewport {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(toaster) = scoped::<Toaster>(cx) else {
            return div().into_any_element();
        };
        let queue = toaster.0.read(cx);
        let t = cx.theme();
        let viewport = window.viewport_size();
        // Newest nearest the edge the stack hangs from.
        let mut toasts: Vec<_> = queue
            .toasts
            .iter()
            .map(|entry| (entry.id, entry.options.clone()))
            .collect();
        if self.placement == ToastPlacement::Top {
            toasts.reverse();
        }
        let toasts: Vec<AnyElement> = toasts
            .into_iter()
            .map(|(id, options)| toast(&toaster, id, options, cx))
            .collect();

        // The window's width and its toasts' height, padded, pinned to one
        // edge — an empty viewport is still that strip, as React's is.
        let stack = div()
            .w(viewport.width)
            .flex()
            .flex_col()
            .items_center()
            .gap(px(t.spacing_2))
            .p(px(t.spacing_4))
            .type_style(t.body_medium)
            .text_color(t.color_content.hsla())
            .children(toasts);
        let (corner, origin) = match self.placement {
            ToastPlacement::Bottom => (Corner::BottomLeft, point(px(0.), viewport.height)),
            ToastPlacement::Top => (Corner::TopLeft, point(px(0.), px(0.))),
        };
        crate::foundation::portal::portal(
            anchored()
                .position_mode(AnchoredPositionMode::Window)
                .anchor(corner)
                .position(origin)
                .child(stack),
            2,
        )
        .into_any_element()
    }
}

fn toast(toaster: &Toaster, id: u64, options: ToastOptions, cx: &App) -> AnyElement {
    let t = cx.theme();
    let tint = options.tint;
    let dismiss_hover = t
        .control_color_normal_surface_hovered
        .resolve(Tint::Neutral.ramp(t));
    div()
        .id(("dz-toast", id))
        .debug_selector(|| "dz-toast".into())
        .flex()
        .items_center()
        .gap(px(t.spacing_2_5))
        .min_h(px(t.spacing_9))
        .max_w(px(t.toast_max_width))
        .py(px(t.spacing_1_5))
        .pl(px(t.spacing_3))
        .pr(px(t.spacing_1_5))
        .bg(t.color_surface_overlay.hsla())
        .border(px(t.stroke_hairline))
        .border_color(t.color_border_strong.hsla())
        .rounded(px(t.radius_big))
        .shadow(shadows(t.shadow_md))
        .occlude()
        .when_some(options.icon, |this, icon| {
            this.child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .text_color(tint.ramp(t).shade(600).hsla())
                    .child(Icon::new(icon).size(t.icon_medium)),
            )
        })
        .child(
            div()
                .min_w_0()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .py(px(t.spacing_0_5))
                .text_color(t.color_content.hsla())
                .type_style(t.label_medium)
                .line_height(px(t.label_medium.size * t.toast_message_line_height))
                .child(crate::foundation::text_child(options.title)),
        )
        .when_some(options.action, |this, action| {
            let toaster = toaster.clone();
            this.child(
                div().flex_none().ml_auto().child(
                    Button::new(("dz-toast-action", id))
                        .size(Size::Small)
                        .tint(tint)
                        .variant(Variant::Plain)
                        .child(action.label.clone())
                        .on_click(move |_, window, cx| {
                            (action.on_click)(window, cx);
                            toaster.close(id, cx);
                        }),
                ),
            )
        })
        .when(options.dismissable, |this| {
            let toaster = toaster.clone();
            this.child(
                div()
                    .id(("dz-toast-dismiss", id))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .size(px(t.control_tiny_size))
                    .rounded(px(t.radius_small))
                    .text_color(t.color_content_subtle.hsla())
                    .cursor_pointer()
                    .hover(move |style| style.bg(dismiss_hover).text_color(t.color_content.hsla()))
                    .on_click(move |_, _, cx| toaster.close(id, cx))
                    .child(icon_slot(
                        t.icon_small,
                        Icon::new(glyphs::DISMISS_12_REGULAR),
                    )),
            )
        })
        .into_any_element()
}
