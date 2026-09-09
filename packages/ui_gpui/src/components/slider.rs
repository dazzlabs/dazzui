use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, Bounds, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, Pixels, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled,
    Window, canvas, div, px, relative,
};

use crate::prelude::*;
use crate::{
    ControlPart, ControlSize, ControlState, ControlVariant, SharedValueListener, Tint,
    ValueListener, half,
};

/// What a drag on a slider carries. GPUI's drag events hand back the bounds of
/// the element the drag started on, which is the only way an element with no
/// state of its own can turn a pointer position into a fraction.
#[derive(Clone, Debug)]
struct SliderDrag;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SliderSize {
    fn control(self) -> ControlSize {
        match self {
            SliderSize::Small => ControlSize::Small,
            SliderSize::Medium => ControlSize::Medium,
            SliderSize::Large => ControlSize::Large,
        }
    }
}

/// A value picked along a range.
///
/// A range is the same component: pass more than one value and it draws a
/// thumb per value. There is no second component, because everything but the
/// number of thumbs is identical.
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    /// One entry per thumb, in order, and never empty — a slider with no value
    /// is a groove.
    values: Vec<f32>,
    min: f32,
    max: f32,
    /// What one arrow key moves.
    step: f32,
    size: SliderSize,
    tint: Tint,
    /// Printed at the trailing edge. A slider whose value the reader has to be
    /// able to say — a font size, a threshold — needs it; one they only have
    /// to feel — a volume — does not.
    value_label: Option<SharedString>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_change: Option<ValueListener<Vec<f32>>>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, value: f32) -> Self {
        Slider {
            id: id.into(),
            values: vec![value],
            min: 0.,
            max: 100.,
            step: 1.,
            size: SliderSize::Medium,
            tint: Tint::Primary,
            value_label: None,
            disabled: false,
            focus_handle: None,
            on_change: None,
        }
    }

    /// A range: one thumb per value.
    pub fn range(id: impl Into<ElementId>, values: Vec<f32>) -> Self {
        let mut slider = Slider::new(id, 0.);
        slider.values = values;
        slider
    }

    pub fn bounds(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn size(mut self, size: SliderSize) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    pub fn value_label(mut self, label: impl Into<SharedString>) -> Self {
        self.value_label = Some(label.into());
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

    pub fn on_change(
        mut self,
        listener: impl Fn(&Vec<f32>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(listener));
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let vars = cx.theme().vars;
        let Slider {
            id,
            values,
            min,
            max,
            step,
            size,
            tint,
            value_label,
            disabled,
            focus_handle,
            on_change,
        } = self;

        let track = match size {
            SliderSize::Small => vars.slider_small_track,
            SliderSize::Medium => vars.slider_medium_track,
            SliderSize::Large => vars.slider_large_track,
        };
        let thumb = match size {
            SliderSize::Small => vars.slider_small_thumb,
            SliderSize::Medium => vars.slider_medium_thumb,
            SliderSize::Large => vars.slider_large_thumb,
        };
        // The row's height comes off the control profile, so a slider in a
        // form sits on the same rhythm as the fields around it.
        let height = vars.metrics(size.control()).size;
        let accent = vars.ramp(tint).shade(600);

        // Disabled flattens to the groove, like every other control: no lift,
        // no accent, one dead look.
        let fill = if disabled {
            vars.control_color(
                ControlVariant::Normal,
                ControlPart::Content,
                tint,
                ControlState::Disabled,
            )
        } else {
            accent
        };

        let span = max - min;
        // `move`, because the key handler outlives this frame: the closure has
        // to own the three numbers rather than borrow them.
        let fraction = move |value: f32| {
            if span <= 0. {
                0.
            } else {
                ((value - min) / span).clamp(0., 1.)
            }
        };
        let start = if values.len() > 1 {
            fraction(values.first().copied().unwrap_or(min))
        } else {
            0.
        };
        let end = fraction(values.last().copied().unwrap_or(min));

        let on_change: Option<SharedValueListener<Vec<f32>>> = on_change.map(Rc::from);

        // The control's bounds, captured by a canvas that paints nothing.
        // GPUI hands bounds to a drag but not to a click, and a press on the
        // track has to jump the nearest thumb — which is a position the
        // element cannot otherwise know.
        let measured: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));

        let commit = {
            let on_change = on_change.clone();
            let values = values.clone();
            move |x: Pixels, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App| {
                let Some(listener) = on_change.as_ref() else {
                    return;
                };
                let width = f32::from(bounds.size.width);
                if width <= 0. {
                    return;
                }
                let ratio = ((f32::from(x - bounds.left())) / width).clamp(0., 1.);
                let raw = min + ratio * span;
                let snapped = if step > 0. {
                    min + ((raw - min) / step).round() * step
                } else {
                    raw
                };

                // Which thumb the pointer took hold of. A range whose thumbs
                // sit on the same value is the case this exists for.
                let mut next = values.clone();
                let index = next
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        (**a - snapped)
                            .abs()
                            .partial_cmp(&(**b - snapped).abs())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(index, _)| index)
                    .unwrap_or(0);
                // A range's thumbs cannot pass each other, which is the least
                // surprising of the three behaviours a range can have.
                let low = if index == 0 { min } else { next[index - 1] };
                let high = if index + 1 == next.len() {
                    max
                } else {
                    next[index + 1]
                };
                next[index] = snapped.clamp(low, high);
                listener(&next, window, cx);
            }
        };

        let control = div()
            .id(id)
            .relative()
            .flex_1()
            .h(height)
            .when(!disabled, |el| el.cursor_pointer())
            .child(
                canvas(
                    {
                        let measured = measured.clone();
                        move |bounds, _, _| measured.set(bounds)
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            // The groove, and the filled run between the thumbs. Flat tint
            // rather than the progress gradient: a gradient reads as travel,
            // and a slider is not going anywhere on its own.
            .child(
                div()
                    .absolute()
                    .top(half(height - track))
                    .left_0()
                    .right_0()
                    .h(track)
                    .overflow_hidden()
                    .rounded(half(track))
                    .bg(vars.color_surface_sunken)
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .bottom_0()
                            .left(relative(start))
                            .w(relative(end - start))
                            .bg(fill),
                    ),
            )
            .children(values.iter().map(|value| {
                // The thumb is the raised control — paper, a hairline edge and
                // the small lift a `filled` button carries — so it reads as
                // sitting on the groove rather than in it. Its centre sits on
                // the control's edge at either end, so half of it hangs
                // outside the track's box.
                div()
                    .absolute()
                    .top(half(height - thumb))
                    .left(relative(fraction(*value)))
                    .ml(-half(thumb))
                    .size(thumb)
                    .rounded(vars.radius_full)
                    .border(vars.stroke_hairline)
                    .map(|el| {
                        if disabled {
                            el.bg(vars.control_color(
                                ControlVariant::Normal,
                                ControlPart::Surface,
                                tint,
                                ControlState::Disabled,
                            ))
                            .border_color(vars.color_border)
                        } else {
                            el.bg(vars.color_surface)
                                .border_color(vars.color_border_strong)
                                .token_shadow(vars.shadow_xs)
                                .hover(|style| style.border_color(accent))
                        }
                    })
            }))
            .when(!disabled, |el| {
                el.on_drag(SliderDrag, |_, _, _, cx| cx.new(|_| gpui::Empty))
                    .on_drag_move::<SliderDrag>({
                        let commit = commit.clone();
                        move |event, window, cx| {
                            commit(event.event.position.x, event.bounds, window, cx);
                            window.refresh();
                        }
                    })
                    .on_click({
                        let commit = commit.clone();
                        let measured = measured.clone();
                        move |event, window, cx| {
                            commit(event.position().x, measured.get(), window, cx);
                            window.refresh();
                        }
                    })
            })
            .when_some(focus_handle.filter(|_| !disabled), |el, handle| {
                let commit = commit.clone();
                let measured = measured.clone();
                el.track_focus(&handle)
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let bounds = measured.get();
                        let width = f32::from(bounds.size.width);
                        if width <= 0. || span <= 0. {
                            return;
                        }
                        // The keys move the value, and the value is turned
                        // back into a position so one path commits it: the
                        // clamp, the snap and the thumb choice are all there.
                        let current = values.last().copied().unwrap_or(min);
                        let delta = match event.keystroke.key.as_str() {
                            "right" | "up" => step,
                            "left" | "down" => -step,
                            "home" => min - current,
                            "end" => max - current,
                            _ => return,
                        };
                        let at = bounds.left() + px(fraction(current + delta) * width);
                        commit(at, bounds, window, cx);
                        window.refresh();
                    })
            });

        div()
            .flex()
            .items_center()
            .w_full()
            .gap(vars.spacing_3)
            .child(control)
            .when_some(value_label, |el, label| {
                el.child(
                    div()
                        .flex_none()
                        .min_w(vars.spacing_10)
                        .text_right()
                        // A reading rather than a label: the mono face, so the
                        // row does not reflow as the digits change under the
                        // pointer.
                        .font_family("ui-monospace")
                        .text_size(vars.label_small.size)
                        .line_height(relative(1.0))
                        .text_color(vars.color_content_muted)
                        .child(label),
                )
            })
    }
}
