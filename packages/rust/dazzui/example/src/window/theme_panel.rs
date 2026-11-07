//! Theme families and the shell's menu presentation preference.

use dazzui::{
    ActiveTheme, Icon, OptionCard, Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup,
    SidebarGroupContent, SidebarGroupLabel, SidebarSide, Size, Switch, Theme, Toggle, themed,
};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, IntoElement, ParentElement, Styled, Window,
    div, prelude::*, px, relative,
};
use std::rc::Rc;

use crate::lib::themes::{THEME_FAMILY_GROUPS, ThemeFamily};

/// A theme a person can pick: a family, in one of its halves.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeChoice {
    pub family: String,
    pub dark: bool,
}

impl ThemeChoice {
    /// The theme's name, as the web's `data-theme` writes it.
    pub fn id(&self) -> String {
        format!(
            "{}-{}",
            self.family,
            if self.dark { "dark" } else { "light" }
        )
    }
}

type SetTheme = Rc<dyn Fn(ThemeChoice, &mut Window, &mut App)>;
type SetNativeMenus = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// The panel, its width set by the shell that keeps it.
pub fn theme_panel(
    theme: &ThemeChoice,
    native_menus: bool,
    width: f32,
    set_theme: SetTheme,
    set_native_menus: SetNativeMenus,
    scroll: gpui::ScrollHandle,
    resizer: AnyElement,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let t = cx.theme();
    let dark = theme.dark;
    let toggle = {
        let theme = theme.clone();
        let set_theme = set_theme.clone();
        Toggle::new("theme-mode")
            .label(if dark {
                "Switch to light"
            } else {
                "Switch to dark"
            })
            .icon_only(true)
            .pressed(dark)
            .on_pressed_change(move |pressed, window, cx| {
                set_theme(
                    ThemeChoice {
                        dark: pressed,
                        ..theme.clone()
                    },
                    window,
                    cx,
                )
            })
            .child(Icon::new(if dark {
                "glyph/WeatherMoon20Regular"
            } else {
                "glyph/WeatherSunny20Regular"
            }))
    };

    // Three to a row: a theme is told apart by its colours before its name.
    let inner = width - 2.0 * t.spacing_4;
    let tile = ((inner - 2.0 * t.spacing_2) / 3.0).floor();
    let groups =
        THEME_FAMILY_GROUPS.iter().map(|group| {
            SidebarGroup::new()
                .child(SidebarGroupLabel::new().child(group.title))
                .child(SidebarGroupContent::new().child(
                    div().flex().flex_wrap().gap(px(t.spacing_2)).children(
                        group.families.iter().map(|family| {
                            tile_for(family, theme, tile, set_theme.clone(), window, cx)
                        }),
                    ),
                ))
        });

    div()
        .id("theme-panel")
        .relative()
        .h_full()
        .min_h_0()
        .flex_none()
        .child(
            Sidebar::new()
                .side(SidebarSide::Right)
                .collapsible(SidebarCollapsible::None)
                .width(width)
                .child(
                    // One row, the height of the toolbar beside it.
                    div().child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(t.frame_titlebar_size))
                            .px(px(t.spacing_4))
                            .border_b(px(t.stroke_hairline))
                            .border_color(t.color_border.hsla())
                            .child(
                                div()
                                    .font_weight(t.label_strong.font_weight())
                                    .child("Theme"),
                            )
                            .child(toggle),
                    ),
                )
                .child(
                    SidebarContent::new().scroll_handle(scroll).child(
                        div()
                            .id("themes")
                            .flex()
                            .flex_col()
                            .flex_none()
                            .child(
                                SidebarGroup::new()
                                    .child(SidebarGroupLabel::new().child("Behavior"))
                                    .child(
                                        SidebarGroupContent::new().child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .gap(px(t.spacing_2))
                                                .child("Native menus")
                                                .child(
                                                    Switch::new("native-menus")
                                                        .size(Size::Small)
                                                        .checked(native_menus)
                                                        .disabled(!cfg!(feature = "native-menus"))
                                                        .on_checked_change(
                                                            move |native, window, cx| {
                                                                set_native_menus(native, window, cx)
                                                            },
                                                        ),
                                                ),
                                        ),
                                    ),
                            )
                            .children(groups),
                    ),
                ),
        )
        .child(resizer)
        .into_any_element()
}

fn tile_for(
    family: &ThemeFamily,
    theme: &ThemeChoice,
    width: f32,
    set_theme: SetTheme,
    _: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let t = cx.theme();
    let row = ThemeChoice {
        family: family.value.to_string(),
        dark: theme.dark,
    };
    let active = family.value == theme.family;
    let title = div()
        .flex()
        .flex_col()
        .min_w_0()
        .child(preview(&row.id(), t.spacing_10))
        .child(
            div()
                .h(px(t.spacing_6))
                .px(px(t.spacing_1_5))
                .text_size(px(11.0))
                .line_height(px(t.spacing_6))
                .font_weight(FontWeight::MEDIUM)
                .flex()
                .justify_center()
                // One line, centred; a name too long for it is cut at its end.
                .child(div().min_w_0().truncate().child(family.title)),
        );
    div()
        .w(px(width))
        .child(
            OptionCard::new(
                ElementId::Name(format!("theme-{}", family.value).into()),
                title,
            )
            .selected(active)
            .flush(true)
            .on_click(move |_: &ClickEvent, window, cx| set_theme(row.clone(), window, cx)),
        )
        .into_any_element()
}

/// A theme in miniature: a window of its own — the column, the paper, two
/// lines of its ink and a button in its accent — drawn in that theme.
fn preview(id: &str, height: f32) -> AnyElement {
    let Some(entry) = Theme::named(id) else {
        return div().into_any_element();
    };
    themed(entry, Preview { height }).into_any_element()
}

#[derive(IntoElement)]
struct Preview {
    height: f32,
}

impl gpui::RenderOnce for Preview {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = cx.theme();
        let edge = t.color_border_strong.hsla();
        div()
            .flex()
            .w_full()
            .h(px(self.height))
            .overflow_hidden()
            .bg(t.color_surface.hsla())
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .child(
                div()
                    .flex_none()
                    .w(relative(0.28))
                    .h_full()
                    .bg(t.color_surface_column.hsla())
                    .border_r(px(t.stroke_hairline))
                    .border_color(edge),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .flex_col()
                    .gap(px(t.spacing_1))
                    .p(px(t.spacing_1_5))
                    .child(
                        div()
                            .h(px(t.spacing_0_5))
                            .rounded(px(t.spacing_px))
                            .bg(t.color_content.hsla()),
                    )
                    .child(
                        div()
                            .w(relative(0.6))
                            .h(px(t.spacing_0_5))
                            .rounded(px(t.spacing_px))
                            .bg(t.color_content.hsla())
                            .opacity(0.4),
                    )
                    .child(
                        div()
                            .mt_auto()
                            .w(px(t.spacing_6))
                            .h(px(t.spacing_2_5))
                            .rounded(px(t.spacing_1))
                            .bg(t.color_primary.shade(500).hsla()),
                    ),
            )
    }
}
