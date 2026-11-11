//! A selectable paragraph grows at its actual wrapping width.

#[allow(dead_code)]
mod support;

use dazzui::foundation::text_input::{InputState, TextInput};
use gpui::{
    AppContext, Context, Entity, IntoElement, ObservedPaint, ParentElement, Render, Styled,
    TestAppContext, Window, div, px, white,
};
use std::{cell::RefCell, rc::Rc};
use support::paint::PaintProbe;

struct Harness {
    input: Entity<InputState>,
    width: f32,
    auto_height: bool,
    max_rows: Option<usize>,
    paints: Rc<RefCell<Vec<ObservedPaint>>>,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let input = TextInput::new("paragraph")
            .state(self.input.clone())
            .multiline(true)
            .read_only(true)
            .rows(2)
            .auto_height(self.auto_height);
        let input = if let Some(max_rows) = self.max_rows {
            input.max_rows(max_rows)
        } else {
            input
        };
        div().w(px(self.width)).child(PaintProbe {
            child: div()
                .bg(white())
                .text_size(px(13.))
                .line_height(px(20.))
                .child(input)
                .into_any_element(),
            paints: self.paints.clone(),
        })
    }
}

#[gpui::test]
fn paragraph_grows_wraps_and_shrinks_while_fixed_rows_keep_their_viewport(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let (view, cx) = cx.add_window_view(|window, cx| Harness {
        input: cx.new(|cx| InputState::new(window, cx)),
        width: 240.,
        auto_height: false,
        max_rows: None,
        paints: Rc::default(),
    });
    let input = view.read_with(cx, |view, _| view.input.clone());
    input.update(cx, |input, cx| input.set_value("one\ntwo\nthree", cx));
    cx.run_until_parked();
    let height = |cx: &mut gpui::VisualTestContext| {
        view.read_with(cx, |view, _| {
            view.paints
                .borrow()
                .last()
                .unwrap()
                .decorated_surface
                .as_ref()
                .unwrap()
                .bounds
                .size
                .height
        })
    };
    assert_eq!(height(cx), px(40.));
    view.update(cx, |view, cx| {
        view.auto_height = true;
        cx.notify();
    });
    cx.run_until_parked();
    assert_eq!(height(cx), px(60.));
    input.update(cx, |input, cx| {
        input.set_value(
            "A selectable paragraph follows its wrapping width across several lines.",
            cx,
        )
    });
    cx.run_until_parked();
    let wide = height(cx);
    view.update(cx, |view, cx| {
        view.width = 90.;
        cx.notify();
    });
    cx.run_until_parked();
    assert!(height(cx) > wide);
    input.update(cx, |input, cx| input.set_value("one", cx));
    cx.run_until_parked();
    assert_eq!(height(cx), px(40.));

    view.update(cx, |view, cx| {
        view.max_rows = Some(3);
        cx.notify();
    });
    input.update(cx, |input, cx| input.set_value("one\ntwo\nthree\nfour", cx));
    cx.run_until_parked();
    assert_eq!(height(cx), px(60.));
    input.update(cx, |input, cx| input.set_value("one", cx));
    cx.run_until_parked();
    assert_eq!(height(cx), px(40.));
}
