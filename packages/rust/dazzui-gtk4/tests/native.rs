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
fn check_notes() {
    let window = gtk::Window::new();
    let noted_check = checkbox_with_note(
        "每周摘要",
        "周一发送，保留原生换行。",
        Tint::Primary,
        Size::Medium,
    );
    window.set_child(Some(&noted_check));
    window.set_default_size(420, 100);
    let note_label = noted_check
        .child()
        .unwrap()
        .first_child()
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap();
    assert_eq!(note_label.text(), "每周摘要 周一发送，保留原生换行。");
    for name in THEMES {
        let theme = Theme::new(name).unwrap();
        theme.apply(&window);
        window.present();
        drain();
        let natural = noted_check.measure(gtk::Orientation::Horizontal, -1).1;
        let narrow = noted_check
            .measure(gtk::Orientation::Horizontal, -1)
            .0
            .max(natural / 2);
        assert!(
            noted_check.measure(gtk::Orientation::Vertical, narrow).1
                > noted_check.measure(gtk::Orientation::Vertical, natural).1,
            "check notes wrap in the same paragraph in {name}"
        );
        let attrs = note_label.attributes().unwrap();
        let mut iter = attrs.iterator();
        assert!(iter.next_style_change());
        let ink = iter
            .get(gtk::pango::AttrType::Foreground)
            .unwrap()
            .downcast::<gtk::pango::AttrColor>()
            .unwrap();
        assert_eq!(
            ink.start_index(),
            "每周摘要".len() as u32,
            "note attributes use UTF-8 byte offsets"
        );
        let expected = theme.color("color-content-faint").unwrap();
        assert_eq!(
            ink.color().red(),
            (expected.red() * 65535.).round() as u16,
            "note follows the theme in {name}"
        );
        noted_check.emit_activate();
        assert!(noted_check.is_active());
        noted_check.emit_activate();
        assert!(!noted_check.is_active());
    }
    window.destroy();
}
fn tag_label_updates() {
    let window = gtk::Window::new();
    let tag = tag("Short", Variant::Tinted, Tint::Primary, Size::Medium);
    tag.set_halign(gtk::Align::Start);
    tag.set_valign(gtk::Align::Start);
    window.set_child(Some(&tag));
    for name in ["studio-light", "studio-dark"] {
        Theme::new(name).unwrap().apply(&window);
        tag.set_label("Short");
        window.present();
        drain();
        let width = tag.measure(gtk::Orientation::Horizontal, -1).1;
        let height = tag.measure(gtk::Orientation::Vertical, -1).1;
        tag.set_label("A longer native label");
        drain();
        assert_eq!(tag.label().as_deref(), Some("A longer native label"));
        assert!(tag.measure(gtk::Orientation::Horizontal, -1).1 > width);
        assert_eq!(tag.measure(gtk::Orientation::Vertical, -1).1, height);
        tag.set_active(false);
        tag.emit_clicked();
        assert!(
            tag.is_active(),
            "label updates retain native toggle activation"
        );
    }
    window.destroy();
}
fn main() {
    // GTK must run on the process main thread (including on macOS).
    gtk::init().expect("GTK display required; on Linux run under xvfb-run");
    gtk::Settings::default()
        .unwrap()
        .set_gtk_enable_animations(false);
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
    let badges = [Size::Small, Size::Medium, Size::Large].map(|size| {
        let badge = badge("M", Variant::Tinted, Tint::Primary, size);
        badge.set_halign(gtk::Align::Start);
        root.append(&badge);
        (size, badge)
    });
    let caps = [Size::Small, Size::Medium, Size::Large].map(|size| {
        let cap = key_cap("Ctrl+K");
        cap.set_dazz_size(size);
        cap.set_halign(gtk::Align::Start);
        root.append(&cap);
        (size, cap)
    });
    let action_cap = key_cap("⌘K");
    let sidebar_count = sidebar_menu_badge("12");
    sidebar_count.set_halign(gtk::Align::Start);
    root.append(&sidebar_count);
    let action_face = gtk::Box::new(gtk::Orientation::Horizontal, 4);
    action_face.append(&gtk::Label::new(Some("Command")));
    action_face.append(&action_cap);
    let action = button("", Variant::Normal, Tint::Primary, Size::Medium);
    action.set_child(Some(&action_face));
    root.append(&action);
    let recorder = shortcut_recorder(ShortcutRecorderOptions {
        value: "⌘K".into(),
        label: Some("Shortcut".into()),
        ..Default::default()
    });
    root.append(&recorder);
    let otp = otp_field(OtpOptions {
        length: 4,
        default_value: "123".into(),
        ..Default::default()
    })
    .unwrap();
    root.append(&otp);

    let choices = [Size::Small, Size::Medium, Size::Large].map(|size| {
        let choice = Combobox::new(ComboboxOptions {
            options: vec![ComboboxItem::Option(ComboboxOption {
                value: "jp".into(),
                label: "Japan".into(),
                disabled: false,
            })],
            default_value: Some("jp".into()),
            size,
            ..Default::default()
        });
        root.append(&choice);
        let free = autocomplete(AutocompleteOptions {
            items: vec!["Inter".into()],
            default_value: "Custom Font".into(),
            size,
            ..Default::default()
        });
        root.append(&free);
        (size, choice, free)
    });
    let sliders = [Size::Small, Size::Medium, Size::Large].map(|size| {
        let slider = slider_field(SliderOptions {
            default_values: vec![20., 80.],
            size,
            show_value: true,
            ..Default::default()
        })
        .unwrap();
        root.append(&slider);
        (size, slider)
    });

    let sizes = [Size::Tiny, Size::Small, Size::Medium, Size::Large];
    let buttons: Vec<_> = sizes
        .into_iter()
        .map(|s| button("Save", Variant::Filled, Tint::Primary, s))
        .collect();
    for b in &buttons {
        b.set_halign(gtk::Align::Start);
        root.append(b);
    }
    let full_width = button("", Variant::Filled, Tint::Primary, Size::Small);
    let caption_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    caption_box.append(&gtk::Label::new(Some("Subscribe")));
    full_width.set_child(Some(&caption_box));
    full_width.set_hexpand(true);
    root.append(&full_width);
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
    let other_choice = option_card("Other choice", "The same native radio group", Some(&choice));
    root.append(&other_choice);
    other_choice.set_active(true);
    assert!(!choice.is_active() && other_choice.is_active());
    choice.set_active(true);
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
        for (size, badge) in &badges {
            let font = theme
                .dimension(if *size == Size::Large {
                    "label-medium-font-size"
                } else {
                    "label-small-font-size"
                })
                .unwrap();
            let padding = theme
                .dimension(if *size == Size::Large {
                    "spacing-1"
                } else {
                    "spacing-0-5"
                })
                .unwrap();
            let expected = (font * 1.4).round()
                + 2. * padding
                + 2. * theme.dimension("stroke-hairline").unwrap();
            assert_eq!(
                badge.measure(gtk::Orientation::Vertical, -1).1,
                expected as i32,
                "badge line box {size:?} in {name}"
            );
        }
        for (size, cap) in &caps {
            assert_eq!(
                cap.measure(gtk::Orientation::Vertical, -1).1,
                theme
                    .dimension(if *size == Size::Large {
                        "label-medium-font-size"
                    } else {
                        "label-small-font-size"
                    })
                    .unwrap()
                    .round() as i32,
                "a key hint retains its own solid line box in {name}"
            );
        }
        assert_eq!(
            action_cap.measure(gtk::Orientation::Vertical, -1).1,
            theme.dimension("label-small-font-size").unwrap().round() as i32,
            "button label styles do not replace a nested key hint's line box in {name}"
        );
        assert_eq!(
            sidebar_count.measure(gtk::Orientation::Vertical, -1).1,
            theme.dimension("spacing-5").unwrap().round() as i32,
            "a sidebar count retains its fixed height in {name}"
        );

        for (size, choice, free) in &choices {
            let expected = theme
                .dimension(&format!("control-{}-size", size.name()))
                .unwrap() as i32;
            assert_eq!(
                choice.measure(gtk::Orientation::Vertical, -1).1,
                expected,
                "combobox height {size:?} in {name}"
            );
            assert_eq!(
                free.measure(gtk::Orientation::Vertical, -1).1,
                expected,
                "autocomplete height {size:?} in {name}"
            );
            assert_eq!(choice.value().as_deref(), Some("jp"));
            assert_eq!(choice.entry().text(), "Japan");
            assert_eq!(free.value().as_deref(), Some("Custom Font"));
        }
        for (size, slider) in &sliders {
            let name_size = match size {
                Size::Small => "small",
                Size::Large => "large",
                _ => "medium",
            };
            assert_eq!(slider.values(), [20., 80.]);
            assert_eq!(slider.reading().text(), "20 – 80");
            let expected = theme
                .dimension(&format!("control-{name_size}-size"))
                .unwrap() as i32;
            assert_eq!(
                slider.measure(gtk::Orientation::Vertical, -1).1,
                expected,
                "slider line box {size:?} in {name}"
            );
            for (i, scale) in slider.scales().iter().enumerate() {
                let (a, b) = scale.slider_range();
                let offset = scale
                    .compute_point(&slider.control(), &gtk::graphene::Point::new(0., 0.))
                    .unwrap()
                    .x();
                let actual = f64::from(a + b) / 2. + f64::from(offset);
                let expected = slider.values()[i] / 100. * f64::from(slider.control().width());
                assert!(
                    (actual - expected).abs() < 0.01,
                    "slider thumb centre {size:?} in {name}"
                );
                assert_eq!(
                    b - a,
                    theme
                        .dimension(&format!("slider-{name_size}-thumb"))
                        .unwrap() as i32,
                    "slider thumb diameter {size:?} in {name}"
                );
            }
        }

        assert_eq!(otp.value(), "123");
        let edge = theme.dimension("control-medium-size").unwrap() as i32;
        for entry in otp.cells() {
            assert_eq!(
                entry.measure(gtk::Orientation::Horizontal, -1).1,
                edge,
                "OTP cell width in {name}"
            );
            assert_eq!(
                entry.measure(gtk::Orientation::Vertical, -1).1,
                edge,
                "OTP cell height in {name}"
            );
        }
        assert_eq!(
            recorder.value(),
            "⌘K",
            "theme changes keep a recorded binding in {name}"
        );
        assert_eq!(
            recorder.button().measure(gtk::Orientation::Vertical, -1).1,
            theme.dimension("control-medium-size").unwrap() as i32,
            "shortcut recorder field height in {name}"
        );
        assert_eq!(
            recorder
                .button()
                .measure(gtk::Orientation::Horizontal, -1)
                .1,
            theme.dimension("shortcut-recorder-width").unwrap() as i32,
            "shortcut recorder minimum in {name}"
        );
        let outer = full_width.compute_bounds(&root).unwrap();
        let caption = caption_box.compute_bounds(&root).unwrap();
        assert!(
            (outer.x() + outer.width() / 2. - caption.x() - caption.width() / 2.).abs() < 0.01,
            "full-width composite button caption stays centered in {name}"
        );
        assert_eq!(Theme::of(&field), Some(theme));
        assert_eq!(Theme::of(&nested_button).unwrap().name(), "bright-dark");
        assert_eq!(field.text(), "persist@example.com");
        assert!(check.is_active());
        assert!(sw.is_active());
        for card in [&choice, &other_choice] {
            assert_eq!(
                card.first_child()
                    .unwrap()
                    .measure(gtk::Orientation::Horizontal, -1)
                    .1,
                0,
                "option cards use their border rather than a separate radio column in {name}"
            );
        }
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
    let retained_label = caps[0].1.label();
    caps[0].1.set_text("A much longer shortcut");
    drain();
    let long_width = caps[0].1.measure(gtk::Orientation::Horizontal, -1).1;
    caps[0].1.set_text("A");
    drain();
    assert!(caps[0].1.measure(gtk::Orientation::Horizontal, -1).1 < long_width);
    assert_eq!(
        caps[0].1.label(),
        retained_label,
        "text updates retain the native label"
    );
    Theme::default().apply(&window);
    drain();
    #[cfg(target_os = "macos")]
    {
        let label = buttons[0].child().and_downcast::<gtk::Label>().unwrap();
        let face = label
            .layout()
            .iter()
            .run_readonly()
            .unwrap()
            .item()
            .analysis()
            .font()
            .describe();
        assert_eq!(
            face.family().as_deref(),
            Some(".AppleSystemUIFont"),
            "the browser's system face must resolve before CJK fallbacks"
        );
    }
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
    // Fractional marker strokes must not shrink the idle row. Each status
    // keeps the same icon box, independently of native border rounding.
    let list = step_list();
    let steps = [StepStatus::Done, StepStatus::Active, StepStatus::Idle]
        .map(|status| step("Step", None, status, Tint::Primary));
    for step in &steps {
        list.append(step);
    }
    window.set_child(Some(&list));
    drain();
    let marker_size = Theme::of(&window)
        .unwrap_or_default()
        .dimension("icon-medium")
        .unwrap() as i32;
    for step in &steps {
        let marker = step.first_child().unwrap();
        assert_eq!(
            (marker.width(), marker.height()),
            (marker_size, marker_size)
        );
        assert_eq!(step.height(), marker_size, "step status keeps row geometry");
    }
    let loading = box_(gtk::Orientation::Vertical, "0");
    loading.set_size_request(192, -1);
    let bar = skeleton(SkeletonShape::Text, None, None);
    let proportion = proportional_width(&bar, 0.4);
    loading.append(&proportion);
    let paragraph = skeleton_text(4);
    loading.append(&paragraph);
    let fixed = gtk::Fixed::new();
    fixed.put(&loading, 0., 0.);
    window.set_child(Some(&fixed));
    drain();
    assert_eq!(
        bar.width(),
        77,
        "fractional width rounds the whole line once"
    );
    let theme = Theme::of(&window).unwrap_or_default();
    assert_eq!(
        paragraph.height(),
        (theme.dimension("body-small-line-height").unwrap() * 4.
            + theme.dimension("spacing-1-5").unwrap() * 3.) as i32
    );
    loading.set_size_request(160, -1);
    proportion.set_direction(gtk::TextDirection::Rtl);
    drain();
    assert_eq!(bar.width(), 64, "fractional bars follow parent resizing");
    assert_eq!(
        bar.compute_bounds(&proportion).unwrap().x(),
        96.,
        "RTL bars start at the trailing side"
    );
    assert_eq!(
        paragraph
            .last_child()
            .unwrap()
            .first_child()
            .unwrap()
            .width(),
        96,
        "the last paragraph line takes its ds proportion"
    );

    let numbers = box_(gtk::Orientation::Vertical, "0");
    let (field, input) = number_field_with_unit(
        0.,
        100.,
        5.,
        Some(label("%", "label-small").upcast_ref()),
        Size::Medium,
    );
    numbers.append(&field);
    window.set_child(Some(&numbers));
    input.set_value(80.);
    let stepper = field
        .last_child()
        .and_downcast::<gtk::SpinButton>()
        .unwrap();
    assert_eq!(
        stepper.value(),
        80.,
        "the native input and arrows share one value"
    );
    stepper.spin(gtk::SpinType::StepForward, 0.);
    assert_eq!(input.value(), 85.);
    for _ in 0..5 {
        stepper.spin(gtk::SpinType::StepForward, 0.);
    }
    assert_eq!(
        input.value(),
        100.,
        "native stepping clamps to the upper bound"
    );
    input.set_text("35");
    input.update();
    assert_eq!(
        stepper.value(),
        35.,
        "typed numbers update the same adjustment"
    );
    drain();
    assert_eq!(
        field.compute_bounds(&numbers).unwrap().height() as i32,
        theme.dimension("control-medium-size").unwrap() as i32
    );
    let buttons: Vec<_> = (0..stepper.observe_children().n_items())
        .filter_map(|i| {
            stepper
                .observe_children()
                .item(i)
                .and_downcast::<gtk::Button>()
        })
        .collect();
    let up = buttons.iter().find(|b| b.has_css_class("up")).unwrap();
    let down = buttons.iter().find(|b| b.has_css_class("down")).unwrap();
    assert!(
        up.compute_bounds(&field).unwrap().y() < down.compute_bounds(&field).unwrap().y(),
        "native buttons form a vertical stepper"
    );
    assert!(
        up.height() > 0 && down.height() > 0,
        "half-height native buttons remain hit targets"
    );
    input.set_value(100.);
    assert!(
        !up.is_sensitive() && down.is_sensitive(),
        "native button states track bounds"
    );
    let (fractional, input) = number_field(0., 10., 0.25, Size::Medium);
    input.set_value(1.5);
    fractional
        .last_child()
        .and_downcast::<gtk::SpinButton>()
        .unwrap()
        .spin(gtk::SpinType::StepForward, 0.);
    assert_eq!(
        input.value(),
        1.75,
        "fractional native steps retain their precision"
    );
    window.destroy();
    check_notes();
    tag_label_updates();
    println!(
        "CSS, {} themes, geometry, state, radio exclusivity, focus and modal ownership passed",
        THEMES.len()
    );
}
