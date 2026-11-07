//! CSS gaps still separate children after auto margins absorb free space.

use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, TestAppContext,
    Window, div, px,
};

struct AutoMarginGap {
    vertical: bool,
    reverse: bool,
}

impl Render for AutoMarginGap {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let trailing = div()
            .size(px(20.))
            .flex_none()
            .debug_selector(|| "trailing".into());
        let trailing = match (self.vertical, self.reverse) {
            (false, false) => trailing.ml_auto(),
            (false, true) => trailing.mr_auto(),
            (true, false) => trailing.mt_auto(),
            (true, true) => trailing.mb_auto(),
        };
        let row = div().size(px(200.)).flex().items_start().gap(px(10.));
        let row = match (self.vertical, self.reverse) {
            (false, false) => row.flex_row(),
            (false, true) => row.flex_row_reverse(),
            (true, false) => row.flex_col(),
            (true, true) => row.flex_col_reverse(),
        };
        row.child(
            div()
                .size(px(20.))
                .flex_none()
                .debug_selector(|| "first".into()),
        )
        .child(
            div()
                .size(px(20.))
                .flex_none()
                .debug_selector(|| "second".into()),
        )
        .child(trailing)
    }
}

#[gpui::test]
fn auto_margins_preserve_gaps_and_the_trailing_edge_in_every_direction(cx: &mut TestAppContext) {
    for vertical in [false, true] {
        for reverse in [false, true] {
            let (_, cx) = cx.add_window_view(|_, _| AutoMarginGap { vertical, reverse });
            cx.run_until_parked();
            let mut position = |id| {
                let bounds = cx.debug_bounds(id).unwrap();
                if vertical {
                    bounds.origin.y
                } else {
                    bounds.origin.x
                }
            };
            assert_eq!(position("first"), px(if reverse { 180. } else { 0. }));
            assert_eq!(position("second"), px(if reverse { 150. } else { 30. }));
            assert_eq!(position("trailing"), px(if reverse { 0. } else { 180. }));
        }
    }
}

struct WindowAnchor;

impl Render for WindowAnchor {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .ml(px(10.5))
            .mt(px(10.5))
            .size(px(40.))
            .child(
                gpui::anchored()
                    .position(gpui::point(px(0.), px(0.)))
                    .child(
                        div()
                            .size(px(20.))
                            .debug_selector(|| "window-anchored".into()),
                    ),
            )
    }
}

#[gpui::test]
fn a_fractional_parent_does_not_move_the_window_anchor(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| WindowAnchor);
    cx.run_until_parked();
    assert_eq!(
        cx.debug_bounds("window-anchored").unwrap().origin,
        gpui::point(px(0.), px(0.))
    );
}

struct BaselineGroup {
    graphic: f32,
}

impl Render for BaselineGroup {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_baseline()
            .debug_selector(|| "baseline-group".into())
            // A 20px line strut, a replaced graphic, a smaller code box and
            // a text box. These are border-box baselines, not bottom edges.
            .child(div().flex_none().w_0().h(px(20.)).baseline_offset(px(15.)))
            .child(
                div()
                    .flex_none()
                    .size(px(self.graphic))
                    .debug_selector(|| "graphic".into()),
            )
            .child(
                div()
                    .flex_none()
                    .w(px(40.))
                    .h(px(17.))
                    .baseline_offset(px(12.))
                    .debug_selector(|| "code".into()),
            )
            .child(
                div()
                    .flex_none()
                    .w(px(40.))
                    .h(px(20.))
                    .baseline_offset(px(15.))
                    .debug_selector(|| "text".into()),
            )
    }
}

#[gpui::test]
fn mixed_inline_boxes_share_a_baseline_and_preserve_the_strut_descent(cx: &mut TestAppContext) {
    for graphic in [12., 16., 32.] {
        let (_, cx) = cx.add_window_view(|_, _| BaselineGroup { graphic });
        cx.run_until_parked();
        let group = cx.debug_bounds("baseline-group").unwrap();
        let graphic_box = cx.debug_bounds("graphic").unwrap();
        let code = cx.debug_bounds("code").unwrap();
        let text = cx.debug_bounds("text").unwrap();
        let baseline = group.top() + px(graphic.max(15.));
        assert_eq!(graphic_box.bottom(), baseline);
        assert_eq!(code.top() + px(12.), baseline);
        assert_eq!(text.top() + px(15.), baseline);
        assert_eq!(group.size.height, px(graphic.max(15.) + 5.));
    }
}

struct NestedBaseline {
    wrap: bool,
}

impl Render for NestedBaseline {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let nested = if self.wrap {
            div()
                .flex()
                .flex_wrap()
                .items_start()
                .content_center()
                .w(px(30.))
                .h(px(100.))
                .children((0..3).map(|_| div().flex_none().size(px(20.)).baseline_offset(px(15.))))
        } else {
            div()
                .flex()
                .items_center()
                .size(px(16.))
                .child(div().flex_none().size(px(20.)).baseline_offset(px(15.)))
        };
        div()
            .flex()
            .items_baseline()
            .child(
                div()
                    .flex_none()
                    .w_0()
                    .h(px(20.))
                    .baseline_offset(px(15.))
                    .debug_selector(|| "strut".into()),
            )
            .child(
                nested
                    .flex_none()
                    .debug_selector(|| "nested-baseline".into()),
            )
    }
}

#[gpui::test]
fn nested_baselines_include_alignment_offsets_exactly_once(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| NestedBaseline { wrap: false });
    cx.run_until_parked();
    assert_eq!(cx.debug_bounds("nested-baseline").unwrap().top(), px(2.));
    assert_eq!(cx.debug_bounds("strut").unwrap().top(), px(0.));
}

#[gpui::test]
fn a_wrapped_containers_baseline_includes_its_first_lines_offset(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| NestedBaseline { wrap: true });
    cx.run_until_parked();
    assert_eq!(cx.debug_bounds("nested-baseline").unwrap().top(), px(0.));
    assert_eq!(cx.debug_bounds("strut").unwrap().top(), px(20.));
}
