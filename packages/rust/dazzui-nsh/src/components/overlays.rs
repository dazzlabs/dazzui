//! These builders retain all of NSH's popup ownership, focus and dismissal APIs.
use crate::{Theme, styles};
use nsh_core::prelude::*;
pub fn popover() -> nsh_base::PopoverElement {
    let theme = Theme::current();
    nsh_base::popover()
        .offset(move || px(theme.tokens().spacing_1))
        .popup_style(move |s| s.style_with(move |s| styles::popup(s, theme.tokens())))
}
pub fn tooltip() -> nsh_base::TooltipElement {
    let theme = Theme::current();
    nsh_base::tooltip()
        .offset(move || px(theme.tokens().spacing_1))
        .delay(move || f64::from(theme.tokens().tooltip_open_delay))
        .style_with(move |s| styles::popup(s, theme.tokens()).min_w(px(0.0)))
}
pub fn menu() -> nsh_base::MenuElement {
    let theme = Theme::current();
    nsh_base::menu()
        .offset(move || px(theme.tokens().spacing_1))
        .popup_style(move |s| s.style_with(move |s| styles::popup(s, theme.tokens())))
}
pub fn menu_item() -> nsh_base::MenuItemElement {
    let theme = Theme::current();
    nsh_base::menu_item()
        .style_with(move |s| styles::popup_item(s, theme.tokens()))
        .hover(move |s| s.bg(move || theme.tokens().color_surface_subtle.hsla()))
        .focus(move |s| s.bg(move || theme.tokens().color_surface_subtle.hsla()))
}
