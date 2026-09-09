use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window, deferred, div,
};

use crate::components::popover::{anchor, popover_panel};
use crate::prelude::*;
use crate::{PopoverAlign, PopoverSide, latched};

/// What is behind a link, shown without following it — a repository, a person,
/// an issue.
///
/// It is a [`Tooltip`](crate::Tooltip) that holds a card rather than a label,
/// and the difference is what may go in it: a tooltip must carry nothing a
/// reader needs, because a pointer never reaches it; a preview card stays open
/// while the pointer travels, so it can hold a link of its own. It is never
/// the only way to reach what it shows.
///
/// The sheet is the [`Popover`](crate::Popover)'s — a preview card is a
/// popover the pointer opens, so a second drawing of it could only drift. What
/// is its own is a ceiling on the width: a popover is as wide as what a caller
/// put in it; this is filled from whatever it previews, so it needs a width it
/// cannot exceed no matter what comes back.
#[derive(IntoElement)]
pub struct PreviewCard {
    id: ElementId,
    /// What it previews — a link, usually.
    trigger: Option<AnyElement>,
    child: Option<AnyElement>,
    side: PopoverSide,
    align: PopoverAlign,
    width: Option<gpui::Pixels>,
}

impl PreviewCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        PreviewCard {
            id: id.into(),
            trigger: None,
            child: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            width: None,
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl RenderOnce for PreviewCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let PreviewCard {
            id,
            trigger,
            child,
            side,
            align,
            width,
        } = self;

        latched::<bool>(id.clone(), move |open, _window, _cx| {
            let is_open = open.get();

            div()
                .relative()
                .flex()
                .flex_none()
                .id("preview-card-trigger")
                // The pointer opens it and the pointer closes it. The card is
                // a child of the trigger's own box, so the hover that keeps it
                // up is the same one that raised it — which is what lets a
                // reader travel into the card without it going out from under
                // them.
                .on_hover({
                    let open = open.clone();
                    move |hovered, window, _| {
                        if open.get() != *hovered {
                            open.set(*hovered);
                            window.refresh();
                        }
                    }
                })
                .when_some(trigger, |el, trigger| el.child(trigger))
                .when(is_open, |el| {
                    el.child(deferred(
                        anchor(popover_panel(vars).absolute(), side, align, vars.spacing_1)
                            .max_w(width.unwrap_or(vars.preview_card_width))
                            .occlude()
                            .when_some(child, |el, child| el.child(child)),
                    ))
                })
                .into_any_element()
        })
    }
}
