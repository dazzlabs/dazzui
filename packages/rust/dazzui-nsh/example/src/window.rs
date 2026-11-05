use crate::runtime::scene::{Scene, Story};
use dazzui_nsh::{prelude::*, styles};
use nsh_core::element::{AnyElement, scroll};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize)]
struct Group {
    title: String,
    families: Vec<Family>,
}
#[derive(Deserialize)]
struct Family {
    value: String,
    title: String,
}

pub fn gallery() -> impl IntoElement {
    let theme = Theme::new("bright-light");
    provide_context(theme);
    let selected = signal(String::new());
    let query = signal(SharedString::default());
    let dark = signal(false);
    let stories: Rc<Vec<Story>> = Rc::new(
        crate::stories::STORIES
            .iter()
            .map(|s| serde_json::from_str(s).expect("generated story"))
            .collect(),
    );
    let navigation = stories.clone();
    let pages = stories.clone();
    let title_stories = stories.clone();
    view()
        .size_full()
        .flex_row()
        .style_with(move |s| {
            styles::typography(s, theme.tokens().body_medium)
                .bg(theme.tokens().color_surface.hsla())
                .text_color(theme.tokens().color_content.hsla())
        })
        .child(
            view()
                .flex_col()
                .flex_shrink_0()
                .h_full()
                .w(move || px(theme.tokens().frame_sidebar_width))
                .bg(move || theme.tokens().color_surface_column.hsla())
                .child(
                    view()
                        .h(move || px(theme.tokens().frame_titlebar_size))
                        .px(move || px(theme.tokens().spacing_4))
                        .flex_row()
                        .items_center()
                        .gap(move || px(theme.tokens().spacing_2))
                        .child(
                            text("DazzUI").style_with(move |s| {
                                styles::typography(s, theme.tokens().title_small)
                            }),
                        )
                        .child(
                            text("NSH")
                                .text_color(move || theme.tokens().color_content_faint.hsla()),
                        ),
                )
                .child(
                    view()
                        .px(move || px(theme.tokens().spacing_3))
                        .pb(move || px(theme.tokens().spacing_3))
                        .child(
                            text_field()
                                .size(dazzui_nsh::Size::Small)
                                .placeholder("Search components")
                                .accessibility_label("Search components")
                                .value(move || Some(query.get()))
                                .on_change(move |v: &SharedString| query.set(v.clone())),
                        ),
                )
                .child(
                    scroll().flex_1().min_h(px(0.0)).child(
                        view()
                            .p(move || px(theme.tokens().spacing_2))
                            .gap(move || px(theme.tokens().spacing_0_5))
                            .child(nav_button("Overview", "", selected, theme))
                            .child(dynamic(move || {
                                navigation
                                    .iter()
                                    .filter(|s| {
                                        s.name.to_lowercase().contains(&query.get().to_lowercase())
                                    })
                                    .map(|s| nav_button(&s.name, &s.id, selected, theme))
                                    .collect::<Vec<_>>()
                            })),
                    ),
                ),
        )
        .child(divider().vertical(true))
        .child(
            view()
                .flex_1()
                .min_w(px(0.0))
                .h_full()
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .h(move || px(theme.tokens().frame_titlebar_size))
                        .px(move || px(theme.tokens().spacing_4))
                        .gap(move || px(theme.tokens().spacing_2))
                        .child(
                            text(move || {
                                title_stories
                                    .iter()
                                    .find(|s| s.id == selected.get())
                                    .map(|s| s.name.clone())
                                    .unwrap_or_else(|| "Overview".into())
                            })
                            .style_with(move |s| {
                                styles::typography(s, theme.tokens().label_medium)
                            }),
                        )
                        .child(view().flex_1())
                        .child(
                            text("Native Shell · Rust")
                                .text_color(move || theme.tokens().color_content_secondary.hsla()),
                        ),
                )
                .child(divider())
                .child(scroll().flex_1().min_h(px(0.0)).child(dynamic(move || {
                    if let Some(story) = pages.iter().find(|s| s.id == selected.get()) {
                        document(story, theme)
                    } else {
                        overview(&pages, selected, theme)
                    }
                }))),
        )
        .child(divider().vertical(true))
        .child(theme_panel(theme, dark))
}
fn nav_button(title: &str, id: &str, selected: Signal<String>, theme: Theme) -> AnyElement {
    let id = id.to_owned();
    let active = id.clone();
    let test = format!("Open {title}");
    button()
        .variant(move || {
            if selected.get() == active {
                Variant::Tinted
            } else {
                Variant::Plain
            }
        })
        .tint(Tint::Neutral)
        .full_width(true)
        .style_with(move |s| s.justify_start().h(px(theme.tokens().control_medium_size)))
        .on_press(move || selected.set(id.clone()))
        .child(text(title.to_owned()))
        .accessibility_label(test)
        .into_element()
}
fn overview(stories: &[Story], selected: Signal<String>, theme: Theme) -> AnyElement {
    let cards = stories
        .iter()
        .map(|story| {
            let id = story.id.clone();
            card()
                .variant(dazzui_nsh::CardVariant::Outlined)
                .w(px(256.0))
                .flex_grow(1.0)
                .child(
                    button()
                        .variant(Variant::Plain)
                        .full_width(true)
                        .on_press(move || selected.set(id.clone()))
                        .child(text(story.name.clone())),
                )
                .child(
                    view()
                        .min_h(px(128.0))
                        .items_center()
                        .justify_center()
                        .overflow_hidden()
                        .child(story.preview.render()),
                )
                .into_element()
        })
        .collect::<Vec<_>>();
    view()
        .p(move || px(theme.tokens().spacing_6))
        .gap(move || px(theme.tokens().spacing_6))
        .child(
            text("One design. Native controls.")
                .style_with(move |s| styles::typography(s, theme.tokens().title_large)),
        )
        .child(
            text(format!(
                "{} component pages · every DazzUI theme",
                stories.len()
            ))
            .text_color(move || theme.tokens().color_content_secondary.hsla()),
        )
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap(move || px(theme.tokens().spacing_3))
                .child(cards),
        )
        .into_element()
}
fn document(story: &Story, theme: Theme) -> AnyElement {
    view()
        .p(move || px(theme.tokens().spacing_6))
        .gap(move || px(theme.tokens().spacing_6))
        .child(
            text(story.summary.clone())
                .text_color(move || theme.tokens().color_content_secondary.hsla()),
        )
        .child(
            story
                .examples
                .iter()
                .map(|scene| section(scene, theme))
                .collect::<Vec<_>>(),
        )
        .into_element()
}
fn section(scene: &Scene, theme: Theme) -> AnyElement {
    view()
        .gap(move || px(theme.tokens().spacing_2))
        .child(
            text(scene.title.clone())
                .style_with(move |s| styles::typography(s, theme.tokens().label_medium)),
        )
        .child(scene.note.as_ref().map(|note| {
            text(note.clone()).text_color(move || theme.tokens().color_content_secondary.hsla())
        }))
        .child(
            card()
                .variant(dazzui_nsh::CardVariant::Outlined)
                .min_h(px(160.0))
                .items_center()
                .justify_center()
                .child(scene.render()),
        )
        .into_element()
}
fn theme_panel(theme: Theme, dark: Signal<bool>) -> AnyElement {
    let groups: Vec<Group> =
        serde_json::from_str(include_str!("generated/themes.json")).expect("theme families");
    view().w(move||px(theme.tokens().frame_aside_width)).h_full().flex_shrink_0().bg(move||theme.tokens().color_surface_column.hsla())
        .child(view().h(move||px(theme.tokens().frame_titlebar_size)).px(move||px(theme.tokens().spacing_4)).flex_row().items_center().justify_between()
            .child(text("Theme").style_with(move|s|styles::typography(s,theme.tokens().label_medium)))
            .child(switch().size(dazzui_nsh::Size::Small).accessibility_label("Dark appearance").checked(move || Some(dark.get())).on_change(move|value:&bool|{
                dark.set(*value);let name=theme.name();let family=name.strip_suffix("-light").or_else(||name.strip_suffix("-dark")).unwrap_or(name);theme.set(&format!("{family}-{}",if *value{"dark"}else{"light"}));
            })))
        .child(divider()).child(scroll().flex_1().min_h(px(0.0)).child(view().p(move||px(theme.tokens().spacing_3)).gap(move||px(theme.tokens().spacing_4)).child(groups.into_iter().map(|group|{
            view().gap(move||px(theme.tokens().spacing_2)).child(text(group.title).text_color(move||theme.tokens().color_content_secondary.hsla()))
                .child(view().grid().grid_cols(3).gap(move||px(theme.tokens().spacing_2)).child(group.families.into_iter().map(|family|{
                    let name=family.value.clone();let target=family.value.clone();let current=family.value.clone();
                    let palette=computed(move||Theme::all().iter().find(|entry|entry.name==format!("{name}-{}",if dark.get(){"dark"}else{"light"})).expect("generated theme").tokens);
                    let test=format!("{} theme",family.title);
                    nsh_base::button().accessibility_label(family.title.clone()).accessibility_label(test).gap(move||px(theme.tokens().spacing_1)).p(move||px(theme.tokens().spacing_1))
                        .rounded(move||px(theme.tokens().radius_large)).border(move||px(theme.tokens().stroke_control))
                        .border_color(move||if theme.name().starts_with(&format!("{current}-")){theme.tokens().control_color_filled_surface_normal.resolve(&theme.tokens().color_primary)}else{dazzui_nsh::tokens::Color::TRANSPARENT.hsla()})
                        .on_press(move||{theme.set(&format!("{target}-{}",if dark.get(){"dark"}else{"light"}));})
                        .child(view().h(px(44.0)).w_full().flex_row().overflow_hidden().rounded(move||px(palette.get().radius_large)).border(move||px(palette.get().stroke_hairline)).border_color(move||palette.get().color_border.hsla()).bg(move||palette.get().color_surface.hsla())
                            .child(view().w(px(14.0)).h_full().bg(move||palette.get().color_surface_column.hsla()))
                            .child(view().flex_1().p(px(6.0)).gap(px(4.0)).child(view().h(px(4.0)).w_full().bg(move||palette.get().color_content.hsla()))
                                .child(view().h(px(12.0)).w_full().rounded(move||px(palette.get().radius_tiny)).bg(move||palette.get().control_color_filled_surface_normal.resolve(&palette.get().color_primary)))))
                        .child(text(family.title).w_full().min_w(px(0.0)).truncate().text_size(move||px(theme.tokens().caption_small.size))).into_element()
                }).collect::<Vec<_>>())).into_element()
        }).collect::<Vec<_>>()))).into_element()
}
