//! Navigation tabs that retain the active name while their peers give way.

use crate::components::checkbox::kit;
use crate::foundation::motion::{Motion, cubic_bezier};
use crate::foundation::press::ButtonPress;
use crate::foundation::{ControlColors, State, StyledExt, focus_visible, pointer_down};
use crate::generated::ThemeTokens;
use crate::{ActiveTheme, Icon, IconButton, Menu, MenuItem, Size, Tint, Variant, glyphs};
use gpui::{
    AnyElement, App, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};
use std::rc::Rc;

type OnValueChange = Rc<dyn Fn(SharedString, &mut Window, &mut App)>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabStatus {
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
}

impl TabStatus {
    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "neutral" => Self::Neutral,
            "primary" => Self::Primary,
            "success" => Self::Success,
            "warning" => Self::Warning,
            "danger" => Self::Danger,
            _ => return None,
        })
    }

    fn color(self, t: &ThemeTokens) -> gpui::Hsla {
        match self {
            Self::Neutral => t.color_border_strong.hsla(),
            Self::Primary => t.color_primary.shade(500).hsla(),
            Self::Success => t.color_success.shade(500).hsla(),
            Self::Warning => t.color_warning.shade(400).hsla(),
            Self::Danger => t.color_danger.shade(500).hsla(),
        }
    }
}

#[derive(Clone)]
pub struct TabItem {
    pub value: SharedString,
    pub label: SharedString,
    /// An asset path, as accepted by Icon.
    pub icon: Option<SharedString>,
    pub count: Option<SharedString>,
    pub status: Option<TabStatus>,
    pub busy: bool,
    pub detail: Option<SharedString>,
    pub closable: bool,
    pub title: Option<SharedString>,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
            count: None,
            status: None,
            busy: false,
            detail: None,
            closable: false,
            title: None,
            disabled: false,
        }
    }
    pub fn icon(mut self, path: impl Into<SharedString>) -> Self {
        self.icon = Some(path.into());
        self
    }
    pub fn count(mut self, count: impl ToString) -> Self {
        self.count = Some(count.to_string().into());
        self
    }
    pub fn status(mut self, status: TabStatus) -> Self {
        self.status = Some(status);
        self
    }
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Default)]
struct Metrics {
    pending: bool,
    width: Option<f32>,
    whole: Vec<f32>,
    compact: Vec<f32>,
    trailing: f32,
    hovered: Option<SharedString>,
    handles: Vec<(SharedString, FocusHandle, FocusHandle)>,
}

struct Fit {
    compact: bool,
    order: Vec<usize>,
    shown: usize,
    basis: f32,
    floor: f32,
}

fn fit(metrics: &Metrics, active: Option<usize>, trailing: f32, more: f32) -> Fit {
    let count = metrics.whole.len();
    let basis = metrics.whole.iter().sum::<f32>() + trailing;
    let floor = basis.min(
        metrics
            .whole
            .get(active.unwrap_or(0))
            .copied()
            .unwrap_or(0.)
            + if count > 1 { more } else { 0. }
            + trailing,
    );
    let room = metrics.width.unwrap_or(basis).round() - trailing;
    let compact = metrics.whole.iter().sum::<f32>() > room;
    let mut shown = count;
    if compact {
        while shown > 1 {
            let mut width = metrics.compact[..shown].iter().sum::<f32>();
            if let Some(active) = active {
                if active < shown {
                    width += metrics.whole[active] - metrics.compact[active];
                } else {
                    width += metrics.whole[active] - metrics.compact[shown - 1];
                }
            }
            if shown < count {
                width += more;
            }
            if width <= room {
                break;
            }
            shown -= 1;
        }
    }
    let mut order: Vec<_> = (0..count).collect();
    if let Some(active) = active.filter(|index| *index >= shown && shown > 0) {
        order.remove(active);
        order.insert(shown - 1, active);
    }
    Fit {
        compact,
        order,
        shown,
        basis,
        floor,
    }
}

/// A controlled tab row. With no value, none is selected; the caller draws
/// the panel and accepts or declines selection and close proposals.
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    items: Vec<TabItem>,
    value: Option<SharedString>,
    on_value_change: Option<OnValueChange>,
    on_close: Option<OnValueChange>,
    trailing: Option<AnyElement>,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>, items: Vec<TabItem>) -> Self {
        Self {
            id: id.into(),
            items,
            value: None,
            on_value_change: None,
            on_close: None,
            trailing: None,
        }
    }
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn on_value_change(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
    pub fn on_close(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
    pub fn trailing(mut self, trailing: impl IntoElement) -> Self {
        self.trailing = Some(trailing.into_any_element());
        self
    }
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
}

fn measure(metrics: Entity<Metrics>, part: usize, index: usize) -> impl IntoElement {
    Measure {
        metrics,
        part,
        index,
        element: div().absolute().inset_0().into_any_element(),
    }
}

struct Measure {
    metrics: Entity<Metrics>,
    part: usize,
    index: usize,
    element: AnyElement,
}
impl IntoElement for Measure {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl gpui::Element for Measure {
    type RequestLayoutState = gpui::LayoutId;
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, gpui::LayoutId) {
        let id = self.element.request_layout(window, cx);
        (id, id)
    }
    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: gpui::Bounds<gpui::Pixels>,
        layout: &mut gpui::LayoutId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let metrics = self.metrics.clone();
        let part = self.part;
        let index = self.index;
        let width = f32::from(window.unrounded_layout_size(*layout).width).round();
        let previous = match part {
            0 => metrics.read(cx).width.unwrap_or(-1.),
            1 => metrics.read(cx).whole[index],
            2 => metrics.read(cx).compact[index],
            _ => metrics.read(cx).trailing,
        };
        if previous != width {
            metrics.update(cx, |metrics, _| match part {
                0 => metrics.width = Some(width),
                1 => metrics.whole[index] = width,
                2 => metrics.compact[index] = width,
                _ => metrics.trailing = width,
            });
            if !metrics.read(cx).pending {
                metrics.update(cx, |metrics, _| metrics.pending = true);
                // Notify after the current view's paint has committed.
                // An inline prepaint notification can be cleared by it.
                cx.spawn(async move |cx| {
                    let _ = cx.update(|cx| {
                        metrics.update(cx, |metrics, cx| {
                            metrics.pending = false;
                            cx.notify();
                        })
                    });
                })
                .detach();
            }
        }
        self.element.prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: gpui::Bounds<gpui::Pixels>,
        _: &mut gpui::LayoutId,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.element.paint(window, cx);
    }
}

fn chrome(t: &ThemeTokens, selected: bool) -> gpui::Div {
    div()
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .map(|mut tab| {
            tab.style().align_self = Some(gpui::AlignSelf::Stretch);
            tab
        })
        .gap(px(t.spacing_1_5))
        .px(px(t.spacing_2_5))
        .font_family(t.label_medium.face.family())
        .text_size(px(t.label_medium.size))
        .font_weight(if selected {
            t.label_strong.font_weight()
        } else {
            t.label_medium.font_weight()
        })
        .line_height(px(t.label_medium.size))
        .whitespace_nowrap()
        .text_color(if selected {
            t.color_content.hsla()
        } else {
            t.color_content_subtle.hsla()
        })
}

fn body(
    item: &TabItem,
    compact: bool,
    id: &ElementId,
    measuring: bool,
    t: &ThemeTokens,
    window: &mut Window,
    cx: &mut App,
) -> Vec<AnyElement> {
    let mut body = Vec::new();
    if let Some(status) = item.status {
        let opacity = if measuring {
            1.
        } else {
            pulse(id, item.busy, t, window, cx)
        };
        body.push(
            div()
                .flex_none()
                .size(px(t.spacing_1_5))
                .rounded(px(t.radius_full))
                .bg(status.color(t))
                .opacity(opacity)
                .into_any_element(),
        );
    }
    if let Some(icon) = &item.icon {
        body.push(
            Icon::new(icon.clone())
                .size(t.tabs_icon_size)
                .into_any_element(),
        );
    }
    if !compact || item.icon.is_none() {
        body.push(
            div()
                .min_w_0()
                .overflow_hidden()
                .text_ellipsis()
                .child(crate::foundation::text_child(item.label.clone()))
                .into_any_element(),
        );
        if let Some(detail) = item.detail.as_ref().filter(|detail| !detail.is_empty()) {
            body.push(
                div()
                    .text_color(t.color_content_subtle.hsla())
                    .font_weight(t.label_medium.font_weight())
                    .child(crate::foundation::text_child(detail.clone()))
                    .into_any_element(),
            );
        }
    }
    if let Some(count) = &item.count {
        body.push(
            div()
                .text_size(px(t.caption_small.size))
                .text_color(t.color_content_subtle.hsla())
                .font_weight(t.label_medium.font_weight())
                .tabular_nums()
                .child(crate::foundation::text_child(count.clone()))
                .into_any_element(),
        );
    }
    body
}

#[derive(Default)]
struct Pulse {
    started: std::time::Duration,
    running: bool,
}

fn pulse(id: &ElementId, busy: bool, t: &ThemeTokens, window: &mut Window, cx: &mut App) -> f32 {
    let frame = Motion::frame(cx);
    let state = window.use_keyed_state(kit::child_id(id, "pulse"), cx, |_, _| Pulse::default());
    state.update(cx, |state, _| {
        if !busy {
            state.running = false;
            return 1.;
        }
        if !state.running {
            state.running = true;
            state.started = if frame.frozen && frame.settled {
                std::time::Duration::ZERO
            } else {
                frame.elapsed
            };
        }
        if frame.reduced {
            return 1.;
        }
        let phase = (frame.elapsed.saturating_sub(state.started).as_secs_f32() * 1000.
            / t.tabs_pulse_duration)
            .fract();
        let progress = cubic_bezier(t.motion_easing, (phase * 2.).fract());
        if !frame.frozen {
            window.request_animation_frame();
        }
        if phase < 0.5 {
            1. + (t.tabs_pulse_opacity - 1.) * progress
        } else {
            t.tabs_pulse_opacity + (1. - t.tabs_pulse_opacity) * progress
        }
    })
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let metrics = window.use_keyed_state(kit::child_id(&self.id, "metrics"), cx, |_, _| {
            Metrics::default()
        });
        let handles: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let id = kit::child_id(&self.id, &format!("tab-{}", item.value));
                (
                    item.value.clone(),
                    kit::focus_handle(&id, window, cx),
                    kit::focus_handle(&kit::child_id(&id, "close"), window, cx),
                )
            })
            .collect();
        for (value, tab, close) in &metrics.read(cx).handles {
            if !self.items.iter().any(|item| &item.value == value)
                && (tab.is_focused(window) || close.is_focused(window))
            {
                window.blur();
            }
        }
        metrics.update(cx, |metrics, _| {
            metrics.whole.resize(self.items.len(), 0.);
            metrics.compact.resize(self.items.len(), 0.);
            metrics.handles = handles.clone();
        });
        let active = self
            .items
            .iter()
            .position(|item| Some(&item.value) == self.value.as_ref());
        let trailing = if self.trailing.is_some() {
            metrics.read(cx).trailing + t.spacing_1
        } else {
            0.
        };
        let fit = fit(
            metrics.read(cx),
            active,
            trailing,
            t.control_tiny_size + t.spacing_1,
        );
        let hovered = metrics.read(cx).hovered.clone();
        let mut shown = Vec::new();
        for &index in &fit.order[..fit.shown] {
            let item = &self.items[index];
            let selected = Some(index) == active;
            let compact = fit.compact && !selected;
            let id = kit::child_id(&self.id, &format!("tab-{}", item.value));
            let (_, focus, close_focus) = &handles[index];
            if item.disabled && focus.is_focused(window) {
                window.blur();
            }
            let press = ButtonPress::new(&id, focus, item.disabled, window, cx);
            let mut tab = chrome(t, selected)
                .id(id.clone())
                .children(body(item, compact, &id, false, t, window, cx));
            if item.disabled {
                tab = tab.opacity(0.5).cursor_not_allowed();
            } else {
                let ink = t.color_content.hsla();
                tab = tab
                    .cursor_pointer()
                    .hover(move |style| style.text_color(ink))
                    .track_focus(&focus.clone().tab_stop(selected))
                    .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                        pointer_down(window, cx)
                    });
            }
            if focus_visible(focus, window, cx) && !item.disabled {
                tab = tab.child(
                    div()
                        .absolute()
                        .inset(px(t.focus_offset - t.focus_width))
                        .border(px(t.focus_width))
                        .border_color(crate::foundation::ring_color(t)),
                );
            }
            let title = item
                .title
                .clone()
                .or_else(|| compact.then(|| item.label.clone()))
                .filter(|title| !title.is_empty());
            tab = kit::hint(tab, title);
            tab =
                crate::foundation::motion::color_transitions(tab, id.clone(), t, None, window, cx);
            let handler = self.on_value_change.clone();
            let value = item.value.clone();
            let tab = press.bind(
                tab,
                Some(Box::new(move |_, window, cx| {
                    if let Some(handler) = &handler {
                        handler(value.clone(), window, cx);
                    }
                })),
            );
            let hovered_state = metrics.clone();
            let hovered_value = item.value.clone();
            let mut row = div()
                .id(kit::child_id(&id, "item"))
                .relative()
                .flex()
                .flex_none()
                .items_center()
                .on_hover(move |hovered, _, cx| {
                    hovered_state.update(cx, |state, cx| {
                        if *hovered {
                            state.hovered = Some(hovered_value.clone());
                        } else if state.hovered.as_ref() == Some(&hovered_value) {
                            state.hovered = None;
                        }
                        cx.notify();
                    });
                })
                .child(tab);
            if item.closable && !compact {
                if let Some(handler) = self.on_close.clone() {
                    let value = item.value.clone();
                    let visible = selected
                        || hovered.as_ref() == Some(&item.value)
                        || focus_visible(close_focus, window, cx);
                    row = row
                        .pr(px(t.control_tiny_size - t.spacing_1_5 + t.spacing_1))
                        .child(
                            div()
                                .absolute()
                                .top_0()
                                .bottom_0()
                                .right(px(t.spacing_1))
                                .flex()
                                .items_center()
                                .flex_none()
                                .when(!visible, |close| close.invisible())
                                .child(
                                    IconButton::new(
                                        kit::child_id(&id, "close"),
                                        format!("Close {}", item.label),
                                    )
                                    .focus_handle(close_focus.clone())
                                    .size(Size::Tiny)
                                    .variant(Variant::Plain)
                                    .tint(Tint::Neutral)
                                    .on_click(move |_, window, cx| {
                                        handler(value.clone(), window, cx)
                                    })
                                    .child(
                                        Icon::new(glyphs::DISMISS_REGULAR)
                                            .size(t.tabs_close_icon_size),
                                    ),
                                ),
                        );
                }
            } else if close_focus.is_focused(window) {
                window.blur();
            }
            if selected {
                row = row.child(
                    div()
                        .absolute()
                        .left(px(t.spacing_2_5))
                        .right(px(t.spacing_2_5))
                        .bottom_0()
                        .h(px(t.stroke_control))
                        .rounded(px(t.radius_full))
                        .bg(ControlColors::resolve(
                            t,
                            Variant::Filled,
                            Tint::Primary,
                            State::Normal,
                        )
                        .surface),
                );
            }
            shown.push(row.into_any_element());
        }
        for &index in &fit.order[fit.shown..] {
            if handles[index].1.is_focused(window) || handles[index].2.is_focused(window) {
                window.blur();
            }
        }
        let enabled: Vec<_> = fit.order[..fit.shown]
            .iter()
            .copied()
            .filter(|index| !self.items[*index].disabled)
            .collect();
        let keys = handles.clone();
        let handler = self.on_value_change.clone();
        let list = div()
            .flex()
            .min_w_0()
            .children(shown)
            .on_key_down(move |event, window, cx| {
                if !matches!(
                    event.keystroke.key.as_str(),
                    "left" | "right" | "home" | "end"
                ) || enabled.is_empty()
                {
                    return;
                }
                let at = enabled
                    .iter()
                    .position(|index| Some(*index) == active)
                    .map(|index| index as isize)
                    .unwrap_or(-1);
                let len = enabled.len() as isize;
                let to = match event.keystroke.key.as_str() {
                    "home" => 0,
                    "end" => len - 1,
                    "right" => (at + 1 + len) % len,
                    _ => (at - 1 + len) % len,
                } as usize;
                let index = enabled[to];
                window.prevent_default();
                if let Some(handler) = &handler {
                    handler(keys[index].0.clone(), window, cx);
                }
                window.focus(&keys[index].1);
                cx.stop_propagation();
            });
        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_shrink()
            .map(|mut root| {
                root.style().align_self = Some(gpui::AlignSelf::Stretch);
                root
            })
            .min_h(px(t.control_small_size))
            .min_w(px(fit.floor))
            .overflow_hidden()
            .when(fit.basis > 0., |root| root.flex_basis(px(fit.basis)))
            .child(list)
            .child(measure(metrics.clone(), 0, 0));
        if fit.shown < self.items.len() {
            let items = fit.order[fit.shown..]
                .iter()
                .map(|index| {
                    let item = &self.items[*index];
                    let handler = self.on_value_change.clone();
                    let value = item.value.clone();
                    let label = match &item.count {
                        Some(count) => format!("{} · {count}", item.label),
                        None => item.label.to_string(),
                    };
                    let mut entry = MenuItem::new(label).disabled(item.disabled).on_select(
                        move |window, cx| {
                            if let Some(handler) = &handler {
                                handler(value.clone(), window, cx);
                            }
                        },
                    );
                    if let Some(icon) = &item.icon {
                        entry = entry.icon(Icon::new(icon.clone()).size(t.tabs_icon_size));
                    }
                    entry
                })
                .collect::<Vec<_>>();
            let id = kit::child_id(&self.id, "more");
            let label = format!("{} more", items.len());
            root =
                root.child(
                    div()
                        .map(|mut more| {
                            more.style().align_self = Some(gpui::AlignSelf::Center);
                            more
                        })
                        .flex_none()
                        .child(Menu::new(id.clone()).items(items).trigger(
                            move |press, _, _, _| {
                                IconButton::new(id, label)
                                    .on_click(move |_, window, cx| press(window, cx))
                                    .child(
                                        Icon::new(glyphs::MORE_HORIZONTAL_REGULAR)
                                            .size(t.tabs_icon_size),
                                    )
                                    .into_any_element()
                            },
                        )),
                );
        }
        if let Some(trailing) = self.trailing {
            root = root.child(
                div()
                    .relative()
                    .flex()
                    .flex_none()
                    .items_center()
                    .map(|mut tail| {
                        tail.style().align_self = Some(gpui::AlignSelf::Center);
                        tail
                    })
                    .ml(px(t.spacing_1))
                    .child(trailing)
                    .child(measure(metrics.clone(), 3, 0)),
            );
        }
        let mut ruler = div()
            .absolute()
            .top_0()
            .left_0()
            .flex()
            .h_0()
            .overflow_hidden()
            .invisible();
        for (index, item) in self.items.iter().enumerate() {
            let id = kit::child_id(&self.id, &format!("ruler-{index}"));
            let mut whole = div().relative().flex().flex_none().items_center().child(
                chrome(t, Some(index) == active)
                    .children(body(item, false, &id, true, t, window, cx)),
            );
            if item.closable && self.on_close.is_some() {
                whole = whole.pr(px(t.control_tiny_size - t.spacing_1_5 + t.spacing_1));
            }
            ruler = ruler
                .child(whole.child(measure(metrics.clone(), 1, index)))
                .child(
                    div()
                        .relative()
                        .flex()
                        .flex_none()
                        .child(
                            chrome(t, false).children(body(item, true, &id, true, t, window, cx)),
                        )
                        .child(measure(metrics.clone(), 2, index)),
                );
        }
        root.child(ruler)
    }
}
