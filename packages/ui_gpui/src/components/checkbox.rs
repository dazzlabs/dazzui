use gpui::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::assets::icons;
use crate::components::Icon;
use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, Tint, ValueListener, colors, focus_ring, pressable,
};

/// The three checkbox sizes. Only the gap comes off the control profile; the
/// box keeps its own per-size square, which is what `checkbox.<size>.box` is
/// for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// A box and a label on one row.
///
/// The box defines the row — there is no minimum height, so a stacked
/// checklist sits at the density the boxes set — and the label carries the
/// selection signal twice over: it rests quiet (one ink step back) and takes
/// the full ink and label weight only when its box is checked. A checked row
/// you cannot pick out of the list by its label alone is not signalling.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    /// Neither on nor off — the state a parent arrives at when only some of
    /// its children are checked. A lone checkbox can never reach it on its
    /// own, which is why it is a property rather than a third value.
    indeterminate: bool,
    size: CheckboxSize,
    tint: Tint,
    /// A de-emphasised aside on the label — a count, a size.
    note: Option<SharedString>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_change: Option<ValueListener<bool>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, checked: bool) -> Self {
        Checkbox {
            id: id.into(),
            label: label.into(),
            checked,
            indeterminate: false,
            size: CheckboxSize::Medium,
            tint: Tint::Primary,
            note: None,
            disabled: false,
            focus_handle: None,
            on_change: None,
        }
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn note(mut self, note: impl Into<SharedString>) -> Self {
        self.note = Some(note.into());
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

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Checkbox {
            id,
            label,
            checked,
            indeterminate,
            size,
            tint,
            note,
            disabled,
            focus_handle,
            on_change,
        } = self;

        let box_size = match size {
            CheckboxSize::Small => vars.checkbox_small_box,
            CheckboxSize::Medium => vars.checkbox_medium_box,
            CheckboxSize::Large => vars.checkbox_large_box,
        };
        let filled = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Surface,
            tint,
            ControlState::Normal,
        );
        let on_filled = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Content,
            tint,
            ControlState::Normal,
        );

        // Mixed fills like checked: a parent that is partly on is on. What
        // separates the two is the glyph, not the box.
        let marked = checked || indeterminate;

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
                // A mixed parent does not take the label weight: it is not an
                // answer yet.
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
                        .size(box_size)
                        .rounded(vars.checkbox_radius)
                        // Neutral while empty: an unchecked box is an
                        // outline, not a quiet accent.
                        .border(vars.checkbox_thickness)
                        .border_color(if marked {
                            filled
                        } else {
                            vars.color_border_muted
                        })
                        .bg(if marked { filled } else { colors::TRANSPARENT })
                        .text_color(on_filled)
                        .when(marked, |el| {
                            // A tick says every one; a dash says some. The
                            // glyph sits a step under the label so it clears
                            // the box's corners.
                            el.child(
                                Icon::new(if indeterminate {
                                    icons::SUBTRACT_12_FILLED
                                } else {
                                    icons::CHECKMARK_12_FILLED
                                })
                                .size(vars.label_small.size)
                                .color(on_filled),
                            )
                        }),
                )
                .child(
                    div()
                        .flex()
                        .min_w_0()
                        .line_height(relative(1.0))
                        .child(label)
                        // A trailing note rides on the label rather than the
                        // row, so it wraps with it instead of being pushed to
                        // the far edge like a control would be.
                        .when_some(note, |el, note| {
                            el.child(
                                div()
                                    .pl(vars.spacing_1)
                                    .text_color(vars.color_content_faint)
                                    .child(note),
                            )
                        }),
                )
                .when(focused, |el| {
                    el.child(focus_ring(vars, vars.checkbox_radius))
                })
                .when_some(on_change.filter(|_| state.enabled), |el, listener| {
                    el.on_click(move |_, window, cx| listener(&!checked, window, cx))
                })
                .into_any_element()
        })
    }
}
