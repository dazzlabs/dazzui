//! A pointer target across the column edge, outside sequential focus order.
use super::*;
use crate::components::checkbox::kit;
use gpui::{CursorStyle, SharedString, relative};

#[derive(Clone, Copy)]
pub(super) struct RailColumn {
    pub side: SidebarSide,
    pub collapsed: bool,
}

#[derive(IntoElement)]
pub struct SidebarRail {
    id: ElementId,
    label: SharedString,
    disabled: bool,
    on_click: Option<OnClick>,
}
impl SidebarRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: "Toggle sidebar".into(),
            disabled: false,
            on_click: None,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Replaces the default toggle, matching a button's supplied onClick.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}
impl RenderOnce for SidebarRail {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let column = scoped::<RailColumn>(cx).unwrap_or(RailColumn {
            side: SidebarSide::Left,
            collapsed: false,
        });
        let left = column.side == SidebarSide::Left;
        let cursor = if left != column.collapsed {
            CursorStyle::ResizeLeft
        } else {
            CursorStyle::ResizeRight
        };
        let focus = kept_focus(part_id(&self.id, "focus"), window, cx).tab_stop(false);
        let press = ButtonPress::new(&self.id, &focus, self.disabled, window, cx);
        let context = SidebarContext::current(cx);
        let border = t.color_border.hsla();
        let line = crate::foundation::motion::background_transition(
            div()
                .id(part_id(&self.id, "line"))
                .absolute()
                .top_0()
                .bottom_0()
                .left(relative(0.5))
                .w(px(t.spacing_0_5))
                .bg(gpui::transparent_black())
                .group_hover("dz-sidebar-rail", move |style| style.bg(border)),
            part_id(&self.id, "line-paint"),
            t,
            false,
            window,
            cx,
        );
        let rail = div()
            .id(self.id)
            .absolute()
            .top_0()
            .bottom_0()
            .w(px(t.spacing_4))
            .group("dz-sidebar-rail")
            .cursor(cursor)
            .map(|rail| {
                if left {
                    rail.right(px(-t.spacing_4 / 2.))
                } else {
                    rail.left(px(-t.spacing_4 / 2.))
                }
            })
            .when(!self.disabled, |rail| {
                rail.track_focus(&focus)
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        pointer_down(window, cx);
                        window.focus(&focus);
                    })
            })
            .child(line);
        let click = self.on_click;
        let rail = kit::hint(rail, Some(self.label));
        press.bind(
            rail,
            Some(Box::new(move |event, window, cx| {
                if let Some(click) = &click {
                    click(event, window, cx);
                } else if let Some(context) = &context {
                    context.toggle(window, cx);
                }
            })),
        )
    }
}
