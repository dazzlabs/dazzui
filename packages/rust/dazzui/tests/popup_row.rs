//! A list row keeps one height whatever it carries: an icon overhangs into
//! the row's padding rather than deepening it.

use dazzui::popup::{PopupRow, row_height};
use dazzui::theme::ActiveTheme;
use dazzui::{Icon, glyphs};
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, TestAppContext,
    Window, div, px,
};

struct Rows;

impl Render for Rows {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(200.))
            .flex()
            .flex_col()
            .child(
                div()
                    .debug_selector(|| "plain".into())
                    .child(PopupRow::new("plain", "Rename")),
            )
            .child(div().debug_selector(|| "icon".into()).child(
                PopupRow::new("icon", "Rename").icon(Icon::new(glyphs::CHECKMARK_12_FILLED)),
            ))
    }
}

#[gpui::test]
fn a_row_with_an_icon_is_as_tall_as_a_row_without(cx: &mut TestAppContext) {
    let (_, cx) = cx.add_window_view(|_, _| Rows);
    cx.run_until_parked();
    let plain = cx.debug_bounds("plain").unwrap().size.height;
    let icon = cx.debug_bounds("icon").unwrap().size.height;
    let expected = cx.update(|_, cx| row_height(cx.theme()));
    assert_eq!(plain, px(expected));
    assert_eq!(icon, plain);
}
