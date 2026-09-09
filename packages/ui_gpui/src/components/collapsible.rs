use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, relative,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, SharedActionListener, Tint, colors,
    latched, pressable,
};

/// The header row of a disclosure, and the panel under it.
///
/// [`Collapsible`] is one of them; an [`Accordion`](crate::Accordion) is
/// several that know about each other, and it wears this same drawing. So the
/// two are shared the way a menu's panel and rows are — one drawing, not two
/// that resemble each other.
///
/// The row is not on the shared control state machine. `normal` is the right
/// recipe for a row's wash, but its resting surface is opaque paper, and a
/// header that painted itself would cut a card in half; so the wash is read on
/// hover alone and the row rests transparent — exactly what a menu row does.
#[derive(IntoElement)]
pub(crate) struct DisclosureHeader {
    id: ElementId,
    title: SharedString,
    open: bool,
    icon: Option<Icon>,
    meta: Option<AnyElement>,
    disabled: bool,
    on_toggle: Option<ActionListener>,
}

impl DisclosureHeader {
    pub(crate) fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        open: bool,
    ) -> Self {
        DisclosureHeader {
            id: id.into(),
            title: title.into(),
            open,
            icon: None,
            meta: None,
            disabled: false,
            on_toggle: None,
        }
    }

    pub(crate) fn icon(mut self, icon: Option<Icon>) -> Self {
        self.icon = icon;
        self
    }

    pub(crate) fn meta(mut self, meta: Option<AnyElement>) -> Self {
        self.meta = meta;
        self
    }

    pub(crate) fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub(crate) fn on_toggle(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for DisclosureHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let DisclosureHeader {
            id,
            title,
            open,
            icon,
            meta,
            disabled,
            on_toggle,
        } = self;
        let on_toggle: Option<SharedActionListener> = on_toggle.map(std::rc::Rc::from);

        pressable(id.clone(), !disabled, move |state, _, _| {
            let wash = vars.control_color(
                ControlVariant::Normal,
                ControlPart::Surface,
                Tint::Neutral,
                ControlState::Hovered,
            );
            let content = if disabled {
                vars.control_color(
                    ControlVariant::Normal,
                    ControlPart::Content,
                    Tint::Neutral,
                    ControlState::Disabled,
                )
            } else {
                vars.color_content
            };

            div()
                .id(id)
                .flex()
                .items_center()
                .gap(vars.spacing_2)
                .w_full()
                .p(vars.spacing_2)
                .rounded(vars.radius_medium)
                .type_style(vars.label_medium)
                .text_color(content)
                .map(|el| {
                    if disabled {
                        el.cursor_not_allowed()
                    } else {
                        el.cursor_pointer().bg(if state.hovered || state.pressed {
                            wash
                        } else {
                            colors::TRANSPARENT
                        })
                    }
                })
                // The chevron points along the row when shut and down the page
                // when open — one glyph turned rather than two swapped, so the
                // turn is the animation. It is the only moving part.
                .child(
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .w(vars.spacing_3)
                        .child(
                            Icon::new(if open {
                                icons::CHEVRON_DOWN_12_REGULAR
                            } else {
                                icons::CHEVRON_RIGHT_12_REGULAR
                            })
                            .size(vars.label_small.size)
                            .color(if disabled {
                                content
                            } else {
                                vars.color_content_subtle
                            }),
                        ),
                )
                .when_some(icon, |el, icon| {
                    el.child(icon.size(vars.spacing_4).color(if disabled {
                        content
                    } else {
                        vars.color_content_muted
                    }))
                })
                .child(div().flex_1().min_w_0().child(title))
                // A trailing aside — a count, a duration — steps back the way
                // a menu row's shortcut does, and never takes the row's weight.
                .when_some(meta, |el, meta| {
                    el.child(
                        div()
                            .flex()
                            .flex_none()
                            .items_center()
                            .gap(vars.spacing_2)
                            .type_style(vars.label_quiet)
                            .text_size(vars.label_small.size)
                            .text_color(vars.color_content_faint)
                            .child(meta),
                    )
                })
                .when_some(on_toggle.clone().filter(|_| state.enabled), |el, toggle| {
                    el.on_click(move |_, window, cx| toggle(window, cx))
                })
                .into_any_element()
        })
    }
}

/// What a disclosure discloses.
///
/// The content is indented to the chevron's far edge, so a nested list lines
/// up under the label rather than under the glyph.
#[derive(IntoElement)]
pub(crate) struct DisclosurePanel {
    child: AnyElement,
}

impl DisclosurePanel {
    pub(crate) fn new(child: AnyElement) -> Self {
        DisclosurePanel { child }
    }
}

impl RenderOnce for DisclosurePanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .pl(vars.spacing_2 + vars.spacing_3 + vars.spacing_2)
            .pr(vars.spacing_2)
            .pt(vars.spacing_1)
            .pb(vars.spacing_3)
            .type_style(vars.body_small)
            .line_height(relative(1.6))
            .text_color(vars.color_content_secondary)
            .child(self.child)
    }
}

/// One thing that opens.
///
/// The header row is the affordance and the panel is what it discloses. It is
/// the smallest of the three ways this system hides something: a collapsible
/// keeps its subject in place and only folds it, where a
/// [`Popover`](crate::Popover) moves it off the page and a
/// [`Dialog`](crate::Dialog) stops the page.
///
/// The open state is the element's own, kept in
/// [`latched`](crate::latched) — a host that wants to drive it passes
/// `open` and `on_toggle` instead.
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    title: SharedString,
    child: Option<AnyElement>,
    icon: Option<Icon>,
    meta: Option<AnyElement>,
    /// Drive it from outside. Left `None` the element holds its own state.
    open: Option<bool>,
    initially_open: bool,
    disabled: bool,
    on_toggle: Option<ActionListener>,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Collapsible {
            id: id.into(),
            title: title.into(),
            child: None,
            icon: None,
            meta: None,
            open: None,
            initially_open: false,
            disabled: false,
            on_toggle: None,
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn meta(mut self, meta: impl IntoElement) -> Self {
        self.meta = Some(meta.into_any_element());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn initially_open(mut self, open: bool) -> Self {
        self.initially_open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_toggle(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let Collapsible {
            id,
            title,
            child,
            icon,
            meta,
            open,
            initially_open,
            disabled,
            on_toggle,
        } = self;
        let on_toggle: Option<SharedActionListener> = on_toggle.map(std::rc::Rc::from);

        // The latch remembers whether it has been touched, so `initially_open`
        // is what it starts at rather than what it snaps back to every frame.
        latched::<(bool, bool)>(id.clone(), move |cell, _window, _cx| {
            let (touched, latched_open) = cell.get();
            let is_open = open.unwrap_or(if touched {
                latched_open
            } else {
                initially_open
            });

            div()
                .flex()
                .flex_col()
                .w_full()
                .child(
                    DisclosureHeader::new(
                        SharedString::from(format!("{id:?}-header")),
                        title,
                        is_open,
                    )
                    .icon(icon)
                    .meta(meta)
                    .disabled(disabled)
                    .on_toggle({
                        let cell = cell.clone();
                        let on_toggle = on_toggle.clone();
                        move |window, cx| {
                            cell.set((true, !is_open));
                            window.refresh();
                            if let Some(listener) = on_toggle.as_ref() {
                                listener(window, cx);
                            }
                        }
                    }),
                )
                .when_some(child.filter(|_| is_open), |el, child| {
                    el.child(DisclosurePanel::new(child))
                })
                .into_any_element()
        })
    }
}
