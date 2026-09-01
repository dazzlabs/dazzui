use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, relative,
};

use crate::prelude::*;
use crate::{
    ControlPart, ControlSize, ControlState, ControlVariant, SharedValueListener, Tint,
    ValueListener, pressable,
};

/// How the picked segment is picked out.
///
/// `raised` lifts a lighter capsule off the track, the way a desktop
/// segmented control draws it. `filled` paints it with the tint — louder, and
/// worth reserving for a choice that has to shout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedActiveStyle {
    #[default]
    Raised,
    Filled,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedControlSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SegmentedControlSize {
    fn control(self) -> ControlSize {
        match self {
            SegmentedControlSize::Small => ControlSize::Small,
            SegmentedControlSize::Medium => ControlSize::Medium,
            SegmentedControlSize::Large => ControlSize::Large,
        }
    }
}

/// A recessed track with one segment picked out.
///
/// The nesting is derived, not restated: the track takes the control corner,
/// the capsule sits `segmented_control_inset` inside it, and its corner is
/// the track's minus the inset — the only relationship that keeps the two
/// curves concentric. The segment's height is likewise the control height
/// minus the inset on both sides, so a segmented control lines up with the
/// buttons beside it.
#[derive(IntoElement)]
pub struct SegmentedControl {
    id: ElementId,
    items: Vec<(SharedString, SharedString, bool)>,
    selected: SharedString,
    size: SegmentedControlSize,
    tint: Tint,
    active_style: SegmentedActiveStyle,
    /// Fill the available width, splitting it evenly between the segments.
    stretch: bool,
    on_select: Option<ValueListener<SharedString>>,
}

impl SegmentedControl {
    pub fn new(id: impl Into<ElementId>, selected: impl Into<SharedString>) -> Self {
        SegmentedControl {
            id: id.into(),
            items: Vec::new(),
            selected: selected.into(),
            size: SegmentedControlSize::Medium,
            tint: Tint::Primary,
            active_style: SegmentedActiveStyle::Raised,
            stretch: false,
            on_select: None,
        }
    }

    pub fn item(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.items.push((value.into(), label.into(), false));
        self
    }

    pub fn disabled_item(
        mut self,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        self.items.push((value.into(), label.into(), true));
        self
    }

    pub fn size(mut self, size: SegmentedControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn active_style(mut self, style: SegmentedActiveStyle) -> Self {
        self.active_style = style;
        self
    }

    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
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

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let metrics = vars.metrics(self.size.control());
        let inset = vars.segmented_control_inset;
        // The medium track takes the regular control corner — one step above
        // the button beside it, because the track is a container for capsules
        // rather than a control face itself.
        let track_radius = match self.size {
            SegmentedControlSize::Medium => vars.radius_medium,
            _ => metrics.radius,
        };
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

        let SegmentedControl {
            id,
            items,
            selected,
            active_style,
            stretch,
            on_select,
            ..
        } = self;
        let on_select: Option<SharedValueListener<SharedString>> = on_select.map(std::rc::Rc::from);

        div()
            .id(id)
            .flex()
            .when(!stretch, |el| el.flex_none())
            .when(stretch, |el| el.w_full())
            .p(inset)
            .rounded(track_radius)
            .bg(vars.color_surface_inset)
            .children(
                items
                    .into_iter()
                    .enumerate()
                    .map(|(index, (value, label, disabled))| {
                        let active = value == selected;
                        let on_select = on_select.clone();

                        pressable(
                            SharedString::from(format!("segment-{index}")),
                            !disabled && !active,
                            move |state, _, _| {
                                div()
                                    .id(SharedString::from(format!("segment-body-{index}")))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .when(stretch, |el| el.flex_1())
                                    .h(metrics.size - inset * 2.0)
                                    .px(vars.spacing_3)
                                    .rounded(track_radius - inset)
                                    .type_style(vars.label_quiet.size(metrics.content.size).weight(
                                        if active {
                                            // Selection is a weight shift as much
                                            // as a fill.
                                            vars.label_medium.weight
                                        } else {
                                            vars.label_quiet.weight
                                        },
                                    ))
                                    .line_height(relative(1.0))
                                    .map(|el| match (active, active_style) {
                                        (true, SegmentedActiveStyle::Raised) => el
                                            .bg(vars.color_surface_raised)
                                            .text_color(vars.color_content),
                                        (true, SegmentedActiveStyle::Filled) => {
                                            el.bg(filled_surface).text_color(filled_content)
                                        }
                                        (false, _) => el.text_color(if state.hovered {
                                            vars.color_content
                                        } else {
                                            vars.color_content_secondary
                                        }),
                                    })
                                    // Both active styles lift with the hairline
                                    // shadow.
                                    .when(active, |el| el.token_shadow(vars.shadow_2xs))
                                    .map(|el| {
                                        if disabled {
                                            el.cursor_not_allowed().opacity(0.5)
                                        } else {
                                            el.cursor_pointer()
                                        }
                                    })
                                    .when_some(
                                        on_select.clone().filter(|_| !disabled && !active),
                                        |el, listener| {
                                            let value = value.clone();
                                            el.on_click(move |_, window, cx| {
                                                listener(&value, window, cx)
                                            })
                                        },
                                    )
                                    .child(label.clone())
                                    .into_any_element()
                            },
                        )
                    }),
            )
    }
}
