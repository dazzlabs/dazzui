//! The DazzUI playground for GPUI.
//!
//! Organised the way `apps/storybook` is — one page per component, reached
//! from a story tree — and drawn with the design system it documents, so a
//! theme switch carries the furniture with it too. It is the counterpart of
//! `packages/ui_flutter/example` and `packages/ui_swiftui/Example`.

mod stories;
mod storybook;

#[cfg(test)]
mod stories_test;

use std::collections::HashSet;

use dazzui::prelude::*;
use dazzui::{
    ActiveTheme, Badge, BadgeSize, Divider, NavItem, NavItemSize, SearchField, SegmentedControl,
    SegmentedControlSize, Sidebar, SidebarGroup, TextBuffer, Theme, ThemeName, themed,
};
use gpui::{
    AnyElement, App, Application, Bounds, Context, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, WindowBounds, WindowOptions, div, px, relative, size,
};
use storybook::{StoryLayout, StoryRef};

/// The playground's whole state.
///
/// One view holds it because the stories are specimens rather than screens: a
/// story has no state a neighbouring story could not read, and threading an
/// entity per component through the window would say otherwise.
pub struct Playground {
    /// Where the canvas is pointed.
    pub selection: StoryRef,
    /// Which pages the tree has open. Storybook opens the page you select and
    /// leaves the rest as they were, so this is a set rather than one name.
    pub opened: HashSet<&'static str>,
    /// The tree's filter.
    pub filter: Entity<TextBuffer>,
    pub theme: ThemeName,

    // What the interactive stories remember between frames.
    pub dialog: bool,
    pub toast: bool,
    pub pinned: bool,
    pub tab: SharedString,
    pub mode: SharedString,
    pub nav: SharedString,
    pub rail: SharedString,
    pub library: SharedString,
    pub build: SharedString,
    pub name: Entity<TextBuffer>,
    pub email: Entity<TextBuffer>,
    pub notes: Entity<TextBuffer>,
    pub search: Entity<TextBuffer>,
    /// What the search field currently holds, mirrored so a story can filter
    /// against it without a context to read the buffer with.
    pub searched: String,
    pub region: SharedString,
    pub newsletter: bool,
    pub telemetry: bool,
    pub plan: SharedString,
    pub notify: bool,
    pub appearance_card: SharedString,
    pub launch: bool,
    pub updates: bool,
    pub sounds: bool,
    pub appearance: SharedString,
    pub palette_shortcut: SharedString,
    pub hidden_files: bool,
    pub show_sidebar: bool,
    pub progress: f32,
    pub reached: usize,
    pub notified: Vec<SharedString>,
}

impl Playground {
    fn new(cx: &mut Context<Self>) -> Self {
        let filter = cx.new(|cx| TextBuffer::new(cx, "Find a component"));
        let name = cx.new(|cx| TextBuffer::new(cx, "Acme").with_value("DazzUI"));
        let email = cx.new(|cx| TextBuffer::new(cx, "you@example.com").with_value("you@"));
        let notes =
            cx.new(|cx| TextBuffer::new(cx, "Anything the team should know…").multi_line(true));
        let search = cx.new(|cx| TextBuffer::new(cx, "Search components"));

        // A field edits itself; the page it sits on has to hear about it to
        // redraw around it.
        for field in [&filter, &name, &email, &notes] {
            cx.observe(field, |_, _, cx| cx.notify()).detach();
        }
        // The search field is also read back by the stories that filter a
        // list with it, so its value is mirrored on the way past.
        cx.observe(&search, |this, buffer, cx| {
            this.searched = buffer.read(cx).value().to_string();
            cx.notify();
        })
        .detach();

        Playground {
            selection: stories::FIRST,
            opened: HashSet::from([stories::FIRST.page]),
            filter,
            theme: ThemeName::StudioLight,
            dialog: false,
            toast: false,
            pinned: true,
            tab: "all".into(),
            mode: "design".into(),
            nav: "projects".into(),
            rail: "studio-light".into(),
            library: "projects".into(),
            build: "aurora".into(),
            name,
            email,
            notes,
            search,
            searched: String::new(),
            region: "us".into(),
            newsletter: true,
            telemetry: false,
            plan: "team".into(),
            notify: true,
            appearance_card: "auto".into(),
            launch: true,
            updates: true,
            sounds: false,
            appearance: "auto".into(),
            palette_shortcut: "⌘K".into(),
            hidden_files: true,
            show_sidebar: false,
            progress: 15.,
            reached: 1,
            notified: vec!["aurora".into()],
        }
    }

    /// The tree's filter, as a lowercase needle.
    fn needle(&self, cx: &App) -> String {
        self.filter.read(cx).value().trim().to_lowercase()
    }

    /// The same for the search field a story filters a list with. It is read
    /// from the story rather than from the window, so the buffer has to be
    /// reachable without a context.
    pub fn search_needle(&self) -> String {
        self.searched.trim().to_lowercase()
    }
}

/// A listener that only has to reach the playground's own state.
///
/// GPUI's `cx.listener` carries an event; most of this system's callbacks
/// carry nothing, so this is the same thing without one.
pub fn act<F>(entity: &Entity<Playground>, f: F) -> impl Fn(&mut Window, &mut App) + 'static
where
    F: Fn(&mut Playground, &mut Window, &mut Context<Playground>) + 'static,
{
    let entity = entity.clone();
    move |window, cx| {
        entity.update(cx, |this, cx| f(this, window, cx));
        cx.refresh_windows();
    }
}

/// The same, for a callback that hands back a value.
pub fn act_with<T, F>(
    entity: &Entity<Playground>,
    f: F,
) -> impl Fn(&T, &mut Window, &mut App) + 'static
where
    T: Clone + 'static,
    F: Fn(&mut Playground, &T, &mut Window, &mut Context<Playground>) + 'static,
{
    let entity = entity.clone();
    move |value, window, cx| {
        entity.update(cx, |this, cx| f(this, value, window, cx));
        cx.refresh_windows();
    }
}

/// A button's click, which carries the event this state does not need.
pub fn on_click(
    entity: &Entity<Playground>,
    f: impl Fn(&mut Playground) + 'static,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static {
    let entity = entity.clone();
    move |_, _, cx| {
        entity.update(cx, |this, cx| {
            f(this);
            cx.notify();
        });
        cx.refresh_windows();
    }
}

impl Render for Playground {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        #[cfg(test)]
        tests::RENDERS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let theme = self.theme;
        let vars = theme.variables();
        let this = cx.entity();

        themed(
            theme,
            div()
                .relative()
                .size_full()
                .flex()
                .flex_col()
                .bg(vars.color_canvas)
                .text_color(vars.color_content)
                .type_style(vars.body_medium)
                .child(self.toolbar(&this))
                .child(Divider::horizontal())
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .min_h_0()
                        .child(self.navigator(&this, cx))
                        .child(self.canvas(&this, cx)),
                ),
        )
    }
}

/// The six families, in the order the toolbar lists them. The id is the
/// prefix of a theme's `data-theme` attribute, which is also the mode name in
/// `terrazzo.config.mjs`.
const FAMILIES: [(&str, &str); 6] = [
    ("studio", "Studio"),
    ("bright", "Bright"),
    ("frost", "Frost"),
    ("graphite", "Graphite"),
    ("ember", "Ember"),
    ("nocturne", "Nocturne"),
];

fn family_of(name: ThemeName) -> &'static str {
    FAMILIES
        .into_iter()
        .map(|(id, _)| id)
        .find(|id| name.attribute().starts_with(id))
        .unwrap_or("studio")
}

/// The one theme with this family and this brightness.
fn theme_for(family: &str, dark: bool) -> ThemeName {
    ThemeName::ALL
        .into_iter()
        .find(|name| family_of(*name) == family && name.is_dark() == dark)
        .unwrap_or(ThemeName::StudioLight)
}

impl Playground {
    fn toolbar(&self, this: &Entity<Self>) -> AnyElement {
        let vars = self.theme.variables();
        // A family and a brightness rather than ten names: that is the shape
        // of the graph — a family is a mode, `dark` is a mode — and ten names
        // would not fit a titlebar in any case.
        let active = self.theme;
        let mut families = SegmentedControl::new("theme-family", family_of(active))
            .size(SegmentedControlSize::Small)
            .on_select(act_with(this, |state, value: &SharedString, _, _| {
                state.theme = theme_for(value.as_ref(), state.theme.is_dark());
            }));
        for (id, label) in FAMILIES {
            families = families.item(id, label);
        }

        let brightness = SegmentedControl::new(
            "theme-brightness",
            if active.is_dark() { "dark" } else { "light" },
        )
        .size(SegmentedControlSize::Small)
        .item("light", "Light")
        .item("dark", "Dark")
        .on_select(act_with(this, |state, value: &SharedString, _, _| {
            state.theme = theme_for(family_of(state.theme), value.as_ref() == "dark");
        }));

        div()
            .flex()
            .items_center()
            .gap(vars.spacing_3)
            .flex_none()
            .h(vars.frame_titlebar_size)
            .px(vars.spacing_4)
            .bg(vars.color_surface_muted)
            .child(
                div()
                    .type_style(vars.title_small)
                    .text_color(vars.color_content)
                    .child("DazzUI"),
            )
            .child(Badge::new("GPUI").size(BadgeSize::Small))
            .child(div().flex_1())
            .child(families)
            .child(brightness)
            .into_any_element()
    }

    /// The story tree: the two groups, the pages inside them, and a page's own
    /// stories under it once it is open.
    ///
    /// It is the package's own `Sidebar`, `NavItem` and `SearchField` — the
    /// playground is built out of the system it documents.
    fn navigator(&self, this: &Entity<Self>, cx: &mut Context<Self>) -> AnyElement {
        let vars = cx.theme().vars;
        let needle = self.needle(cx);
        let mut sidebar = Sidebar::new("navigator")
            .resizable(true)
            .header(SearchField::new("filter", &self.filter));

        for group in stories::GROUPS {
            let mut rows = SidebarGroup::new()
                .label(group.name)
                .first(group.name == stories::GROUPS[0].name);
            let mut any = false;

            for page in group.pages {
                let page_matches = page.name.to_lowercase().contains(&needle);
                // A page matches by its own name — and then keeps all of its
                // stories, since the reader was looking for the component — or
                // by any story's, in which case only those are worth showing.
                let stories: Vec<_> = page
                    .stories
                    .iter()
                    .filter(|story| {
                        needle.is_empty()
                            || page_matches
                            || story.name.to_lowercase().contains(&needle)
                    })
                    .collect();
                if stories.is_empty() {
                    continue;
                }
                any = true;

                let name = page.name;
                let first = page.stories.first().map(|story| story.name);
                rows = rows.child(
                    NavItem::new(SharedString::from(format!("page-{name}")), name)
                        .active(self.selection.page == name)
                        .on_click(act(this, move |state, _, _| {
                            // Selecting a page opens it on its `Overview`,
                            // which is the one story that says what the
                            // component is. Selecting it again folds it away.
                            if state.opened.contains(name) && state.selection.page == name {
                                state.opened.remove(name);
                                return;
                            }
                            state.opened.insert(name);
                            if let Some(story) = first {
                                state.selection = StoryRef::new(name, story);
                            }
                        })),
                );

                // A filtered list is already the answer to the question, so it
                // opens itself; without a filter a page is open because it was
                // asked for.
                if needle.is_empty() && !self.opened.contains(name) {
                    continue;
                }

                for story in stories {
                    let reference = StoryRef::new(name, story.name);
                    rows = rows.child(
                        // The story rows hang off their page rather than
                        // sitting level with it, which is the whole of what
                        // makes the list a tree.
                        div().pl(vars.spacing_4).child(
                            NavItem::new(
                                SharedString::from(format!("story-{name}-{}", story.name)),
                                story.name,
                            )
                            .size(NavItemSize::Small)
                            .active(self.selection == reference)
                            .on_click(act(this, move |state, _, _| {
                                state.selection = reference;
                            })),
                        ),
                    );
                }
            }

            if any {
                sidebar = sidebar.child(rows);
            }
        }

        sidebar.into_any_element()
    }

    /// One story, presented the way its layout asks for.
    fn canvas(&self, this: &Entity<Self>, cx: &mut Context<Self>) -> AnyElement {
        let vars = cx.theme().vars;
        let story = storybook::story(self.selection);

        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .bg(vars.color_surface)
            .child(self.breadcrumb(cx))
            .child(Divider::horizontal())
            .child(match story {
                None => div().flex_1().child("No story selected").into_any_element(),
                Some(story) => {
                    let content = (story.render)(self, this, cx);
                    match story.layout {
                        // A bare specimen sits in the middle of the canvas
                        // rather than in its corner, which is what
                        // `layout: 'centered'` buys a component file.
                        StoryLayout::Centered => div()
                            .id("canvas")
                            .flex_1()
                            .min_h_0()
                            .overflow_y_scroll()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .w_full()
                                    .min_h(relative(1.))
                                    .p(vars.spacing_8)
                                    .child(content),
                            )
                            .into_any_element(),
                        // The page brings its own padding, as
                        // `layout: 'fullscreen'` does on an `Overview`.
                        StoryLayout::Page => div()
                            .id("canvas")
                            .flex_1()
                            .min_h_0()
                            .overflow_y_scroll()
                            .child(content)
                            .into_any_element(),
                        // A scrim or a toast viewport is fixed to the canvas
                        // the way it would be to the viewport, so it takes the
                        // whole of it, unscrolled.
                        StoryLayout::Fullscreen => div()
                            .relative()
                            .flex_1()
                            .min_h_0()
                            .child(content)
                            .into_any_element(),
                    }
                }
            })
            .into_any_element()
    }

    /// Where the canvas is pointed, in words.
    fn breadcrumb(&self, cx: &mut Context<Self>) -> AnyElement {
        let vars = cx.theme().vars;

        div()
            .flex()
            .items_center()
            .gap(vars.spacing_2)
            .flex_none()
            .h(vars.frame_titlebar_size)
            .px(vars.spacing_4)
            .bg(vars.color_surface_muted)
            .child(
                div()
                    .type_style(vars.label_medium)
                    .text_color(vars.color_content)
                    .child(self.selection.page),
            )
            .child(
                div()
                    .type_style(vars.caption_medium)
                    .text_color(vars.color_content_faint)
                    .child("/"),
            )
            .child(
                div()
                    .type_style(vars.caption_medium)
                    .text_color(vars.color_content_muted)
                    .child(self.selection.story),
            )
            .into_any_element()
    }
}

fn main() {
    Application::new()
        .with_assets(dazzui::assets::Assets)
        .run(|cx: &mut App| {
            Theme::set(ThemeName::StudioLight, cx);

            let bounds = Bounds::centered(None, size(px(1180.), px(780.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("DazzUI Playground".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(Playground::new),
            )
            .unwrap();
            cx.activate(true);
        });
}

/// The story index is the coverage: every story, in every theme, laid out and
/// painted headlessly. It is the counterpart of the Flutter example's widget
/// test, and it is what catches a story nobody has opened lately asking for a
/// bad length or losing its element state.
#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use gpui::TestAppContext;

    use super::*;

    /// How many times the playground has drawn, so the test can tell a window
    /// that drew from one that never got round to it.
    pub(super) static RENDERS: AtomicUsize = AtomicUsize::new(0);

    #[gpui::test]
    fn every_story_draws(cx: &mut TestAppContext) {
        for theme in ThemeName::ALL {
            let (playground, cx) = cx.add_window_view(|_, cx| Playground::new(cx));

            for group in stories::GROUPS {
                for page in group.pages {
                    for story in page.stories {
                        let drawn = RENDERS.load(Ordering::SeqCst);
                        playground.update(cx, |state, cx| {
                            state.theme = theme;
                            state.selection = StoryRef::new(page.name, story.name);
                            cx.notify();
                        });
                        cx.run_until_parked();

                        assert!(
                            RENDERS.load(Ordering::SeqCst) > drawn,
                            "{} / {} / {} never drew",
                            theme.label(),
                            page.name,
                            story.name
                        );
                    }
                }
            }
        }
    }

    /// Every story the sidebar lists is one the canvas can find, which is what
    /// keeps a renamed story from becoming an empty canvas.
    #[gpui::test]
    fn every_story_is_reachable(_cx: &mut TestAppContext) {
        for group in stories::GROUPS {
            for page in group.pages {
                assert!(!page.stories.is_empty(), "{} has no stories", page.name);
                for story in page.stories {
                    let reference = StoryRef::new(page.name, story.name);
                    assert!(
                        storybook::story(reference).is_some(),
                        "{} / {} is not in the index",
                        page.name,
                        story.name
                    );
                }
            }
        }

        assert!(
            storybook::story(stories::FIRST).is_some(),
            "the window opens on a story that is not in the index"
        );
    }
}
