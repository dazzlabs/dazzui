use super::*;
use crate::components::collapsible::kept_focus;
use crate::{
    Icon,
    foundation::{pointer_down, press::ButtonPress},
};
use gpui::{InteractiveElement, MouseButton, linear_color_stop, linear_gradient, rgb, rgba};

// The platform glyphs are transcribed from window-frame.tsx, not Fluent
// replacements. Stroke widths and view boxes are part of each platform.
pub(crate) const SVGS: &[(&str, &str)] = &[
    (
        "dazzui/window-mac-close.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12" fill="none" stroke="black" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"><path d="M3.5 3.5l5 5M8.5 3.5l-5 5"/></svg>"#,
    ),
    (
        "dazzui/window-mac-minimize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12" fill="none" stroke="black" stroke-width="1.3" stroke-linecap="round"><path d="M2.8 6h6.4"/></svg>"#,
    ),
    (
        "dazzui/window-mac-maximize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 12 12" fill="none" stroke="black" stroke-width="1.3" stroke-linecap="round"><path d="M6 2.8v6.4M2.8 6h6.4"/></svg>"#,
    ),
    (
        "dazzui/window-windows-close.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10" fill="none" stroke="black" stroke-width="1" stroke-linecap="butt" stroke-linejoin="round"><path d="M0.5 0.5l9 9M9.5 0.5l-9 9"/></svg>"#,
    ),
    (
        "dazzui/window-windows-minimize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10" fill="none" stroke="black" stroke-width="1" stroke-linecap="butt"><path d="M0 5.5h10"/></svg>"#,
    ),
    (
        "dazzui/window-windows-maximize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10" fill="none" stroke="black" stroke-width="1"><rect x="0.5" y="0.5" width="9" height="9" rx="1"/></svg>"#,
    ),
    (
        "dazzui/window-adwaita-close.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 5l6 6M11 5l-6 6"/></svg>"#,
    ),
    (
        "dazzui/window-adwaita-minimize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5" stroke-linecap="round"><path d="M4.5 11.5h7"/></svg>"#,
    ),
    (
        "dazzui/window-adwaita-maximize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5"><rect x="4.5" y="4.5" width="7" height="7" rx="1"/></svg>"#,
    ),
    (
        "dazzui/window-breeze-close.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 4.5l7 7M11.5 4.5l-7 7"/></svg>"#,
    ),
    (
        "dazzui/window-breeze-minimize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 6l4 4 4-4"/></svg>"#,
    ),
    (
        "dazzui/window-breeze-maximize.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M4 10l4-4 4 4"/></svg>"#,
    ),
];
#[derive(IntoElement)]
pub(super) struct Controls {
    pub id: ElementId,
    pub platform: WindowFramePlatform,
    pub controls: WindowFrameControls,
    pub handlers: Handlers,
    pub inactive: bool,
    pub height: f32,
    pub main: bool,
}
#[derive(IntoElement)]
pub(super) struct FloatingControls {
    pub controls: WindowFrameControls,
}
impl RenderOnce for FloatingControls {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let frame = scoped::<FrameContext>(cx).unwrap();
        let height = scoped::<TopBand>(cx).unwrap().height;
        if frame.main_controls.get() {
            return div().into_any_element();
        }
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .flex()
            .justify_end()
            .h(px(height))
            .child(Controls {
                id: part_id(&frame.id, "floating-controls"),
                platform: frame.platform,
                controls: self.controls,
                handlers: frame.handlers,
                inactive: frame.inactive,
                height,
                main: false,
            })
            .into_any_element()
    }
}
impl RenderOnce for Controls {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = self.platform;
        let mac = p.mac();
        let win = p == WindowFramePlatform::Windows;
        let mut row = div().flex().items_center();
        if mac {
            let edge = if p == WindowFramePlatform::Macos {
                14.
            } else {
                12.
            };
            let inset = (self.height - edge) / 2.;
            row = row
                .group("dz-window-traffic")
                .absolute()
                .top(px(inset))
                .left(px(inset))
                .gap(px(if p == WindowFramePlatform::Macos {
                    9.
                } else {
                    8.
                }));
        } else if win {
            row = row.h(px(self.height)).ml_auto();
        } else if self.main {
            row = row
                .ml(px(cx.theme().spacing_2))
                .gap(px(if p == WindowFramePlatform::Kde {
                    6.
                } else {
                    12.
                }));
        } else {
            row = row
                .absolute()
                .top_0()
                .bottom_0()
                .right(px(p.header_inset()))
                .gap(px(if p == WindowFramePlatform::Kde {
                    6.
                } else {
                    12.
                }));
        }
        let mut order = vec![
            ("minimize", self.controls.minimize, self.handlers.minimize),
            ("maximize", self.controls.maximize, self.handlers.maximize),
            ("close", self.controls.close, self.handlers.close),
        ];
        if mac {
            order.rotate_right(1);
        }
        for (kind, state, handler) in order {
            let state = state.unwrap_or(WindowControlState::Disabled);
            if if mac {
                state == WindowControlState::Hidden
            } else {
                state != WindowControlState::Enabled
            } {
                continue;
            }
            row = row.child(Control {
                id: part_id(&self.id, kind),
                platform: p,
                kind,
                state,
                handler,
                inactive: self.inactive,
                height: self.height,
            });
        }
        row
    }
}
#[derive(IntoElement)]
struct Control {
    id: ElementId,
    platform: WindowFramePlatform,
    kind: &'static str,
    state: WindowControlState,
    handler: Option<Handler>,
    inactive: bool,
    height: f32,
}
impl RenderOnce for Control {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let p = self.platform;
        let mac = p.mac();
        let win = p == WindowFramePlatform::Windows;
        let disabled = self.state != WindowControlState::Enabled;
        let focus = kept_focus(part_id(&self.id, "focus"), window, cx).tab_stop(false);
        let press = ButtonPress::new(&self.id, &focus, disabled, window, cx);
        let pointer = window.use_keyed_state(part_id(&self.id, "pointer-press"), cx, |_, _| false);
        if disabled {
            pointer.update(cx, |pressed, _| *pressed = false);
        }
        let pressed = !disabled && (press.pressed(cx) || *pointer.read(cx));
        let scale = if p == WindowFramePlatform::Macos {
            let frame = crate::Motion::frame(cx);
            let motion =
                window.use_keyed_state(part_id(&self.id, "scale"), cx, |_, _| LightMotion {
                    pressed: false,
                    transition: crate::foundation::motion::Transition::new(1., frame.elapsed),
                    duration: 420.,
                    curve: [0.34, 1.9, 0.5, 1.],
                });
            motion.update(cx, |motion, _| {
                if pressed != motion.pressed {
                    // Retarget with the old timing before switching to the
                    // new duration, preserving CSS reversal shortening.
                    motion.transition.target(
                        if pressed { 0.8 } else { 1. },
                        frame,
                        motion.duration,
                        motion.curve,
                    );
                    motion.pressed = pressed;
                    motion.duration = if pressed { 90. } else { 420. };
                    motion.curve = if pressed {
                        [0., 0., 0.58, 1.]
                    } else {
                        [0.34, 1.9, 0.5, 1.]
                    };
                }
                if !frame.frozen && motion.transition.running(frame.elapsed, motion.duration) {
                    window.request_animation_frame();
                }
                motion
                    .transition
                    .value(frame.elapsed, motion.duration, motion.curve)
            })
        } else {
            1.
        };
        let family = if mac {
            "mac"
        } else if win {
            "windows"
        } else if p == WindowFramePlatform::Kde {
            "breeze"
        } else {
            "adwaita"
        };
        let glyph = format!("dazzui/window-{family}-{}.svg", self.kind);
        let kind = self.kind;
        let mut button = div()
            .id(self.id)
            .debug_selector(move || format!("window-control-{kind}"))
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .cursor_default();
        if !disabled {
            let focus = focus.clone();
            let down = pointer.clone();
            let up = pointer.clone();
            let outside = pointer;
            button = button
                .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                    down.update(cx, |pressed, cx| {
                        *pressed = true;
                        cx.notify();
                    })
                })
                .on_mouse_up(MouseButton::Left, move |_, _, cx| {
                    up.update(cx, |pressed, cx| {
                        *pressed = false;
                        cx.notify();
                    })
                })
                .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                    outside.update(cx, |pressed, cx| {
                        *pressed = false;
                        cx.notify();
                    })
                })
                .track_focus(&focus.clone())
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    pointer_down(window, cx);
                    window.focus(&focus);
                });
        }
        if mac {
            let size = if p == WindowFramePlatform::Macos {
                14.
            } else {
                12.
            };
            let colour = match self.kind {
                "close" => rgb(0xff5f57),
                "minimize" => rgb(0xfebc2e),
                _ => rgb(0x28c840),
            };
            button = button
                .size(px(size))
                .rounded_full()
                .bg(if disabled || self.inactive {
                    t.color_surface_sunken.hsla()
                } else {
                    colour.into()
                });
            if !disabled && self.inactive {
                button = button.group_hover("dz-window-traffic", move |s| s.bg(colour));
            }
            let mut edge = div()
                .absolute()
                .inset_0()
                .rounded_full()
                .border(px(0.5))
                .border_color(rgba(0x0000001f));
            if p == WindowFramePlatform::Macos {
                edge = edge
                    .bg(linear_gradient(
                        180.,
                        linear_color_stop(rgba(0xffffff38), 0.),
                        linear_color_stop(rgba(0xffffff00), 0.55),
                    ))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .rounded_full()
                            .border_t(px(0.5))
                            .border_color(rgba(0xffffff8c)),
                    );
            }
            button = button.child(edge).child(
                div()
                    .flex()
                    .map(|mut s| {
                        s.style().visibility = Some(gpui::Visibility::Hidden);
                        s
                    })
                    .when(!disabled, |s| {
                        s.group_hover("dz-window-traffic", |mut s| {
                            s.visibility = Some(gpui::Visibility::Visible);
                            s
                        })
                    })
                    .child(Icon::new(glyph).size(10.).color(rgba(0x0000008c))),
            );
        } else {
            let ink = if self.inactive {
                t.color_content_faint.hsla()
            } else {
                t.color_content.hsla()
            };
            button = button.text_color(ink);
            if win {
                let close = self.kind == "close";
                let hover = t.color_surface_subtle.hsla();
                button = button.w(px(46.)).h(px(self.height)).hover(move |s| {
                    if close {
                        s.bg(rgb(0xc42b1c)).text_color(rgb(0xffffff))
                    } else {
                        s.bg(hover)
                    }
                });
            } else {
                button = button
                    .size(px(if p == WindowFramePlatform::Kde {
                        18.
                    } else {
                        24.
                    }))
                    .rounded_full()
                    .when(p != WindowFramePlatform::Kde, |s| {
                        s.bg(t.color_surface_subtle.hsla())
                    })
                    .hover(move |s| s.bg(crate::foundation::fade(ink, 0.16)));
            }
            button = button.child(Icon::new(glyph).size(if win { 10. } else { 16. }));
        }
        crate::foundation::transform::scaled(
            press.bind(
                button,
                self.handler.map(|handler| {
                    Box::new(
                        move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                            handler(window, cx)
                        },
                    ) as crate::foundation::press::OnClick
                }),
            ),
            scale,
        )
    }
}
struct LightMotion {
    pressed: bool,
    transition: crate::foundation::motion::Transition,
    duration: f32,
    curve: [f32; 4],
}
