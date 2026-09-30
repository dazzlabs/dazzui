//! One thing that opens: a header row, and the panel it discloses.
//!
//! The row and the panel are the disclosure drawing — collapsible.css's
//! `dz-disclosure__*` — and an [`Accordion`](crate::Accordion) wears the same
//! one, so both are drawn by the crate-private helpers here.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, SharedString, Stateful, StatefulInteractiveElement, Styled, Window,
    div, prelude::FluentBuilder, px,
};

use crate::components::icon::{Icon, icon_slot};
use crate::foundation::{StyledExt, Tint, focus_visible, pointer_down, ring};
use crate::glyphs;
use crate::theme::ActiveTheme;

/// An open-state change: the new state, as a controlled prop hears it.
pub use crate::components::popup::OnOpenChange;

/// A part of a component, keyed under the component's own id.
pub(crate) fn part_id(id: &ElementId, part: impl Into<SharedString>) -> ElementId {
    ElementId::NamedChild(Box::new(id.clone()), part.into())
}

/// A focus handle kept for the life of the element `id` names.
pub(crate) fn kept_focus(id: ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    window
        .use_keyed_state(id, cx, |_, cx| cx.focus_handle())
        .read(cx)
        .clone()
}

/// The header row: chevron, optional glyph, title and trailing aside. It rests
/// transparent and washes with the `normal` recipe's hover — read on the
/// neutral ramp, since a disclosure never re-tints.
#[allow(clippy::too_many_arguments)]
pub(crate) fn disclosure_row(
    id: ElementId,
    focus: &FocusHandle,
    open: bool,
    disabled: bool,
    icon: Option<AnyElement>,
    title: Option<AnyElement>,
    meta: Option<AnyElement>,
    window: &Window,
    cx: &App,
) -> Stateful<Div> {
    let t = cx.theme();
    let hover = t
        .control_color_normal_surface_hovered
        .resolve(Tint::Neutral.ramp(t));
    let visible = !disabled && focus_visible(focus, window, cx);
    let chevron = if open {
        glyphs::CHEVRON_DOWN_12_REGULAR
    } else {
        glyphs::CHEVRON_RIGHT_12_REGULAR
    };

    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(t.spacing_2))
        .w_full()
        .py(px(t.spacing_2))
        .px(px(t.spacing_2))
        .rounded(px(t.control_container_radius))
        .type_style(t.label_medium)
        .map(|this| {
            if disabled {
                this.text_color(
                    t.control_color_normal_content_disabled
                        .resolve(Tint::Neutral.ramp(t)),
                )
                .cursor_not_allowed()
            } else {
                this.text_color(t.color_content.hsla())
                    .cursor_pointer()
                    .hover(move |style| style.bg(hover))
                    .track_focus(&focus.clone().tab_stop(true))
                    .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
            }
        })
        .when(visible, |this| this.shadow(vec![ring(t)]))
        .child(
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .w(px(t.icon_small))
                .text_color(t.color_content_subtle.hsla())
                .child(Icon::new(chevron).size(t.icon_small)),
        )
        .when_some(icon, |this, icon| {
            this.child(icon_slot(
                t.icon_medium,
                div()
                    .flex()
                    .flex_none()
                    .text_color(t.color_content_muted.hsla())
                    .child(icon),
            ))
        })
        .child(
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .whitespace_nowrap()
                .text_ellipsis()
                .children(title),
        )
        .when_some(meta, |this, meta| {
            this.child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(t.spacing_2))
                    .text_color(t.color_content_faint.hsla())
                    .font_family(t.label_quiet.face.family())
                    .text_size(px(t.label_small.size))
                    .font_weight(t.label_quiet.font_weight())
                    .child(meta),
            )
        })
}

/// The disclosed panel: indented to the chevron's far edge, in the body ink.
pub(crate) fn disclosure_panel(children: Vec<AnyElement>, cx: &App) -> Div {
    let t = cx.theme();
    div()
        .pt(px(t.spacing_1))
        .pb(px(t.spacing_3))
        .pl(px(t.spacing_2 + t.icon_small + t.spacing_2))
        .pr(px(t.spacing_2))
        .text_color(t.color_content_secondary.hsla())
        .type_style(t.body_small)
        .children(children)
}

/// A single disclosure. Several that know about each other are an
/// [`Accordion`](crate::Accordion), which wears this same drawing.
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    title: AnyElement,
    icon: Option<AnyElement>,
    meta: Option<AnyElement>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<OnOpenChange>,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>, title: impl IntoElement) -> Self {
        Collapsible {
            id: id.into(),
            title: title.into_any_element(),
            icon: None,
            meta: None,
            open: None,
            default_open: false,
            on_open_change: None,
            disabled: false,
            children: Vec::new(),
        }
    }

    /// A glyph before the title.
    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    /// A trailing aside on the header — a count, a duration, a `Badge`.
    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta = Some(meta.into_any_element());
        self
    }

    /// Controlled: whether the panel shows. Pair with `on_open_change`.
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl ParentElement for Collapsible {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Collapsible {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default_open = self.default_open;
        let own = window.use_keyed_state(part_id(&self.id, "open"), cx, |_, _| default_open);
        let open = self.open.unwrap_or(*own.read(cx));
        let focus = kept_focus(part_id(&self.id, "focus"), window, cx);
        let controlled = self.open.is_some();
        let on_change = self.on_open_change.clone();

        let mut row = disclosure_row(
            part_id(&self.id, "trigger"),
            &focus,
            open,
            self.disabled,
            self.icon,
            Some(self.title),
            self.meta,
            window,
            cx,
        );
        if !self.disabled {
            row = row.on_click(move |_, window, cx| {
                if !controlled {
                    own.update(cx, |open, cx| {
                        *open = !*open;
                        cx.notify();
                    });
                }
                if let Some(on_change) = &on_change {
                    on_change(!open, window, cx);
                }
                window.refresh();
            });
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .child(row)
            .when(open, |this| this.child(disclosure_panel(self.children, cx)))
    }
}
