use dazzui_gtk4::{gtk, prelude::*, *};
use std::{cell::RefCell, rc::Rc};
fn drain() {
    let end = std::time::Instant::now() + std::time::Duration::from_millis(40);
    while std::time::Instant::now() < end {
        while gtk::glib::MainContext::default().pending() {
            gtk::glib::MainContext::default().iteration(false);
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}
fn main() {
    // GTK must run on the process main thread (including on macOS).
    gtk::init().expect("GTK display required; on Linux run under xvfb-run");
    let errors = Rc::new(RefCell::new(vec![]));
    for css in [TOKENS_CSS, COMPONENTS_CSS] {
        let provider = gtk::CssProvider::new();
        let errors = errors.clone();
        provider.connect_parsing_error(move |_, section, error| {
            errors.borrow_mut().push(format!("{section:?}: {error}"))
        });
        provider.load_from_string(css);
    }
    assert!(
        errors.borrow().is_empty(),
        "CSS diagnostics: {:?}",
        errors.borrow()
    );
    assert!(Theme::new("missing").is_err());
    let window = gtk::Window::new();
    let root = box_(gtk::Orientation::Vertical, "4");
    let sizes = [Size::Tiny, Size::Small, Size::Medium, Size::Large];
    let buttons: Vec<_> = sizes
        .into_iter()
        .map(|s| button("Save", Variant::Filled, Tint::Primary, s))
        .collect();
    for b in &buttons {
        b.set_halign(gtk::Align::Start);
        root.append(b);
    }
    let field = text_field("Email", Size::Medium);
    field.set_text("persist@example.com");
    root.append(&field);
    let check = checkbox("Remember", Tint::Success, Size::Medium);
    check.set_active(true);
    root.append(&check);
    let sw = switch(Tint::Primary, Size::Medium);
    sw.set_active(true);
    root.append(&sw);
    let r1 = radio("A", None, Tint::Primary, Size::Medium);
    let r2 = radio("B", Some(&r1), Tint::Primary, Size::Medium);
    root.append(&r1);
    root.append(&r2);
    r1.set_active(true);
    r2.set_active(true);
    assert!(!r1.is_active() && r2.is_active());
    let nested = box_(gtk::Orientation::Vertical, "2");
    let nested_button = button("Nested", Variant::Normal, Tint::Neutral, Size::Small);
    nested.append(&nested_button);
    root.append(&nested);
    Theme::new("bright-dark").unwrap().apply(&nested);
    let pressed = Rc::new(RefCell::new(0));
    let count = pressed.clone();
    buttons[0].connect_clicked(move |_| *count.borrow_mut() += 1);
    buttons[0].emit_clicked();
    assert_eq!(*pressed.borrow(), 1);
    window.set_child(Some(&root));
    window.set_default_size(640, 480);
    for name in THEMES {
        let theme = Theme::new(name).unwrap();
        theme.apply(&window);
        window.present();
        drain();
        assert_eq!(Theme::of(&field), Some(theme));
        assert_eq!(Theme::of(&nested_button).unwrap().name(), "bright-dark");
        assert_eq!(field.text(), "persist@example.com");
        assert!(check.is_active());
        assert!(sw.is_active());
        for b in &buttons {
            assert!(
                b.width() > 0 && b.compute_bounds(&root).unwrap().height() as i32 > 0,
                "unallocated button in {name}"
            );
        }
    }
    Theme::default().apply(&window);
    drain();
    println!(
        "Studio button heights: {:?}",
        buttons
            .iter()
            .map(|b| b.compute_bounds(&root).unwrap().height() as i32)
            .collect::<Vec<_>>()
    );
    for (b, expected) in buttons.iter().zip([24, 26, 28, 32]) {
        assert_eq!(
            b.compute_bounds(&root).unwrap().height() as i32,
            expected,
            "control profile height"
        );
    }
    check.set_inconsistent(true);
    assert!(check.is_inconsistent());
    field.set_dazz_invalid(true);
    assert!(field.has_css_class("dz-invalid"));
    field.set_dazz_invalid(false);
    assert!(!field.has_css_class("dz-invalid"));
    field.set_sensitive(false);
    assert!(!field.is_sensitive());
    field.set_sensitive(true);
    field.grab_focus();
    drain();
    assert!(field.has_focus() || field.is_focus() || field.focus_child().is_some());
    let modal = dialog(&window, "Confirm", &label("Proceed?", "body-medium"));
    assert!(modal.is_modal());
    assert_eq!(Theme::of(&modal), Some(Theme::default()));
    assert_eq!(modal.transient_for(), Some(window.clone()));
    modal.destroy();
    window.destroy();
    println!(
        "CSS, {} themes, geometry, state, radio exclusivity, focus and modal ownership passed",
        THEMES.len()
    );
}
