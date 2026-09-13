use gpui::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, Tint, focus_ring, hairline,
    pressable,
};

/// One choice in a radio group of cards — the shape a small set of mutually
/// exclusive choices takes when each needs a line of explanation and a radio
/// row would not carry it.
///
/// The type runs title-over-description in the strong/caption pair. The card
/// is one of the two places the hierarchy is carried by *both* size and
/// weight, because the two lines sit closer than anywhere else in the system.
#[derive(IntoElement)]
pub struct OptionCard {
    id: ElementId,
    title: SharedString,
    title_content: Option<AnyElement>,
    description: Option<SharedString>,
    selected: bool,
    tint: Tint,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_select: Option<ActionListener>,
}

impl OptionCard {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        OptionCard {
            id: id.into(),
            title: title.into(),
            title_content: None,
            description: None,
            selected: false,
            tint: Tint::Primary,
            disabled: false,
            focus_handle: None,
            on_select: None,
        }
    }

    /// A title that is more than a string — a name beside its badge.
    pub fn title_content(mut self, content: impl IntoElement) -> Self {
        self.title_content = Some(content.into_any_element());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
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

    pub fn focus_handle(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn on_select(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for OptionCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let rule = hairline(vars);
        let OptionCard {
            id,
            title,
            title_content,
            description,
            selected,
            tint,
            disabled,
            focus_handle,
            on_select,
        } = self;

        let chosen_border = vars.control_color(
            ControlVariant::Outlined,
            ControlPart::Border,
            tint,
            ControlState::Normal,
        );
        let disabled_surface = vars.control_color(
            ControlVariant::Normal,
            ControlPart::Surface,
            tint,
            ControlState::Disabled,
        );
        let disabled_content = vars.control_color(
            ControlVariant::Normal,
            ControlPart::Content,
            tint,
            ControlState::Disabled,
        );

        pressable(id.clone(), !disabled, move |state, window, _| {
            let focused = focus_handle
                .as_ref()
                .is_some_and(|handle| handle.is_focused(window));

            div()
                .id(id)
                .relative()
                .flex()
                .flex_1()
                .flex_col()
                .gap(vars.spacing_1_5)
                .py(vars.spacing_3_5)
                .px(vars.spacing_3)
                .rounded(vars.radius_large)
                .map(|el| {
                    if !state.enabled {
                        el.border(rule)
                            .border_color(vars.color_border)
                            .bg(disabled_surface)
                            .text_color(disabled_content)
                            .cursor_not_allowed()
                    } else if selected {
                        // Selected is the surface wash with the accent drawn
                        // around it at the control stroke — the border
                        // *thickens* as well as colours, which is what makes
                        // the chosen card survive a glance across its
                        // neighbours.
                        el.border(vars.stroke_control)
                            .border_color(chosen_border)
                            .bg(vars.wash(tint, vars.wash_surface))
                            .cursor_pointer()
                    } else {
                        el.border(rule)
                            .border_color(vars.color_border)
                            .bg(if state.hovered {
                                vars.color_surface_subtle
                            } else {
                                vars.color_surface_muted
                            })
                            .cursor_pointer()
                    }
                })
                .child(
                    div()
                        .type_style(vars.label_strong)
                        .text_color(if state.enabled {
                            vars.color_content
                        } else {
                            disabled_content
                        })
                        .map(|el| match title_content {
                            Some(content) => el.child(content),
                            None => el.child(title.clone()),
                        }),
                )
                .when_some(description.clone(), |el, description| {
                    el.child(
                        div()
                            .type_style(vars.caption_small.size(vars.label_small.size))
                            .line_height(relative(1.5))
                            .text_color(if !state.enabled {
                                disabled_content
                            } else if selected {
                                vars.color_content_muted
                            } else {
                                vars.color_content_subtle
                            })
                            .child(description),
                    )
                })
                .when(focused, |el| el.child(focus_ring(vars, vars.radius_large)))
                .when_some(on_select.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(window, cx))
                })
                .into_any_element()
        })
    }
}
