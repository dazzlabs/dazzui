//! A value picked along a range — or a range, with a thumb per end.

use std::rc::Rc;

use gpui::{
    App, Bounds, ElementId, InteractiveElement, IntoElement, MouseButton, MouseMoveEvent,
    MouseUpEvent, ParentElement, Pixels, RenderOnce, SharedString, Styled, Window, canvas, div,
    fill, point, prelude::*, px, relative, size,
};

use crate::components::checkbox::kit;
use crate::foundation::{
    ControlColors, Size, State, StyledExt, Tint, Variant, focus_visible, pointer_down,
};
use crate::theme::ActiveTheme;
use crate::tokens::{Face, shadows};

type OnValueChange = Rc<dyn Fn(&[f64], &mut Window, &mut App) + 'static>;
type Commit = Rc<dyn Fn(usize, f64, Input, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy)]
enum Input {
    Pointer,
    Keyboard,
}

/// What a slider keeps between frames: its values when uncontrolled, the
/// thumb a drag holds, and where the track was last laid out.
struct SliderState {
    values: Vec<f64>,
    dragging: Option<usize>,
    track: Bounds<Pixels>,
    hovered: Option<usize>,
    grab_offset: Pixels,
}

/// A groove with a thumb on it. The row is as tall as the control profile
/// so a press anywhere along it lands; the groove and the thumb are the
/// component's own sizes. The arrow keys step, Shift or Page Up/Down step by
/// ten, Home and End go to the ends, and a thumb drags.
#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    size: Size,
    tint: Tint,
    value: Option<Vec<f64>>,
    default_value: Vec<f64>,
    min: f64,
    max: f64,
    step: f64,
    large_step: f64,
    show_value: bool,
    disabled: bool,
    on_value_change: Option<OnValueChange>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Slider {
            id: id.into(),
            size: Size::Medium,
            tint: Tint::Primary,
            value: None,
            default_value: vec![0.0],
            min: 0.0,
            max: 100.0,
            step: 1.0,
            large_step: 10.0,
            show_value: false,
            disabled: false,
            on_value_change: None,
        }
    }

    /// `small`, `medium` or `large`; `tiny` draws as `small`.
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn tint(mut self, tint: Tint) -> Self {
        self.tint = tint;
        self
    }

    /// Controlled: one value, or two for a range.
    pub fn value(mut self, value: impl IntoIterator<Item = f64>) -> Self {
        self.value = Some(value.into_iter().collect());
        self
    }

    /// Uncontrolled: where the thumbs start — one value, or two for a range.
    pub fn default_value(mut self, value: impl IntoIterator<Item = f64>) -> Self {
        self.default_value = value.into_iter().collect();
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn large_step(mut self, large_step: f64) -> Self {
        self.large_step = large_step;
        self
    }

    /// Prints the current value at the trailing edge of the row.
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The accessible name. GPUI has no accessibility tree to give it to.
    pub fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&[f64], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

/// A number as `Intl.NumberFormat` prints it by default.
fn format(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        let text = format!("{value:.3}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let state = window.use_keyed_state(kit::child_id(&self.id, "state"), cx, {
            let values = self.default_value.clone();
            move |_, _| SliderState {
                values,
                dragging: None,
                track: Bounds::default(),
                hovered: None,
                grab_offset: px(0.),
            }
        });
        let values = self
            .value
            .clone()
            .unwrap_or_else(|| state.read(cx).values.clone());
        if self.disabled
            || state
                .read(cx)
                .dragging
                .is_some_and(|index| index >= values.len())
        {
            state.update(cx, |state, _| state.dragging = None);
        }
        let dragging = state.read(cx).dragging;
        let hovered = state.read(cx).hovered;
        let handles = kit::focus_handles(&self.id, values.len(), window, cx);
        let (min, max, step) = (self.min, self.max, self.step);
        let span = (max - min).max(f64::EPSILON);
        let (height, track, thumb) = match self.size {
            Size::Tiny | Size::Small => (
                t.control_small_size,
                t.slider_small_track,
                t.slider_small_thumb,
            ),
            Size::Medium => (
                t.control_medium_size,
                t.slider_medium_track,
                t.slider_medium_thumb,
            ),
            Size::Large => (
                t.control_large_size,
                t.slider_large_track,
                t.slider_large_thumb,
            ),
        };
        let accent = self.tint.ramp(t).shade(600).hsla();
        let thumb_fill = t.color_content.mix(t.color_surface, t.slider_thumb_ink);
        let thumb_border = t
            .color_surface
            .mix(t.color_border_strong, t.slider_thumb_ink);
        let thumb_hot_border = t
            .color_surface
            .mix(self.tint.ramp(t).shade(600), t.slider_thumb_ink);
        let disabled_colors =
            ControlColors::resolve(t, Variant::Normal, self.tint, State::Disabled);

        // Keyboard changes stop at the neighbours. Base UI's default pointer
        // collision policy pushes them instead, retaining the active thumb.
        let commit: Commit = {
            let values = values.clone();
            let controlled = self.value.is_some();
            let state = state.clone();
            let handler = self.on_value_change.clone();
            Rc::new(move |index, value, input, window, cx| {
                let current = if controlled {
                    values.clone()
                } else {
                    state.read(cx).values.clone()
                };
                if index >= current.len() {
                    return;
                }
                let mut value = (((value - min) / step).round() * step + min).clamp(min, max);
                if matches!(input, Input::Keyboard) {
                    if index > 0 {
                        value = value.max(current[index - 1]);
                    }
                    if index + 1 < current.len() {
                        value = value.min(current[index + 1]);
                    }
                }
                let mut next = current.clone();
                next[index] = value;
                if matches!(input, Input::Pointer) {
                    for index in index + 1..next.len() {
                        next[index] = next[index].max(next[index - 1]);
                    }
                    for index in (0..index).rev() {
                        next[index] = next[index].min(next[index + 1]);
                    }
                }
                if current == next {
                    return;
                }
                if !controlled {
                    let next = next.clone();
                    state.update(cx, |state, cx| {
                        state.values = next;
                        cx.notify();
                    });
                }
                if let Some(handler) = &handler {
                    handler(&next, window, cx);
                }
            })
        };
        let value_at = move |x: Pixels, track: Bounds<Pixels>| {
            let width = f32::from(track.size.width).max(1.0) as f64;
            min + (f32::from(x - track.origin.x) as f64 / width) * span
        };
        let fraction = |value: f64| (((value - min) / span).clamp(0.0, 1.0)) as f32;

        let (start, end) = if values.len() > 1 {
            (fraction(values[0]), fraction(values[values.len() - 1]))
        } else {
            (0.0, values.first().copied().map(fraction).unwrap_or(0.0))
        };
        let indicator = div()
            .absolute()
            .top_0()
            .bottom_0()
            .left(relative(start))
            .w(relative(end - start))
            .rounded(px(track / 2.))
            .bg(if self.disabled {
                disabled_colors.content
            } else {
                accent
            });

        let thumbs = values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let focus = handles[index].clone();
                let visible = !self.disabled && focus_visible(&focus, window, cx);
                let hot = !self.disabled && (hovered == Some(index) || dragging == Some(index));
                let scale = if hot { t.slider_thumb_hot_scale } else { 1. };
                let disabled = self.disabled;
                let border = if disabled {
                    t.color_border.hsla()
                } else if hot {
                    thumb_hot_border
                } else {
                    thumb_border
                };
                let surface = if disabled {
                    disabled_colors.surface
                } else {
                    thumb_fill
                };
                let mut lift = shadows(t.shadow_xs);
                for shadow in &mut lift {
                    shadow.color.a *= t.slider_thumb_lift;
                }
                let mut knob = div()
                    .id(kit::child_id(&self.id, &format!("thumb-{index}")))
                    .debug_selector(move || format!("dz-slider-thumb-{index}").into())
                    .absolute()
                    .left(relative(fraction(*value)))
                    .ml(px(-thumb / 2.))
                    .top(px((track - thumb) / 2.))
                    .size(px(thumb))
                    .rounded(px(t.radius_full))
                    // Paint the scale around the centre without changing the
                    // thumb's layout, value position or keyboard focus target.
                    .child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, _, window, _| {
                                let edge = bounds.size.width * scale;
                                let bounds = Bounds::new(
                                    point(
                                        bounds.center().x - edge / 2.,
                                        bounds.center().y - edge / 2.,
                                    ),
                                    size(edge, edge),
                                );
                                if !disabled {
                                    window.paint_shadows(bounds, (edge / 2.).into(), &lift);
                                }
                                window.paint_quad(
                                    fill(bounds, surface)
                                        .corner_radii(edge / 2.)
                                        .border_widths(
                                            px(crate::foundation::border_width(
                                                t.slider_thumb_edge_width,
                                            )) * scale,
                                        )
                                        .border_color(border),
                                );
                            },
                        )
                        .size_full(),
                    )
                    .when(visible, |this| this.child(kit::outline(t, thumb / 2.)));
                if !self.disabled {
                    let commit = commit.clone();
                    let current = *value;
                    let large = self.large_step;
                    let hovered_state = state.clone();
                    let pressed_state = state.clone();
                    let pressed_handles = handles.clone();
                    let pressed_values = values.clone();
                    let pressed_focus = focus.clone();
                    knob = knob
                        .on_hover(move |hovered, _, cx| {
                            hovered_state.update(cx, |state, cx| {
                                if *hovered {
                                    state.hovered = Some(index);
                                } else if state.hovered == Some(index) {
                                    state.hovered = None;
                                }
                                cx.notify();
                            });
                        })
                        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                            pointer_down(window, cx);
                            // At the maximum, overlapping thumbs must be able
                            // to move back into the range from the first end.
                            let mut active = index;
                            if current == max {
                                while active > 0 && pressed_values[active - 1] == max {
                                    active -= 1;
                                }
                            }
                            window.focus(if active == index {
                                &pressed_focus
                            } else {
                                &pressed_handles[active]
                            });
                            pressed_state.update(cx, |state, cx| {
                                let center = state.track.left()
                                    + state.track.size.width * (((current - min) / span) as f32);
                                state.grab_offset = event.position.x - center;
                                state.dragging = Some(active);
                                cx.notify();
                            });
                            cx.stop_propagation();
                        })
                        .track_focus(&focus.tab_stop(true))
                        .on_key_down(move |event, window, cx| {
                            let keystroke = &event.keystroke;
                            let delta = if keystroke.modifiers.shift {
                                large
                            } else {
                                step
                            };
                            let next = match keystroke.key.as_str() {
                                "left" | "down" => current - delta,
                                "right" | "up" => current + delta,
                                "pageup" => current + large,
                                "pagedown" => current - large,
                                "home" => min,
                                "end" => max,
                                _ => return,
                            };
                            commit(index, next, Input::Keyboard, window, cx);
                            cx.stop_propagation();
                        });
                }
                knob
            })
            .collect::<Vec<_>>();

        let listen = !self.disabled;
        let groove = div()
            .relative()
            .w_full()
            .h(px(track))
            .rounded(px(track / 2.))
            .bg(t.color_surface_sunken.hsla())
            .child(
                canvas(
                    {
                        let state = state.clone();
                        move |bounds, _, cx| {
                            state.update(cx, |state, _| state.track = bounds);
                        }
                    },
                    {
                        let state = state.clone();
                        let commit = commit.clone();
                        move |_, _, window, _| {
                            if !listen {
                                return;
                            }
                            let moved = state.clone();
                            window.on_mouse_event(
                                move |event: &MouseMoveEvent, phase, window, cx| {
                                    if !phase.bubble() {
                                        return;
                                    }
                                    let Some(index) = moved.read(cx).dragging else {
                                        return;
                                    };
                                    if !event.dragging() {
                                        moved.update(cx, |state, cx| {
                                            state.dragging = None;
                                            cx.notify();
                                        });
                                        return;
                                    }
                                    let track = moved.read(cx).track;
                                    let x = event.position.x - moved.read(cx).grab_offset;
                                    commit(index, value_at(x, track), Input::Pointer, window, cx);
                                },
                            );
                            window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                                if phase.bubble() && state.read(cx).dragging.is_some() {
                                    state.update(cx, |state, cx| {
                                        state.dragging = None;
                                        cx.notify();
                                    });
                                }
                            });
                        }
                    },
                )
                .absolute()
                .size_full(),
            )
            .child(indicator)
            .children(thumbs);

        let mut control = div()
            .id(kit::child_id(&self.id, "control"))
            .flex()
            .items_center()
            .flex_1()
            .min_w_0()
            .h(px(height))
            .child(groove);
        if self.disabled {
            control = control.cursor_not_allowed();
        } else {
            let state = state.clone();
            let values = values.clone();
            control = control.cursor_pointer().on_mouse_down(
                MouseButton::Left,
                move |event, window, cx| {
                    pointer_down(window, cx);
                    let track = state.read(cx).track;
                    if f32::from(track.size.width) <= 0.0 {
                        return;
                    }
                    if values.is_empty() {
                        return;
                    }
                    let value = value_at(event.position.x, track);
                    // The nearest thumb; of two on one spot, the one the
                    // press is past.
                    let index = values
                        .iter()
                        .enumerate()
                        .rev()
                        .min_by(|(_, a), (_, b)| (*a - value).abs().total_cmp(&(*b - value).abs()))
                        .map(|(index, _)| index)
                        .unwrap_or(0);
                    window.focus(&handles[index]);
                    state.update(cx, |state, cx| {
                        state.dragging = Some(index);
                        state.grab_offset = px(0.);
                        cx.notify();
                    });
                    commit(index, value, Input::Pointer, window, cx);
                },
            );
        }

        let reading = self.show_value.then(|| {
            let small = t.label_small.size;
            div()
                .flex_none()
                .min_w(px(t.spacing_10))
                .text_right()
                .text_color(t.color_content_muted.hsla())
                .font_family(Face::Mono.family())
                .tabular_nums()
                .text_size(px(small))
                .line_height(px(small))
                .child(
                    values
                        .iter()
                        .map(|value| format(*value))
                        .collect::<Vec<_>>()
                        .join(" – "),
                )
        });

        div()
            .id(self.id)
            .w_full()
            .flex()
            .items_center()
            .gap(px(t.spacing_3))
            .child(control)
            .children(reading)
    }
}
