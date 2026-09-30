//! A desktop application: three full-height columns — the source list, the
//! document under its toolbar, and the theme panel — the React example's
//! `components/shell.tsx`, drawn from the kit alone.

use std::collections::HashMap;
use std::rc::Rc;

use dazzui::{
    ActiveTheme, Button, Card, CardVariant, Icon, IconButton, SearchField, SectionLabel, Sidebar,
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuBadge, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarProvider, SidebarTrigger, Size, Theme, Tooltip, Variant,
    popup::Side as PopupSide,
};
use gpui::{
    AnyElement, App, Context, ElementId, Entity, InteractiveElement, IntoElement, MouseButton,
    MouseMoveEvent, ParentElement, Render, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};

use crate::lib::scene::SceneView;
use crate::lib::stage::page_type;
use crate::lib::story::{Scene, Story, matching, stories, story};
use crate::window::brand::Brand;
use crate::window::resizer::{Drag, LIST, Resizer, Side, THEMES};
use crate::window::theme_panel::{ThemeChoice, theme_panel};

/// Everything the window keeps: which page, the search, the theme, and the
/// two columns' widths.
pub struct Shell {
    page: Option<String>,
    query: String,
    inspecting: bool,
    list_open: bool,
    list_width: f32,
    themes_width: f32,
    theme: ThemeChoice,
    drag: Option<Drag>,
    /// A scene's view, kept while it is on screen so its slots survive a
    /// redraw.
    scenes: HashMap<String, Entity<SceneView>>,
}

impl Shell {
    pub fn new(page: Option<String>, theme: &str, cx: &mut App) -> Shell {
        let cut = theme.rfind('-').unwrap_or(theme.len());
        let (family, mode) = theme.split_at(cut);
        let choice = ThemeChoice {
            family: if family.is_empty() { "studio" } else { family }.to_string(),
            dark: mode == "-dark",
        };
        Theme::set(&choice.id(), cx);
        Shell {
            page: page.filter(|id| story(id).is_some()),
            query: String::new(),
            inspecting: true,
            list_open: true,
            list_width: LIST.initial,
            themes_width: THEMES.initial,
            theme: choice,
            drag: None,
            scenes: HashMap::new(),
        }
    }

    fn open(&mut self, page: Option<String>, cx: &mut Context<Self>) {
        self.page = page;
        self.scenes.clear();
        cx.notify();
    }

    fn scene(&mut self, scene: &'static Scene, cx: &mut Context<Self>) -> Entity<SceneView> {
        self.scenes
            .entry(scene.key.clone())
            .or_insert_with(|| SceneView::new(scene, cx))
            .clone()
    }

    fn set_theme(&mut self, theme: ThemeChoice, cx: &mut Context<Self>) {
        Theme::set(&theme.id(), cx);
        self.theme = theme;
        cx.notify();
    }
}

impl Render for Shell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = cx.theme();
        let entity = cx.entity();
        let current = self.page.as_deref().and_then(story);

        // The list.
        let open_page = |page: Option<String>| {
            let entity = entity.clone();
            move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut App| {
                entity.update(cx, |shell, cx| shell.open(page.clone(), cx))
            }
        };
        let search = {
            let entity = entity.clone();
            SearchField::new("search", self.query.clone())
                .size(Size::Small)
                .placeholder("Search")
                .aria_label("Search components")
                .on_value_change(move |query, _, cx| {
                    let query = query.to_string();
                    entity.update(cx, |shell, cx| {
                        shell.query = query;
                        cx.notify();
                    })
                })
        };
        let rows = matching(&self.query).into_iter().map(|item| {
            SidebarMenuItem::new().child(
                SidebarMenuButton::new(ElementId::Name(format!("page-{}", item.id).into()))
                    .size(SidebarMenuButtonSize::Small)
                    .active(Some(item.id.as_str()) == self.page.as_deref())
                    .on_click(open_page(Some(item.id.clone())))
                    .child(item.name.clone()),
            )
        });
        let list = Sidebar::new()
            .width(self.list_width)
            .child(SidebarHeader::new().child(Brand::new(open_page(None))))
            .child(
                SidebarContent::new()
                    .child(SidebarGroup::new().child(search))
                    .child(
                        SidebarGroup::new().child(
                            SidebarMenu::new().child(
                                SidebarMenuItem::new()
                                    .badge(
                                        SidebarMenuBadge::new().child(stories().len().to_string()),
                                    )
                                    .child(
                                        SidebarMenuButton::new("page-overview")
                                            .size(SidebarMenuButtonSize::Small)
                                            .icon(Icon::new("glyph/Home20Regular"))
                                            .active(current.is_none())
                                            .on_click(open_page(None))
                                            .child("Overview"),
                                    ),
                            ),
                        ),
                    )
                    .child(
                        SidebarGroup::new()
                            .child(SidebarGroupLabel::new().child("Components"))
                            .child(
                                SidebarGroupContent::new().child(SidebarMenu::new().children(rows)),
                            ),
                    ),
            );

        // The toolbar.
        let toggle_panel = {
            let entity = entity.clone();
            move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut App| {
                entity.update(cx, |shell, cx| {
                    shell.inspecting = !shell.inspecting;
                    cx.notify();
                })
            }
        };
        let toolbar = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(t.spacing_2))
            .h(px(t.frame_titlebar_size))
            .px(px(t.spacing_4))
            .border_b(px(t.stroke_hairline))
            .border_color(t.color_border.hsla())
            .child(
                Tooltip::new("toggle-sidebar-tip", "Toggle sidebar  ⌘B")
                    .side(PopupSide::Bottom)
                    .child(SidebarTrigger::new("toggle-sidebar")),
            )
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(px(t.spacing_2))
                    .pl(px(t.spacing_1))
                    .child(
                        div()
                            .font_weight(t.label_strong.font_weight())
                            .child(current.map_or("Overview".to_string(), |s| s.name.clone())),
                    )
                    .child(quiet(
                        current.map_or(format!("{} components", stories().len()), |s| {
                            format!("{} examples", s.examples.len())
                        }),
                        cx,
                    )),
            )
            .child(div().flex_1())
            .child(
                Tooltip::new("toggle-panel-tip", "Toggle theme panel")
                    .side(PopupSide::Bottom)
                    .child(
                        IconButton::new("toggle-panel", "Toggle theme panel")
                            .variant(Variant::Plain)
                            .tint(dazzui::Tint::Neutral)
                            .on_click(toggle_panel)
                            .child(Icon::new("glyph/PanelRight20Regular")),
                    ),
            );

        let page: AnyElement = match current {
            Some(story) => self.document(story, cx),
            None => self.overview(window, cx),
        };

        let left_resizer = {
            let entity = entity.clone();
            let width = self.list_width;
            Resizer::new(Side::Left, move |event, _, cx| {
                let start = f32::from(event.position.x);
                entity.update(cx, |shell, _| {
                    shell.drag = Some(Drag {
                        side: Side::Left,
                        start,
                        width,
                    })
                })
            })
        };
        let inset = SidebarInset::new().child(
            div()
                .relative()
                .flex()
                .flex_col()
                .size_full()
                .min_h_0()
                .overflow_hidden()
                .when(self.list_open, |this| this.child(left_resizer))
                .child(toolbar)
                .child(
                    div()
                        .id("document")
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .child(page),
                ),
        );

        let panel = self.inspecting.then(|| {
            let set_theme = {
                let entity = entity.clone();
                Rc::new(move |theme: ThemeChoice, _: &mut Window, cx: &mut App| {
                    entity.update(cx, |shell, cx| shell.set_theme(theme, cx))
                })
            };
            let resizer = {
                let entity = entity.clone();
                let width = self.themes_width;
                Resizer::new(Side::Right, move |event, _, cx| {
                    let start = f32::from(event.position.x);
                    entity.update(cx, |shell, _| {
                        shell.drag = Some(Drag {
                            side: Side::Right,
                            start,
                            width,
                        })
                    })
                })
                .into_any_element()
            };
            theme_panel(
                &self.theme,
                self.themes_width,
                set_theme,
                resizer,
                window,
                cx,
            )
        });

        let on_open_change = {
            let entity = entity.clone();
            move |open: bool, _: &mut Window, cx: &mut App| {
                entity.update(cx, |shell, cx| {
                    shell.list_open = open;
                    cx.notify();
                })
            }
        };
        page_type(div(), cx)
            .id("shell")
            .size_full()
            .bg(t.color_surface.hsla())
            .on_mouse_move(cx.listener(|shell, event: &MouseMoveEvent, _, cx| {
                let Some(drag) = shell.drag else { return };
                if event.pressed_button != Some(MouseButton::Left) {
                    shell.drag = None;
                    return;
                }
                let moved = f32::from(event.position.x) - drag.start;
                match drag.side {
                    Side::Left => shell.list_width = LIST.clamp(drag.width + moved),
                    Side::Right => shell.themes_width = THEMES.clamp(drag.width - moved),
                }
                cx.notify();
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|shell, _, _, _| shell.drag = None),
            )
            .when(self.drag.is_some(), |this| this.cursor_col_resize())
            .child(
                SidebarProvider::new("window")
                    .open(self.list_open)
                    .on_open_change(on_open_change)
                    .child(list)
                    .child(inset)
                    .children(panel),
            )
    }
}

impl Shell {
    fn overview(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let t = cx.theme();
        // `repeat(auto-fill, minmax(16rem, 1fr))`: as many columns of at
        // least 16rem as the document holds, sharing what is left.
        let list = if self.list_open { self.list_width } else { 0.0 };
        let panel = if self.inspecting {
            self.themes_width
        } else {
            0.0
        };
        let room = f32::from(window.viewport_size().width) - list - panel - 2.0 * t.spacing_6;
        let gap = t.spacing_3;
        let columns = ((room + gap) / (256.0 + gap)).floor().max(1.0);
        let tile = ((room - gap * (columns - 1.0)) / columns).floor();

        let entity = cx.entity();
        let tiles: Vec<AnyElement> = matching(&self.query)
            .into_iter()
            .map(|story| {
                let scene = self.scene(&story.preview, cx);
                let id = story.id.clone();
                let entity = entity.clone();
                div()
                    .w(px(tile))
                    .child(
                        Card::new().variant(CardVariant::Outlined).child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(t.spacing_2))
                                .min_w_0()
                                .child(stage(scene.into_any_element(), 128.0))
                                .child(
                                    Button::new(ElementId::Name(
                                        format!("tile-{}", story.id).into(),
                                    ))
                                    .variant(Variant::Plain)
                                    .full_width(true)
                                    .on_click(move |_, _, cx| {
                                        let id = id.clone();
                                        entity.update(cx, |shell, cx| shell.open(Some(id), cx))
                                    })
                                    .child(story.name.clone()),
                                ),
                        ),
                    )
                    .into_any_element()
            })
            .collect();
        page(cx)
            .child(div().flex().flex_wrap().gap(px(gap)).children(tiles))
            .into_any_element()
    }

    fn document(&mut self, story: &'static Story, cx: &mut Context<Self>) -> AnyElement {
        let t = cx.theme();
        let sections: Vec<AnyElement> = story
            .examples
            .iter()
            .map(|example| {
                let scene = self.scene(&example.demo, cx);
                div()
                    .flex()
                    .flex_col()
                    .gap(px(t.spacing_2))
                    .child(SectionLabel::new().child(example.title.clone()))
                    .child(
                        Card::new()
                            .variant(CardVariant::Outlined)
                            .size(Size::Large)
                            .child(stage(scene.into_any_element(), 160.0)),
                    )
                    .children(example.note.clone().map(|note| quiet(note, cx)))
                    .into_any_element()
            })
            .collect();
        page(cx)
            .child(
                div()
                    .max_w(px(640.0))
                    .text_color(t.color_content_secondary.hsla())
                    .child(story.summary.clone()),
            )
            .children(sections)
            .into_any_element()
    }
}

fn page(cx: &App) -> gpui::Div {
    let t = cx.theme();
    div()
        .flex()
        .flex_col()
        .gap(px(t.spacing_6))
        .pt(px(t.spacing_5))
        .px(px(t.spacing_6))
        .pb(px(t.spacing_10))
}

/// Where a scene stands: centred, at least `min_height` tall.
fn stage(scene: AnyElement, min_height: f32) -> AnyElement {
    div()
        .flex()
        .w_full()
        .items_center()
        .justify_center()
        .min_h(px(min_height))
        .child(scene)
        .into_any_element()
}

fn quiet(text: impl Into<gpui::SharedString>, cx: &App) -> AnyElement {
    let t = cx.theme();
    div()
        .text_color(t.color_content_secondary.hsla())
        .text_size(px(t.caption_large.size))
        .child(text.into())
        .into_any_element()
}
