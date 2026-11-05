use super::{brand::brand, stage::stage, theme_panel::ThemePanel};
use crate::{stories, support::story::Story};
use dazzui_gtk4::{gtk, prelude::*, *};
use std::rc::Rc;

struct Context {
    stack: gtk::Stack,
    title: gtk::Label,
    count: gtk::Label,
    list: gtk::ListBox,
    grid: gtk::FlowBox,
}
impl Context {
    fn open(&self, id: Option<&str>) {
        let story = id.and_then(stories::story_of);
        if let Some(story) = story {
            if self.stack.child_by_name(&story.id).is_none() {
                self.stack.add_named(&document(story), Some(&story.id));
            }
            self.stack.set_visible_child_name(&story.id);
            self.title.set_text(&story.name);
            self.count
                .set_text(&format!("{} examples", story.examples.len()));
        } else {
            self.stack.set_visible_child_name("overview");
            self.title.set_text("Overview");
            self.count
                .set_text(&format!("{} components", stories::all().len()));
        }
        let id = story.map(|s| s.id.as_str()).unwrap_or("overview");
        let mut child = self.list.first_child();
        while let Some(w) = child {
            child = w.next_sibling();
            if w.widget_name() == id {
                self.list.select_row(w.downcast_ref::<gtk::ListBoxRow>());
                break;
            }
        }
    }
    fn filter(&self, query: &str) {
        let ids: Vec<_> = stories::matching(query)
            .iter()
            .map(|s| s.id.as_str())
            .collect();
        for parent in [
            self.list.upcast_ref::<gtk::Widget>(),
            self.grid.upcast_ref(),
        ] {
            let mut child = parent.first_child();
            while let Some(w) = child {
                child = w.next_sibling();
                w.set_visible(
                    w.widget_name() == "overview"
                        || w.widget_name() == "components-heading"
                        || ids.contains(&w.widget_name().as_str()),
                );
            }
        }
    }
}
pub struct Shell {
    pub widget: gtk::Paned,
    pub search: gtk::SearchEntry,
    pub themes: ThemePanel,
    pub list_toggle: gtk::Button,
    pub theme_toggle: gtk::Button,
    context: Rc<Context>,
}
impl Shell {
    pub fn new(window: &gtk::ApplicationWindow, page: Option<&str>, theme: Theme) -> Self {
        theme.apply(window);
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("shell.css"));
        gtk::style_context_add_provider_for_display(
            &gtk::prelude::WidgetExt::display(window),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        let list_column = box_(gtk::Orientation::Vertical, "0");
        list_column.add_css_class("gk-list");
        list_column.set_size_request(200, -1);
        let brand = brand();
        list_column.append(&brand);
        let search = search_field("Search", Size::Small);
        search.set_dazz_label("Search components");
        search.add_css_class("gk-search");
        list_column.append(&search);
        let list = gtk::ListBox::new();
        list.add_css_class("gk-navigation");
        list.set_dazz_tint(Tint::Primary);
        list.set_activate_on_single_click(true);
        list.set_selection_mode(gtk::SelectionMode::Single);
        let overview_row = gtk::ListBoxRow::new();
        overview_row.set_widget_name("overview");
        let overview_label = box_(gtk::Orientation::Horizontal, "1-5");
        overview_label.append(&icon("Home20Regular"));
        let name = gtk::Label::new(Some("Overview"));
        name.set_xalign(0.);
        name.set_hexpand(true);
        overview_label.append(&name);
        overview_label.append(&gtk::Label::new(Some(&stories::all().len().to_string())));
        overview_row.set_child(Some(&overview_label));
        list.append(&overview_row);
        let heading = section_label("Components");
        heading.add_css_class("gk-list-heading");
        let heading_row = gtk::ListBoxRow::new();
        heading_row.set_widget_name("components-heading");
        heading_row.add_css_class("gk-list-heading-row");
        heading_row.set_selectable(false);
        heading_row.set_activatable(false);
        heading_row.set_child(Some(&heading));
        list.append(&heading_row);
        for story in stories::all() {
            let row = gtk::ListBoxRow::new();
            row.set_widget_name(&story.id);
            let label = gtk::Label::new(Some(&story.name));
            label.set_xalign(0.);
            row.set_child(Some(&label));
            list.append(&row);
        }
        let navigation = gtk::ScrolledWindow::builder()
            .child(&list)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .build();
        list_column.append(&navigation);
        let middle = box_(gtk::Orientation::Vertical, "0");
        middle.set_size_request(320, -1);
        middle.set_hexpand(true);
        middle.set_vexpand(true);
        let toolbar = box_(gtk::Orientation::Horizontal, "2");
        toolbar.add_css_class("gk-toolbar");
        let list_toggle = icon_button(
            "dazzui-PanelLeft20Regular-symbolic",
            "Toggle sidebar",
            Variant::Plain,
            Tint::Neutral,
            Size::Tiny,
        );
        list_toggle.set_dazz_label("Toggle sidebar");
        list_toggle.set_tooltip_text(Some("Toggle sidebar  ⌘B"));
        list_toggle.set_valign(gtk::Align::Center);
        toolbar.append(&list_toggle);
        let title = label("Overview", "body-medium");
        title.add_css_class("gk-title");
        toolbar.append(&title);
        let count = label("", "caption-large");
        count.add_css_class("gk-quiet");
        count.set_hexpand(true);
        toolbar.append(&count);
        let theme_toggle = icon_button(
            "dazzui-PanelRight20Regular-symbolic",
            "Toggle theme panel",
            Variant::Plain,
            Tint::Neutral,
            Size::Tiny,
        );
        theme_toggle.set_dazz_label("Toggle theme panel");
        theme_toggle.set_tooltip_text(Some("Toggle theme panel"));
        theme_toggle.set_valign(gtk::Align::Center);
        toolbar.append(&theme_toggle);
        middle.append(&toolbar);
        let stack = gtk::Stack::new();
        stack.set_hexpand(true);
        stack.set_vexpand(true);
        middle.append(&stack);
        let grid = gtk::FlowBox::new();
        grid.set_selection_mode(gtk::SelectionMode::None);
        grid.set_homogeneous(true);
        grid.set_max_children_per_line(8);
        grid.set_min_children_per_line(1);
        grid.add_css_class("gk-grid");
        grid.set_valign(gtk::Align::Start);
        let spacing_grid = grid.downgrade();
        window.connect_css_classes_notify(move |window| {
            if let Some(grid) = spacing_grid.upgrade() {
                let spacing = Theme::of(window)
                    .unwrap_or_default()
                    .dimension("spacing-3")
                    .unwrap() as u32;
                grid.set_row_spacing(spacing);
                grid.set_column_spacing(spacing);
            }
        });
        let spacing = theme.dimension("spacing-3").unwrap() as u32;
        grid.set_row_spacing(spacing);
        grid.set_column_spacing(spacing);
        let overview = gtk::ScrolledWindow::builder()
            .child(&grid)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        stack.add_named(&overview, Some("overview"));
        let context = Rc::new(Context {
            stack,
            title,
            count,
            list: list.clone(),
            grid: grid.clone(),
        });
        for story in stories::all() {
            let tile = card(CardVariant::Outlined, Tint::Neutral, Size::Medium);
            tile.add_css_class("gk-tile");
            tile.append(&stage(&story.preview, true));
            let open = button(&story.name, Variant::Plain, Tint::Neutral, Size::Medium);
            open.set_hexpand(true);
            tile.append(&open);
            let weak = Rc::downgrade(&context);
            let id = story.id.clone();
            open.connect_clicked(move |_| {
                if let Some(context) = weak.upgrade() {
                    context.open(Some(&id));
                }
            });
            let item = gtk::FlowBoxChild::new();
            item.set_widget_name(&story.id);
            item.set_child(Some(&tile));
            grid.insert(&item, -1);
        }
        let weak = Rc::downgrade(&context);
        list.connect_row_activated(move |_, row| {
            if let Some(context) = weak.upgrade() {
                context.open(Some(row.widget_name().as_str()));
            }
        });
        let weak = Rc::downgrade(&context);
        search.connect_search_changed(move |search| {
            if let Some(context) = weak.upgrade() {
                context.filter(search.text().as_str());
            }
        });
        let weak = Rc::downgrade(&context);
        brand.connect_clicked(move |_| {
            if let Some(context) = weak.upgrade() {
                context.open(None);
            }
        });
        let themes = ThemePanel::new(window, theme);
        let document_and_themes = gtk::Paned::new(gtk::Orientation::Horizontal);
        document_and_themes.set_start_child(Some(&middle));
        document_and_themes.set_end_child(Some(&themes.widget));
        document_and_themes.set_resize_end_child(false);
        document_and_themes.set_shrink_end_child(false);
        let shell = gtk::Paned::new(gtk::Orientation::Horizontal);
        shell.set_start_child(Some(&list_column));
        shell.set_end_child(Some(&document_and_themes));
        shell.set_resize_start_child(false);
        shell.set_shrink_start_child(false);
        shell.set_position(272);
        shell.add_css_class("gk");
        let weak = list_column.downgrade();
        list_toggle.connect_clicked(move |_| {
            if let Some(list) = weak.upgrade() {
                list.set_visible(!list.is_visible());
            }
        });
        let weak = themes.widget.downgrade();
        theme_toggle.connect_clicked(move |_| {
            if let Some(themes) = weak.upgrade() {
                themes.set_visible(!themes.is_visible());
            }
        });
        let shortcuts = gtk::ShortcutController::new();
        shortcuts.set_scope(gtk::ShortcutScope::Managed);
        let weak = list_toggle.downgrade();
        shortcuts.add_shortcut(gtk::Shortcut::new(
            gtk::ShortcutTrigger::parse_string("<Primary>b"),
            Some(gtk::CallbackAction::new(move |_, _| {
                if let Some(button) = weak.upgrade() {
                    button.emit_clicked();
                }
                gtk::glib::Propagation::Stop
            })),
        ));
        shell.add_controller(shortcuts);
        context.open(page);
        Self {
            widget: shell,
            search,
            themes,
            list_toggle,
            theme_toggle,
            context,
        }
    }
    pub fn open(&self, page: Option<&str>) {
        self.context.open(page);
    }
    pub fn page(&self) -> String {
        self.context.stack.visible_child_name().unwrap().into()
    }
    pub fn filter(&self, query: &str) {
        self.context.filter(query);
    }
    pub fn visible_stories(&self) -> usize {
        let mut count = 0;
        let mut child = self.context.grid.first_child();
        while let Some(w) = child {
            child = w.next_sibling();
            if w.is_visible() {
                count += 1;
            }
        }
        count
    }
}
/// Documents show examples, while the preview is reserved for Overview.
fn document(story: &Story) -> gtk::ScrolledWindow {
    let page = box_(gtk::Orientation::Vertical, "6");
    page.add_css_class("gk-page");
    let lead = label(&story.summary, "body-medium");
    lead.set_wrap(true);
    lead.add_css_class("gk-quiet");
    page.append(&lead);
    for example in &story.examples {
        let section = box_(gtk::Orientation::Vertical, "2");
        section.append(&section_label(&example.title));
        section.append(&stage(example, false));
        if let Some(note) = &example.note {
            let note = label(note, "caption-large");
            note.set_wrap(true);
            note.add_css_class("gk-quiet");
            section.append(&note);
        }
        page.append(&section);
    }
    gtk::ScrolledWindow::builder()
        .child(&page)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .build()
}
