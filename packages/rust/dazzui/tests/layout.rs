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
