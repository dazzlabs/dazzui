use dazzui_gtk4::{gtk, prelude::*, *};
use dazzui_gtk4_example::{
    components::shell::Shell,
    conformance::{screenshot, settle},
    stories,
    support::themes::FAMILY_GROUPS,
};
fn mapped(root: &gtk::Widget, class: &str) -> Vec<gtk::Widget> {
    if !root.is_mapped() {
        return vec![];
    }
    let mut result = if root.has_css_class(class) {
        vec![root.clone()]
    } else {
        vec![]
    };
    let mut child = root.first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        result.extend(mapped(&node, class));
    }
    result
}
fn main() {
    gtk::init().expect("GTK display required");
    gtk::Settings::default()
        .unwrap()
        .set_gtk_enable_animations(false);
    let application = gtk::Application::builder()
        .application_id("dev.dazzlabs.dazzui.gtk4.shelltest")
        .build();
    application
        .register(None::<&gtk::gio::Cancellable>)
        .unwrap();
    let window = gtk::ApplicationWindow::builder()
        .application(&application)
        .default_width(1280)
        .default_height(820)
        .build();
    let shell = Shell::new(&window, None, Theme::default());
    window.set_child(Some(&shell.widget));
    window.present();
    settle();
    assert_eq!(shell.page(), "overview");
    assert_eq!(shell.visible_stories(), stories::all().len());
    for story in stories::all() {
        assert_eq!(story.preview.id, format!("{}/preview", story.id));
        assert!(
            story
                .examples
                .iter()
                .enumerate()
                .all(|(i, e)| e.id == format!("{}/{i}", story.id))
        );
    }
    assert_eq!(
        FAMILY_GROUPS
            .iter()
            .map(|g| g.families.len())
            .sum::<usize>()
            * 2,
        THEMES.len()
    );
    shell.filter("button");
    assert_eq!(shell.visible_stories(), stories::matching("button").len());
    shell.filter("nothing-matches-this-query");
    assert_eq!(shell.visible_stories(), 0);
    shell.filter("");
    shell.open(Some("button"));
    settle();
    assert_eq!(shell.page(), "button");
    shell.open(Some("missing"));
    assert_eq!(shell.page(), "overview");
    shell.themes.select(Theme::new("bright-light").unwrap());
    shell.themes.set_dark(true);
    settle();
    assert_eq!(shell.themes.theme().name(), "bright-dark");
    assert_eq!(Theme::of(&shell.search).unwrap().name(), "bright-dark");
    shell.theme_toggle.emit_clicked();
    assert!(!shell.themes.widget.is_visible());
    shell.theme_toggle.emit_clicked();
    assert!(shell.themes.widget.is_visible());
    shell.list_toggle.emit_clicked();
    shell.list_toggle.emit_clicked();
    shell.themes.select(Theme::default());
    shell.open(None);
    settle();
    let directory = std::path::Path::new("/tmp/dazzui-gtk4-shell");
    std::fs::create_dir_all(directory).unwrap();
    screenshot(
        shell.widget.upcast_ref(),
        shell.widget.upcast_ref(),
        window.upcast_ref(),
        &directory.join("overview.png"),
        1.,
    )
    .unwrap();
    shell.open(Some("button"));
    settle();
    screenshot(
        shell.widget.upcast_ref(),
        shell.widget.upcast_ref(),
        window.upcast_ref(),
        &directory.join("button.png"),
        1.,
    )
    .unwrap();
    shell.themes.select(Theme::new("studio-light").unwrap());
    for width in [1120, 1000, 1280] {
        window.set_default_size(width, 720);
        settle();
        let viewports = mapped(shell.widget.upcast_ref(), "gk-stage-viewport");
        assert_eq!(
            viewports.len(),
            stories::story_of("button").unwrap().examples.len()
        );
        let mut checked = 0;
        gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
        assert!(
            viewports[0]
                .parent()
                .unwrap()
                .child_focus(gtk::DirectionType::TabForward),
            "the scrolling frame retains native keyboard entry"
        );
        assert!(
            gtk::prelude::GtkWindowExt::focus(&window)
                .is_some_and(|focus| focus.is_ancestor(&viewports[0]))
        );
        for viewport in viewports {
            for button in mapped(&viewport, "dz-button") {
                let bounds = button.compute_bounds(&viewport).unwrap();
                assert!(
                    bounds.y() >= -0.5
                        && bounds.y() + bounds.height() <= viewport.height() as f32 + 0.5,
                    "wrapped button is clipped at window width {width}: {bounds:?}, viewport height {}",
                    viewport.height()
                );
                checked += 1;
            }
        }
        assert!(checked > 0);
    }
    window.set_default_size(1120, 720);
    for story in stories::all() {
        shell.open(Some(&story.id));
        settle();
        for viewport in mapped(shell.widget.upcast_ref(), "gk-stage-viewport") {
            let child = viewport
                .downcast_ref::<gtk::ScrolledWindow>()
                .unwrap()
                .child()
                .unwrap();
            let content = child
                .downcast_ref::<gtk::Viewport>()
                .and_then(|v| v.child())
                .unwrap_or(child);
            let bounds = content.compute_bounds(&viewport).unwrap();
            assert!(
                bounds.y() >= -0.5
                    && bounds.y() + bounds.height() <= viewport.height() as f32 + 0.5,
                "{} example content is clipped: {bounds:?}, viewport height {}",
                story.id,
                viewport.height()
            );
        }
    }
    window.destroy();
    println!(
        "Registry, preview/examples split, search, navigation, theme modes and panel toggles passed"
    );
}
