use dazzui_gtk4::{gtk, prelude::*, *};
use dazzui_gtk4_example::{
    components::shell::Shell,
    conformance::{screenshot, settle},
    stories,
    support::themes::FAMILY_GROUPS,
};
fn main() {
    gtk::init().expect("GTK display required");
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
    assert_eq!(shell.visible_stories(), 51);
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
    window.destroy();
    println!(
        "Registry, preview/examples split, search, navigation, theme modes and panel toggles passed"
    );
}
