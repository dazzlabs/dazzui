//! Resize-grip drawing derived from Chromium's ScrollableAreaPainter.
//! Copyright 2014 The Chromium Authors. BSD license: LICENSE-CHROMIUM.
//!
//! Browser textarea resize chrome, separate from editing and theme geometry.
//! Reference: https://chromium.googlesource.com/chromium/src/+/main/third_party/blink/renderer/core/paint/scrollable_area_painter.cc
//!
//! The 15px pointer corner and minimum drag size are browser UI metrics.
//! See Chromium's PaintLayerScrollableArea::CornerRect and Resize and
//! ScrollableAreaPainter::DrawPlatformResizerImage. The field's ds minimum
//! height still constrains the resulting border box.

use gpui::{
    AnyElement, App, Bounds, CursorStyle, DispatchPhase, Element, ElementId, Entity,
    GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PathBuilder, Pixels, Point,
    Subscription, Window, black, point, px, size, white,
};

const CORNER: f32 = 15.;

#[derive(Clone, Copy)]
struct Drag {
    position: Point<Pixels>,
    height: Pixels,
    changed: bool,
}

pub(super) struct ResizeState {
    pub height: Option<Pixels>,
    drag: Option<Drag>,
    _activation: Subscription,
}

pub(super) fn state(
    id: &ElementId,
    multiline: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ResizeState> {
    let state =
        window.use_keyed_state(super::part_id(id, "resize"), cx, |window, cx| ResizeState {
            height: None,
            drag: None,
            _activation: cx.observe_window_activation(
                window,
                |state: &mut ResizeState, window, cx| {
                    if !window.is_window_active() && state.drag.take().is_some() {
                        cx.notify();
                    }
                },
            ),
        });
    if !multiline {
        state.update(cx, |state, _| {
            state.height = None;
            state.drag = None;
        });
    }
    state
}

pub(super) struct ResizeField {
    pub child: AnyElement,
    pub state: Option<Entity<ResizeState>>,
    pub border: Pixels,
}

impl IntoElement for ResizeField {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for ResizeField {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Hitbox> {
        self.child.prepaint(window, cx);
        self.state.as_ref().map(|_| {
            let edge = px(CORNER).min(bounds.size.width).min(bounds.size.height);
            window.insert_hitbox(
                Bounds::new(
                    point(
                        bounds.right() - self.border - edge,
                        bounds.bottom() - self.border - edge,
                    ),
                    size(edge, edge),
                ),
                HitboxBehavior::Normal,
            )
        })
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
        let (Some(hitbox), Some(state)) = (hitbox.as_ref(), self.state.as_ref()) else {
            return;
        };
        window.set_cursor_style(CursorStyle::ResizeUpDown, hitbox);
        if state.read(cx).drag.is_some() {
            window.set_window_cursor_style(CursorStyle::ResizeUpDown);
        }
        paint_grip(hitbox.bounds, window);

        let down = state.downgrade();
        let hitbox = hitbox.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase != DispatchPhase::Capture
                || event.button != MouseButton::Left
                || !hitbox.is_hovered(window)
            {
                return;
            }
            let _ = down.update(cx, |state, cx| {
                state.drag = Some(Drag {
                    position: event.position,
                    height: bounds.size.height.round(),
                    changed: false,
                });
                cx.notify();
            });
            window.prevent_default();
            cx.stop_propagation();
        });

        let moved = state.downgrade();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
            if phase != DispatchPhase::Capture {
                return;
            }
            let active = moved
                .update(cx, |state, cx| {
                    let Some(drag) = state.drag else { return false };
                    if event.pressed_button != Some(MouseButton::Left) {
                        state.drag = None;
                        cx.notify();
                        return false;
                    }
                    let delta = (event.position.y - drag.position.y).round();
                    if delta != px(0.) || drag.changed {
                        state.drag.as_mut().unwrap().changed = true;
                        let height = (drag.height + delta).max(px(CORNER));
                        if state.height != Some(height) {
                            state.height = Some(height);
                            cx.notify();
                        }
                    }
                    true
                })
                .unwrap_or(false);
            if active {
                cx.stop_propagation();
            }
        });

        let released = state.downgrade();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
            if phase == DispatchPhase::Capture && event.button == MouseButton::Left {
                let active = released
                    .update(cx, |state, cx| {
                        let active = state.drag.take().is_some();
                        if active {
                            cx.notify();
                        }
                        active
                    })
                    .unwrap_or(false);
                if active {
                    cx.stop_propagation();
                }
            }
        });
    }
}

fn paint_grip(bounds: Bounds<Pixels>, window: &mut Window) {
    // Chromium's fixed-size resizer is two diagonals, with a light edge
    // one CSS pixel below their dark edge. Both remain visible on dark paper.
    for (offset, color) in [(0., black().opacity(0.6)), (1., white().opacity(0.6))] {
        let mut path = PathBuilder::stroke(px(1.));
        for length in [7., 3.] {
            path.move_to(point(
                bounds.right() - px(2.),
                bounds.bottom() - px(2. + length - offset),
            ));
            path.line_to(point(
                bounds.right() - px(2. + length - offset),
                bounds.bottom() - px(2.),
            ));
        }
        if let Ok(path) = path.build() {
            window.paint_path(path, color);
        }
    }
}
