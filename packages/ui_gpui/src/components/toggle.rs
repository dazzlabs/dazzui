use gpui::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div,
};

use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, Tint, ValueListener, focus_ring, pressable,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A switch: the track carries the state, the thumb slides to the side that
/// says which.
///
/// Both ends are laid out with `justify-content` rather than a translate, so
/// the thumb lands on the padding whatever the track size is.
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    checked: bool,
    size: ToggleSize,
    tint: Tint,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_change: Option<ValueListener<bool>>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>, checked: bool) -> Self {
        Toggle {
            id: id.into(),
            checked,
            size: ToggleSize::Medium,
            tint: Tint::Primary,
            disabled: false,
            focus_handle: None,
            on_change: None,
        }
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
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

    pub fn on_change(mut self, listener: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Toggle {
            id,
            checked,
            size,
            tint,
            disabled,
            focus_handle,
            on_change,
        } = self;

        let (width, height, thumb) = match size {
            ToggleSize::Small => (
                vars.toggle_small_width,
                vars.toggle_small_height,
                vars.toggle_small_thumb,
            ),
            ToggleSize::Medium => (
                vars.toggle_medium_width,
                vars.toggle_medium_height,
                vars.toggle_medium_thumb,
            ),
            ToggleSize::Large => (
                vars.toggle_large_width,
                vars.toggle_large_height,
                vars.toggle_large_thumb,
            ),
        };
        let filled = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Surface,
            tint,
            ControlState::Normal,
        );

        pressable(id.clone(), !disabled, move |state, window, _| {
            let focused = focus_handle
                .as_ref()
                .is_some_and(|handle| handle.is_focused(window));

            div()
                .id(id)
                .relative()
                .flex()
                .flex_none()
                .items_center()
                .map(|el| {
                    if checked {
                        el.justify_end()
                    } else {
                        el.justify_start()
                    }
                })
                .w(width)
                .h(height)
                .p(vars.spacing_0_5)
                .rounded(vars.radius_full)
                .bg(if checked {
                    filled
                } else {
                    vars.color_surface_sunken
                })
                .map(|el| {
                    if state.enabled {
                        el.cursor_pointer()
                    } else {
                        el.cursor_not_allowed().opacity(0.6)
                    }
                })
                .child(
                    // The thumb changes colour with the state: faint ink
                    // while off — a grey thumb in a grey groove, deliberately
                    // low-contrast — and only brightens to the raised white
                    // once the track fills. `surface-raised` rather than the
                    // paper: on a dark canvas the paper is darker than the
                    // groove, and a thumb painted with it would sink.
                    div().size(thumb).rounded(vars.radius_full).bg(if checked {
                        vars.color_surface_raised
                    } else {
                        vars.color_content_faint
                    }),
                )
                .when(focused, |el| el.child(focus_ring(vars, vars.radius_full)))
                .when_some(on_change.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(&!checked, window, cx))
                })
                .into_any_element()
        })
    }
}
