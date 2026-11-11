//! A group heading reserves the line its button needs, and a heading that
//! carries one gives the button a column of its own beside the label.

use dazzui::{
    ActiveTheme, Sidebar, SidebarGroup, SidebarGroupAction, SidebarGroupLabel, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarProvider, Theme,
};
use gpui::{
    Bounds, Context, InteractiveElement, IntoElement, ParentElement, Pixels, Render, Styled,
    TestAppContext, VisualTestContext, Window, div, px,
};

struct Harness {
    title: &'static str,
    action: bool,
    width: f32,
}

impl Render for Harness {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut label = SidebarGroupLabel::new()
            .child(div().debug_selector(|| "title".into()).child(self.title));
        if self.action {
            label = label.action(
                div()
                    .debug_selector(|| "action".into())
                    .child(SidebarGroupAction::new("add", "Add").child("+")),
            );
        }
        div().size_full().flex().child(
            SidebarProvider::new("provider").open(true).child(
                Sidebar::new().width(self.width).child(
                    SidebarGroup::new()
                        .child(div().debug_selector(|| "heading".into()).child(label))
                        .child(
                            SidebarMenu::new().child(
                                SidebarMenuItem::new()
                                    .child(SidebarMenuButton::new("row").child("Documents")),
                            ),
                        ),
                ),
            ),
        )
    }
}

fn setup<'a>(
    title: &'static str,
    action: bool,
    width: f32,
    cx: &'a mut TestAppContext,
) -> &'a mut VisualTestContext {
    cx.update(|cx| {
        dazzui::init(cx);
        Theme::set("bright-light", cx);
    });
    let (_, cx) = cx.add_window_view(|_, _| Harness {
        title,
        action,
        width,
    });
    cx.run_until_parked();
    cx
}

fn bounds(cx: &mut VisualTestContext, selector: &'static str) -> Bounds<Pixels> {
    cx.debug_bounds(selector).unwrap()
}

#[gpui::test]
fn a_heading_reserves_the_line_its_button_needs(cx: &mut TestAppContext) {
    let cx = setup("Library", false, 220., cx);
    let line = cx.update(|_, cx| cx.theme().sidebar_group_label_height);
    let heading = bounds(cx, "heading");
    assert_eq!(heading.size.height, px(line));
    assert_eq!(
        bounds(cx, "title").center().y,
        heading.center().y,
        "the label sits on the line's centre"
    );
}

#[gpui::test]
fn a_heading_gives_its_button_a_column_of_its_own(cx: &mut TestAppContext) {
    let cx = setup("Library", true, 220., cx);
    let (line, gap, inset) = cx.update(|_, cx| {
        let t = cx.theme();
        (t.sidebar_group_label_height, t.spacing_1, t.spacing_2_5)
    });
    let heading = bounds(cx, "heading");
    let action = bounds(cx, "action");
    assert_eq!(heading.size.height, px(line), "no taller than without one");
    assert_eq!(action.center().y, heading.center().y, "one centre");
    assert_eq!(
        action.right(),
        heading.right(),
        "on the group's trailing edge"
    );
    assert!(bounds(cx, "title").right() <= action.left() - px(gap + inset));
}

#[gpui::test]
fn a_long_label_wraps_before_the_button(cx: &mut TestAppContext) {
    let cx = setup("A label long enough to reach the button", true, 160., cx);
    let (line, gap, inset) = cx.update(|_, cx| {
        let t = cx.theme();
        (t.sidebar_group_label_height, t.spacing_1, t.spacing_2_5)
    });
    let heading = bounds(cx, "heading");
    let action = bounds(cx, "action");
    let title = bounds(cx, "title");
    assert!(heading.size.height > px(line), "the label wraps");
    assert!(
        title.right() <= action.left() - px(gap + inset),
        "never under the button"
    );
    assert_eq!(action.center().y, heading.center().y);
}
