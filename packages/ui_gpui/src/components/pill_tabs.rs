use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{
    ControlPart, ControlState, ControlVariant, SharedValueListener, Tint, ValueListener, pressable,
};

/// Free-standing filter pills.
///
/// Unlike a segmented control there is no track: each pill stands on the
/// page, so an unselected one needs its own fill to be a target at all — the
/// inset step, one deeper than the cards around it, with the groove as its
/// hover.
///
/// One size, drawn tight: the tiny control height on 11px type. The active
/// pill jumps two weight steps as well as filling, because at this size the
/// fill alone can vanish under a thumb.
#[derive(IntoElement)]
pub struct PillTabs {
    id: ElementId,
    items: Vec<(SharedString, SharedString, Option<usize>, bool)>,
    selected: SharedString,
    tint: Tint,
    on_select: Option<ValueListener<SharedString>>,
}

impl PillTabs {
    pub fn new(id: impl Into<ElementId>, selected: impl Into<SharedString>) -> Self {
        PillTabs {
            id: id.into(),
            items: Vec::new(),
            selected: selected.into(),
            tint: Tint::Primary,
            on_select: None,
        }
    }

    pub fn item(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.items.push((value.into(), label.into(), None, false));
        self
    }

    /// A tab with a trailing count, set inside the pill.
    pub fn counted_item(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
        count: usize,
    ) -> Self {
        self.items
            .push((value.into(), label.into(), Some(count), false));
        self
    }

    /// Any tab: an optional count, and whether it can be taken.
    pub fn entry(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
        count: Option<usize>,
        disabled: bool,
    ) -> Self {
        self.items
            .push((value.into(), label.into(), count, disabled));
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

impl RenderOnce for PillTabs {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let filled_surface = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Surface,
            self.tint,
            ControlState::Normal,
        );
        let filled_content = vars.control_color(
            ControlVariant::Filled,
            ControlPart::Content,
            self.tint,
            ControlState::Normal,
        );

        let PillTabs {
            id,
            items,
            selected,
            on_select,
            ..
        } = self;
        let on_select: Option<SharedValueListener<SharedString>> = on_select.map(std::rc::Rc::from);

        div()
            .id(id)
            .flex()
            .items_center()
            .gap(vars.spacing_2)
            .children(items.into_iter().enumerate().map(
                |(index, (value, label, count, disabled))| {
                    let active = value == selected;
                    let on_select = on_select.clone();

                    pressable(
                        SharedString::from(format!("pill-{index}")),
                        !disabled,
                        move |state, _, _| {
                            div()
                                .id(SharedString::from(format!("pill-body-{index}")))
                                .flex()
                                .items_center()
                                .gap(vars.control_tiny_gap)
                                .h(vars.control_tiny_size)
                                .px(vars.spacing_3)
                                .rounded(vars.radius_full)
                                .type_style(vars.label_small.weight(if active {
                                    vars.label_strong.weight
                                } else {
                                    vars.label_small.weight
                                }))
                                .line_height(relative(1.0))
                                .map(|el| {
                                    if active {
                                        el.bg(filled_surface).text_color(filled_content)
                                    } else if state.hovered {
                                        el.bg(vars.color_surface_sunken)
                                            .text_color(vars.color_content)
                                    } else {
                                        el.bg(vars.color_surface_inset)
                                            .text_color(vars.color_content_nav)
                                    }
                                })
                                .map(|el| {
                                    if disabled {
                                        el.cursor_not_allowed().opacity(0.5)
                                    } else {
                                        el.cursor_pointer()
                                    }
                                })
                                .child(label.clone())
                                // The count runs on in the label's own ink — one
                                // label, not a label wearing a badge.
                                .when_some(count, |el, count| el.child(count.to_string()))
                                .when_some(
                                    on_select.clone().filter(|_| !disabled),
                                    |el, listener| {
                                        let value = value.clone();
                                        el.on_click(move |_, window, cx| {
                                            listener(&value, window, cx)
                                        })
                                    },
                                )
                                .into_any_element()
                        },
                    )
                },
            ))
    }
}
