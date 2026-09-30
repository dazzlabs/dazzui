//! The name, and which of its implementations this is.

use dazzui::{ActiveTheme, Badge, BadgeVariant, Size, StyledExt};
use gpui::{
    App, ClickEvent, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, px,
};

#[derive(IntoElement)]
pub struct Brand {
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
}

impl Brand {
    pub fn new(on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        Brand {
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for Brand {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        div()
            .id("brand")
            .flex()
            .items_center()
            .gap(px(t.spacing_2))
            .type_style(t.title_small)
            .cursor_pointer()
            .on_click(self.on_click)
            .child(
                div()
                    .size(px(t.spacing_5))
                    .rounded(px(t.radius_small))
                    .bg(t.color_primary.shade(500).hsla()),
            )
            .child("DazzUI")
            .child(
                Badge::new()
                    .size(Size::Small)
                    .variant(BadgeVariant::Tinted)
                    .child("GPUI"),
            )
    }
}
