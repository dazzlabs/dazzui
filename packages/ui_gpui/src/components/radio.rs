use gpui::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{
    ActionListener, ControlPart, ControlState, ControlVariant, SharedValueListener, Tint,
    ValueListener, colors, focus_ring, pressable,
};

/// The three radio sizes. The ring and its dot keep their own per-size
/// dimensions, which is what `radio.<size>.box` and `radio.<size>.dot` are
/// for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// One choice.
///
/// The row, the label and the disabled treatment are the checkbox's — what
/// differs is the shape: a radio is a ring that keeps its outline when
/// selected and grows a dot inside it, where a checkbox fills solid.
#[derive(IntoElement)]
pub struct Radio {
    id: ElementId,
    label: SharedString,
    checked: bool,
    size: RadioSize,
    tint: Tint,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_select: Option<ActionListener>,
}

impl Radio {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, checked: bool) -> Self {
        Radio {
            id: id.into(),
            label: label.into(),
            checked,
            size: RadioSize::Medium,
            tint: Tint::Primary,
            disabled: false,
            focus_handle: None,
            on_select: None,
        }
    }

    pub fn size(mut self, size: RadioSize) -> Self {
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

    pub fn on_select(mut self, listener: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Radio {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Radio {
            id,
            label,
            checked,
            size,
            tint,
            disabled,
            focus_handle,
            on_select,
        } = self;

        let (ring, dot) = match size {
            RadioSize::Small => (vars.radio_small_box, vars.radio_small_dot),
            RadioSize::Medium => (vars.radio_medium_box, vars.radio_medium_dot),
            RadioSize::Large => (vars.radio_large_box, vars.radio_large_dot),
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
                .items_center()
                .gap(vars.spacing_2_5)
                .type_style(vars.label_quiet)
                .when(checked, |el| {
                    el.text_color(vars.color_content)
                        .font_weight(vars.label_medium.weight)
                })
                .when(!checked, |el| el.text_color(vars.color_content_muted))
                .map(|el| {
                    if state.enabled {
                        el.cursor_pointer()
                    } else {
                        el.cursor_not_allowed().opacity(0.6)
                    }
                })
                .child(
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .size(ring)
                        .rounded(vars.radius_full)
                        .border(vars.radio_thickness)
                        .border_color(if checked {
                            filled
                        } else {
                            vars.color_border_muted
                        })
                        .bg(colors::TRANSPARENT)
                        .when(checked, |el| {
                            el.child(div().size(dot).rounded(vars.radius_full).bg(filled))
                        }),
                )
                .child(div().min_w_0().line_height(relative(1.0)).child(label))
                .when(focused, |el| el.child(focus_ring(vars, vars.radius_full)))
                .when_some(on_select.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(window, cx))
                })
                .into_any_element()
        })
    }
}

/// A column of radios, which is the shape they are always used in.
#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    options: Vec<(SharedString, SharedString, bool)>,
    selected: SharedString,
    size: RadioSize,
    tint: Tint,
    on_select: Option<ValueListener<SharedString>>,
}

impl RadioGroup {
    pub fn new(id: impl Into<ElementId>, selected: impl Into<SharedString>) -> Self {
        RadioGroup {
            id: id.into(),
            options: Vec::new(),
            selected: selected.into(),
            size: RadioSize::Medium,
            tint: Tint::Primary,
            on_select: None,
        }
    }

    /// One choice: the value it stands for and the label it prints.
    pub fn option(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        self.options.push((value.into(), label.into(), false));
        self
    }

    /// A choice that cannot be taken.
    pub fn disabled_option(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        self.options.push((value.into(), label.into(), true));
        self
    }

    pub fn size(mut self, size: RadioSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn on_select(
        mut self,
        listener: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let RadioGroup {
            id,
            options,
            selected,
            size,
            tint,
            on_select,
        } = self;
        let on_select: Option<SharedValueListener<SharedString>> = on_select.map(std::rc::Rc::from);

        // The group carries the id, so its rows are scoped under it and can
        // be numbered rather than named.
        div()
            .id(id)
            .flex()
            .flex_col()
            .gap(vars.spacing_2_5)
            .children(
                options
                    .into_iter()
                    .enumerate()
                    .map(|(index, (value, label, disabled))| {
                        let on_select = on_select.clone();
                        Radio::new(
                            ElementId::from(SharedString::from(format!("radio-{index}"))),
                            label,
                            value == selected,
                        )
                        .size(size)
                        .tint(tint)
                        .disabled(disabled)
                        .on_select(move |window, cx| {
                            if let Some(listener) = on_select.as_ref() {
                                listener(&value, window, cx)
                            }
                        })
                    }),
            )
    }
}
