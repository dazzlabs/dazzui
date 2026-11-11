//! A run of native buttons, each drawing a miniature window in its own
//! palette. Only selection and focus use the surrounding preference theme.
use super::*;
use crate::Theme;
use crate::components::checkbox::kit;
use crate::components::tooltip::Tooltip;
use crate::foundation::press::ButtonPress;
use gpui::{BoxShadow, Component, SharedString, point, relative};

/// A value and accessible label, optionally previewed in a different theme.
#[derive(Clone)]
pub struct PreferenceThemeOption {
    pub value: SharedString,
    pub label: SharedString,
    pub theme: Option<SharedString>,
}
impl PreferenceThemeOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            theme: None,
        }
    }
    /// An empty name previews the root palette; an unknown name inherits
    /// the host, as an unmatched data-theme does in React.
    pub fn theme(mut self, name: impl Into<SharedString>) -> Self {
        self.theme = Some(name.into());
        self
    }
}

/// Small theme previews. These are ordinary buttons with radio semantics:
/// every option is a Tab stop, and Enter/Space activate on native phases.
#[derive(IntoElement)]
pub struct PreferenceThemeSwatches {
    id: ElementId,
    options: Vec<PreferenceThemeOption>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    on_value_change: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App)>>,
}
impl PreferenceThemeSwatches {
    pub fn new(id: impl Into<ElementId>, options: Vec<PreferenceThemeOption>) -> Self {
        Self {
            id: id.into(),
            options,
            value: None,
            default_value: None,
            on_value_change: None,
        }
    }
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }
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
impl RenderOnce for PreferenceThemeSwatches {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let host = cx.theme();
        let own = window.use_keyed_state(part_id(&self.id, "value"), cx, move |_, _| {
            self.default_value
        });
        let current = self.value.or_else(|| own.read(cx).clone());
        let mut buttons = Vec::with_capacity(self.options.len());
        for option in self.options {
            let name = option.theme.as_ref().unwrap_or(&option.value);
            // Bright Light is :root, with no data-theme selector. Explicit
            // root previews override only the five colours read by React;
            // geometry still inherits the host's metrics.
            let named = Theme::named(name).filter(|entry| entry.name != "bright-light");
            let metrics = host;
            // Colour families do not reset geometry in the CSS cascade.
            // Only the imported platform blocks explicitly set control size.
            let size = named
                .filter(|entry| {
                    matches!(
                        entry.name.rsplit_once('-').map(|(family, _)| family),
                        Some(
                            "macos27"
                                | "macos15"
                                | "windows11"
                                | "ubuntu"
                                | "debian"
                                | "fedora"
                                | "kde"
                                | "spotify"
                        )
                    )
                })
                .map_or(host.control_small_size, |entry| {
                    entry.tokens.control_small_size
                });
            let palette = if name.is_empty() {
                Theme::named("bright-light").unwrap().tokens
            } else {
                named.map_or(host, |entry| entry.tokens)
            };
            let radius = host.spacing_2;
            // Only the column meets the window's rounded inner corners.
            let column_radius = (radius - metrics.stroke_hairline).max(0.);
            let id = part_id(&self.id, option.value.clone());
            let focus = kept_focus(part_id(&id, "focus"), window, cx);
            let visible = focus_visible(&focus, window, cx);
            let press = ButtonPress::new(&id, &focus, false, window, cx);
            let chosen = current.as_ref() == Some(&option.value);
            let preview = div()
                .flex()
                .flex_none()
                .w(px(size * 1.5))
                .h(px(size))
                .border(px(metrics.stroke_hairline))
                .border_color(palette.color_border_strong.hsla())
                .rounded(px(radius))
                .bg(palette.color_surface.hsla())
                .child(
                    div()
                        .flex_none()
                        .w(relative(0.3))
                        .h_full()
                        .rounded_tl(px(column_radius))
                        .rounded_bl(px(column_radius))
                        .border_r(px(metrics.stroke_hairline))
                        .border_color(palette.color_border_strong.hsla())
                        .bg(palette.color_surface_column.hsla()),
                )
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .flex_col()
                        .justify_between()
                        .p(px(metrics.spacing_1))
                        .child(
                            div()
                                .h(px(metrics.spacing_0_5))
                                .rounded(px(metrics.spacing_px))
                                .bg(palette.color_content.hsla()),
                        )
                        .child(
                            div()
                                .w(relative(0.6))
                                .h(px(metrics.spacing_1_5))
                                .rounded(px(metrics.spacing_px))
                                .bg(palette.color_primary.shade(500).hsla()),
                        ),
                );
            let button = div()
                .id(id.clone())
                .relative()
                .flex_none()
                .rounded(px(radius))
                .cursor_pointer()
                .track_focus(&focus.clone().tab_stop(true))
                .on_mouse_down(MouseButton::Left, |_, window, cx| pointer_down(window, cx))
                .when(chosen, |button| {
                    button.shadow(vec![
                        BoxShadow {
                            color: host
                                .control_color_outlined_border_normal
                                .resolve(Tint::Primary.ramp(host)),
                            offset: point(px(0.), px(0.)),
                            blur_radius: px(0.),
                            spread_radius: px(host.stroke_hairline + host.stroke_control),
                        },
                        BoxShadow {
                            color: host.color_surface.hsla(),
                            offset: point(px(0.), px(0.)),
                            blur_radius: px(0.),
                            spread_radius: px(host.stroke_hairline),
                        },
                    ])
                })
                .child(preview)
                .when(visible, |button| button.child(kit::outline(host, radius)));
            let state = own.clone();
            let change = self.on_value_change.clone();
            let value = option.value;
            buttons.push(
                Tooltip::new(part_id(&id, "tooltip"), option.label).bind_button(
                    button,
                    &focus,
                    press,
                    Some(Box::new(move |_, window, cx| {
                        state.update(cx, |own, cx| {
                            *own = Some(value.clone());
                            cx.notify();
                        });
                        if let Some(change) = &change {
                            change(value.clone(), window, cx);
                        }
                    })),
                    window,
                    cx,
                ),
            );
        }
        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .justify_end()
            .gap(px(host.spacing_1_5))
            .children(buttons)
    }
}

pub(super) fn is_swatches(element: &mut AnyElement) -> bool {
    if element.downcast_mut::<AnyElement>().is_some() {
        return is_swatches(element.downcast_mut::<AnyElement>().unwrap());
    }
    if element.transparent_child_mut().is_some() {
        return is_swatches(element.transparent_child_mut().unwrap());
    }
    element
        .downcast_mut::<Component<PreferenceThemeSwatches>>()
        .is_some()
}
