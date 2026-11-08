//! Real layout, paint and input for the presentation window, inside a host.
#[allow(dead_code)]
mod support;
use dazzui::{
    ActiveTheme, Button, Length, Sidebar, SidebarCollapsible, SidebarProvider, Theme, WindowBody,
    WindowContent, WindowControlState as ControlState, WindowFooter, WindowFrame,
    WindowFrameControls as Controls, WindowFramePlatform as Platform, WindowMain, WindowTitlebar,
    WindowTitlebarSlots,
};
use gpui::{
    Context, Entity, FocusHandle, InteractiveElement, IntoElement, KeyUpEvent, Keystroke,
    Modifiers, ObservedPaint, ParentElement, Render, Styled, TestAppContext, VisualTestContext,
    Window, div, px, size,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;
struct Harness {
    platform: Platform,
    controls: Option<Controls>,
    band: bool,
    workspace: bool,
    collapsed: bool,
    icon: bool,
    inactive: bool,
    slots: bool,
    width: Option<Length>,
    min_width: Option<Length>,
    max_height: Option<Length>,
    events: Vec<&'static str>,
    next: FocusHandle,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}
impl Render for Harness {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.paints.borrow_mut().clear();
        let mut frame = WindowFrame::new("frame")
            .platform(self.platform)
            .title("Document")
            .inactive(self.inactive);
        for name in ["close", "minimize", "maximize"] {
            let view = cx.entity().downgrade();
            let action = move |_: &mut Window, cx: &mut gpui::App| {
                let _ = view.update(cx, |view, _| view.events.push(name));
            };
            frame = match name {
                "close" => frame.on_close(action),
                "minimize" => frame.on_minimize(action),
                _ => frame.on_maximize(action),
            };
        }
        if let Some(controls) = self.controls {
            frame = frame.controls(controls);
        }
        if let Some(width) = self.width {
            frame = frame.width(width);
        }
        if let Some(width) = self.min_width {
            frame = frame.min_width(width);
        }
        if let Some(height) = self.max_height {
            frame = frame.max_height(height);
        }
        if self.slots {
            frame = frame.titlebar(
                WindowTitlebarSlots::new()
                    .leading(Button::new("leading").child("Leading"))
                    .toolbar(Button::new("toolbar").child("Toolbar"))
                    .trailing(Button::new("trailing").child("Trailing")),
            );
        }
        if !self.band {
            frame = frame.without_titlebar();
        }
        frame = if self.workspace {
            frame
                .full_size_content(true)
                .height(Length::Px(220.))
                .child(
                    WindowBody::new().child(
                        SidebarProvider::new("provider")
                            .open(!self.collapsed)
                            .child(Sidebar::new().collapsible(if self.icon {
                                SidebarCollapsible::Icon
                            } else {
                                SidebarCollapsible::Offcanvas
                            }))
                            .child(
                                WindowMain::new()
                                    .child(
                                        WindowTitlebar::new("pane-titlebar").title("Main").leading(
                                            div()
                                                .debug_selector(|| "leading-slot".into())
                                                .child(Button::new("toggle").child("Toggle")),
                                        ),
                                    )
                                    .child(
                                        WindowContent::new().child(
                                            div()
                                                .debug_selector(|| "pane-content".into())
                                                .child("Content"),
                                        ),
                                    )
                                    .child(WindowFooter::new().child("Footer")),
                            ),
                    ),
                )
        } else {
            frame.child(
                div()
                    .debug_selector(|| "frame-content".into())
                    .h(px(80.))
                    .child("Content"),
            )
        };
        div()
            .size_full()
            .p(px(24.))
            .child(div().w(px(800.)).child(PaintProbe {
                child: frame.into_any_element(),
                paints: self.paints.clone(),
            }))
            .child(div().id("next").track_focus(&self.next).child("Next"))
    }
}
fn setup(cx: &mut TestAppContext, platform: Platform) -> (Entity<Harness>, &mut VisualTestContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        dazzui::Motion::set_preference(dazzui::MotionPreference::Reduced, cx);
        Theme::set("studio-light", cx);
    });
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        platform,
        controls: None,
        band: true,
        workspace: false,
        collapsed: false,
        icon: false,
        inactive: false,
        slots: false,
        width: Some(Length::Px(320.)),
        min_width: None,
        max_height: None,
        events: Vec::new(),
        next: cx.focus_handle().tab_stop(true),
        paints: Rc::default(),
    });
    cx.simulate_resize(size(px(1000.), px(800.)));
    cx.update(|window, _| window.activate_window());
    cx.run_until_parked();
    (view, cx)
}
fn paint(view: &Entity<Harness>, cx: &VisualTestContext) -> ObservedPaint {
    view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone())
}
fn click(cx: &mut VisualTestContext, kind: &str) {
    let bounds = cx
        .debug_bounds(match kind {
            "close" => "window-control-close",
            "minimize" => "window-control-minimize",
            _ => "window-control-maximize",
        })
        .unwrap();
    cx.simulate_click(bounds.center(), Modifiers::none());
    cx.run_until_parked();
}
#[gpui::test]
fn platform_bands_have_their_own_dimensions_and_controls(cx: &mut TestAppContext) {
    for (platform, height, button) in [
        (Platform::Macos, 32., 14.),
        (Platform::Macos15, 28., 12.),
        (Platform::Windows, 32., 46.),
        (Platform::Gnome, 47., 24.),
        (Platform::Ubuntu, 47., 24.),
        (Platform::Kde, 30., 18.),
        (Platform::Omarchy, 0., 0.),
    ] {
        let (view, cx) = setup(cx, platform);
        let content = cx.debug_bounds("frame-content").unwrap();
        assert_eq!(
            content.top(),
            px(24.
                + height
                + if platform == Platform::Omarchy {
                    22.
                } else {
                    0.
                }),
            "{platform:?}"
        );
        if platform != Platform::Omarchy {
            let close = cx.debug_bounds("window-control-close").unwrap();
            assert_eq!(close.size.width, px(button));
            click(cx, "close");
            assert_eq!(view.read_with(cx, |view, _| view.events.clone()), ["close"]);
        } else {
            assert!(
                paint(&view, cx).surfaces.iter().any(|s| s
                    .style
                    .border_widths
                    .left
                    .to_pixels(s.rem_size)
                    == px(2.))
            );
        }
    }
}
#[gpui::test]
fn partial_disabled_and_hidden_controls_follow_platform_policy(cx: &mut TestAppContext) {
    for platform in [Platform::Macos, Platform::Windows, Platform::Gnome] {
        let (view, cx) = setup(cx, platform);
        view.update(cx, |view, cx| {
            view.controls = Some(Controls::default().close(true));
            cx.notify();
        });
        cx.run_until_parked();
        click(cx, "close");
        assert_eq!(view.read_with(cx, |view, _| view.events.len()), 1);
        // Disabled macOS lights remain drawn and cannot be activated.
        if platform == Platform::Macos {
            click(cx, "minimize");
            click(cx, "maximize");
            assert_eq!(view.read_with(cx, |view, _| view.events.len()), 1);
        }
        let before = paint(&view, cx).surfaces.len();
        view.update(cx, |view, cx| {
            view.controls = Some(
                Controls::default()
                    .close(ControlState::Hidden)
                    .minimize(ControlState::Hidden)
                    .maximize(ControlState::Hidden),
            );
            cx.notify();
        });
        cx.run_until_parked();
        assert!(paint(&view, cx).surfaces.len() < before);
    }
}
#[gpui::test]
fn window_controls_skip_tab_and_keep_native_enter_space_phases(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Platform::Windows);
    cx.simulate_keystrokes("tab");
    let next = view.read_with(cx, |view, _| view.next.clone());
    assert!(cx.update(|window, _| next.is_focused(window)));
    click(cx, "close");
    cx.simulate_keystrokes("enter enter space");
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.events.len()), 3);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    assert_eq!(view.read_with(cx, |view, _| view.events.len()), 4);
    cx.simulate_keystrokes("tab");
    assert!(cx.update(|window, _| next.is_focused(window)));
}
#[gpui::test]
fn pane_handoff_and_collapsed_sidebar_clearance_retain_content_bounds(cx: &mut TestAppContext) {
    for platform in [
        Platform::Macos,
        Platform::Windows,
        Platform::Ubuntu,
        Platform::Kde,
    ] {
        let (view, cx) = setup(cx, platform);
        view.update(cx, |view, cx| {
            view.width = Some(Length::Px(700.));
            view.workspace = true;
            view.band = false;
            cx.notify();
        });
        cx.run_until_parked();
        let control = cx.debug_bounds("window-control-close").unwrap();
        let leading = cx.debug_bounds("leading-slot").unwrap();
        if platform == Platform::Macos {
            assert!(control.right() < leading.left());
        } else {
            assert!(control.left() > leading.right());
            click(cx, "close");
            assert_eq!(view.read_with(cx, |view, _| view.events.len()), 1);
        }
        let content = cx.debug_bounds("pane-content").unwrap();
        assert_eq!(
            content.top(),
            px(24. + cx.update(|_, cx| cx.theme().frame_titlebar_size))
        );
        if platform == Platform::Macos {
            for icon in [false, true] {
                view.update(cx, |view, cx| {
                    view.collapsed = true;
                    view.icon = icon;
                    cx.notify();
                });
                cx.run_until_parked();
                assert_eq!(cx.debug_bounds("leading-slot").unwrap().left(), px(104.));
            }
        }
    }
}
#[gpui::test]
fn width_constraints_and_toolbar_height_use_border_box_geometry(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Platform::Macos);
    view.update(cx, |view, cx| {
        view.width = Some(Length::Px(200.));
        view.min_width = Some(Length::Px(320.));
        view.max_height = Some(Length::Px(100.));
        view.slots = true;
        cx.notify();
    });
    cx.run_until_parked();
    let content = cx.debug_bounds("frame-content").unwrap();
    assert_eq!(content.size.width, px(320.));
    assert_eq!(content.top(), px(76.));
    let painted = paint(&view, cx);
    assert!(
        painted
            .surfaces
            .iter()
            .any(|surface| surface.bounds.size.width == px(320.)
                && surface.bounds.size.height == px(100.)
                && surface.style.box_shadow.len() > 1)
    );
}

#[gpui::test]
fn liquid_glass_lights_shrink_spring_back_and_cancel_an_outside_release(cx: &mut TestAppContext) {
    use gpui::MouseButton;
    use std::time::Duration;
    let (view, cx) = setup(cx, Platform::Macos);
    cx.update(|_, cx| {
        dazzui::Motion::set_preference(dazzui::MotionPreference::Full, cx);
        dazzui::Motion::seek(Duration::ZERO, cx);
    });
    cx.run_until_parked();
    click(cx, "close");
    cx.simulate_keystrokes("space");
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(90), cx));
    cx.run_until_parked();
    let small = cx.debug_bounds("window-control-close").unwrap();
    assert!((f32::from(small.size.width) - 11.2).abs() < 0.01);
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(300), cx));
    cx.run_until_parked();
    assert!(
        cx.debug_bounds("window-control-close").unwrap().size.width > px(14.),
        "the release overshoots its resting size"
    );
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(600), cx));
    cx.run_until_parked();
    let resting = cx.debug_bounds("window-control-close").unwrap();
    assert_eq!(resting.size.width, px(14.));
    cx.simulate_mouse_down(resting.center(), MouseButton::Left, Modifiers::none());
    cx.run_until_parked();
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(700), cx));
    cx.run_until_parked();
    cx.simulate_mouse_move(
        gpui::point(px(700.), px(400.)),
        Some(MouseButton::Left),
        Modifiers::none(),
    );
    cx.simulate_mouse_up(
        gpui::point(px(700.), px(400.)),
        MouseButton::Left,
        Modifiers::none(),
    );
    cx.run_until_parked();
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(1200), cx));
    cx.run_until_parked();
    assert_eq!(
        cx.debug_bounds("window-control-close").unwrap().size.width,
        px(14.)
    );
    assert_eq!(
        view.read_with(cx, |view, _| view.events.len()),
        2,
        "outside release must not activate"
    );
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(1300), cx));
    cx.run_until_parked();
    cx.simulate_keystrokes("space");
    cx.run_until_parked();
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(1310), cx));
    cx.run_until_parked();
    cx.simulate_event(KeyUpEvent {
        keystroke: Keystroke::parse("space").unwrap(),
    });
    cx.run_until_parked();
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(1400), cx));
    cx.run_until_parked();
    assert_eq!(
        cx.debug_bounds("window-control-close").unwrap().size.width,
        px(14.),
        "a quick reversal shortens the return transition"
    );
}

#[gpui::test]
fn inactive_shadow_transitions_without_reflowing_the_frame(cx: &mut TestAppContext) {
    use std::time::Duration;
    let (view, cx) = setup(cx, Platform::Macos);
    let shadow = |paint: ObservedPaint| {
        paint
            .surfaces
            .into_iter()
            .find(|surface| surface.style.box_shadow.len() > 1)
            .unwrap()
            .style
            .box_shadow
    };
    let before = shadow(paint(&view, cx));
    cx.update(|_, cx| {
        dazzui::Motion::set_preference(dazzui::MotionPreference::Full, cx);
        dazzui::Motion::seek(Duration::ZERO, cx);
    });
    cx.run_until_parked();
    view.update(cx, |view, cx| {
        view.inactive = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(shadow(paint(&view, cx)), before);
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(100), cx));
    cx.run_until_parked();
    let between = shadow(paint(&view, cx));
    assert_ne!(between, before);
    cx.update(|_, cx| dazzui::Motion::seek(Duration::from_millis(250), cx));
    cx.run_until_parked();
    assert_ne!(shadow(paint(&view, cx)), between);
    assert_eq!(cx.debug_bounds("frame-content").unwrap().top(), px(56.));
}

#[gpui::test]
fn caption_close_hover_uses_windows_red_and_white_ink(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Platform::Windows);
    let close = cx.debug_bounds("window-control-close").unwrap();
    cx.simulate_mouse_move(close.center(), None, Modifiers::none());
    cx.run_until_parked();
    let painted = paint(&view, cx);
    let close = painted
        .surfaces
        .iter()
        .find(|surface| {
            surface
                .style
                .background
                .as_ref()
                .and_then(|background| background.color())
                .and_then(|color| color.solid_color())
                == Some(gpui::rgb(0xc42b1c).into())
        })
        .unwrap();
    assert_eq!(close.style.text.color, Some(gpui::rgb(0xffffff).into()));
}

#[gpui::test]
fn explicitly_hidden_frame_controls_do_not_reappear_during_pane_handoff(cx: &mut TestAppContext) {
    let (view, cx) = setup(cx, Platform::Windows);
    view.update(cx, |view, cx| {
        view.workspace = true;
        view.band = false;
        view.width = Some(Length::Px(700.));
        view.controls = Some(
            Controls::default()
                .close(ControlState::Hidden)
                .minimize(ControlState::Hidden)
                .maximize(ControlState::Hidden),
        );
        cx.notify();
    });
    cx.run_until_parked();
    assert!(paint(&view, cx).glyph_ink.is_none());
}
