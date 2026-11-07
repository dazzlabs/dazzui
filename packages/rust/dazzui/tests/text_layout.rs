//! Erasing a label's type must not change a control's geometry.

use dazzui::{
    ActiveTheme, Badge, Button, Checkbox, KeyCap, SectionLabel, SidebarMenuBadge, StyledExt, Theme,
};
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, TestAppContext,
    Window, div, px, size,
};

struct Labels;

impl Render for Labels {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .gap_2()
            .child(
                div()
                    .w(px(80.))
                    .type_style(cx.theme().label_quiet)
                    .debug_selector(|| "wrapped".into())
                    .child(dazzui::foundation::text_child(
                        "A longer sentence has several words",
                    )),
            )
            .child(
                div()
                    .w(px(80.))
                    .type_style(cx.theme().label_quiet)
                    .truncate()
                    .debug_selector(|| "truncated".into())
                    .child(dazzui::foundation::text_child(
                        "A longer sentence has several words",
                    )),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "single-line".into())
                    .child(SectionLabel::new().child("Archived")),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "button-plain".into())
                    .child(Button::new("plain").child("outlined")),
            )
            .child(div().flex().debug_selector(|| "button-boxed".into()).child(
                Button::new("boxed").child("outlined".into_any_element().into_any_element()),
            ))
            .child(
                div()
                    .flex()
                    .debug_selector(|| "badge-plain".into())
                    .child(Badge::new().child(String::from("Synced"))),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "badge-boxed".into())
                    .child(Badge::new().child(String::from("Synced").into_any_element())),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "key-plain".into())
                    .child(KeyCap::new().child("⌘K")),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "key-boxed".into())
                    .child(KeyCap::new().child("⌘K".into_any_element())),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "count-ones".into())
                    .child(SidebarMenuBadge::new().child("1111")),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "count-eights".into())
                    .child(SidebarMenuBadge::new().child("8888")),
            )
            .child(
                Badge::new().child(
                    div()
                        .debug_selector(|| "rich-child".into())
                        .w(px(17.))
                        .h(px(9.)),
                ),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "checkbox-without-note".into())
                    .child(Checkbox::new("without-note").child("Digest")),
            )
            .child(
                div()
                    .flex()
                    .debug_selector(|| "checkbox-empty-note".into())
                    .child(Checkbox::new("empty-note").note("").child("Digest")),
            )
    }
}

#[gpui::test]
fn labels_keep_geometry_through_type_erasure(cx: &mut TestAppContext) {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("studio-light", cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Labels);
    cx.simulate_resize(size(px(600.), px(400.)));
    for theme in ["studio-light", "bright-dark", "graphite-light"] {
        cx.update(|_, cx| Theme::set(theme, cx));
        cx.run_until_parked();
        let single_line = cx.update(|_, cx| px(cx.theme().label_quiet.line_height));
        assert!(cx.debug_bounds("wrapped").unwrap().size.height > single_line);
        assert_eq!(
            cx.debug_bounds("truncated").unwrap().size.height,
            single_line
        );
        assert_eq!(
            cx.debug_bounds("single-line").unwrap().size.height,
            cx.update(|_, cx| px(cx.theme().label_small.line_height)),
            "{theme}: a fitted label does not wrap its last glyph"
        );
        for (plain, boxed) in [
            ("button-plain", "button-boxed"),
            ("badge-plain", "badge-boxed"),
            ("key-plain", "key-boxed"),
            ("count-ones", "count-eights"),
            ("checkbox-without-note", "checkbox-empty-note"),
        ] {
            assert_eq!(
                cx.debug_bounds(plain).unwrap().size,
                cx.debug_bounds(boxed).unwrap().size,
                "{theme}: {plain}"
            );
        }
        assert_eq!(
            cx.debug_bounds("rich-child").unwrap().size,
            size(px(17.), px(9.))
        );
    }
}

struct PackedLabels;

impl Render for PackedLabels {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_start()
            .font_family(cx.theme().body_medium.face.family())
            .text_size(px(13.1))
            .line_height(px(20.))
            .child(
                div()
                    .flex()
                    .flex_none()
                    .debug_selector(|| "packed-labels".into())
                    .children((0..16).map(|_| dazzui::foundation::text_child("i"))),
            )
    }
}

#[gpui::test]
fn separate_labels_do_not_accumulate_pixel_rounding_in_a_row(cx: &mut TestAppContext) {
    cx.update(dazzui::init);
    let (_, cx) = cx.add_window_view(|_, _| PackedLabels);
    cx.run_until_parked();
    let shaped = cx.update(|window, cx| {
        let style = gpui::TextStyle {
            font_family: cx.theme().body_medium.face.family().into(),
            ..Default::default()
        };
        window
            .text_system()
            .shape_line("i".into(), px(13.1), &[style.to_run(1)], None)
            .width
            * 16.
    });
    let actual = cx.debug_bounds("packed-labels").unwrap().size.width;
    assert!(
        (f32::from(actual - shaped)).abs() < 0.5,
        "separate runs should occupy their shaped advances: {actual:?}, {shaped:?}"
    );
}
