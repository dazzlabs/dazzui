//! The DazzUI playground for GPUI.
//!
//! Six pages of components, drawn with the design system it documents — so a
//! theme switch carries the furniture with it too. It is the counterpart of
//! `packages/ui_flutter/example` and `packages/ui_swiftui/Example`.

mod gallery;
mod pages;

use dazzui::assets::{Assets, icons};
use dazzui::prelude::*;
use dazzui::{
    Badge, BadgeSize, Dialog, DialogBody, DialogFooter, DialogHeader, DialogScrim, Divider, Icon,
    NavItem, SegmentedControl, SegmentedControlSize, Sidebar, SidebarGroup, TextBuffer, Theme,
    ThemeName, Toast, ToastViewport, themed,
};
use dazzui::{Button, ControlVariant, Tint};
use gpui::{
    AnyElement, App, Application, Bounds, Context, Entity, IntoElement, ParentElement, Render,
    SharedString, Styled, Window, WindowBounds, WindowOptions, div, px, size,
};

/// The pages, in the order the sidebar lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Controls,
    Forms,
    Data,
    Feedback,
    Settings,
    Themes,
}

impl Page {
    const ALL: [Page; 6] = [
        Page::Controls,
        Page::Forms,
        Page::Data,
        Page::Feedback,
        Page::Settings,
        Page::Themes,
    ];

    fn title(self) -> &'static str {
        match self {
            Page::Controls => "Controls",
            Page::Forms => "Forms",
            Page::Data => "Data",
            Page::Feedback => "Feedback",
            Page::Settings => "Settings",
            Page::Themes => "Themes",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Page::Controls => icons::SQUARE_MULTIPLE_16_REGULAR,
            Page::Forms => icons::CHECKBOX_CHECKED_16_REGULAR,
            Page::Data => icons::TABLE_16_REGULAR,
            Page::Feedback => icons::ALERT_16_REGULAR,
            Page::Settings => icons::SETTINGS_16_REGULAR,
            Page::Themes => icons::PAINT_BRUSH_16_REGULAR,
        }
    }
}

/// The playground's whole state.
///
/// One view holds it because the pages are specimens rather than screens: a
/// page has no state a neighbouring page could not read, and threading six
/// entities through the window would say otherwise.
pub struct Playground {
    pub page: Page,
    pub theme: ThemeName,
    pub dialog: bool,
    pub toast: bool,

    // Controls
    pub pinned: bool,
    pub tab: SharedString,
    pub mode: SharedString,

    // Forms
    pub name: Entity<TextBuffer>,
    pub email: Entity<TextBuffer>,
    pub notes: Entity<TextBuffer>,
    pub search: Entity<TextBuffer>,
    pub region: SharedString,
    pub newsletter: bool,
    pub telemetry: bool,
    pub plan: SharedString,
    pub notify: bool,
    pub appearance_card: SharedString,

    // Data
    pub build: SharedString,

    // Settings
    pub launch: bool,
    pub updates: bool,
    pub sounds: bool,
    pub appearance: SharedString,
    pub palette_shortcut: SharedString,
}

impl Playground {
    fn new(cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| TextBuffer::new(cx, "Acme").with_value("DazzUI"));
        let email = cx.new(|cx| TextBuffer::new(cx, "you@example.com"));
        let notes =
            cx.new(|cx| TextBuffer::new(cx, "Anything the team should know…").multi_line(true));
        let search = cx.new(|cx| TextBuffer::new(cx, "Search components"));

        // A field edits itself; the page it sits on has to hear about it to
        // redraw around it.
        for field in [&name, &email, &notes, &search] {
            cx.observe(field, |_, _, cx| cx.notify()).detach();
        }

        Playground {
            page: Page::Controls,
            theme: ThemeName::StudioLight,
            dialog: false,
            toast: false,
            pinned: true,
            tab: "all".into(),
            mode: "design".into(),
            name,
            email,
            notes,
            search,
            region: "us".into(),
            newsletter: true,
            telemetry: false,
            plan: "team".into(),
            notify: true,
            appearance_card: "auto".into(),
            build: "aurora".into(),
            launch: true,
            updates: true,
            sounds: false,
            appearance: "auto".into(),
            palette_shortcut: "⌘K".into(),
        }
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

impl Render for Playground {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                .child(self.toolbar(&this, cx))
                .child(Divider::horizontal())
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .min_h_0()
                        .child(self.navigator(&this, cx))
                        .child(self.page_host(&this, cx)),
                )
                // The overlays sit outside the scrolled column so the scrim
                // covers the window, which is what a portal target buys the
                // React package.
                .when(self.dialog, |el| el.child(self.dialog_overlay(&this)))
                .when(self.toast, |el| el.child(self.toast_overlay(&this))),
        )
    }
}

impl Playground {
    fn toolbar(&self, this: &Entity<Self>, _cx: &mut Context<Self>) -> AnyElement {
        let vars = self.theme.variables();
        let mut themes = SegmentedControl::new("theme", self.theme.attribute())
            .size(SegmentedControlSize::Small)
            .on_select(act_with(this, |state, value: &SharedString, _, _| {
                state.theme = ThemeName::ALL
                    .into_iter()
                    .find(|name| name.attribute() == value.as_ref())
                    .unwrap_or(ThemeName::StudioLight);
            }));
        for name in ThemeName::ALL {
            themes = themes.item(name.attribute(), name.label());
        }

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
            .child(themes)
            .into_any_element()
    }

    fn navigator(&self, this: &Entity<Self>, _cx: &mut Context<Self>) -> AnyElement {
        let mut group = SidebarGroup::new().label("Components").first(true);
        for page in Page::ALL {
            group = group.child(
                NavItem::new(page.title(), page.title())
                    .icon(Icon::new(page.icon()))
                    .active(self.page == page)
                    .on_click(act(this, move |state, _, _| state.page = page)),
            );
        }

        Sidebar::new("navigator")
            .resizable(true)
            .child(group)
            .into_any_element()
    }

    fn page_host(&self, this: &Entity<Self>, cx: &mut Context<Self>) -> AnyElement {
        let vars = self.theme.variables();

        div()
            .id("page")
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(vars.spacing_6)
            .p(vars.spacing_6)
            .overflow_y_scroll()
            .bg(vars.color_surface)
            .children(match self.page {
                Page::Controls => pages::controls(self, this, cx),
                Page::Forms => pages::forms(self, this, cx),
                Page::Data => pages::data(self, this, cx),
                Page::Feedback => pages::feedback(self, this, cx),
                Page::Settings => pages::settings(self, this, cx),
                Page::Themes => pages::themes(self, this, cx),
            })
            .into_any_element()
    }

    fn dialog_overlay(&self, this: &Entity<Self>) -> AnyElement {
        DialogScrim::new()
            .child(
                Dialog::new()
                    .child(DialogHeader::new("Discard these changes?").subtitle(
                        "The three files you edited will go back to their last saved state.",
                    ))
                    .child(DialogBody::new().child("This cannot be undone."))
                    .child(
                        DialogFooter::new()
                            .child(
                                Button::new("keep", "Keep editing")
                                    .on_click(dismiss(this, |state| state.dialog = false)),
                            )
                            .child(
                                Button::new("discard", "Discard")
                                    .variant(ControlVariant::Filled)
                                    .tint(Tint::Danger)
                                    .on_click(dismiss(this, |state| state.dialog = false)),
                            ),
                    ),
            )
            .into_any_element()
    }

    fn toast_overlay(&self, this: &Entity<Self>) -> AnyElement {
        ToastViewport::new()
            .child(
                Toast::new("toast", "Tokens rebuilt — three packages refreshed.")
                    .tint(Tint::Success)
                    .on_dismiss(act(this, |state, _, _| state.toast = false)),
            )
            .into_any_element()
    }
}

/// A button's click, which carries the event this state does not need.
fn dismiss(
    entity: &Entity<Playground>,
    f: impl Fn(&mut Playground) + 'static,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static {
    let entity = entity.clone();
    move |_, _, cx| {
        entity.update(cx, |this, cx| {
            f(this);
            cx.notify();
        });
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        Theme::set(ThemeName::StudioLight, cx);

        let bounds = Bounds::centered(None, size(px(1080.), px(720.)), cx);
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
