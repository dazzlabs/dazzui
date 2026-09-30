//! What is behind a link, shown without following it.
//!
//! A `Tooltip` that holds a card rather than a label — the sheet a
//! `Popover` wears, opened by the pointer resting on its trigger and kept
//! open while the pointer travels onto it, so it may hold a link of its own.

use std::time::Duration;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::popup::{
    Align, Placement, PopupState, Side, measured, positioned, prose_sheet,
};
use crate::foundation::Length;
use crate::theme::ActiveTheme;

/// A preview card.
#[derive(IntoElement)]
pub struct PreviewCard {
    id: ElementId,
    trigger: Option<AnyElement>,
    side: Side,
    align: Align,
    delay: Duration,
    close_delay: Duration,
    width: Option<Length>,
    default_open: bool,
    children: Vec<AnyElement>,
}

impl PreviewCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        PreviewCard {
            id: id.into(),
            trigger: None,
            side: Side::Bottom,
            align: Align::Center,
            delay: Duration::from_millis(600),
            close_delay: Duration::from_millis(300),
            width: None,
            default_open: false,
            children: Vec::new(),
        }
    }

    /// What it previews — a link, usually.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
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

    /// How long the pointer has to rest before it opens.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// How long it stays after the pointer leaves.
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = delay;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = Some(width);
        self
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
}

impl ParentElement for PreviewCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PreviewCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, self.default_open, window, cx);
        let anchor = state.read(cx).anchor.clone();
        let open = state.read(cx).open;
        let (delay, close_delay) = (self.delay, self.close_delay);
        let hover = move |part: usize| {
            let state = state.clone();
            move |hovered: &bool, window: &mut Window, cx: &mut App| {
                state.update(cx, |state, cx| {
                    state.hover(part, *hovered, delay, close_delay, window, cx)
                })
            }
        };

        let popup = open.then(|| {
            // `.dz-preview-card`: a ceiling on the width, whatever comes back.
            let card = prose_sheet(t)
                .id("card")
                .max_w(px(t.preview_card_width))
                .when_some(self.width, |this, width| this.w(width.definite()))
                .occlude()
                .on_hover(hover(1))
                .children(self.children);
            positioned(&anchor, Placement::side(self.side, self.align, t), card)
        });

        div()
            .id(self.id)
            .flex()
            .on_hover(hover(0))
            .children(self.trigger.map(|trigger| measured(&anchor, trigger)))
            .children(popup)
    }
}
