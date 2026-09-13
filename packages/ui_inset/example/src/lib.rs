#![feature(arbitrary_self_types)]
use dazzui_inset as ui;
use inset::*;
use std::{collections::BTreeSet, rc::Rc};
pub mod generated_stories;
pub mod scene_state;
pub mod stories;
pub mod storybook;
#[derive(Debug)]
pub struct DemoState {
    pub checked: bool,
    pub selected: usize,
    pub value: f64,
    pub otp: String,
    pub checks: BTreeSet<String>,
}
#[derive(Debug, Clone)]
pub struct Demo {
    pub state: Entity<DemoState>,
    pub text: Handle<TextEditingController>,
    pub toast: ui::ToastManager,
}
impl Demo {
    pub fn new(app: &mut App) -> Self {
        Self {
            state: app.new_entity(|_| DemoState {
                checked: false,
                selected: 0,
                value: 35.0,
                otp: String::new(),
                checks: BTreeSet::new(),
            }),
            text: TextEditingController::new(app),
            toast: ui::ToastManager::new(app),
        }
    }
}
/// Legacy overviews remain available only until their shared scene takes over.
pub fn legacy_pages() -> Vec<&'static storybook::StoryPage> {
    stories::PAGES
        .iter()
        .filter(|page| {
            let id = storybook::story_id(page.name, "Overview");
            !generated_stories::STORIES
                .iter()
                .any(|(generated, _)| *generated == id)
        })
        .collect()
}
#[derive(Debug)]
pub struct GalleryState {
    pub theme: usize,
    pub page: usize,
}
#[derive(Debug, Clone)]
pub struct Gallery {
    pub state: Entity<GalleryState>,
    pub demo: Demo,
}
impl Gallery {
    pub fn new(app: &mut App) -> Self {
        Self {
            state: app.new_entity(|_| GalleryState { theme: 0, page: 0 }),
            demo: Demo::new(app),
        }
    }
}
impl StatelessWidget for Gallery {
    fn build(&self, app: &mut App, context: BuildContext) -> WidgetRef {
        let state = app.read(&self.state);
        let theme = state.theme;
        let page = state.page;
        let v = ui::THEMES[theme].1;
        let theme_items = ui::THEMES
            .iter()
            .enumerate()
            .map(|(i, (name, _))| {
                let state = self.state.clone();
                let mut item = ui::MenuItem::new(*name).on_select(move |app| {
                    app.update(&state, |s, cx| {
                        s.theme = i;
                        cx.notify();
                    })
                });
                item.checked = Some(i == theme);
                item
            })
            .collect();
        let legacy = legacy_pages();
        let mut nav = vec![
            Text::new("DazzUI / Inset")
                .style(v.title_medium.text_style())
                .into_widget(),
            ui::Menu::new(ui::THEMES[theme].0, theme_items).into_widget(),
        ];
        nav.extend(legacy.iter().enumerate().map(|(i, p)| {
            let state = self.state.clone();
            ui::NavItem::new(p.name)
                .selected(i == page)
                .on_pressed(move |app| {
                    app.update(&state, |s, cx| {
                        s.page = i;
                        cx.notify();
                    })
                })
                .into_widget()
        }));
        nav.extend(
            generated_stories::STORIES
                .iter()
                .enumerate()
                .map(|(i, (_, title))| {
                    let state = self.state.clone();
                    let index = legacy.len() + i;
                    ui::NavItem::new(*title)
                        .selected(index == page)
                        .on_pressed(move |app| {
                            app.update(&state, |s, cx| {
                                s.page = index;
                                cx.notify();
                            })
                        })
                        .into_widget()
                }),
        );
        let content = if page >= legacy.len() {
            let (id, _) = generated_stories::STORIES[page - legacy.len()];
            Center::new()
                .child(generated_stories::build(id).expect("registered generated story"))
                .into_widget()
        } else {
            let entry = legacy[page];
            let render = entry.render;
            let demo = self.demo.clone();
            let specimen = Builder::new(move |app, context| {
                let v = ui::Theme::of(app, context);
                let widget = render(app, &demo);
                Container::new()
                    .width(v.spacing_20 * 5.0)
                    .child(widget)
                    .into_widget()
            });
            storybook::gallery::Overview::new(entry.name, entry.description, specimen).into_widget()
        };
        let main = Row::new()
            .cross_axis_alignment(CrossAxisAlignment::Stretch)
            .children([
                ui::Sidebar::new(nav).into_widget(),
                Expanded::new(
                    Container::new()
                        .color(v.color_surface)
                        .child(SingleChildScrollView::new().child(content)),
                )
                .into_widget(),
            ]);
        let _ = context;
        ui::Theme::new(
            v,
            Stack::new().children([
                main.into_widget(),
                Positioned::new(ui::ToastViewport::new(self.demo.toast.clone()))
                    .right(0.0)
                    .bottom(0.0)
                    .into_widget(),
            ]),
        )
        .into_widget()
    }
}
pub fn host<K>(child: impl IntoWidget<K>) -> WidgetsApp {
    WidgetsApp::new(ui::STUDIO_LIGHT.color_primary.shade(600))
        .title("DazzUI — Inset")
        .debug_show_checked_mode_banner(false)
        .localizations_delegates(vec![
            inset_cupertino::DefaultCupertinoLocalizations::delegate(),
        ])
        .page_route_builder(|app, settings, builder| {
            let route = PageRouteBuilder::new(
                app,
                Rc::new(move |app, context, _, _| builder(app, context)),
            )
            .settings(app, RouteSettingsRef::Settings(settings.clone()));
            PageRoute::as_page_route(route)
        })
        .home(child)
}
