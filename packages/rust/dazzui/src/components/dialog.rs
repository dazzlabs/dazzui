//! The sheet a modal form or a confirmation is built in: a scrim over the
//! whole window, the sheet centred on it, and the header, body and footer as
//! separate parts so a sheet can leave one out.
//!
//! Escape and a press on the scrim close it (unless it is not `dismissible`);
//! focus moves into the sheet when it opens and back to where it was when it
//! closes. Tab is not yet trapped inside the sheet.

use std::rc::Rc;

use gpui::{
    AnchoredPositionMode, AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, Stateful, StatefulInteractiveElement,
    Styled, Window, anchored, deferred, div, point, prelude::FluentBuilder, px,
};

use crate::components::collapsible::{OnOpenChange, part_id};
use crate::foundation::{Length, StyledExt, Tint, provide, scoped};
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

/// `danger` is what a destructive confirmation wears.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DialogTone {
    #[default]
    Default,
    Danger,
}

impl DialogTone {
    pub fn parse(name: &str) -> Option<DialogTone> {
        match name {
            "default" => Some(DialogTone::Default),
            "danger" => Some(DialogTone::Danger),
            _ => None,
        }
    }
}

/// What the parts inside an open sheet use to shut it — a `DialogClose`, a
/// `DrawerClose`, or a host's own control.
#[derive(Clone)]
pub struct DialogHandle {
    set_open: OnOpenChange,
}

impl DialogHandle {
    pub fn close(&self, window: &mut Window, cx: &mut App) {
        (self.set_open)(false, window, cx);
    }

    /// The handle of the sheet being drawn, for a part inside it.
    pub fn current(cx: &App) -> Option<DialogHandle> {
        scoped::<DialogHandle>(cx)
    }
}

pub(crate) struct ModalState {
    open: bool,
    was_open: bool,
    restore: Option<FocusHandle>,
    focus: FocusHandle,
}

/// The open state of a modal — controlled or kept — and the setter every
/// press that opens or shuts it goes through.
pub(crate) struct Modal {
    pub open: bool,
    pub set_open: OnOpenChange,
    pub focus: FocusHandle,
}

pub(crate) fn modal(
    id: &ElementId,
    controlled: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    window: &mut Window,
    cx: &mut App,
) -> Modal {
    let state: Entity<ModalState> =
        window.use_keyed_state(part_id(id, "modal"), cx, move |_, cx| ModalState {
            open: default_open,
            was_open: false,
            restore: None,
            focus: cx.focus_handle(),
        });
    let open = controlled.unwrap_or(state.read(cx).open);

    // Focus goes into the sheet as it opens, and back where it came from as
    // it shuts.
    let (was_open, focus) = {
        let state = state.read(cx);
        (state.was_open, state.focus.clone())
    };
    if open != was_open {
        let restore = if open { window.focused(cx) } else { None };
        let back = state.update(cx, |state, _| {
            state.was_open = open;
            if open {
                state.restore = restore;
                None
            } else {
                state.restore.take()
            }
        });
        let into = focus.clone();
        window.defer(cx, move |window, _| {
            if open {
                window.focus(&into);
            } else if let Some(back) = back {
                window.focus(&back);
            }
        });
    }

    let kept = controlled.is_none();
    let set_open: OnOpenChange = Rc::new(move |next, window, cx| {
        if kept {
            state.update(cx, |state, cx| {
                state.open = next;
                cx.notify();
            });
        }
        if let Some(on_open_change) = &on_open_change {
            on_open_change(next, window, cx);
        }
        window.refresh();
    });
    Modal {
        open,
        set_open,
        focus,
    }
}

/// A trigger, drawn in place and wired to open the sheet.
pub(crate) fn trigger(id: ElementId, trigger: AnyElement, set_open: OnOpenChange) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .flex_none()
        .on_click(move |_, window, cx| set_open(true, window, cx))
        .child(trigger)
}

/// The layer a modal is drawn on: over everything, covering the window, the
/// scrim behind whatever `sheet` positions on it. The layer sets the text
/// style afresh, since a deferred element inherits none.
pub(crate) fn modal_layer(
    id: &ElementId,
    modal: &Modal,
    dismissible: bool,
    sheet: impl IntoElement,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let t = cx.theme();
    let viewport = window.viewport_size();
    let scrim = t.dialog_scrim.resolve(Tint::Neutral.ramp(t));
    let handle = DialogHandle {
        set_open: modal.set_open.clone(),
    };
    let dismiss = modal.set_open.clone();
    let escape = modal.set_open.clone();

    let layer = div()
        .id(part_id(id, "layer"))
        .relative()
        .w(viewport.width)
        .h(viewport.height)
        .type_style(t.body_medium)
        .text_color(t.color_content.hsla())
        .track_focus(&modal.focus)
        .on_key_down(move |event: &KeyDownEvent, window, cx| {
            if dismissible && event.keystroke.key == "escape" {
                escape(false, window, cx);
                cx.stop_propagation();
            }
        })
        .child(
            div()
                .id(part_id(id, "scrim"))
                .absolute()
                .inset_0()
                .bg(scrim)
                .occlude()
                .when(dismissible, |this| {
                    this.on_click(move |_, window, cx| dismiss(false, window, cx))
                }),
        )
        .child(provide(handle, sheet));

    deferred(
        anchored()
            .position_mode(AnchoredPositionMode::Window)
            .position(point(px(0.), px(0.)))
            .child(layer),
    )
    .with_priority(1)
    .into_any_element()
}

/// A modal sheet: a scrim, the sheet centred on it, focus moved in.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    width: Option<Length>,
    tone: DialogTone,
    dismissible: bool,
    trigger: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Dialog {
            id: id.into(),
            open: None,
            default_open: false,
            on_open_change: None,
            width: None,
            tone: DialogTone::Default,
            dismissible: true,
            trigger: None,
            children: Vec::new(),
        }
    }

    /// Controlled: whether the sheet is up. Pair with `on_open_change`.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Overrides `dialog.width`.
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn tone(mut self, tone: DialogTone) -> Self {
        self.tone = tone;
        self
    }

    /// `false` makes it an alert: neither Escape nor the scrim closes it.
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Drawn in place and wired to open the sheet — a `Button`, usually.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let modal = modal(
            &self.id,
            self.open,
            self.default_open,
            self.on_open_change.clone(),
            window,
            cx,
        );
        let trigger_el = self
            .trigger
            .map(|el| trigger(part_id(&self.id, "trigger"), el, modal.set_open.clone()));

        let sheet = modal.open.then(|| {
            let t = cx.theme();
            let border = match self.tone {
                DialogTone::Default => t.color_border_strong.hsla(),
                DialogTone::Danger => t.color_danger.shade(600).alpha(0.34),
            };
            let width = self
                .width
                .map(Length::definite)
                .unwrap_or(px(t.dialog_width).into());
            let sheet = div()
                .id(part_id(&self.id, "sheet"))
                .debug_selector(|| "dz-dialog".into())
                .flex()
                .flex_col()
                .overflow_hidden()
                .w(width)
                .max_w_full()
                .max_h_full()
                .bg(t.color_surface_overlay.hsla())
                .border(px(t.stroke_hairline))
                .border_color(border)
                .rounded(px(t.frame_window_radius))
                .shadow(shadows(t.shadow_lg))
                .occlude()
                .children(self.children);
            let centred = div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .p(px(t.spacing_6))
                .child(sheet);
            modal_layer(&self.id, &modal, self.dismissible, centred, window, cx)
        });

        div().children(trigger_el).children(sheet)
    }
}

/// The sheet's head row: a title, and a subtitle under it.
#[derive(IntoElement)]
pub struct DialogHeader {
    title: AnyElement,
    subtitle: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl DialogHeader {
    pub fn new(title: impl IntoElement) -> Self {
        DialogHeader {
            title: title.into_any_element(),
            subtitle: None,
            children: Vec::new(),
        }
    }

    pub fn subtitle(mut self, subtitle: impl IntoElement) -> Self {
        self.subtitle = Some(subtitle.into_any_element());
        self
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

/// The header drawing a dialog and a drawer share.
pub(crate) fn sheet_header(
    title: Option<AnyElement>,
    subtitle: Option<AnyElement>,
    children: Vec<AnyElement>,
    cx: &App,
) -> Div {
    let t = cx.theme();
    div()
        .flex()
        .flex_col()
        .justify_center()
        .flex_none()
        .gap(px(t.spacing_1))
        .min_h(px(t.spacing_11))
        .py(px(t.spacing_3))
        .px(px(t.spacing_5))
        .border_b(px(t.stroke_hairline))
        .border_color(t.color_border.hsla())
        .when_some(title, |this, title| {
            this.child(
                div()
                    .font_family(t.label_strong.face.family())
                    .font_weight(t.label_strong.font_weight())
                    .text_size(px(t.title_small.size))
                    .line_height(px(t.title_small.size))
                    .child(title),
            )
        })
        .when_some(subtitle, |this, subtitle| {
            this.child(
                div()
                    .text_color(t.color_content_subtle.hsla())
                    .type_style(t.caption_small)
                    .child(subtitle),
            )
        })
        .children(children)
}

/// The body drawing a dialog and a drawer share: the only part that may
/// shrink, and the one that scrolls.
pub(crate) fn sheet_body(id: ElementId, children: Vec<AnyElement>, cx: &App) -> Stateful<Div> {
    let t = cx.theme();
    div()
        .id(id)
        .flex()
        .flex_col()
        .gap(px(t.spacing_4))
        .min_h_0()
        .overflow_y_scroll()
        .py(px(t.spacing_4 + t.spacing_0_5))
        .px(px(t.spacing_5))
        .children(children)
}

pub(crate) fn sheet_footer(children: Vec<AnyElement>, cx: &App) -> Div {
    let t = cx.theme();
    div()
        .flex()
        .items_center()
        .flex_none()
        .gap(px(t.spacing_2))
        .py(px(t.spacing_3))
        .px(px(t.spacing_5))
        .bg(t.color_surface_chrome.hsla())
        .border_t(px(t.stroke_hairline))
        .border_color(t.color_border.hsla())
        .children(children)
}

impl RenderOnce for DialogHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        sheet_header(Some(self.title), self.subtitle, self.children, cx)
    }
}

#[derive(IntoElement, Default)]
pub struct DialogBody {
    children: Vec<AnyElement>,
}

impl DialogBody {
    pub fn new() -> Self {
        DialogBody::default()
    }
}

impl ParentElement for DialogBody {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogBody {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        sheet_body("dz-dialog-body".into(), self.children, cx)
    }
}

#[derive(IntoElement, Default)]
pub struct DialogFooter {
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        DialogFooter::default()
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        sheet_footer(self.children, cx)
    }
}

/// Wraps a control so pressing it closes the sheet it sits in. It reads the
/// sheet from where it is drawn, so it works in a dialog and a drawer alike.
#[derive(IntoElement)]
pub struct DialogClose {
    id: ElementId,
    child: AnyElement,
}

impl DialogClose {
    pub fn new(id: impl Into<ElementId>, child: impl IntoElement) -> Self {
        DialogClose {
            id: id.into(),
            child: child.into_any_element(),
        }
    }
}

impl RenderOnce for DialogClose {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let handle = DialogHandle::current(cx);
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .when_some(handle, |this, handle| {
                this.on_click(move |_, window, cx| handle.close(window, cx))
            })
            .child(self.child)
    }
}
