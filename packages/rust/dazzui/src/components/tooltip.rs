//! The label a control surfaces on hover. It never carries anything a
//! reader has to have: a tooltip is a reminder of what an icon means.

use std::time::Duration;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::popup::{Align, Placement, PopupState, Side, measured, positioned};
use crate::foundation::StyledExt;
use crate::theme::ActiveTheme;
use crate::tokens::shadows;

/// Base UI's default wait before a tooltip opens.
const DELAY: Duration = Duration::from_millis(600);

/// A tooltip over its one child.
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    label: SharedString,
    side: Side,
    align: Align,
    delay: Duration,
    close_delay: Duration,
    disabled: bool,
    child: Option<AnyElement>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Tooltip {
            id: id.into(),
            label: label.into(),
            side: Side::Top,
            align: Align::Center,
            delay: DELAY,
            close_delay: Duration::ZERO,
            disabled: false,
            child: None,
        }
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// How long the pointer rests before it opens.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// How long it stays once the pointer leaves.
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = delay;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        // One control: the last one given is the one it describes.
        if let Some(element) = elements.into_iter().last() {
            self.child = Some(element);
        }
    }
}

impl RenderOnce for Tooltip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = PopupState::keyed(&self.id, false, window, cx);
        let anchor = state.read(cx).anchor.clone();
        let open = state.read(cx).open && !self.disabled;
        let (delay, close_delay) = (self.delay, self.close_delay);

        let popup = open.then(|| {
            // `.dz-tooltip`: the raised card, a hairline, the small label.
            let label = div()
                .max_w(px(t.toast_max_width))
                .py(px(t.spacing_1 + t.spacing_0_5))
                .px(px(t.spacing_2))
                .bg(t.color_surface_raised.hsla())
                .border(px(t.stroke_hairline))
                .border_color(t.color_border.hsla())
                .rounded(px(t.radius_small))
                .shadow(shadows(t.shadow_sm))
                .text_color(t.color_content.hsla())
                .type_style(t.label_small)
                .child(crate::foundation::text_child(self.label));
            positioned(&anchor, Placement::side(self.side, self.align, t), label)
        });

        let hover = state.clone();
        let press = state.clone();
        div()
            .id(self.id)
            .flex()
            .when(!self.disabled, |this| {
                this.on_hover(move |hovered, window, cx| {
                    hover.update(cx, |state, cx| {
                        state.hover(0, *hovered, delay, close_delay, window, cx)
                    })
                })
                // A press is the control being used, not read about.
                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    press.update(cx, |state, cx| {
                        state.open = false;
                        cx.notify();
                    })
                })
            })
            .children(self.child.map(|child| measured(&anchor, child)))
            .children(popup)
    }
}
