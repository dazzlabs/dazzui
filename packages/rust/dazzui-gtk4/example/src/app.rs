use crate::components::shell::Shell;
use dazzui_gtk4::{Theme, gtk, prelude::*};

/// Application state and lifecycle are separate from the shell's layout.
pub fn run(page: Option<String>, theme: Theme, new_instance: bool) -> gtk::glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("dev.dazzlabs.dazzui.gtk4")
        .flags(if new_instance {
            gtk::gio::ApplicationFlags::NON_UNIQUE
        } else {
            gtk::gio::ApplicationFlags::empty()
        })
        .build();
    app.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("DazzUI · GTK4")
            .default_width(1280)
            .default_height(820)
            .build();
        let shell = Shell::new(&window, page.as_deref(), theme);
        window.set_child(Some(&dazzui_gtk4::toast_layer(&shell.widget)));
        // Keep the controllers alive with the window; their window references
        // are weak, so closing it releases the shell and its signal closures.
        window.connect_destroy(move |_| {
            let _ = &shell;
        });
        window.present();
    });
    app.run_with_args::<&str>(&[])
}
