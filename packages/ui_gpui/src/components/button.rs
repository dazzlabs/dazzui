use gpui::{
    App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
};

use crate::components::Icon;
use crate::prelude::*;
use crate::{ClickListener, ControlSize, ControlVariant, Tint, focus_ring, pressable};

/// A button.
///
/// Colour comes entirely from the shared control state machine: a variant
/// picks the recipe, a tint picks the ramp it resolves against, and hover,
/// press and disabled follow on their own. So this component is only the box.
///
/// The label is a string and the leading glyph an [`Icon`], where React takes
/// arbitrary children: a button's label is a word, and an element tree in the
/// place of one is a composition the caller can build out of a `div`.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    size: ControlSize,
    variant: ControlVariant,
    tint: Tint,
    full_width: bool,
    /// Trailing hint — the key that also triggers this action.
    shortcut: Option<SharedString>,
    icon: Option<Icon>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_click: Option<ClickListener>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Button {
            id: id.into(),
            label: label.into(),
            size: ControlSize::Small,
            variant: ControlVariant::Recessed,
            tint: Tint::Primary,
            full_width: false,
            shortcut: None,
            icon: None,
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

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// A leading glyph, set at the profile's own type size.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draw the focus ring while this handle is focused.
    ///
    /// Keyboard focus in GPUI belongs to the view that owns the handle rather
    /// than to the element, so a button is told about it instead of keeping
    /// one — the one place this differs from `:focus-visible` doing it alone.
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

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size);
        let Button {
            id,
            label,
            variant,
            tint,
            full_width,
            shortcut,
            icon,
            disabled,
            focus_handle,
            on_click,
            ..
        } = self;

        pressable(id.clone(), !disabled, move |state, window, _| {
            let colors = vars.control_colors(variant, tint, state.control_state());
            let focused = focus_handle
                .as_ref()
                .is_some_and(|handle| handle.is_focused(window));

            // The one action a view points at also carries the extra weight,
            // and gives it up when it flattens.
            let face = if variant == ControlVariant::Filled && state.enabled {
                metrics.content.weight(vars.label_strong.weight)
            } else {
                metrics.content
            };

            div()
                .id(id)
                .relative()
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .h(metrics.size)
                .gap(metrics.gap)
                .px(metrics.padding_inline)
                .py(metrics.padding_block)
                .rounded(metrics.radius)
                .border(vars.spacing_px)
                .when(full_width, |el| el.w_full())
                .type_style(face)
                .control_colors(colors)
                .map(|el| {
                    if state.enabled {
                        el.cursor_pointer()
                    } else {
                        el.cursor_not_allowed()
                    }
                })
                // The one action a view points at carries the hairline lift —
                // flat like every AppKit button, but separable from a tinted
                // surface by that one pixel of shadow. It goes when the
                // button is disabled and flattens.
                .when(variant == ControlVariant::Filled && state.enabled, |el| {
                    el.token_shadow(vars.shadow_2xs)
                })
                .when_some(icon, |el, icon| {
                    el.child(icon.size(metrics.content.size).color(colors.content))
                })
                .child(label)
                .when_some(shortcut, |el, shortcut| {
                    // A trailing shortcut is a hint, not part of the label —
                    // it takes the display cut and steps back.
                    el.child(
                        div()
                            .font_family(vars.label_strong.font_family())
                            .opacity(0.7)
                            .child(shortcut),
                    )
                })
                .when(focused, |el| el.child(focus_ring(vars, metrics.radius)))
                .when_some(on_click.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |event, window, cx| listener(event, window, cx))
                })
                .into_any_element()
        })
    }
}
