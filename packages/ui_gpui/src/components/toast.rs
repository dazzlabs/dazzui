use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, Tint, colors, hairline, pressable,
};

/// A transient receipt for something that happened out of view.
///
/// One line, no wrapping — anything needing a second sentence belongs in an
/// [`Callout`](crate::Callout) in the flow instead.
///
/// Lifetime belongs to the host — the system stays stateless — but the
/// contract worth keeping is: a few seconds for a plain notice, longer when
/// it carries an action, and a failure stays until it is dismissed.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    message: SharedString,
    tint: Tint,
    /// Leading node — a glyph, or a `Spinner` for work still in flight.
    icon: Option<AnyElement>,
    /// Trailing affordance — undo, retry.
    action: Option<AnyElement>,
    /// Accessible name of the dismiss button.
    dismiss_label: SharedString,
    on_dismiss: Option<ActionListener>,
}

impl Toast {
    pub fn new(id: impl Into<ElementId>, message: impl Into<SharedString>) -> Self {
        Toast {
            id: id.into(),
            message: message.into(),
            tint: Tint::Neutral,
            icon: None,
            action: None,
            dismiss_label: "Dismiss".into(),
            on_dismiss: None,
        }
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn dismiss_label(mut self, label: impl Into<SharedString>) -> Self {
        self.dismiss_label = label.into();
        self
    }

    /// Renders the dismiss button; wire it to remove the toast.
    pub fn on_dismiss(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_dismiss = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Toast {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Toast {
            id,
            message,
            tint,
            icon,
            action,
            on_dismiss,
            ..
        } = self;
        let wash = vars.control_color(
            ControlVariant::Normal,
            ControlPart::Surface,
            Tint::Neutral,
            ControlState::Hovered,
        );

        div()
            .flex()
            .items_center()
            .gap(vars.spacing_2_5)
            .min_h(vars.spacing_9)
            // Sized to its content but never past the viewport, so the stack
            // needs no magic width and a long message truncates instead of
            // pushing wide.
            .max_w(vars.toast_max_width)
            .py(vars.spacing_1_5)
            // Asymmetric on purpose: the text gets breathing room, the
            // dismiss button sits tight in the corner it lives in.
            .pl(vars.spacing_3)
            .pr(vars.spacing_1_5)
            .rounded(vars.radius_big)
            .bg(vars.color_surface)
            .border(hairline(vars, window))
            .border_color(vars.color_border_strong)
            .token_shadow(vars.shadow_md)
            .when_some(icon, |el, icon| {
                el.child(
                    div()
                        .flex()
                        .flex_none()
                        .text_color(vars.ramp(tint).shade(600))
                        .child(icon),
                )
            })
            .child(
                div()
                    .min_w_0()
                    .overflow_hidden()
                    .py(vars.spacing_0_5)
                    .type_style(vars.label_medium)
                    .line_height(relative(1.4))
                    .text_color(vars.color_content)
                    .child(message),
            )
            .when_some(action, |el, action| {
                el.child(div().flex_none().ml_auto().child(action))
            })
            .when_some(on_dismiss, |el, listener| {
                el.child(pressable(id.clone(), true, move |state, _, _| {
                    div()
                        .id(id.clone())
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .size(vars.control_tiny_size)
                        .rounded(vars.radius_small)
                        .cursor_pointer()
                        .bg(if state.hovered {
                            wash
                        } else {
                            colors::TRANSPARENT
                        })
                        .text_color(if state.hovered {
                            vars.color_content
                        } else {
                            vars.color_content_subtle
                        })
                        .child(Icon::new(icons::DISMISS_12_REGULAR).size(vars.spacing_3))
                        .on_click(move |_, window, cx| listener(window, cx))
                        .into_any_element()
                }))
            })
    }
}

/// Which edge a stack of toasts hangs from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPlacement {
    #[default]
    Bottom,
    Top,
}

/// Where a window's toasts land.
///
/// Transparent to the pointer between them, so the page underneath stays
/// usable while one is on screen. Column direction keeps the newest toast
/// nearest the edge it hangs from.
#[derive(IntoElement)]
pub struct ToastViewport {
    placement: ToastPlacement,
    children: Vec<AnyElement>,
}

impl ToastViewport {
    pub fn new() -> Self {
        ToastViewport {
            placement: ToastPlacement::Bottom,
            children: Vec::new(),
        }
    }

    pub fn placement(mut self, placement: ToastPlacement) -> Self {
        self.placement = placement;
        self
    }
}

impl Default for ToastViewport {
    fn default() -> Self {
        ToastViewport::new()
    }
}

impl ParentElement for ToastViewport {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToastViewport {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;

        div()
            .absolute()
            .left_0()
            .right_0()
            .map(|el| match self.placement {
                ToastPlacement::Bottom => el.bottom_0().flex().flex_col(),
                ToastPlacement::Top => el.top_0().flex().flex_col_reverse(),
            })
            .items_center()
            .gap(vars.spacing_2)
            .p(vars.spacing_4)
            .children(self.children)
    }
}
