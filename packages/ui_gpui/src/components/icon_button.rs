use gpui::{
    App, ClickEvent, ElementId, FocusHandle, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::components::Icon;
use crate::components::tooltip::Tooltip;
use crate::prelude::*;
use crate::{ClickListener, ControlSize, ControlVariant, Tint, colors, focus_ring, pressable};

/// The glyph a square button draws, whatever the profile's own type size.
const GLYPH_SIZE: gpui::Pixels = px(16.);

/// A square button.
///
/// Colour comes from the shared control state machine, but the default
/// posture — plain variant, neutral tint — carries its own quiet chrome: a
/// subtle-ink glyph that washes grey and darkens to full ink under the
/// pointer, and turns accent when held on. A toolbar row of these has to read
/// as chrome until touched, and the recipes' neutral ramp is too flat a grey
/// for that.
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: Icon,
    /// Accessible name, also surfaced as the tooltip.
    label: SharedString,
    size: ControlSize,
    variant: ControlVariant,
    tint: Tint,
    /// A persistent on-state — a pin held down, a panel kept open.
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_click: Option<ClickListener>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, icon: Icon) -> Self {
        IconButton {
            id: id.into(),
            icon,
            label: label.into(),
            size: ControlSize::Tiny,
            variant: ControlVariant::Plain,
            // Neutral by default: a toolbar of these should read as chrome,
            // not as a row of accented actions. Pass a tint where one button
            // genuinely leads.
            tint: Tint::Neutral,
            disabled: false,
            focus_handle: None,
            on_click: None,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: ControlVariant) -> Self {
        self.variant = variant;
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

    pub fn focus_handle(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn on_click(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size);
        let IconButton {
            id,
            icon,
            label,
            variant,
            tint,
            disabled,
            focus_handle,
            on_click,
            ..
        } = self;

        // Is this the quiet chrome posture the toolbar wants?
        let chrome = variant == ControlVariant::Plain && tint == Tint::Neutral;

        pressable(id.clone(), !disabled, move |state, window, _| {
            let colors = vars.control_colors(variant, tint, state.control_state());
            let focused = focus_handle
                .as_ref()
                .is_some_and(|handle| handle.is_focused(window));

            let foreground: Hsla = if chrome && state.enabled {
                if state.hovered || state.pressed {
                    vars.color_content
                } else {
                    vars.color_content_subtle
                }
            } else {
                colors.content
            };

            let surface: Hsla = if chrome && state.enabled {
                if state.hovered || state.pressed {
                    vars.color_surface_subtle
                } else {
                    colors::TRANSPARENT
                }
            } else {
                colors.surface
            };

            div()
                .id(id)
                .relative()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .size(metrics.size)
                .rounded(metrics.radius)
                .border(vars.spacing_px)
                .bg(surface)
                .text_color(foreground)
                .border_color(colors.border)
                .map(|el| {
                    if state.enabled {
                        el.cursor_pointer()
                    } else {
                        el.cursor_not_allowed()
                    }
                })
                .tooltip({
                    // React surfaces the accessible name as the native
                    // `title` tooltip; GPUI's counterpart is a view.
                    let label = label.clone();
                    move |_, cx| Tooltip::view(label.clone(), cx)
                })
                .child(icon.size(GLYPH_SIZE).color(foreground))
                .when(focused, |el| el.child(focus_ring(vars, metrics.radius)))
                .when_some(on_click.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |event, window, cx| listener(event, window, cx))
                })
                .into_any_element()
        })
    }
}
