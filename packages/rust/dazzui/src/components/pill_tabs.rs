//! Free-standing filter pills: a tab list with no track.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};

use crate::components::checkbox::kit;
use crate::components::icon::icon_slot;
use crate::foundation::{ControlColors, State, StyledExt, Tint, Variant, pointer_down};
use crate::theme::ActiveTheme;

type OnValueChange = Rc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>;

/// One pill of [`PillTabs`].
pub struct PillTabItem {
    pub value: SharedString,
    pub label: AnyElement,
    /// Trailing count, set inside the pill in the label's own ink.
    pub count: Option<SharedString>,
    pub disabled: bool,
}

impl PillTabItem {
    pub fn new(value: impl Into<SharedString>, label: impl IntoElement) -> Self {
        PillTabItem {
            value: value.into(),
            label: crate::foundation::text_child(label),
            count: None,
            disabled: false,
        }
    }

    pub fn count(mut self, count: impl ToString) -> Self {
        self.count = Some(count.to_string().into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// The tab list alone: the pills say which is showing, and the caller
/// renders the panel for `value`. One size, drawn tight — the tiny control
/// height on small type. The list is one tab stop; the arrow keys move
/// focus and Space or Enter picks.
#[derive(IntoElement)]
pub struct PillTabs {
    id: ElementId,
    items: Vec<PillTabItem>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    tint: Tint,
    on_value_change: Option<OnValueChange>,
}

impl PillTabs {
    pub fn new(id: impl Into<ElementId>, items: Vec<PillTabItem>) -> Self {
        PillTabs {
            id: id.into(),
            items,
            value: None,
            default_value: None,
            tint: Tint::Primary,
            on_value_change: None,
        }
    }

    /// Controlled: the pill shown active.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Uncontrolled: where it starts. Without one the first pill is active,
    /// as Base UI's tabs start.
    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for PillTabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = window.use_keyed_state(kit::child_id(&self.id, "value"), cx, {
            let default = self
                .default_value
                .clone()
                .or_else(|| self.items.first().map(|item| item.value.clone()));
            move |_, _| default
        });
        let current = self.value.clone().or_else(|| state.read(cx).clone());
        let count = self.items.len();
        let handles = kit::focus_handles(&self.id, count, window, cx);
        let enabled: Vec<bool> = self.items.iter().map(|item| !item.disabled).collect();
        let active = self
            .items
            .iter()
            .position(|item| Some(&item.value) == current.as_ref());
        let tab_stop = kit::focused(&handles, window)
            .or(active.filter(|index| enabled[*index]))
            .or_else(|| enabled.iter().position(|e| *e));
        let filled = ControlColors::resolve(t, Variant::Filled, self.tint, State::Normal);
        let small = t.label_small;
        let controlled = self.value.is_some();

        let pills = self
            .items
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let selected = active == Some(index);
                let focus = handles[index].clone();
                let visible = crate::foundation::focus_visible(&focus, window, cx);
                let mut pill = div()
                    .id(kit::child_id(&self.id, &format!("tab-{index}")))
                    .relative()
                    .flex()
                    .items_center()
                    .h(px(t.control_tiny_size))
                    .gap(px(t.control_tiny_gap))
                    .px(px(t.spacing_3))
                    .font_family(small.face.family())
                    .text_size(px(small.size))
                    .font_weight(small.font_weight())
                    .line_height(px(small.size))
                    .rounded(px(t.radius_full))
                    .whitespace_nowrap()
                    .map(|this| {
                        if selected {
                            this.bg(filled.surface)
                                .text_color(filled.content)
                                .font_weight(t.label_strong.font_weight())
                        } else {
                            this.bg(t.color_surface_inset.hsla())
                                .text_color(t.color_content_nav.hsla())
                        }
                    })
                    .child(crate::foundation::text_child(item.label))
                    .children(item.count.map(|count| {
                        div()
                            .tabular_nums()
                            .child(crate::foundation::text_child(count))
                    }))
                    .when(visible, |this| this.child(kit::outline(t, t.radius_full)));
                if item.disabled {
                    pill = pill.opacity(0.5).cursor_not_allowed();
                } else {
                    let sunken = t.color_surface_sunken.hsla();
                    let content = t.color_content.hsla();
                    let handler = self.on_value_change.clone();
                    let state = state.clone();
                    let value = item.value.clone();
                    pill = pill
                        .cursor_pointer()
                        .when(!selected, |this| {
                            this.hover(move |style| style.bg(sunken).text_color(content))
                        })
                        .track_focus(&focus.tab_stop(tab_stop == Some(index)))
                        .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                        .on_click(move |_, window, cx| {
                            if selected {
                                return;
                            }
                            if !controlled {
                                let value = value.clone();
                                state.update(cx, |current, cx| {
                                    *current = Some(value);
                                    cx.notify();
                                });
                            }
                            if let Some(handler) = &handler {
                                handler(value.clone(), window, cx);
                            }
                        });
                }
                pill
            })
            .collect::<Vec<_>>();

        icon_slot(
            t.icon_medium,
            div()
                .id(self.id)
                .flex()
                .items_center()
                .gap(px(t.spacing_2))
                .on_key_down(move |event, window, cx| {
                    let Some(step) = kit::arrow(&event.keystroke.key, false) else {
                        return;
                    };
                    let Some(current) = kit::focused(&handles, window) else {
                        return;
                    };
                    if let Some(next) = kit::target(current, &enabled, step) {
                        window.focus(&handles[next]);
                    }
                    cx.stop_propagation();
                })
                .children(pills),
        )
    }
}
