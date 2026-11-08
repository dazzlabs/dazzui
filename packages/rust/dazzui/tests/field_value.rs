//! Reported values keep the browser's block baseline and stay outside Tab traversal.

#[allow(dead_code)]
mod support;

use dazzui::{FieldValue, Motion, Size, Theme, tokens::Face};
use gpui::{
    Context, FocusHandle, InteractiveElement, IntoElement, ObservedPaint, ParentElement, Render,
    Styled, TestAppContext, Window, div, px,
};
use std::{cell::RefCell, rc::Rc, time::Duration};
use support::paint::PaintProbe;

struct Harness {
    size: Size,
    mono: bool,
    next: FocusHandle,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p(px(24.))
            .child(
                div().w(px(240.)).child(PaintProbe {
                    child: FieldValue::new("reported")
                        .size(self.size)
                        .mono(self.mono)
                        .child("Build abc-123")
                        .into_any_element(),
                    paints: self.paints.clone(),
                }),
            )
            .child(div().id("next").track_focus(&self.next).child("Next"))
    }
}

#[gpui::test]
fn reported_values_keep_block_baselines_and_are_not_editors(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|_, cx| Harness {
        size: Size::Medium,
        mono: false,
        next: cx.focus_handle().tab_stop(true),
        paints: Rc::new(RefCell::new(Vec::new())),
    });
    for family in ["bright", "studio", "frost", "graphite", "ember", "nocturne"] {
        for mode in ["light", "dark"] {
            cx.update(|_, cx| {
                Theme::set(&format!("{family}-{mode}"), cx);
                Motion::freeze_at(Duration::ZERO, cx);
            });
            for (size, height, baseline) in [
                (Size::Small, 26., 15.),
                (Size::Medium, 28., 17.),
                (Size::Large, 32., 17.),
            ] {
                for mono in [false, true] {
                    view.update(cx, |view, cx| {
                        view.size = size;
                        view.mono = mono;
                        cx.notify();
                    });
                    cx.run_until_parked();
                    let paint =
                        view.read_with(cx, |view, _| view.paints.borrow().last().unwrap().clone());
                    let surface = paint.decorated_surface.unwrap();
                    let text = paint.text.unwrap();
                    // Chromium's dz-field-value, at width 240px, in all twelve
                    // drawn themes. Large uses the same padding as medium;
                    // centering its line like an input would shift it by 3px.
                    assert_eq!(surface.bounds.size, gpui::size(px(240.), px(height)));
                    assert_eq!(text.baseline.y - surface.bounds.top(), px(baseline));
                    assert_eq!(text.baseline.x - surface.bounds.left(), px(13.));
                    assert_eq!(text.font_size, px(12.));
                    assert_eq!(text.line_height, px(12.));
                    assert!(surface.style.box_shadow.is_empty());
                    let tokens = Theme::named(&format!("{family}-{mode}")).unwrap().tokens;
                    let family = if mono {
                        Face::Mono.family()
                    } else {
                        size.profile(tokens).content.face.family()
                    };
                    assert_eq!(
                        surface
                            .style
                            .text
                            .font_family
                            .as_ref()
                            .map(|name| name.as_ref()),
                        Some(family)
                    );
                    assert_eq!(text.color, tokens.color_content.hsla());
                    assert_eq!(
                        surface
                            .style
                            .background
                            .unwrap()
                            .color()
                            .unwrap()
                            .solid_color(),
                        Some(tokens.color_surface_muted.hsla())
                    );
                }
            }
        }
    }
    cx.simulate_keystrokes("tab");
    assert!(cx.update(|window, cx| view.read(cx).next.is_focused(window)));
}
