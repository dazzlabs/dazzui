use dazzui_gtk4::{gtk, prelude::*, *};
use dazzui_gtk4_example::{conformance::settle, stories, support::scene};
fn main() {
    gtk::init().expect("GTK display required; on Linux run under xvfb-run");
    gtk::Settings::default()
        .unwrap()
        .set_gtk_enable_animations(false);
    let window = gtk::Window::builder().default_width(640).build();
    let mut count = 0;
    for name in ["studio-light", "bright-dark"] {
        Theme::new(name).unwrap().apply(&window);
        for story in stories::all() {
            for scene in story.scenes() {
                if !scene.is_supported() {
                    continue;
                }
                let mut specimens = vec![];
                let widget = scene::render(&scene.content, &mut specimens)
                    .unwrap_or_else(|e| panic!("{}: {e}", scene.id));
                window.set_child(Some(&widget));
                window.present();
                settle();
                assert!(widget.width() > 0 && widget.height() > 0, "{}", scene.id);
                let mut seen = std::collections::HashSet::new();
                for (at, specimen) in specimens {
                    assert!(
                        specimen.compute_bounds(&widget).unwrap().width() > 0.0,
                        "{} / {at} has no width",
                        scene.id
                    );
                    assert!(seen.insert(at), "Duplicate address in {}", scene.id);
                }
                count += 1;
            }
        }
    }
    window.destroy();
    println!("{count} generated scenes rendered in light and dark themes");
}
