use dazzui_gtk4::{gtk, prelude::*, *};
use std::{cell::RefCell, rc::Rc};
#[path = "../example/src/conformance/paint.rs"]
mod paint;
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
    let glyph_row = box_(gtk::Orientation::Horizontal, "2");
    root.append(&glyph_row);
    let glyphed: Vec<_> = sizes
        .into_iter()
        .map(|s| {
            let (lead, trail) = (
                glyph_button(
                    "Add",
                    Some("list-add-symbolic"),
                    None,
                    Variant::Normal,
                    Tint::Neutral,
                    s,
                ),
                glyph_button(
                    "Next",
                    None,
                    Some("go-next-symbolic"),
                    Variant::Normal,
                    Tint::Neutral,
                    s,
                ),
            );
            for b in [&lead, &trail] {
                b.set_valign(gtk::Align::Start);
                glyph_row.append(b);
            }
            (lead, trail)
        })
        .collect();
    let segment_row = box_(gtk::Orientation::Horizontal, "2");
    root.append(&segment_row);
    let labelled = glyph_segmented_control(
        &[
            ("List", Some("view-list-symbolic")),
            ("Grid", Some("view-grid-symbolic")),
        ],
        0,
        false,
    );
    let bare = glyph_segmented_control(
        &[
            ("Left", Some("format-justify-left-symbolic")),
            ("Right", Some("format-justify-right-symbolic")),
        ],
        0,
        true,
    );
    let icon_buttons: Vec<_> = [Size::Tiny, Size::Large]
        .into_iter()
        .map(|s| {
            icon_button(
                "list-add-symbolic",
                "Add",
                Variant::Normal,
                Tint::Neutral,
                s,
            )
        })
        .collect();
    segment_row.append(&labelled);
    segment_row.append(&bare);
    for b in &icon_buttons {
        b.set_valign(gtk::Align::Start);
        segment_row.append(b);
    }
    let pills = pill_tabs(
        &[
            PillTab {
                value: "all",
                label: "All",
                count: Some("24"),
                icon: None,
                disabled: false,
            },
            PillTab {
                value: "unread",
                label: "Unread",
                count: Some("3"),
                icon: None,
                disabled: false,
            },
        ],
        Some("all"),
        Tint::Primary,
    );
    pills.set_halign(gtk::Align::Start);
    root.append(&pills);
    let choice = option_card("Choice", "A selectable surface", None);
    choice.set_active(true);
    root.append(&choice);
    let first_pill = pills
        .first_child()
        .unwrap()
        .downcast::<gtk::ToggleButton>()
        .unwrap();
    let second_pill = first_pill
        .next_sibling()
        .unwrap()
        .downcast::<gtk::ToggleButton>()
        .unwrap();
    second_pill.set_active(true);
    assert!(!first_pill.is_active() && second_pill.is_active());
    let field = text_field("Email", Size::Medium);
    field.set_text("persist@example.com");
    root.append(&field);
    let clear_meter = box_(gtk::Orientation::Vertical, "0");
    clear_meter.append(&progress(0.5, Tint::Primary, Size::Medium));
    root.append(&clear_meter);
    let preference_action = button("Manage", Variant::Recessed, Tint::Neutral, Size::Small);
    root.append(&preference_row(
        "Sessions",
        Some("Connected devices"),
        &preference_action,
    ));
    let searches: Vec<_> = [Size::Small, Size::Medium, Size::Large]
        .into_iter()
        .map(|size| search_field("Search", size))
        .collect();
    for search in &searches {
        root.append(search);
    }
    let grouped = select_options(
        &[
            SelectItem::Option(SelectOption {
                value: "none".into(),
                label: "Unassigned".into(),
                disabled: false,
            }),
            SelectItem::Group {
                label: "People".into(),
                options: vec![
                    SelectOption {
                        value: "ada".into(),
                        label: "Ada".into(),
                        disabled: false,
                    },
                    SelectOption {
                        value: "grace".into(),
                        label: "Grace".into(),
                        disabled: true,
                    },
                ],
            },
        ],
        "Choose one",
        Size::Medium,
    );
    root.append(&grouped);
    let select_sizes: Vec<_> = [Size::Small, Size::Medium, Size::Large]
        .into_iter()
        .map(|size| select_options(&[], "Choose", size))
        .collect();
    for select in &select_sizes {
        root.append(select);
    }
    assert_eq!(grouped.selected(), 0, "selector starts empty");
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
        let corners = paint::corners(&second_pill).expect("painted pill corner");
        for corner in corners {
            assert!(
                (corner - second_pill.height() as f32 / 2.).abs() <= 0.5,
                "pill corner in {name}"
            );
        }
        assert!(
            pills.width() < 300,
            "pills must size to their labels in {name}"
        );
        for b in &buttons {
            assert!(
                b.width() > 0 && b.compute_bounds(&root).unwrap().height() as i32 > 0,
                "unallocated button in {name}"
            );
        }
    }
    Theme::default().apply(&window);
    drain();
    fn labels(widget: &impl IsA<gtk::Widget>) -> Vec<String> {
        let widget = widget.as_ref();
        let mut out = vec![];
        if let Some(label) = widget.downcast_ref::<gtk::Label>() {
            out.push(label.text().into());
        }
        let mut child = widget.first_child();
        while let Some(widget) = child {
            child = widget.next_sibling();
            out.extend(labels(&widget));
        }
        out
    }
    let texts = labels(&grouped);
    assert!(
        texts.iter().any(|text| text == "Choose one"),
        "missing native placeholder: selected={}; {texts:?}",
        grouped.selected()
    );
    grouped.set_selected(2);
    assert_eq!(
        grouped.selected(),
        3,
        "headings cannot become the selected value"
    );
    grouped.set_selected(4);
    assert_eq!(
        grouped.selected(),
        3,
        "disabled options cannot become the selected value"
    );
    grouped.set_selected(0);
    drain();
    assert!(labels(&grouped).iter().any(|text| text == "Choose one"));
    let icons = gtk::IconTheme::for_display(&WidgetExt::display(&window));
    assert!(
        icons.has_icon("dazzui-Add20Regular-symbolic"),
        "Fluent glyph resource"
    );
    // Read the submitted render nodes, not the token we meant to draw.
    // Nested segments must remain concentric even under pill and square themes.
    for (theme, expected, card_corner) in [
        ("studio-light", 10., 12.),
        ("studio-dark", 10., 12.),
        ("bright-light", 14., 12.),
        ("bright-dark", 14., 12.),
        ("graphite-light", 8., 10.),
        ("nocturne-dark", 8., 10.),
    ] {
        Theme::new(theme).unwrap().apply(&window);
        drain();
        assert_eq!(
            paint::corners(&clear_meter),
            None,
            "{theme}: a clear layout must not borrow its child's track corner"
        );
        assert_eq!(
            paint::corners(&labelled),
            Some([expected; 4]),
            "{theme}: track corners"
        );
        let segment = labelled.first_child().unwrap();
        assert_eq!(
            paint::corners(&segment),
            Some([expected - 3.; 4]),
            "{theme}: segment corners"
        );
        assert_eq!(
            paint::corners(&choice),
            Some([card_corner; 4]),
            "{theme}: selected option card corners"
        );
        let field_corner = if theme.starts_with("bright-") {
            None
        } else if theme.starts_with("studio-") {
            Some(10.)
        } else {
            Some(8.)
        };
        assert_eq!(
            field.compute_bounds(&root).unwrap().height(),
            28.,
            "{theme}: text field height"
        );
        assert_eq!(
            preference_action.compute_bounds(&root).unwrap().height(),
            26.,
            "{theme}: preference action must keep its control height"
        );
        if theme.starts_with("bright-") {
            assert_eq!(
                paint::corners(&preference_action),
                Some([13.; 4]),
                "{theme}: preference action pill"
            );
        }
        assert_eq!(
            paint::corners(&field),
            Some([field_corner.unwrap_or(14.); 4]),
            "{theme}: text field corners"
        );
        for (search, height) in searches.iter().zip([26., 28., 32.]) {
            assert_eq!(
                search.compute_bounds(&root).unwrap().height(),
                height,
                "{theme}: search field height"
            );
            let expected = if theme.starts_with("bright-") {
                height / 2.
            } else {
                field_corner.unwrap()
            };
            assert_eq!(
                paint::corners(search),
                Some([expected; 4]),
                "{theme}: search field corners"
            );
        }
        for (select, height) in select_sizes.iter().zip([26., 28., 32.]) {
            assert_eq!(
                select.compute_bounds(&root).unwrap().height(),
                height,
                "{theme}: select height"
            );
            assert_eq!(
                paint::corners(select),
                Some([field_corner.unwrap_or(height / 2.); 4]),
                "{theme}: select corners"
            );
        }
        assert_eq!(
            labelled.compute_bounds(&root).unwrap().height(),
            28.,
            "{theme}: track height"
        );
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
    // A glyph beside a label: shrunk 2px under the profile icon, the profile
    // gap from the label, and its end padded 2px less (Studio).
    for ((lead, trail), (icon, gap)) in glyphed.iter().zip([(10, 4), (14, 6), (14, 8), (18, 8)]) {
        let content = |b: &gtk::Button| b.child().unwrap().downcast::<gtk::Box>().unwrap();
        let (lc, tc) = (content(lead), content(trail));
        let glyph = lc.first_child().unwrap();
        let text = glyph.next_sibling().unwrap();
        let g = glyph.compute_bounds(&lc).unwrap();
        let t = text.compute_bounds(&lc).unwrap();
        assert_eq!(g.width() as i32, icon, "glyph size");
        assert_eq!((t.x() - g.x() - g.width()) as i32, gap, "glyph gap");
        // Border-box insets of the content, start and end, against the row.
        let insets = |b: &gtk::Button, c: &gtk::Box| {
            let (b, c) = (
                b.compute_bounds(&glyph_row).unwrap(),
                c.compute_bounds(&glyph_row).unwrap(),
            );
            (c.x() - b.x(), b.x() + b.width() - c.x() - c.width())
        };
        let (lead_start, lead_end) = insets(lead, &lc);
        let (trail_start, trail_end) = insets(trail, &tc);
        assert_eq!(
            (lead_end - lead_start).round() as i32,
            2,
            "leading glyph inset"
        );
        assert_eq!(
            (trail_start - trail_end).round() as i32,
            2,
            "trailing glyph inset"
        );
    }
    // A segment's glyph beside its label is drawn as Button draws one: 2px
    // under the medium profile's 16px icon, 8px before the label, and the
    // leading end 2px under `spacing-3`. A glyph alone keeps the full 16px.
    let segment = labelled.first_child().unwrap();
    let content = segment.first_child().unwrap();
    let glyph = content.first_child().unwrap();
    let text = glyph.next_sibling().unwrap();
    let g = glyph.compute_bounds(&content).unwrap();
    let t = text.compute_bounds(&content).unwrap();
    assert_eq!(g.width() as i32, 14, "segment glyph size");
    assert_eq!((t.x() - g.x() - g.width()) as i32, 8, "segment glyph gap");
    let (sb, cb) = (
        segment.compute_bounds(&segment_row).unwrap(),
        content.compute_bounds(&segment_row).unwrap(),
    );
    let start = cb.x() - sb.x();
    let end = sb.x() + sb.width() - cb.x() - cb.width();
    assert_eq!((end - start).round() as i32, 2, "segment glyph inset");
    let only = bare.first_child().unwrap().first_child().unwrap();
    assert!(
        only.is::<gtk::Image>(),
        "icon-only segment draws its glyph alone"
    );
    assert_eq!(only.width(), 16, "icon-only segment glyph size");
    // The tiny icon button takes its own 16px glyph; other sizes keep the profile's.
    for (b, expected) in icon_buttons.iter().zip([16, 20]) {
        assert_eq!(
            b.child().unwrap().width(),
            expected,
            "icon-button glyph size"
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
