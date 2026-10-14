//! A selectable card: one of a few exclusive choices that each need a line
//! of explanation.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::foundation::{ControlColors, State, StyledExt, Tint, Variant, pointer_down};
use crate::theme::ActiveTheme;

type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// Title over description, in the strong/caption pair. Selected is the
/// tint's surface wash with the accent drawn around it at the control
/// stroke — the border thickens as well as colours.
#[derive(IntoElement)]
pub struct OptionCard {
    id: ElementId,
    title: AnyElement,
    description: Option<AnyElement>,
    selected: bool,
    tint: Tint,
    disabled: bool,
    flush: bool,
    on_click: Option<OnClick>,
}

impl OptionCard {
    pub fn new(id: impl Into<ElementId>, title: impl IntoElement) -> Self {
        OptionCard {
            id: id.into(),
            title: crate::foundation::text_child(title),
            description: None,
            selected: false,
            tint: Tint::Primary,
            disabled: false,
            flush: false,
            on_click: None,
        }
    }

    pub fn description(mut self, description: impl IntoElement) -> Self {
        self.description = Some(crate::foundation::text_child(description));
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// No inset of its own, so the title runs to the card's edge — what a
    /// React caller does with a class, for a card whose title is a picture.
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for OptionCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let focus = kit::focus_handle(&self.id, window, cx);
        let visible = !self.disabled && crate::foundation::focus_visible(&focus, window, cx);
        let caption = t.caption_small;
        let small = t.label_small.size;

        let description = self.description.map(|description| {
            div()
                .text_color(if self.selected {
                    t.color_content_muted.hsla()
                } else {
                    t.color_content_subtle.hsla()
                })
                .font_family(caption.face.family())
                .text_size(px(small))
                .font_weight(caption.font_weight())
                .line_height(px(small * 1.5))
                .child(crate::foundation::text_child(description))
        });

        let mut card = div()
            .id(self.id)
            .relative()
            .flex()
            .flex_1()
            .flex_col()
            .map(|this| {
                if self.flush {
                    this.overflow_hidden()
                } else {
                    this.gap(px(t.spacing_1_5))
                        .py(px(t.spacing_3_5))
                        .px(px(t.spacing_3))
                }
            })
            .rounded(px(t.radius_large))
            .map(|this| {
                if self.selected {
                    this.border(px(crate::foundation::border_width(t.stroke_control)))
                        .border_color(
                            ControlColors::resolve(t, Variant::Outlined, self.tint, State::Normal)
                                .border,
                        )
                        .bg(self.tint.ramp(t).shade(600).alpha(t.wash_surface))
                } else {
                    this.border(px(t.stroke_hairline))
                        .border_color(t.color_border.hsla())
                        .bg(t.color_surface_muted.hsla())
                }
            })
            .child(
                div()
                    .text_color(t.color_content.hsla())
                    .type_style(t.label_strong)
                    .child(crate::foundation::text_child(self.title)),
            )
            .children(description)
            .when(visible, |this| this.child(kit::outline(t, t.radius_large)));

        if self.disabled {
            let disabled = ControlColors::resolve(t, Variant::Normal, self.tint, State::Disabled);
            card = card
                .cursor_not_allowed()
                .bg(disabled.surface)
                .text_color(disabled.content);
        } else {
            let subtle = t.color_surface_subtle.hsla();
            card = card
                .cursor_pointer()
                .when(!self.selected, |this| {
                    this.hover(move |style| style.bg(subtle))
                })
                .track_focus(&focus.tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx));
            if let Some(on_click) = self.on_click {
                card = card.on_click(on_click);
            }
        }
        card
    }
}
