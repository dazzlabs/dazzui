//! A panel of anything, anchored to what opened it.
//!
//! It wears the same floating sheet a `Menu` does (popup.rs), padded for
//! prose rather than rows — a form, a filter, a short explanation. It closes
//! on an outside press and on Escape, which is why a `Dialog` is the home for
//! anything the reader has to finish.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, prelude::*, px,
};

use crate::components::popup::{
    Align, Anchor, OnOpenChange, Placement, PopupState, Press, Side, TriggerFn, dismissable,
    measured, no_press, open_state, positioned, prose_sheet, toggle_press,
};
use crate::foundation::{Length, StyledExt, provide, scoped};
use crate::theme::ActiveTheme;

/// What a [`PopoverClose`] inside a popover presses.
#[derive(Clone)]
struct CloseScope(Press);

/// A popover.
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Option<TriggerFn>,
    title: Option<AnyElement>,
    description: Option<AnyElement>,
    side: Side,
    align: Align,
    width: Option<Length>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    children: Vec<AnyElement>,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Popover {
            id: id.into(),
            trigger: None,
            title: None,
            description: None,
            side: Side::Bottom,
            align: Align::Center,
            width: None,
            open: None,
            default_open: false,
            on_open_change: None,
            children: Vec::new(),
        }
    }

    /// What opens it, built with the press that does — hand it to a
    /// `Button`'s `on_click` — and whether it is open.
    pub fn trigger(
        mut self,
        trigger: impl FnOnce(Press, bool, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.trigger = Some(Box::new(trigger));
        self
    }

    /// Names the panel as well as heading it.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    /// A line under the title.
    pub fn description(mut self, description: impl IntoElement) -> Self {
        self.description = Some(description.into_any_element());
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Overrides the panel's own width, which is otherwise its content's.
    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    /// Holds the open state for the caller, who hears of changes through
    /// [`on_open_change`](Self::on_open_change).
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
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, self.default_open, window, cx);
        let open = open_state(&state, self.open, cx);
        let anchor: Anchor = state.read(cx).anchor.clone();
        let press = toggle_press(&state, None, self.on_open_change.clone());
        let trigger = self
            .trigger
            .map(|trigger| measured(&anchor, trigger(press, open, window, cx)));

        let popup = open.then(|| {
            let close: Press = {
                let state = state.clone();
                let notify = self.on_open_change.clone();
                Rc::new(move |window, cx| {
                    state.update(cx, |state, cx| state.hide(window, cx));
                    if let Some(notify) = &notify {
                        notify(false, window, cx);
                    }
                })
            };
            let has_head = self.title.is_some() || self.description.is_some();
            let title = self.title.map(|title| {
                div()
                    .text_color(t.color_content.hsla())
                    .type_style(t.label_large)
                    .child(title)
            });
            let description = self.description.map(|description| {
                div()
                    .mt(px(t.spacing_1))
                    .text_color(t.color_content_secondary.hsla())
                    .child(description)
            });
            let body = (!self.children.is_empty()).then(|| {
                div()
                    .flex()
                    .flex_col()
                    .when(has_head, |this| this.mt(px(t.spacing_3)))
                    .children(self.children)
            });
            let sheet = prose_sheet(t)
                .id("sheet")
                .when_some(self.width, |this, width| this.w(width.definite()))
                .children(title)
                .children(description)
                .children(body);
            let sheet = dismissable(sheet, &state, &anchor, self.on_open_change, cx);
            positioned(
                &anchor,
                Placement::side(self.side, self.align, t),
                provide(CloseScope(close), sheet),
            )
        });

        div().id(self.id).flex().children(trigger).children(popup)
    }
}

/// Wraps a control so pressing it closes the popover it sits in. The
/// control is built with that press.
#[derive(IntoElement)]
pub struct PopoverClose {
    render: Box<dyn FnOnce(Press, &mut Window, &mut App) -> AnyElement>,
}

impl PopoverClose {
    pub fn new(render: impl FnOnce(Press, &mut Window, &mut App) -> AnyElement + 'static) -> Self {
        PopoverClose {
            render: Box::new(render),
        }
    }
}

impl RenderOnce for PopoverClose {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let close = scoped::<CloseScope>(cx)
            .map(|scope| scope.0)
            .unwrap_or_else(no_press);
        (self.render)(close, window, cx)
    }
}
