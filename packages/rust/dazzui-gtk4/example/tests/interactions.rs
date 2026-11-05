use dazzui_gtk4::{gtk, prelude::*, *};
use dazzui_gtk4_example::{conformance::settle, stories, support::scene};
fn find(root: &impl IsA<gtk::Widget>, class: &str) -> gtk::Widget {
    fn visit(root: &gtk::Widget, class: &str) -> Option<gtk::Widget> {
        if root.has_css_class(class) {
            return Some(root.clone());
        }
        let mut child = root.first_child();
        while let Some(node) = child {
            child = node.next_sibling();
            if let Some(found) = visit(&node, class) {
                return Some(found);
            }
        }
        None
    }
    visit(root.as_ref(), class).unwrap_or_else(|| panic!("Missing {class}"))
}
// A desktop user may activate another application during this suite. The
// owning window's logical focus still has to survive retained-widget edits.
fn focused(widget: &impl IsA<gtk::Widget>) -> bool {
    widget
        .root()
        .and_downcast::<gtk::Window>()
        .and_then(|window| gtk::prelude::GtkWindowExt::focus(&window))
        .as_ref()
        == Some(widget.as_ref())
}
fn key(widget: &impl IsA<gtk::Widget>, key: gtk::gdk::Key, mods: gtk::gdk::ModifierType) -> bool {
    let controllers = widget.observe_controllers();
    (0..controllers.n_items()).any(|i| {
        controllers
            .item(i)
            .and_downcast::<gtk::EventControllerKey>()
            .filter(|c| c.propagation_phase() == gtk::PropagationPhase::Capture)
            .is_some_and(|c| c.emit_by_name::<bool>("key-pressed", &[&key, &0u32, &mods]))
    })
}
fn drawer_interactions(window: &gtk::Window) {
    for side in [
        DrawerSide::Right,
        DrawerSide::Left,
        DrawerSide::Top,
        DrawerSide::Bottom,
    ] {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let trigger = button("Open filters", Variant::Tinted, Tint::Primary, Size::Small);
        trigger.set_halign(gtk::Align::Center);
        trigger.set_valign(gtk::Align::Center);
        trigger.set_vexpand(true);
        root.append(&trigger);
        window.set_child(Some(&root));
        settle();
        let content = gtk::Switch::new();
        content.set_halign(gtk::Align::Start);
        let close = button("Done", Variant::Filled, Tint::Primary, Size::Small);
        close.connect_clicked(dismiss_from);
        let drawer = Drawer::new(
            &trigger,
            &content,
            Some(close.upcast_ref()),
            DrawerOptions {
                side,
                title: Some("Filters".into()),
                description: Some("What the list leaves out.".into()),
                ..Default::default()
            },
        );
        trigger.grab_focus();
        drawer.set_open(true);
        settle();
        let popup = drawer.popup();
        assert!(popup.is_mapped());
        let viewport = popup.child().unwrap();
        println!(
            "drawer {side:?}: window {}x{}, popup {}x{}, viewport {}x{}, panel {:?}",
            window.width(),
            window.height(),
            popup.width(),
            popup.height(),
            viewport.width(),
            viewport.height(),
            drawer.panel().compute_bounds(&viewport)
        );
        assert_eq!(
            (viewport.width(), viewport.height()),
            (window.width(), window.height())
        );
        let rect = drawer.panel().compute_bounds(&viewport).unwrap();
        assert_eq!(
            rect.x(),
            if side == DrawerSide::Right {
                window.width() as f32 - rect.width()
            } else {
                0.
            }
        );
        assert_eq!(
            rect.y(),
            if side == DrawerSide::Bottom {
                window.height() as f32 - rect.height()
            } else {
                0.
            }
        );
        assert_eq!(
            gtk::prelude::GtkWindowExt::focus(window).as_ref(),
            Some(drawer.panel().upcast_ref()),
            "opening focuses the sheet, as in React"
        );
        let vertical = matches!(side, DrawerSide::Top | DrawerSide::Bottom);
        assert!(
            (if vertical {
                rect.height()
            } else {
                rect.width()
            } - 352.)
                .abs()
                < 0.5
        );
        assert!(
            (if vertical {
                rect.width() - window.width() as f32
            } else {
                rect.height() - window.height() as f32
            })
            .abs()
                < 0.5
        );
        assert!(content.is_mapped());
        window.set_default_size(900, 650);
        settle();
        assert_eq!(
            (viewport.width(), viewport.height()),
            (window.width(), window.height()),
            "an open drawer follows its owner resizing"
        );
        let resized = drawer.panel().compute_bounds(&viewport).unwrap();
        assert_eq!(
            resized.x(),
            if side == DrawerSide::Right {
                window.width() as f32 - resized.width()
            } else {
                0.
            }
        );
        assert_eq!(
            resized.y(),
            if side == DrawerSide::Bottom {
                window.height() as f32 - resized.height()
            } else {
                0.
            }
        );
        content.set_active(true);
        content.grab_focus();
        drawer.set_open(true);
        assert!(
            focused(&content),
            "repeating open does not reset an edit's focus"
        );
        close.emit_clicked();
        settle();
        assert!(!popup.is_visible());
        assert!(focused(&trigger), "closing returns focus to the trigger");
        drawer.set_open(true);
        settle();
        assert!(
            content.is_active(),
            "closing preserves edits in retained native controls"
        );
        if !vertical {
            trigger.set_direction(gtk::TextDirection::Rtl);
            settle();
            let rtl = drawer.panel().compute_bounds(&viewport).unwrap();
            assert_eq!(
                rtl.x(),
                if side == DrawerSide::Left {
                    window.width() as f32 - rtl.width()
                } else {
                    0.
                },
                "inline edges follow the owner's text direction"
            );
            trigger.set_direction(gtk::TextDirection::Ltr);
            settle();
        }
        #[cfg(target_os = "linux")]
        if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
            let title = window.title();
            window.set_title(Some("DazzUI isolated drawer checkpoint"));
            settle();
            for _ in 0..5 {
                let status = std::process::Command::new("python3")
                    .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_key.py"))
                    .args([
                        "DazzUI isolated drawer checkpoint",
                        "Tab",
                        "--preserve-focus",
                    ])
                    .status()
                    .unwrap();
                assert!(status.success());
                settle();
                assert!(
                    gtk::prelude::GtkWindowExt::focus(window).is_some_and(|w| w.is_ancestor(popup)),
                    "native Tab traversal stays within the sheet"
                );
            }
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_key.py"))
                .args([
                    "DazzUI isolated drawer checkpoint",
                    "Escape",
                    "--preserve-focus",
                ])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
            assert!(
                !popup.is_visible(),
                "real Escape dismisses the native modal popup"
            );
            assert!(focused(&trigger));
            drawer.set_open(true);
            settle();
            let (x, y) = match side {
                DrawerSide::Left => (window.width() - 20, 20),
                DrawerSide::Top => (20, window.height() - 20),
                _ => (20, 20),
            };
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_click.py"))
                .args([
                    "DazzUI isolated drawer checkpoint",
                    &x.to_string(),
                    &y.to_string(),
                ])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
            assert!(
                !popup.is_visible(),
                "a real scrim click dismisses the modal popup"
            );
            assert!(focused(&trigger));
            window.set_title(title.as_deref());
            println!("X11 {side:?} drawer focus containment, Escape and scrim dismissal passed");
        }
        drawer.set_open(false);
        settle();
        window.set_child(None::<&gtk::Widget>);
        window.set_default_size(800, 600);
        settle();
    }
    let story = stories::story_of("drawer").unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&story.examples[1], &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    for (at, side) in [(1, "left"), (7, "right"), (13, "top"), (19, "bottom")] {
        let trigger = specimens
            .iter()
            .find(|(a, _)| *a == at)
            .unwrap()
            .1
            .clone()
            .downcast::<gtk::Button>()
            .unwrap();
        trigger.emit_clicked();
        settle();
        let popup = find(&trigger, "dz-drawer-popup")
            .downcast::<gtk::Popover>()
            .unwrap();
        assert!(popup.is_mapped());
        let viewport = popup.child().unwrap();
        for class in [
            "dz-drawer",
            "dz-drawer-header",
            "dz-drawer-title",
            "dz-drawer-subtitle",
            "dz-drawer-body",
            "dz-drawer-footer",
        ] {
            let part = find(&popup, class);
            println!(
                "source scene drawer {side} {class}: {:?}",
                part.compute_bounds(&viewport)
            );
        }
        if let Some(dir) = std::env::var_os("DAZZUI_DRAWER_CAPTURE") {
            std::fs::create_dir_all(&dir).unwrap();
            dazzui_gtk4_example::conformance::screenshot(
                &viewport,
                &viewport,
                window,
                &std::path::Path::new(&dir).join(format!("{side}.png")),
                1.,
            )
            .unwrap();
        }
        find(&popup, "dz-button")
            .downcast::<gtk::Button>()
            .unwrap()
            .emit_clicked();
        settle();
        assert!(
            !popup.is_visible(),
            "generated DrawerClose dismisses the sheet"
        );
    }
}
fn toast_interactions(window: &gtk::Window) {
    use std::{cell::Cell, rc::Rc, time::Duration};
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::new();
    let underneath = button(
        "Underlying control",
        Variant::Filled,
        Tint::Primary,
        Size::Small,
    );
    underneath.set_halign(gtk::Align::Start);
    let clicks = Rc::new(Cell::new(0));
    let count = clicks.clone();
    underneath.connect_clicked(move |_| count.set(count.get() + 1));
    root.append(&entry);
    root.append(&underneath);
    window.set_child(Some(&root));
    window.set_default_size(800, 600);
    settle();
    entry.grab_focus();
    let previous_focus = gtk::prelude::GtkWindowExt::focus(window);
    let queue = ToastQueue::new(&root, ToastPlacement::Bottom, 3);
    let id = queue.add(ToastOptions {
        title: "Saved to Downloads".into(),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    assert!(queue.popup().is_mapped());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window),
        previous_focus,
        "displaying a receipt preserves input focus"
    );
    assert_eq!(
        queue.viewport().compute_bounds(window).unwrap().width(),
        window.width() as f32
    );
    assert_eq!(
        queue.viewport().compute_bounds(window).unwrap().height(),
        70.
    );
    let toast = find(queue.popup(), "dz-toast");
    assert_eq!(queue.ids().as_slice(), std::slice::from_ref(&id));
    println!(
        "Toast bounds {:?}, owner active {}",
        toast.compute_bounds(window),
        window.is_active()
    );
    assert_eq!(
        toast.compute_bounds(queue.viewport()).unwrap().height(),
        38.
    );
    #[cfg(target_os = "linux")]
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        let title = window.title();
        window.set_title(Some("DazzUI isolated toast checkpoint"));
        settle();
        let click = |widget: &gtk::Widget| {
            let bounds = widget.compute_bounds(window).unwrap();
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_click.py"))
                .args([
                    "DazzUI isolated toast checkpoint",
                    &((bounds.x() + bounds.width() / 2.).round() as i32).to_string(),
                    &((bounds.y() + bounds.height() / 2.).round() as i32).to_string(),
                ])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
        };
        click(underneath.upcast_ref());
        assert_eq!(
            clicks.get(),
            1,
            "transparent popup regions pass real clicks to the owning window"
        );
        assert_eq!(queue.ids().as_slice(), std::slice::from_ref(&id));
        click(&find(queue.popup(), "dz-toast-dismiss"));
        assert!(
            queue.ids().is_empty(),
            "the receipt's own dismiss button receives real input"
        );
        window.set_title(title.as_deref());
        println!("X11 toast input pass-through and native dismiss click passed");
    }
    queue.close_all();
    settle();
    let mut ids = vec![];
    for (title, tint) in [
        ("primary", Tint::Primary),
        ("neutral", Tint::Neutral),
        ("info", Tint::Info),
        ("success", Tint::Success),
        ("warning", Tint::Warning),
        ("danger", Tint::Danger),
    ] {
        ids.insert(
            0,
            queue.add(ToastOptions {
                title: format!("A {title} toast"),
                tint,
                timeout: Some(Duration::ZERO),
                ..Default::default()
            }),
        );
    }
    settle();
    assert_eq!(queue.ids(), ids);
    let mut child = queue.stack().first_child();
    let mut receipts = vec![];
    while let Some(node) = child {
        child = node.next_sibling();
        receipts.push(node);
    }
    assert_eq!(
        receipts.len(),
        6,
        "React retains limited receipts in the drawing"
    );
    for (i, receipt) in receipts.iter().enumerate() {
        assert_eq!(receipt.can_target(), i < 3);
        assert_eq!(receipt.can_focus(), i < 3);
        let bounds = receipt.compute_bounds(window).unwrap();
        assert_eq!(bounds.y(), 316. + 46. * i as f32);
        println!("Toast tint {i}: {bounds:?}");
    }
    capture_toasts(&queue, window, "tints");
    #[cfg(target_os = "linux")]
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        let title = window.title();
        window.set_title(Some("DazzUI isolated toast keyboard"));
        settle();
        let send = |name: &str| {
            assert!(
                std::process::Command::new("python3")
                    .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_key.py"))
                    .args(["DazzUI isolated toast keyboard", name, "--preserve-focus"])
                    .status()
                    .unwrap()
                    .success()
            );
            settle();
        };
        underneath.grab_focus();
        send("F6");
        assert_eq!(
            gtk::prelude::GtkWindowExt::focus(window).as_ref(),
            Some(queue.viewport().upcast_ref())
        );
        send("Tab");
        assert!(focused(&receipts[0]));
        send("Tab");
        assert!(focused(&find(&receipts[0], "dz-toast-dismiss")));
        send("Tab");
        assert!(focused(&receipts[1]));
        send("Escape");
        assert_eq!(queue.ids().len(), 5);
        assert!(
            focused(&receipts[2]),
            "dismissal focuses the next remaining receipt"
        );
        for _ in 0..4 {
            send("Tab");
        }
        assert!(
            focused(&underneath),
            "the final Tab restores the originating control"
        );
        send("F6");
        queue.close_all();
        settle();
        assert!(
            focused(&underneath),
            "closing the queue restores the last outside control"
        );
        assert_eq!(
            queue.viewport().compute_bounds(window).unwrap().height(),
            32.
        );
        send("F6");
        assert!(focused(queue.viewport()));
        send("Tab");
        assert!(
            focused(&underneath),
            "the empty source viewport remains keyboard reachable"
        );
        window.set_title(title.as_deref());
        println!("X11 toast F6, Tab, Escape and focus return passed");
    }
    queue.close_all();
    settle();
    let closed = Rc::new(Cell::new(0));
    let changed = closed.clone();
    let first = queue.add(ToastOptions {
        id: Some("stable".into()),
        title: "First".into(),
        timeout: Some(Duration::ZERO),
        on_close: Some(Rc::new(move || changed.set(changed.get() + 1))),
        ..Default::default()
    });
    queue.add(ToastOptions {
        id: Some("second".into()),
        title: "Second".into(),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    let changed = closed.clone();
    queue.add(ToastOptions {
        id: Some("stable".into()),
        title: "Replaced".into(),
        timeout: Some(Duration::ZERO),
        on_close: Some(Rc::new(move || changed.set(changed.get() + 1))),
        ..Default::default()
    });
    settle();
    assert_eq!(queue.ids(), ["second", "stable"]);
    assert_eq!(
        closed.get(),
        0,
        "replacement does not call the old close callback"
    );
    queue.close(&first);
    queue.close(&first);
    assert_eq!(closed.get(), 1);
    queue.close_all();
    settle();
    let old_action = Rc::new(Cell::new(0));
    let old_count = old_action.clone();
    queue.add(ToastOptions {
        id: Some("updating".into()),
        title: "Working".into(),
        action: Some(ToastAction {
            label: "Stop".into(),
            on_click: Rc::new(move || old_count.set(old_count.get() + 1)),
        }),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    let retained = find(queue.popup(), "dz-toast");
    let retained_action = find(&retained, "dz-toast-action");
    retained_action.grab_focus();
    let new_action = Rc::new(Cell::new(0));
    let new_count = new_action.clone();
    queue.add(ToastOptions {
        id: Some("updating".into()),
        title: "Completed".into(),
        tint: Tint::Info,
        action: Some(ToastAction {
            label: "Open".into(),
            on_click: Rc::new(move || new_count.set(new_count.get() + 1)),
        }),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    assert_eq!(find(queue.popup(), "dz-toast"), retained);
    assert_eq!(find(&retained, "dz-toast-action"), retained_action);
    let changed = new_action.clone();
    queue.add(ToastOptions {
        id: Some("updating".into()),
        title: "Failed".into(),
        tint: Tint::Danger,
        action: Some(ToastAction {
            label: "Retry".into(),
            on_click: Rc::new(move || changed.set(changed.get() + 1)),
        }),
        ..Default::default()
    });
    settle();
    let alert = find(queue.popup(), "dz-toast");
    assert_eq!(alert.accessible_role(), gtk::AccessibleRole::AlertDialog);
    assert_eq!(
        find(&alert, "dz-toast-action"),
        retained_action,
        "changing GTK's immutable role retains the action control"
    );
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&retained_action),
        "same-ID updates retain the live native action and its focus"
    );
    retained_action
        .downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    settle();
    assert_eq!(old_action.get(), 0);
    assert_eq!(new_action.get(), 1);
    assert!(queue.ids().is_empty());
    let actions = Rc::new(Cell::new(0));
    let changed = actions.clone();
    queue.add(ToastOptions {
        title: "Project archived".into(),
        action: Some(ToastAction {
            label: "Undo".into(),
            on_click: Rc::new(move || changed.set(changed.get() + 1)),
        }),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    assert_eq!(
        find(queue.popup(), "dz-toast")
            .compute_bounds(queue.viewport())
            .unwrap()
            .height(),
        40.
    );
    capture_toasts(&queue, window, "action");
    find(queue.popup(), "dz-toast-action")
        .downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    settle();
    assert_eq!(actions.get(), 1);
    assert!(queue.ids().is_empty());
    queue.close_all();
    settle();
    queue.add(ToastOptions {
        title: "A long single-line receipt ".repeat(40),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    let long = find(queue.popup(), "dz-toast");
    assert_eq!(long.compute_bounds(queue.viewport()).unwrap().width(), 420.);
    assert!(
        find(&long, "dz-toast-message")
            .downcast::<gtk::Label>()
            .unwrap()
            .layout()
            .is_ellipsized()
    );
    let close = find(&long, "dz-toast-dismiss")
        .compute_bounds(&long)
        .unwrap();
    assert!(
        close.x() + close.width() <= 420.,
        "truncation preserves the dismiss button"
    );
    queue.close_all();
    settle();
    let disposable = ToastQueue::new(&root, ToastPlacement::Bottom, 3);
    let weak = disposable.downgrade();
    disposable.add(ToastOptions {
        title: "Disposable".into(),
        timeout: Some(Duration::ZERO),
        ..Default::default()
    });
    settle();
    drop(disposable);
    settle();
    assert!(
        weak.upgrade().is_none(),
        "dropping a queue releases its popup and callbacks"
    );
    #[cfg(target_os = "linux")]
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        let title = window.title();
        window.set_title(Some("DazzUI isolated toast timers"));
        settle();
        let move_to = |widget: &gtk::Widget| {
            let r = widget.compute_bounds(window).unwrap();
            assert!(
                std::process::Command::new("python3")
                    .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_click.py"))
                    .args([
                        "DazzUI isolated toast timers",
                        &((r.x() + r.width() / 2.) as i32).to_string(),
                        &((r.y() + r.height() / 2.) as i32).to_string(),
                        "--move-only"
                    ])
                    .status()
                    .unwrap()
                    .success()
            );
            settle();
        };
        move_to(underneath.upcast_ref());
        underneath.grab_focus();
        let timed = queue.add(ToastOptions {
            title: "Timed".into(),
            timeout: Some(Duration::from_millis(400)),
            ..Default::default()
        });
        let persistent = queue.add(ToastOptions {
            title: "Failure".into(),
            tint: Tint::Danger,
            ..Default::default()
        });
        settle();
        assert!(window.is_active());
        find(queue.popup(), "dz-toast").grab_focus();
        assert!(
            gtk::prelude::GtkWindowExt::focus(window).is_some_and(|w| w.is_ancestor(queue.popup()))
        );
        for _ in 0..6 {
            settle();
        }
        assert!(
            queue.ids().contains(&timed),
            "focusing a receipt pauses every timer"
        );
        underneath.grab_focus();
        for _ in 0..6 {
            settle();
        }
        assert!(
            !queue.ids().contains(&timed),
            "blur resumes the remaining duration"
        );
        assert_eq!(
            queue.ids().as_slice(),
            std::slice::from_ref(&persistent),
            "danger's default timeout is persistent"
        );
        let hovered = queue.add(ToastOptions {
            title: "Hovered".into(),
            timeout: Some(Duration::from_millis(400)),
            ..Default::default()
        });
        settle();
        move_to(&find(queue.popup(), "dz-toast"));
        for _ in 0..6 {
            settle();
        }
        assert!(
            queue.ids().contains(&hovered),
            "pointer hover pauses expiration"
        );
        move_to(underneath.upcast_ref());
        for _ in 0..6 {
            settle();
        }
        assert_eq!(queue.ids(), [persistent]);
        queue.close_all();
        window.set_title(title.as_deref());
        settle();
        println!("X11 toast timeout, focus/hover pause and default danger persistence passed");
    }
    let top = ToastQueue::new(&root, ToastPlacement::Top, 3);
    for title in ["First", "Second", "Third"] {
        top.add(ToastOptions {
            title: title.into(),
            timeout: Some(Duration::ZERO),
            ..Default::default()
        });
    }
    settle();
    let mut child = top.stack().first_child();
    let mut labels = vec![];
    while let Some(node) = child {
        child = node.next_sibling();
        labels.push(
            find(&node, "dz-toast-message")
                .downcast::<gtk::Label>()
                .unwrap()
                .text()
                .to_string(),
        );
    }
    assert_eq!(
        labels,
        ["First", "Second", "Third"],
        "top placement reverses visual source order"
    );
    assert_eq!(
        find(top.popup(), "dz-toast")
            .compute_bounds(window)
            .unwrap()
            .y(),
        16.
    );
    top.close_all();
    settle();
    let live:dazzui_gtk4_example::support::story::Scene=serde_json::from_value(serde_json::json!({
        "id":"toast-state","title":"State","gaps":[],"state":{"message":"Archived","closed":false},
        "content":{"$el":"ToastProvider","children":[
          {"$el":"Button","$at":0,"children":["Archive"],"props":{"onClick":{"$do":[{"toast":{"title":{"$state":"message"},"timeout":0,"action":{"label":"Undo","onClick":{"$do":[{"set":"message","to":"Restored"}]}},"onClose":{"$do":[{"set":"closed","to":true}]}}}]}}},
          {"$el":"Text","$at":1,"children":[{"$state":"message"}]},
          {"$el":"Switch","$at":2,"props":{"checked":{"$state":"closed"}}},
          {"$el":"ToastViewport","$at":3}
        ]}
    })).unwrap();
    let mut specimens = vec![];
    let provider = scene::render_scene(&live, &mut specimens).unwrap();
    window.set_child(Some(&provider));
    settle();
    specimens
        .iter()
        .find(|(at, _)| *at == 0)
        .unwrap()
        .1
        .downcast_ref::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    settle();
    assert_eq!(
        find(window, "dz-toast-message")
            .downcast::<gtk::Label>()
            .unwrap()
            .text(),
        "Archived"
    );
    find(window, "dz-toast-action")
        .downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    settle();
    assert_eq!(
        specimens
            .iter()
            .find(|(at, _)| *at == 1)
            .unwrap()
            .1
            .first_child()
            .unwrap()
            .downcast::<TextBox>()
            .unwrap()
            .text(),
        "Restored"
    );
    assert!(
        specimens
            .iter()
            .find(|(at, _)| *at == 2)
            .unwrap()
            .1
            .downcast_ref::<gtk::Switch>()
            .unwrap()
            .is_active(),
        "onClose retains the originating scene state"
    );
    let story = stories::story_of("toast").unwrap();
    for family in ["bright", "studio", "frost", "graphite", "ember", "nocturne"] {
        for appearance in ["light", "dark"] {
            let theme = format!("{family}-{appearance}");
            Theme::new(&theme).unwrap().apply(window);
            for (index, name) in [(0, "tints"), (1, "action")] {
                let mut specimens = vec![];
                let root = scene::render_scene(&story.examples[index], &mut specimens).unwrap();
                window.set_child(Some(&root));
                settle();
                for (_, button) in &specimens {
                    if let Some(button) = button.downcast_ref::<gtk::Button>() {
                        button.emit_clicked();
                    }
                }
                settle();
                let popup = specimens
                    .iter()
                    .find(|(_, w)| w.has_css_class("dz-toast-viewport"))
                    .unwrap()
                    .1
                    .clone();
                assert!(popup.is_mapped());
                capture_toast_popup(&popup, window, &format!("{theme}-{name}"));
            }
        }
    }
    Theme::new("studio-light").unwrap().apply(window);
}
fn capture_toasts(queue: &ToastQueue, window: &gtk::Window, name: &str) {
    capture_toast_popup(queue.popup(), window, name);
}
fn capture_toast_popup(popup: &gtk::Widget, window: &gtk::Window, name: &str) {
    let Some(dir) = std::env::var_os("DAZZUI_TOAST_CAPTURE_DIR") else {
        return;
    };
    let dir = std::path::Path::new(&dir);
    std::fs::create_dir_all(dir).unwrap();
    let mut records = vec![];
    let mut child = find(popup, "dz-toast-viewport").first_child();
    while let Some(node) = child {
        child = node.next_sibling();
        let bounds = node.compute_bounds(window).unwrap();
        let label = find(&node, "dz-toast-message")
            .downcast::<gtk::Label>()
            .unwrap();
        records.push(serde_json::json!({"title":label.text().to_string(),"x":bounds.x(),"y":bounds.y(),"width":bounds.width(),"height":bounds.height(),"limited":!node.can_target(),"ellipsized":label.layout().is_ellipsized()}));
    }
    std::fs::write(
        dir.join(format!("{name}.json")),
        serde_json::to_string_pretty(&records).unwrap(),
    )
    .unwrap();
    let frame = window.upcast_ref::<gtk::Widget>();
    dazzui_gtk4_example::conformance::screenshot(
        frame,
        frame,
        window,
        &dir.join(format!("{name}.png")),
        1.,
    )
    .unwrap();
}
fn button_group_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    let group = segmented_control(&["List", "Board", "Timeline"], 0);
    let root = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let after = gtk::Entry::new();
    root.append(&group);
    root.append(&after);
    window.set_child(Some(&root));
    settle();
    let buttons = group.buttons();
    let focus = |i: usize| {
        gtk::prelude::GtkWindowExt::focus(window).as_ref() == Some(buttons[i].upcast_ref())
    };
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(group.child_focus(gtk::DirectionType::TabForward));
    assert!(focus(0));
    for (pressed, index) in [
        (Key::Right, 1),
        (Key::Right, 2),
        (Key::Right, 0),
        (Key::End, 2),
        (Key::Home, 0),
        (Key::Left, 2),
    ] {
        assert!(key(&group, pressed, M::empty()));
        settle();
        assert!(focus(index));
        assert!(
            buttons[0]
                .downcast_ref::<gtk::ToggleButton>()
                .unwrap()
                .is_active(),
            "source arrow movement does not activate a segment"
        );
    }
    assert!(
        !group.child_focus(gtk::DirectionType::TabForward),
        "one Tab stop per group"
    );
    after.grab_focus();
    assert!(group.child_focus(gtk::DirectionType::TabBackward));
    assert!(focus(2), "re-entry retains the last roving target");
    buttons[1].set_sensitive(false);
    key(&group, Key::Left, M::empty());
    assert!(focus(0), "navigation skips disabled segments");
    buttons[2]
        .downcast_ref::<gtk::ToggleButton>()
        .unwrap()
        .emit_clicked();
    assert!(
        buttons[2]
            .downcast_ref::<gtk::ToggleButton>()
            .unwrap()
            .is_active()
    );
    assert!(
        !buttons[0]
            .downcast_ref::<gtk::ToggleButton>()
            .unwrap()
            .is_active()
    );
    group.set_stretch(true);
    group.set_hexpand(true);
    settle();
    let widths: Vec<_> = buttons
        .iter()
        .map(|b| b.compute_bounds(&group).unwrap().width())
        .collect();
    assert!(
        widths.iter().copied().fold(f32::NEG_INFINITY, f32::max)
            - widths.iter().copied().fold(f32::INFINITY, f32::min)
            <= 1.
    );
    group.set_direction(gtk::TextDirection::Rtl);
    settle();
    assert!(
        buttons[0].compute_bounds(&group).unwrap().x()
            > buttons[2].compute_bounds(&group).unwrap().x()
    );
    buttons[0].grab_focus();
    key(&group, Key::Right, M::empty());
    assert!(focus(2), "RTL reverses arrow movement");
    #[cfg(target_os = "linux")]
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        assert_eq!(std::env::var("GDK_BACKEND").unwrap(), "x11");
        let title = window.title();
        window.set_title(Some("DazzUI isolated keyboard checkpoint"));
        group.set_direction(gtk::TextDirection::Ltr);
        buttons[1].set_sensitive(true);
        buttons[0].grab_focus();
        settle();
        let send = |name| {
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_key.py"))
                .args(["DazzUI isolated keyboard checkpoint", name])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
        };
        send("Right");
        assert!(
            focus(1),
            "a real X11 arrow reaches the group's capture controller"
        );
        assert!(
            buttons[2]
                .downcast_ref::<gtk::ToggleButton>()
                .unwrap()
                .is_active()
        );
        send("space");
        assert!(
            buttons[1]
                .downcast_ref::<gtk::ToggleButton>()
                .unwrap()
                .is_active(),
            "native GTK Space activation selects the focused segment"
        );
        assert!(
            !buttons[2]
                .downcast_ref::<gtk::ToggleButton>()
                .unwrap()
                .is_active()
        );
        window.set_title(title.as_deref());
        println!("X11 arrow focus and native Space activation passed");
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
            PillTab {
                value: "archived",
                label: "Archived",
                count: None,
                icon: None,
                disabled: false,
            },
        ],
        Some("unread"),
        Tint::Primary,
    );
    window.set_child(Some(&pills));
    settle();
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(pills.child_focus(gtk::DirectionType::TabForward));
    let tabs = pills.buttons();
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(tabs[1].upcast_ref())
    );
    key(&pills, Key::End, M::empty());
    assert!(
        tabs[1]
            .downcast_ref::<gtk::ToggleButton>()
            .unwrap()
            .is_active(),
        "tab focus does not change the selected filter"
    );
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(tabs[2].upcast_ref())
    );
    tabs[2]
        .downcast_ref::<gtk::ToggleButton>()
        .unwrap()
        .emit_clicked();
    assert!(
        tabs[2]
            .downcast_ref::<gtk::ToggleButton>()
            .unwrap()
            .is_active()
    );
    assert!(
        !tabs[1]
            .downcast_ref::<gtk::ToggleButton>()
            .unwrap()
            .is_active()
    );
}

fn date_range_picker_interactions(window: &gtk::Window) {
    use std::{cell::RefCell, rc::Rc};
    let day = |s| CalendarDate::parse(s).unwrap();
    let clock = |d: &gtk::glib::DateTime| format!("{:02}:{:02}", d.hour(), d.minute());
    let render = |index| {
        let mut specimens = vec![];
        let root = scene::render_scene(
            &stories::story_of("date-range-picker").unwrap().examples[index],
            &mut specimens,
        )
        .unwrap();
        let (fixed, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
        window.set_child(Some(&fixed));
        settle();
        specimens
            .into_iter()
            .filter_map(|(_, w)| w.downcast::<DateRangePicker>().ok())
            .collect::<Vec<_>>()
    };
    let open = |p: &DateRangePicker| {
        p.emit_clicked();
        for _ in 0..10 {
            settle();
            if p.calendar()
                .and_then(|c| c.previous_button())
                .is_some_and(|b| focused(&b))
            {
                break;
            }
        }
        assert!(p.is_open());
    };
    let focused = |w: &gtk::Widget| gtk::prelude::GtkWindowExt::focus(window).as_ref() == Some(w);
    let pickers = render(0);
    let p = &pickers[0];
    let original = p.value().unwrap();
    open(p);
    assert!(focused(
        p.calendar()
            .unwrap()
            .previous_button()
            .unwrap()
            .upcast_ref()
    ));
    p.duration_buttons()[2].emit_clicked();
    assert_eq!(p.draft().duration_minutes(), 120.);
    assert_eq!(
        p.value().unwrap(),
        original,
        "draft changes do not reach controlled state"
    );
    p.cancel_button().emit_clicked();
    settle();
    assert!(!p.is_open() && focused(p.upcast_ref()));
    open(p);
    assert_eq!(p.draft(), original, "cancelled duration is discarded");
    p.all_day_button().emit_clicked();
    assert!(p.draft().all_day);
    assert!(!p.start_time().is_sensitive() && !p.end_time().is_sensitive());
    p.duration_buttons()[1].emit_clicked();
    assert!(!p.draft().all_day && p.start_time().is_sensitive());
    p.all_day_button().emit_clicked();
    p.done_button().emit_clicked();
    settle();
    settle();
    assert!(
        p.value().unwrap().all_day,
        "the generated value-change action accepts the commit"
    );
    assert_eq!(clock(&p.value().unwrap().start), "14:30");
    open(p);
    p.all_day_button().emit_clicked();
    // Exercise the native DropDown's actual selected signal.
    let model = p
        .start_time()
        .model()
        .and_downcast::<gtk::StringList>()
        .unwrap();
    let at = (0..model.n_items())
        .find(|i| model.string(*i).as_deref() == Some("23:30"))
        .unwrap();
    p.start_time().set_selected(at);
    assert_eq!(clock(&p.draft().start), "23:30");
    assert_eq!(clock(&p.draft().end), "00:30");
    assert_eq!(p.draft().days().end, Some(day("2026-10-17")));
    assert!(p.duration_buttons().iter().all(|b| !b.is_sensitive()));
    assert!(key(
        &p.popup(),
        gtk::gdk::Key::Escape,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert!(!p.is_open());
    open(p);
    assert_eq!(clock(&p.draft().start), "14:30");
    assert!(p.draft().all_day);
    // Tab enters the grid once, then the native preset/time/duration controls.
    for expected in [
        p.calendar()
            .unwrap()
            .next_button()
            .unwrap()
            .upcast::<gtk::Widget>(),
        p.calendar()
            .unwrap()
            .day_button(day("2026-10-16"))
            .unwrap()
            .upcast(),
        p.preset_button().upcast(),
        p.duration_buttons()[0].clone().upcast(),
    ] {
        assert!(key(
            &p.popup(),
            gtk::gdk::Key::Tab,
            gtk::gdk::ModifierType::empty()
        ));
        settle();
        assert!(
            focused(&expected),
            "unexpected Tab target: {:?}",
            gtk::prelude::GtkWindowExt::focus(window)
        );
    }
    p.cancel();
    settle();
    open(p);
    assert!(key(
        &p.popup(),
        gtk::gdk::Key::ISO_Left_Tab,
        gtk::gdk::ModifierType::SHIFT_MASK
    ));
    settle();
    assert!(
        !p.is_open() && focused(p.upcast_ref()),
        "backward Tab returns to the trigger"
    );
    // Selecting the native Calendar's day starts a new range and carries times.
    open(p);
    p.calendar()
        .unwrap()
        .day_button(day("2026-10-20"))
        .unwrap()
        .emit_clicked();
    assert_eq!(
        p.draft().days(),
        DateRange {
            start: Some(day("2026-10-20")),
            end: Some(day("2026-10-20"))
        }
    );
    assert_eq!(clock(&p.draft().start), "14:30");
    p.calendar()
        .unwrap()
        .day_button(day("2026-10-23"))
        .unwrap()
        .emit_clicked();
    assert_eq!(p.draft().days().end, Some(day("2026-10-23")));
    p.cancel();
    settle();
    let pickers = render(3);
    let p = &pickers[0];
    open(p);
    assert_eq!(p.duration_buttons().len(), 1);
    assert!(!p.duration_buttons()[0].is_sensitive());
    p.choose_preset(1).unwrap();
    assert_eq!(
        p.draft().days(),
        DateRange {
            start: Some(day("2026-10-16")),
            end: Some(day("2026-10-29"))
        }
    );
    p.choose_preset(0).unwrap();
    p.duration_buttons()[0].emit_clicked();
    assert_eq!(p.draft().duration_minutes(), 480.);
    assert_eq!(p.start_time().model().unwrap().n_items(), 48);
    p.done_button().emit_clicked();
    settle();
    assert_eq!(
        p.value().unwrap().duration_minutes(),
        480.,
        "uncontrolled commits persist"
    );
    let sent = Rc::new(RefCell::new(vec![]));
    let p = DateRangePicker::new(DateRangePickerOptions {
        value: Some(Some(original.clone())),
        today: Some(day("2026-10-16")),
        time_step: 60,
        ..Default::default()
    })
    .unwrap();
    let sent2 = sent.clone();
    p.connect_value_change(move |_, v| sent2.borrow_mut().push(v.clone()));
    let stack = gtk::Box::new(gtk::Orientation::Vertical, 8);
    p.set_halign(gtk::Align::Start);
    stack.append(&p);
    let outside = gtk::Entry::new();
    stack.append(&outside);
    window.set_child(Some(&stack));
    settle();
    open(&p);
    let model = p
        .start_time()
        .model()
        .and_downcast::<gtk::StringList>()
        .unwrap();
    assert_eq!(model.n_items(), 25);
    assert_eq!(
        model.string(0).as_deref(),
        Some("14:30"),
        "off-step current time remains selectable"
    );
    p.duration_buttons()[2].emit_clicked();
    p.done_button().emit_clicked();
    settle();
    assert_eq!(sent.borrow().len(), 1);
    assert_eq!(sent.borrow()[0].duration_minutes(), 120.);
    assert_eq!(
        p.value().unwrap(),
        original,
        "a controlled host can refuse a commit"
    );
    open(&p);
    assert_eq!(p.draft(), original);
    p.choose_preset(1).unwrap();
    outside.grab_focus();
    settle();
    assert!(!p.is_open(), "focus leaving the popup dismisses its draft");
    open(&p);
    assert_eq!(p.draft(), original);
    let controllers = window.observe_controllers();
    let outside_y = outside.compute_bounds(window).unwrap().center().y();
    for i in 0..controllers.n_items() {
        if let Some(click) = controllers.item(i).and_downcast::<gtk::GestureClick>() {
            click.emit_by_name::<()>("pressed", &[&1i32, &10f64, &f64::from(outside_y)]);
        }
    }
    assert!(!p.is_open(), "an outside press discards the draft");
    assert_eq!(sent.borrow().len(), 1);
    p.open().unwrap();
    p.cancel();
    settle();
    assert!(
        !p.is_open() && !p.popup().is_visible(),
        "a cancelled pending open cannot map later"
    );
    p.open().unwrap();
    window.set_child(None::<&gtk::Widget>);
    settle();
    assert!(
        !p.is_open() && !p.popup().is_visible(),
        "unrooting cancels pending presentation"
    );
}

fn calendar_interactions(window: &gtk::Window) {
    use std::{cell::Cell, rc::Rc};
    let day = |s| CalendarDate::parse(s).unwrap();
    let focus = |button: &gtk::Button| {
        gtk::prelude::GtkWindowExt::focus(window).as_ref() == Some(button.upcast_ref())
    };
    let render = |index| {
        let mut specimens = vec![];
        let root = scene::render_scene(
            &stories::story_of("calendar").unwrap().examples[index],
            &mut specimens,
        )
        .unwrap();
        window.set_child(Some(&root));
        settle();
        specimens
            .into_iter()
            .filter_map(|(_, w)| w.downcast::<Calendar>().ok())
            .collect::<Vec<_>>()
    };
    let calendars = render(0);
    let c = &calendars[0];
    assert_eq!(c.day_buttons().len(), 42);
    assert_eq!(c.value(), CalendarValue::Single(Some(day("2026-10-20"))));
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(c.child_focus(gtk::DirectionType::TabForward));
    assert!(focus(&c.previous_button().unwrap()));
    assert!(c.child_focus(gtk::DirectionType::TabForward));
    assert!(focus(&c.next_button().unwrap()));
    assert!(c.child_focus(gtk::DirectionType::TabForward));
    assert!(focus(&c.day_button(day("2026-10-20")).unwrap()));
    assert!(
        !c.child_focus(gtk::DirectionType::TabForward),
        "the grid contributes one tab stop"
    );
    assert!(key(
        c,
        gtk::gdk::Key::Right,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert!(focus(&c.day_button(day("2026-10-21")).unwrap()));
    assert_eq!(
        c.value(),
        CalendarValue::Single(Some(day("2026-10-20"))),
        "movement does not select"
    );
    assert!(key(c, gtk::gdk::Key::Down, gtk::gdk::ModifierType::empty()));
    settle();
    assert!(focus(&c.day_button(day("2026-10-28")).unwrap()));
    assert!(key(
        c,
        gtk::gdk::Key::Page_Down,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert_eq!(c.first_month(), day("2026-11-01"));
    assert!(focus(&c.day_button(day("2026-11-28")).unwrap()));
    assert!(key(
        c,
        gtk::gdk::Key::Page_Up,
        gtk::gdk::ModifierType::SHIFT_MASK
    ));
    settle();
    assert_eq!(c.first_month(), day("2025-11-01"));
    assert!(focus(&c.day_button(day("2025-11-28")).unwrap()));
    assert!(key(c, gtk::gdk::Key::Home, gtk::gdk::ModifierType::empty()));
    settle();
    assert!(focus(&c.day_button(day("2025-11-24")).unwrap()));
    assert!(key(c, gtk::gdk::Key::End, gtk::gdk::ModifierType::empty()));
    settle();
    assert!(focus(&c.day_button(day("2025-11-30")).unwrap()));
    assert!(key(
        c,
        gtk::gdk::Key::Return,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert_eq!(c.value(), CalendarValue::Single(Some(day("2025-11-30"))));
    let calendars = render(0);
    let c = &calendars[0];
    c.day_button(day("2026-10-20")).unwrap().grab_focus();
    c.next_button().unwrap().emit_clicked();
    settle();
    assert!(
        gtk::prelude::GtkWindowExt::focus(window).is_none(),
        "a removed date key releases old focus"
    );
    let calendars = render(1);
    let c = &calendars[0];
    c.day_button(day("2026-10-12")).unwrap().emit_clicked();
    settle();
    assert_eq!(
        c.value(),
        CalendarValue::Range(DateRange {
            start: Some(day("2026-10-12")),
            end: None
        })
    );
    let earlier = c.day_button(day("2026-10-08")).unwrap();
    let controllers = earlier.observe_controllers();
    let motion = (0..controllers.n_items())
        .find_map(|i| {
            controllers
                .item(i)
                .and_downcast::<gtk::EventControllerMotion>()
        })
        .unwrap();
    motion.emit_by_name::<()>("enter", &[&0f64, &0f64]);
    settle();
    assert!(earlier.has_css_class("dz-preview"));
    assert!(
        c.day_button(day("2026-10-10"))
            .unwrap()
            .has_css_class("dz-range")
    );
    earlier.emit_clicked();
    settle();
    assert_eq!(
        c.value(),
        CalendarValue::Range(DateRange {
            start: Some(day("2026-10-08")),
            end: Some(day("2026-10-12"))
        })
    );
    assert!(!earlier.has_css_class("dz-preview"));
    let calendars = render(3);
    let c = &calendars[0];
    c.day_button(day("2026-10-23")).unwrap().grab_focus();
    assert!(key(
        c,
        gtk::gdk::Key::Right,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    let unavailable = c.day_button(day("2026-10-24")).unwrap();
    assert!(focus(&unavailable));
    assert!(unavailable.is_sensitive());
    assert!(unavailable.has_css_class("dz-unavailable"));
    assert!(key(
        c,
        gtk::gdk::Key::Return,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert_eq!(c.value(), CalendarValue::Single(Some(day("2026-10-16"))));
    let c = Calendar::new(CalendarOptions {
        value: Some(CalendarValue::Single(Some(day("2026-10-20")))),
        month: Some(day("2026-10-01")),
        today: Some(day("2026-10-16")),
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&c));
    settle();
    let requested = Rc::new(Cell::new(None));
    let out = requested.clone();
    c.connect_value_change(move |_, v| out.set(Some(v)));
    c.pick_day(day("2026-10-21"));
    settle();
    assert_eq!(
        requested.get(),
        Some(CalendarValue::Single(Some(day("2026-10-21"))))
    );
    assert_eq!(c.value(), CalendarValue::Single(Some(day("2026-10-20"))));
    c.set_controlled_value(requested.get()).unwrap();
    assert!(
        c.day_button(day("2026-10-21"))
            .unwrap()
            .has_css_class("dz-selected")
    );
    let requested = Rc::new(Cell::new(None));
    let out = requested.clone();
    c.connect_month_change(move |_, v| out.set(Some(v)));
    c.move_focus(day("2026-11-28"));
    settle();
    assert_eq!(requested.get(), Some(day("2026-11-01")));
    assert_eq!(c.first_month(), day("2026-10-01"));
    c.set_controlled_month(requested.get());
    settle();
    assert_eq!(c.first_month(), day("2026-11-01"));
    let c = Calendar::new(CalendarOptions {
        months: 2,
        mode: CalendarMode::Range,
        default_value: Some(CalendarValue::Range(DateRange {
            start: Some(day("2026-10-28")),
            end: Some(day("2026-11-03")),
        })),
        today: Some(day("2026-10-16")),
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&c));
    settle();
    assert_eq!(c.day_buttons().len(), 84);
    c.move_focus(day("2026-11-04"));
    settle();
    let duplicate = c.day_button(day("2026-11-04")).unwrap();
    assert!(focus(&duplicate));
    assert!(
        duplicate.has_css_class("dz-outside"),
        "focus restores the first matching date, as React's query does"
    );
    c.set_direction(gtk::TextDirection::Rtl);
    assert!(key(
        &c,
        gtk::gdk::Key::Left,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert!(focus(&c.day_button(day("2026-11-03")).unwrap()));
    let c = Calendar::new(CalendarOptions {
        default_value: Some(CalendarValue::Single(Some(day("2024-02-29")))),
        today: Some(day("2024-02-29")),
        navigation: false,
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&c));
    settle();
    c.move_focus(day("2024-02-29"));
    assert!(key(
        &c,
        gtk::gdk::Key::Page_Down,
        gtk::gdk::ModifierType::SHIFT_MASK
    ));
    settle();
    assert!(focus(&c.day_button(day("2025-02-28")).unwrap()));
    assert!(c.previous_button().is_none() && c.next_button().is_none());
    assert!(key(
        &c,
        gtk::gdk::Key::KP_Page_Up,
        gtk::gdk::ModifierType::SHIFT_MASK
    ));
    settle();
    assert!(focus(&c.day_button(day("2024-02-28")).unwrap()));
    assert!(key(
        &c,
        gtk::gdk::Key::KP_Right,
        gtk::gdk::ModifierType::empty()
    ));
    assert!(key(
        &c,
        gtk::gdk::Key::KP_Right,
        gtk::gdk::ModifierType::empty()
    ));
    assert!(key(
        &c,
        gtk::gdk::Key::KP_Space,
        gtk::gdk::ModifierType::empty()
    ));
    settle();
    assert_eq!(c.value(), CalendarValue::Single(Some(day("2024-03-01"))));
    window.set_child(None::<&gtk::Widget>);
    settle();
}
fn sidebar_interactions(window: &gtk::Window) {
    fn render(window: &gtk::Window, index: usize) -> Vec<(usize, gtk::Widget)> {
        let mut specimens = vec![];
        let root = scene::render_scene(
            &stories::story_of("sidebar").unwrap().examples[index],
            &mut specimens,
        )
        .unwrap();
        window.set_child(Some(&root));
        settle();
        specimens
    }
    let initial = render(window, 0);
    let provider = initial
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<SidebarProvider>())
        .unwrap();
    let sidebar = initial
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<Sidebar>())
        .unwrap();
    let rows: Vec<_> = initial
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<gtk::Button>())
        .filter(|w| w.has_css_class("dz-sidebar-menu-button"))
        .collect();
    let trigger = initial
        .iter()
        .find_map(|(_, w)| {
            w.downcast_ref::<gtk::Button>()
                .filter(|w| w.has_css_class("dz-sidebar-trigger"))
        })
        .unwrap();
    assert_eq!(sidebar.width(), 172);
    assert!(rows[0].has_css_class("dz-active"));
    rows[1].emit_clicked();
    settle();
    assert!(!rows[0].has_css_class("dz-active"));
    assert!(rows[1].has_css_class("dz-active"));
    let title = initial
        .iter()
        .rev()
        .find(|(_, w)| w.has_css_class("dz-sidebar-header"))
        .unwrap()
        .1
        .clone();
    let label = find(&title, "dz-text-strong")
        .downcast::<TextBox>()
        .unwrap()
        .label();
    assert_eq!(label.text(), "Glossaries");
    trigger.emit_clicked();
    settle();
    assert!(!provider.is_open() && sidebar.is_collapsed());
    assert_eq!(sidebar.width(), 0);
    assert!(!rows[1].is_mapped());
    assert!(key(
        window,
        gtk::gdk::Key::b,
        gtk::gdk::ModifierType::CONTROL_MASK
    ));
    settle();
    assert!(provider.is_open() && rows[1].is_mapped());
    assert!(rows[1].has_css_class("dz-active"));
    assert!(!key(
        window,
        gtk::gdk::Key::B,
        gtk::gdk::ModifierType::CONTROL_MASK | gtk::gdk::ModifierType::SHIFT_MASK
    ));
    let icons = render(window, 1);
    let provider = icons
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<SidebarProvider>())
        .unwrap();
    let sidebar = icons
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<Sidebar>())
        .unwrap();
    let row = icons
        .iter()
        .find_map(|(_, w)| {
            w.downcast_ref::<gtk::Button>()
                .filter(|w| w.has_css_class("dz-sidebar-menu-button"))
        })
        .unwrap();
    let caption = find(row, "dz-sidebar-caption");
    let popup = find(row, "dz-tooltip").downcast::<gtk::Popover>().unwrap();
    let motion = row.observe_controllers();
    let hover = (0..motion.n_items())
        .find_map(|i| motion.item(i).and_downcast::<gtk::EventControllerMotion>())
        .unwrap();
    hover.emit_by_name::<()>("enter", &[&0f64, &0f64]);
    settle();
    assert!(
        !popup.is_visible(),
        "expanded rows do not show icon-only hints"
    );
    provider.toggle();
    settle();
    assert_eq!(sidebar.width(), 48);
    assert!(!caption.is_visible());
    assert!(row.is_mapped());
    assert_eq!(row.compute_bounds(row).unwrap().width(), 28.);
    hover.emit_by_name::<()>("leave", &[]);
    hover.emit_by_name::<()>("enter", &[&0f64, &0f64]);
    settle();
    assert!(popup.is_visible());
    provider.toggle();
    settle();
    assert!(caption.is_visible());
    assert!(!popup.is_visible());
    let pair = render(window, 2);
    let providers: Vec<_> = pair
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<SidebarProvider>())
        .collect();
    assert_eq!(providers.len(), 2);
    let hidden = SidebarProvider::new(true);
    hidden.set_visible(false);
    providers[1].append(&hidden);
    assert!(key(
        window,
        gtk::gdk::Key::b,
        gtk::gdk::ModifierType::META_MASK
    ));
    settle();
    assert!(providers.iter().all(|p| !p.is_open()));
    assert!(
        !hidden.is_open(),
        "window shortcuts reach mounted hidden providers too"
    );
    providers[1].remove(&hidden);
    assert!(key(
        window,
        gtk::gdk::Key::b,
        gtk::gdk::ModifierType::CONTROL_MASK
    ));
    settle();
    assert!(providers.iter().all(|p| p.is_open()));
    assert!(
        !hidden.is_open(),
        "detached providers leave the window shortcut scope"
    );
    let provider = SidebarProvider::new(false);
    let sidebar = Sidebar::new(
        &provider,
        SidebarOptions {
            side: SidebarSide::Right,
            collapsible: SidebarCollapsible::None,
            ..Default::default()
        },
    );
    sidebar.append(&dazzui_gtk4::label("Unchanging column", "body-medium"));
    provider.append(&sidebar);
    window.set_child(Some(&provider));
    settle();
    assert_eq!(sidebar.width(), 214);
    assert!(!sidebar.is_collapsed());
    provider.set_controlled_open(Some(true));
    let requested = std::rc::Rc::new(std::cell::Cell::new(None));
    let out = requested.clone();
    provider.connect_open_change(move |_, next| out.set(Some(next)));
    provider.toggle();
    settle();
    assert_eq!(requested.get(), Some(false));
    assert!(
        provider.is_open(),
        "controlled requests do not mutate the owner value"
    );
    let recorder = shortcut_recorder(ShortcutRecorderOptions::default());
    provider.append(&recorder);
    recorder.button().grab_focus();
    recorder.start_recording();
    settle();
    assert!(!key(
        window,
        gtk::gdk::Key::b,
        gtk::gdk::ModifierType::CONTROL_MASK
    ));
    assert!(
        provider.is_open(),
        "a recording target consumes the window shortcut"
    );
    assert!(key(
        &recorder.button(),
        gtk::gdk::Key::b,
        gtk::gdk::ModifierType::CONTROL_MASK
    ));
    assert!(!recorder.is_recording());
    provider.remove(&recorder);
    sidebar.set_options(SidebarOptions {
        width: Some(191.),
        collapsed: Some(true),
        ..Default::default()
    });
    settle();
    assert_eq!(sidebar.width(), 0);
    sidebar.set_options(SidebarOptions {
        width: Some(191.),
        collapsed: Some(false),
        ..Default::default()
    });
    provider.set_controlled_open(Some(false));
    settle();
    assert_eq!(sidebar.width(), 191);
    assert!(
        !sidebar.is_collapsed(),
        "explicit column disclosure outranks its provider"
    );
    sidebar.set_direction(gtk::TextDirection::Rtl);
    sidebar.set_options(SidebarOptions {
        side: SidebarSide::Right,
        collapsible: SidebarCollapsible::Icon,
        ..Default::default()
    });
    settle();
    assert_eq!(sidebar.width(), 48);
    window.set_child(None::<&gtk::Widget>);
    settle();
}
fn form_interactions(window: &gtk::Window) {
    use std::{cell::Cell, collections::BTreeMap, rc::Rc};
    let mut specimens = vec![];
    let story = &stories::story_of("form").unwrap().examples[0];
    let root = scene::render_scene(story, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let form = specimens
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<Form>())
        .unwrap();
    form.capture_defaults();
    form.capture_defaults();
    let fields: Vec<_> = specimens
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<FormField>())
        .collect();
    let name = find(fields[0], "dz-field")
        .downcast::<gtk::Entry>()
        .unwrap();
    let email = find(fields[1], "dz-field")
        .downcast::<gtk::Entry>()
        .unwrap();
    let buttons: Vec<_> = specimens
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<gtk::Button>())
        .collect();
    let reset = buttons
        .iter()
        .find(|w| w.has_css_class("dz-variant-outlined"))
        .unwrap();
    let submit = buttons
        .iter()
        .find(|w| w.has_css_class("dz-variant-filled"))
        .unwrap();
    let focused = |entry: &gtk::Entry| {
        gtk::prelude::GtkWindowExt::focus(window)
            .is_some_and(|w| w == entry.clone().upcast::<gtk::Widget>() || w.is_ancestor(entry))
    };
    let count = Rc::new(Cell::new(0));
    let out = count.clone();
    form.connect_submit(move |_, _| out.set(out.get() + 1));
    assert!(
        fields
            .iter()
            .all(|field| !field.is_invalid() && !field.is_dirty())
    );
    submit.emit_clicked();
    settle();
    assert_eq!(count.get(), 0);
    assert!(
        fields
            .iter()
            .all(|field| field.is_invalid() && !field.is_dirty())
    );
    assert!(focused(&name));
    name.set_text("Ada");
    email.set_text("bad");
    settle();
    assert!(!fields[0].is_invalid());
    assert!(fields[1].is_invalid());
    assert!(fields.iter().all(|field| field.is_dirty()));
    submit.emit_clicked();
    settle();
    assert!(focused(&email));
    assert_eq!(email.selection_bounds(), Some((0, 3)));
    reset.grab_focus();
    reset.emit_clicked();
    settle();
    assert_eq!(name.text(), "");
    assert_eq!(email.text(), "");
    assert!(!fields[0].is_invalid());
    assert!(fields[1].is_invalid());
    assert!(fields.iter().all(|field| field.is_dirty()));
    name.set_text("Ada");
    email.set_text("ada@example.com");
    settle();
    submit.emit_clicked();
    settle();
    assert_eq!(count.get(), 1);
    assert!(fields.iter().all(|field| !field.is_invalid()));
    reset.emit_clicked();
    settle();
    assert_eq!(email.text(), "");
    assert!(fields.iter().all(|field| !field.is_invalid()));
    name.set_sensitive(false);
    email.set_text("bad");
    email.set_editable(false);
    settle();
    assert!(!form.submit());
    assert!(focused(&email));
    assert!(fields[1].is_invalid());
    assert_eq!(count.get(), 1);
    email.set_editable(true);
    name.set_sensitive(true);
    fields[0].set_rules(FieldRules {
        name: Some("name".into()),
        required: true,
        ..Default::default()
    });
    fields[1].set_rules(FieldRules {
        name: Some("email".into()),
        required: true,
        constraint: TextConstraint::Email,
        ..Default::default()
    });
    name.set_text("Ada");
    email.set_text("ada@example.com");
    settle();
    assert!(form.submit());
    assert_eq!(
        form.values().get("email").map(String::as_str),
        Some("ada@example.com")
    );
    form.set_errors(BTreeMap::from([(
        "email".into(),
        vec!["Already taken".into()],
    )]));
    assert!(fields[1].is_invalid());
    assert!(focused(&email));
    email.set_text("new@example.com");
    settle();
    assert!(!fields[1].is_invalid());
    assert!(fields[1].server_errors().is_empty());
    let before = count.get();
    email.emit_by_name::<()>("activate", &[]);
    settle();
    assert_eq!(
        count.get(),
        before + 1,
        "native activation performs one implicit submit"
    );
    let source = &stories::story_of("form-field").unwrap().examples[0];
    let mut specimens = vec![];
    let root = scene::render_scene(source, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let field = specimens
        .iter()
        .find_map(|(_, w)| w.downcast_ref::<FormField>())
        .unwrap();
    let input = find(field, "dz-field").downcast::<gtk::Entry>().unwrap();
    input.set_text("bad");
    settle();
    assert!(!field.is_invalid());
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    settle();
    assert!(
        !field.is_invalid(),
        "default boundary is submission, including standalone fields"
    );
    input.emit_by_name::<()>("activate", &[]);
    settle();
    assert!(field.is_invalid());
    input.set_text("still bad");
    settle();
    assert!(
        !field.is_invalid(),
        "typing clears a previous error before the next validation boundary"
    );
    field.set_rules(FieldRules {
        constraint: TextConstraint::Email,
        validation_mode: Some(ValidationMode::OnBlur),
        ..Default::default()
    });
    input.grab_focus();
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    settle();
    assert!(field.is_invalid());
    field.set_rules(FieldRules {
        constraint: TextConstraint::Email,
        validation_mode: Some(ValidationMode::OnChange),
        ..Default::default()
    });
    input.set_text("wrong");
    settle();
    assert!(field.is_invalid());
    input.set_text("a@b");
    settle();
    assert!(!field.is_invalid());
}
fn combobox_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    use std::{cell::RefCell, rc::Rc};
    let source = &stories::story_of("combobox").unwrap().preview;
    let mut specimens = vec![];
    let root = scene::render_scene(source, &mut specimens).unwrap();
    let (frame, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&frame));
    settle();
    let combo = specimens[0].1.downcast_ref::<Combobox>().unwrap();
    let input = combo.entry();
    let focused = || {
        gtk::prelude::GtkWindowExt::focus(window)
            .is_some_and(|w| w == input.clone().upcast::<gtk::Widget>() || w.is_ancestor(&input))
    };
    let full = ["China", "Germany", "Japan", "New Zealand", "United States"];
    assert_eq!(combo.value().as_deref(), Some("jp"));
    assert_eq!(input.text(), "Japan");
    assert!(combo.clear_button().is_visible());
    assert!(!combo.is_open());
    input.grab_focus();
    combo.open();
    settle();
    assert!(combo.popup().is_mapped());
    let panel = combo.popup().first_child().unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    while panel.width() == 0 && std::time::Instant::now() < deadline {
        settle();
    }
    assert!(panel.width() > 0, "native popup was not allocated");
    let panel_bounds = panel.compute_bounds(&panel).unwrap();
    assert_eq!(
        panel_bounds.width().round() as i32,
        combo.compute_bounds(combo).unwrap().width().round() as i32,
        "popup matches the field's outer width"
    );

    assert!(focused(), "native popup retains entry focus");
    assert_eq!(combo.visible_options(), full);
    assert_eq!(combo.highlighted().as_deref(), Some("jp"));
    input.set_text("g");
    settle();
    assert_eq!(combo.visible_options(), ["Germany"]);
    assert_eq!(combo.highlighted(), None);
    assert_eq!(combo.value().as_deref(), Some("jp"));
    key(&input, Key::Down, M::empty());
    assert_eq!(combo.highlighted().as_deref(), Some("de"));
    key(&input, Key::Return, M::empty());
    settle();
    assert_eq!(combo.value().as_deref(), Some("de"));
    assert_eq!(input.text(), "Germany");
    assert!(!combo.is_open());
    input.set_text("unknown");
    settle();
    assert!(combo.is_open());
    assert!(combo.visible_options().is_empty());
    key(&input, Key::Escape, M::empty());
    settle();
    assert_eq!(input.text(), "Germany");
    assert!(!combo.is_open());
    key(&input, Key::Escape, M::empty());
    settle();
    assert_eq!(input.text(), "");
    assert_eq!(combo.value(), None);
    combo.open();
    key(&input, Key::Up, M::empty());
    assert_eq!(combo.highlighted().as_deref(), Some("us"));
    key(&input, Key::Down, M::empty());
    assert_eq!(combo.highlighted(), None);
    key(&input, Key::Down, M::empty());
    assert_eq!(combo.highlighted().as_deref(), Some("cn"));
    let row = combo.list().row_at_index(2).unwrap();
    row.emit_by_name::<()>("activate", &[]);
    settle();
    assert_eq!(combo.value().as_deref(), Some("jp"));
    combo.clear_button().emit_clicked();
    settle();
    assert_eq!(combo.value(), None);
    assert_eq!(input.text(), "");
    assert!(!combo.clear_button().is_visible());
    assert!(focused());
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(combo.child_focus(gtk::DirectionType::TabForward));
    assert!(focused());
    assert!(!combo.child_focus(gtk::DirectionType::TabForward));
    combo.set_value(Some("jp"));
    combo.set_options(&[ComboboxItem::Option(ComboboxOption {
        value: "jp".into(),
        label: "Changed label".into(),
        disabled: false,
    })]);
    assert_eq!(
        input.text(),
        "Japan",
        "uncontrolled choices keep the selected option object label"
    );
    combo.set_options(&[]);
    assert_eq!(combo.value().as_deref(), Some("jp"));
    assert_eq!(input.text(), "Japan");
    let grouped = &stories::story_of("combobox").unwrap().examples[1];
    let mut specimens = vec![];
    let root = scene::render_scene(grouped, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let grouped = specimens[0].1.downcast_ref::<Combobox>().unwrap();
    grouped.entry().set_text("g");
    settle();
    assert_eq!(grouped.visible_options(), ["Germany"]);
    assert!(!grouped.list().row_at_index(0).unwrap().is_visible());
    assert!(grouped.list().row_at_index(3).unwrap().is_visible());
    let auto = autocomplete(AutocompleteOptions {
        items: vec![
            "Inter".into(),
            "IBM Plex Sans".into(),
            "JetBrains Mono".into(),
            "Source Serif".into(),
        ],
        ..Default::default()
    });
    window.set_child(Some(&auto));
    settle();
    auto.entry().grab_focus();
    assert!(!auto.is_open());
    auto.entry().set_text("I");
    settle();
    assert!(auto.is_open());
    assert_eq!(auto.visible_options().len(), 4);
    key(&auto.entry(), Key::Down, M::empty());
    assert_eq!(auto.highlighted().as_deref(), Some("Inter"));
    key(&auto.entry(), Key::Return, M::empty());
    settle();
    assert_eq!(auto.value().as_deref(), Some("Inter"));
    assert!(!auto.is_open());
    auto.entry().set_text("Custom Font");
    settle();
    key(&auto.entry(), Key::Escape, M::empty());
    assert_eq!(auto.value().as_deref(), Some("Custom Font"));
    let options = vec![
        ComboboxItem::Option(ComboboxOption {
            value: "a".into(),
            label: "Alpha".into(),
            disabled: true,
        }),
        ComboboxItem::Option(ComboboxOption {
            value: "b".into(),
            label: "Bravo".into(),
            disabled: false,
        }),
    ];
    let locked = Combobox::new(ComboboxOptions {
        options,
        value: Some(None),
        ..Default::default()
    });
    window.set_child(Some(&locked));
    settle();
    locked.entry().grab_focus();
    let events = Rc::new(RefCell::new(vec![]));
    let out = events.clone();
    locked.connect_value_changed(move |_, edit| out.borrow_mut().push(edit.value.clone()));
    let cancel = locked.connect_value_changed(|_, edit| edit.cancel());
    key(&locked.entry(), Key::Down, M::empty());
    assert_eq!(locked.highlighted().as_deref(), Some("b"));
    key(&locked.entry(), Key::Return, M::empty());
    assert_eq!(locked.value(), None);
    assert!(locked.is_open());
    locked.disconnect(cancel);
    key(&locked.entry(), Key::Return, M::empty());
    assert_eq!(locked.value(), None);
    locked.connect_value_changed(|field, edit| field.set_value(edit.value.as_deref()));
    key(&locked.entry(), Key::Down, M::empty());
    key(&locked.entry(), Key::Return, M::empty());
    assert_eq!(locked.value().as_deref(), Some("b"));
    locked.set_read_only(true);
    locked.clear_button().emit_clicked();
    key(&locked.entry(), Key::Down, M::empty());
    assert_eq!(locked.value().as_deref(), Some("b"));
    assert!(!locked.is_open());
    locked.set_read_only(false);
    locked.open();
    settle();
    window.set_child(Some(&gtk::Box::new(gtk::Orientation::Vertical, 0)));
    settle();
    assert!(!locked.popup().is_mapped());
}
fn slider_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    use std::{cell::RefCell, rc::Rc};
    let source = &stories::story_of("slider").unwrap().examples[1];
    let mut specimens = vec![];
    let root = scene::render_scene(source, &mut specimens).unwrap();
    let (frame, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&frame));
    settle();
    let field = specimens
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<SliderField>())
        .nth(1)
        .unwrap();
    assert_eq!(field.values(), [20., 80.]);
    assert_eq!(field.reading().text(), "20 – 80");
    let scales = field.scales();
    let control = field.control();
    let width = f64::from(control.width());
    for (i, value) in field.values().iter().enumerate() {
        let (a, b) = scales[i].slider_range();
        let offset = scales[i]
            .compute_point(&control, &gtk::graphene::Point::new(0., 0.))
            .unwrap()
            .x();
        assert!(
            (f64::from(a + b) / 2. + f64::from(offset) - value / 100. * width).abs() < 0.01,
            "fractional thumb centre"
        );
        assert_eq!(
            control.pick(value / 100. * width, 14., gtk::PickFlags::DEFAULT),
            Some(scales[i].clone().upcast()),
            "native hit target for thumb {i}"
        );
    }
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(field.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window),
        Some(scales[0].clone().upcast())
    );
    assert!(field.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window),
        Some(scales[1].clone().upcast())
    );
    assert!(!field.child_focus(gtk::DirectionType::TabForward));
    scales[0].grab_focus();
    key(&scales[0], Key::Right, M::empty());
    assert_eq!(field.values(), [21., 80.]);
    key(&scales[0], Key::Right, M::SHIFT_MASK);
    assert_eq!(field.values(), [31., 80.]);
    key(&scales[0], Key::End, M::empty());
    assert_eq!(field.values(), [80., 80.]);
    key(&scales[0], Key::Right, M::empty());
    assert_eq!(field.values(), [80., 80.]);
    key(&scales[0], Key::Home, M::empty());
    assert_eq!(field.values(), [0., 80.]);
    key(&scales[0], Key::Page_Up, M::empty());
    assert_eq!(field.values(), [10., 80.]);
    let drag = |field: &SliderField, index: usize, points: &[f64]| {
        let scale = field.scales()[index].clone();
        let control = field.control();
        let width = f64::from(control.width());
        let value = field.values()[index];
        let centre = if field.direction() == gtk::TextDirection::Rtl {
            1. - value / 100.
        } else {
            value / 100.
        } * width;
        let shift = f64::from(
            scale
                .compute_point(&control, &gtk::graphene::Point::new(0., 0.))
                .unwrap()
                .x(),
        );
        let controllers = scale.observe_controllers();
        let click = (0..controllers.n_items())
            .filter_map(|i| controllers.item(i).and_downcast::<gtk::GestureClick>())
            .find(|c| c.propagation_phase() == gtk::PropagationPhase::Capture)
            .unwrap();
        let drag = (0..controllers.n_items())
            .find_map(|i| controllers.item(i).and_downcast::<gtk::GestureDrag>())
            .unwrap();
        assert!(
            click
                .group()
                .iter()
                .any(|g| g == drag.upcast_ref::<gtk::Gesture>())
        );
        click.emit_by_name::<()>("pressed", &[&1i32, &(centre - shift), &14f64]);
        for point in points {
            drag.emit_by_name::<()>("drag-update", &[&(point * width - centre), &0f64]);
        }
        drag.emit_by_name::<()>("drag-end", &[&0f64, &0f64]);
    };
    field.set_values(&[20., 80.]).unwrap();
    settle();
    drag(field, 0, &[0.9, 0.3]);
    settle();
    assert_eq!(
        field.values(),
        [30., 90.],
        "pushed neighbour does not spring back"
    );
    field.set_values(&[100., 100.]).unwrap();
    settle();
    assert_eq!(
        control.pick(f64::from(control.width()), 14., gtk::PickFlags::DEFAULT),
        Some(scales[0].clone().upcast()),
        "coincident maximum thumbs select the first"
    );
    field.set_values(&[20., 80.]).unwrap();
    field.set_direction(gtk::TextDirection::Rtl);
    settle();
    key(&scales[0], Key::Right, M::empty());
    assert_eq!(field.values(), [19., 80.]);
    let gap = slider_field(SliderOptions {
        default_values: vec![20., 80.],
        step: 5.,
        min_steps_between_values: 2.,
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&gap));
    settle();
    key(&gap.scales()[0], Key::End, M::empty());
    assert_eq!(gap.values(), [70., 80.]);
    key(&gap.scales()[0], Key::Right, M::empty());
    assert_eq!(gap.values(), [70., 80.]);
    drag(&gap, 0, &[1.]);
    settle();
    assert_eq!(gap.values(), [90., 100.]);
    let fixed = slider_field(SliderOptions {
        default_values: vec![20., 80.],
        step: 5.,
        min_steps_between_values: 2.,
        collision: SliderCollision::None,
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&fixed));
    settle();
    drag(&fixed, 0, &[1.]);
    assert_eq!(
        fixed.values(),
        [70., 80.],
        "none collision stops at the gap"
    );
    fixed.set_values(&[50., 50., 50.]).unwrap();
    settle();
    drag(&fixed, 1, &[0.8]);
    assert_eq!(
        fixed.values(),
        [50., 50., 50.],
        "infeasible neighbours reject movement"
    );
    let locked = slider_field(SliderOptions {
        value: Some(vec![20., 80.]),
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&locked));
    settle();
    let seen = Rc::new(RefCell::new(vec![]));
    let out = seen.clone();
    locked.connect_value_committed(move |_, values, _| out.borrow_mut().push(values.to_vec()));
    let cancel = locked.connect_value_changed(|_, edit| edit.cancel());
    key(&locked.scales()[0], Key::Right, M::empty());
    assert_eq!(locked.values(), [20., 80.]);
    assert!(seen.borrow().is_empty());
    locked.disconnect(cancel);
    key(&locked.scales()[0], Key::Right, M::empty());
    assert_eq!(locked.values(), [20., 80.]);
    assert_eq!(*seen.borrow(), [vec![21., 80.]]);
    locked.connect_value_changed(|field, edit| field.set_values(&edit.values).unwrap());
    key(&locked.scales()[0], Key::Right, M::empty());
    assert_eq!(locked.values(), [21., 80.]);
    let same = locked.scales()[0].clone();
    locked.set_values(&[1., 2., 3.]).unwrap();
    settle();
    assert_eq!(locked.scales()[0], same);
    assert_eq!(locked.scales().len(), 3);
    locked.set_values(&[50.]).unwrap();
    settle();
    assert_eq!(locked.scales()[0], same);
    locked.set_sensitive(false);
    key(&same, Key::Right, M::empty());
    assert_eq!(locked.values(), [50.]);
}
fn otp_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    use std::{cell::RefCell, rc::Rc};
    let root = scene::render_scene(
        &stories::story_of("otp-field").unwrap().preview,
        &mut vec![],
    )
    .unwrap();
    window.set_child(Some(&root));
    settle();
    let otp = root.downcast::<OtpField>().unwrap();
    let cells = otp.cells();
    let focused = |field: &OtpField, index: usize| {
        let focus = gtk::prelude::GtkWindowExt::focus(window).unwrap();
        assert!(
            focus == field.cells()[index].clone().upcast::<gtk::Widget>()
                || focus.is_ancestor(&field.cells()[index]),
            "OTP focus index {index}"
        );
    };
    assert_eq!(otp.value(), "");
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(otp.child_focus(gtk::DirectionType::TabForward));
    focused(&otp, 0);
    assert!(
        !otp.child_focus(gtk::DirectionType::TabForward),
        "OTP is one Tab stop"
    );
    let events = Rc::new(RefCell::new(vec![]));
    let out = events.clone();
    otp.connect_value_changed(move |_, event| {
        out.borrow_mut()
            .push((event.value.clone(), event.reason.as_str().to_owned()))
    });
    let completed = Rc::new(RefCell::new(vec![]));
    let out = completed.clone();
    otp.connect_value_complete(move |_, value, reason| {
        out.borrow_mut().push((value.to_owned(), reason.to_owned()))
    });
    let invalid = Rc::new(RefCell::new(vec![]));
    let out = invalid.clone();
    otp.connect_value_invalid(move |_, raw, _| out.borrow_mut().push(raw.to_owned()));
    let mut position = 0;
    cells[0].insert_text("1", &mut position);
    settle();
    assert_eq!(otp.value(), "1");
    focused(&otp, 1);
    // Native replacement deletes the selection before inserting. It remains one domain change.
    otp.set_value("123");
    otp.focus_cell(1);
    let before = events.borrow().len();
    cells[1].delete_selection();
    cells[1].insert_text("9", &mut 0);
    settle();
    assert_eq!(otp.value(), "193");
    assert_eq!(events.borrow().len(), before + 1);
    focused(&otp, 2);
    assert_eq!(cells[1].text(), "9");
    otp.insert(0, " 12x 3456 ", OtpReason::InputPaste);
    settle();
    assert_eq!(otp.value(), "123456");
    focused(&otp, 5);
    assert_eq!(cells[5].selection_bounds(), Some((0, 1)));
    assert_eq!(*invalid.borrow(), [" 12x 3456 "]);
    assert_eq!(
        completed.borrow().last().unwrap(),
        &("123456".into(), "input-paste".into())
    );
    let before = events.borrow().len();
    otp.insert(0, "123456", OtpReason::InputPaste);
    settle();
    assert_eq!(events.borrow().len(), before);
    assert_eq!(
        completed.borrow().len(),
        2,
        "complete paste repeats completion without a change"
    );
    key(&cells[5], Key::BackSpace, M::empty());
    settle();
    assert_eq!(otp.value(), "12345");
    focused(&otp, 4);
    key(&cells[4], Key::Home, M::empty());
    focused(&otp, 0);
    key(&cells[0], Key::Delete, M::empty());
    settle();
    assert_eq!(otp.value(), "2345");
    focused(&otp, 0);
    key(&cells[0], Key::End, M::empty());
    focused(&otp, 4);
    otp.focus_cell(5);
    focused(&otp, 4);
    key(&cells[4], Key::BackSpace, M::CONTROL_MASK);
    settle();
    assert_eq!(otp.value(), "");
    focused(&otp, 0);
    otp.set_value("123");
    otp.focus_cell(1);
    key(&cells[1], Key::_2, M::empty());
    focused(&otp, 2);
    assert_eq!(otp.value(), "123");
    otp.set_direction(gtk::TextDirection::Rtl);
    key(&cells[2], Key::Right, M::empty());
    focused(&otp, 1);
    key(&cells[1], Key::Left, M::CONTROL_MASK);
    focused(&otp, 3);
    otp.set_read_only(true);
    let before = otp.value();
    otp.insert(0, "99", OtpReason::InputPaste);
    key(&cells[3], Key::BackSpace, M::empty());
    settle();
    assert_eq!(otp.value(), before);
    otp.set_read_only(false);
    otp.set_value("");
    cells[0].set_text("987");
    settle();
    assert_eq!(
        otp.value(),
        "987",
        "native buffer writes feed the same slots"
    );
    focused(&otp, 3);
    otp.set_mask(true);
    assert!(!gtk::prelude::EntryExt::is_visible(&cells[0]));
    otp.set_mask(false);
    assert!(gtk::prelude::EntryExt::is_visible(&cells[0]));
    // Host-controlled edits are cancellable, can be refused, or accepted later.
    let controlled = otp_field(OtpOptions {
        length: 4,
        value: Some("12".into()),
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&controlled));
    settle();
    controlled.focus_cell(0);
    let cancel = controlled.connect_value_changed(|_, event| event.cancel());
    controlled.insert(0, "9999", OtpReason::InputPaste);
    settle();
    assert_eq!(controlled.value(), "12");
    focused(&controlled, 0);
    controlled.disconnect(cancel);
    controlled.insert(0, "3456", OtpReason::InputPaste);
    settle();
    assert_eq!(controlled.value(), "12");
    let done = Rc::new(RefCell::new(vec![]));
    let out = done.clone();
    controlled.connect_value_complete(move |_, value, _| out.borrow_mut().push(value.to_owned()));
    controlled.set_value("3456");
    settle();
    focused(&controlled, 3);
    assert_eq!(*done.borrow(), ["3456"]);
    let accept = controlled.connect_value_changed(|field, event| field.set_value(&event.value));
    controlled.insert(0, "abc7", OtpReason::InputChange);
    settle();
    assert_eq!(controlled.value(), "7456");
    controlled.disconnect(accept);
    let alpha = otp_field(OtpOptions {
        length: 4,
        validation: OtpValidation::Alpha,
        ..Default::default()
    })
    .unwrap();
    window.set_child(Some(&alpha));
    settle();
    alpha.insert(0, " A1b2c3D ", OtpReason::InputPaste);
    settle();
    assert_eq!(alpha.value(), "AbcD");
    let weak = controlled.downgrade();
    drop(controlled);
    settle();
    assert!(weak.upgrade().is_none());
}
fn shortcut_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    use std::{cell::RefCell, rc::Rc};
    let source = &stories::story_of("shortcut-recorder").unwrap().preview;
    let root = scene::render_scene(source, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let recorder = root.clone().downcast::<ShortcutRecorder>().unwrap();
    let button = recorder.button();
    let clear = recorder.clear_button();
    assert_eq!(recorder.value(), "⌘K");
    assert!(clear.is_visible());
    assert_eq!(button.compute_bounds(&root).unwrap().width(), 132.);
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    assert!(recorder.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window),
        Some(button.clone().upcast())
    );
    assert!(recorder.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window),
        Some(clear.clone().upcast())
    );
    assert!(!recorder.child_focus(gtk::DirectionType::TabForward));
    button.emit_clicked();
    assert!(recorder.is_recording());
    assert!(!clear.is_visible());
    assert_eq!(button.label().as_deref(), Some("Press keys…"));
    assert!(key(&button, Key::Alt_L, M::empty()));
    assert_eq!(button.label().as_deref(), Some("⌥"));
    key(&button, Key::Shift_L, M::ALT_MASK);
    assert_eq!(button.label().as_deref(), Some("⌥⇧"));
    let keys = button.observe_controllers();
    let keys = (0..keys.n_items())
        .find_map(|i| keys.item(i).and_downcast::<gtk::EventControllerKey>())
        .unwrap();
    keys.emit_by_name::<()>(
        "key-released",
        &[&Key::Shift_L, &0u32, &(M::ALT_MASK | M::SHIFT_MASK)],
    );
    assert_eq!(button.label().as_deref(), Some("⌥"));
    key(&button, Key::Control_L, M::ALT_MASK);
    key(&button, Key::Control_R, M::ALT_MASK | M::CONTROL_MASK);
    keys.emit_by_name::<()>(
        "key-released",
        &[&Key::Control_L, &0u32, &(M::ALT_MASK | M::CONTROL_MASK)],
    );
    assert_eq!(
        button.label().as_deref(),
        Some("⌃⌥"),
        "the other Control key is still held"
    );
    keys.emit_by_name::<()>(
        "key-released",
        &[&Key::Control_R, &0u32, &(M::ALT_MASK | M::CONTROL_MASK)],
    );
    assert_eq!(button.label().as_deref(), Some("⌥"));
    let code = if button.display().type_().name() == "GdkMacosDisplay" {
        17u32
    } else {
        28
    };
    assert!(keys.emit_by_name::<bool>("key-pressed", &[&Key::dagger, &code, &M::ALT_MASK]));
    assert_eq!(recorder.value(), "⌥T");
    assert!(!recorder.is_recording());
    assert!(clear.is_visible());
    button.emit_clicked();
    key(&button, Key::Escape, M::empty());
    assert_eq!(recorder.value(), "⌥T");
    assert!(!recorder.is_recording());
    button.emit_clicked();
    assert!(key(&button, Key::Tab, M::empty()));
    assert!(button.has_css_class("dz-rejected"));
    assert!(recorder.is_recording());
    for _ in 0..3 {
        settle();
    }
    if button.has_css_class("dz-rejected") {
        key(&button, Key::a, M::empty());
        settle();
        assert!(
            !button.has_css_class("dz-rejected"),
            "repeated invalid presses keep the first rejection timeout"
        );
    }
    key(&button, Key::F5, M::empty());
    assert_eq!(recorder.value(), "F5");
    button.emit_clicked();
    key(&button, Key::BackSpace, M::empty());
    assert_eq!(recorder.value(), "");
    assert_eq!(button.label().as_deref(), Some("Record shortcut"));
    assert!(!clear.is_visible());
    button.emit_clicked();
    key(&button, Key::k, M::CONTROL_MASK);
    assert_eq!(recorder.value(), "⌃K");
    clear.emit_clicked();
    assert_eq!(recorder.value(), "");
    recorder.set_value("⌘K");
    settle();
    let ltr = clear.compute_bounds(&button).unwrap().x();
    recorder.set_direction(gtk::TextDirection::Rtl);
    settle();
    assert!(
        clear.compute_bounds(&button).unwrap().x() < ltr,
        "clear icon mirrors in RTL"
    );
    let after = gtk::Button::with_label("After");
    let column = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_child(None::<&gtk::Widget>);
    column.append(&recorder);
    column.append(&after);
    window.set_child(Some(&column));
    settle();
    button.emit_clicked();
    Theme::new("bright-dark").unwrap().apply(window);
    settle();
    assert!(
        recorder.is_recording(),
        "a theme update keeps the native recording session"
    );
    Theme::new("studio-light").unwrap().apply(window);
    settle();
    after.grab_focus();
    settle();
    assert!(!recorder.is_recording(), "leaving cancels recording");
    button.emit_clicked();
    recorder.set_sensitive(false);
    assert!(!recorder.is_recording());
    assert!(!clear.is_visible());
    button.emit_clicked();
    assert!(!recorder.is_recording());
    recorder.set_sensitive(true);
    button.emit_clicked();
    key(&button, Key::x, M::empty());
    assert!(button.has_css_class("dz-rejected"));
    window.set_child(None::<&gtk::Widget>);
    assert!(!recorder.is_recording());
    assert!(!button.has_css_class("dz-rejected"));
    assert_eq!(recorder.button(), button, "unmapping keeps native controls");
    // No callback may silently change the bound value; custom clear replaces
    // the default empty-value request, and disconnecting restores that default.
    let locked = shortcut_recorder(ShortcutRecorderOptions {
        value: "⌘L".into(),
        ..Default::default()
    });
    let requests = Rc::new(RefCell::new(vec![]));
    let seen = requests.clone();
    locked.connect_value_changed(move |_, v| seen.borrow_mut().push(v.to_owned()));
    window.set_child(Some(&locked));
    settle();
    locked.button().emit_clicked();
    key(&locked.button(), Key::f, M::CONTROL_MASK);
    assert_eq!(locked.value(), "⌘L");
    assert_eq!(*requests.borrow(), ["⌃F"]);
    let custom = Rc::new(RefCell::new(0));
    let seen = custom.clone();
    let handler = locked.connect_clear(move |_| *seen.borrow_mut() += 1);
    locked.clear_button().emit_clicked();
    assert_eq!(*custom.borrow(), 1);
    assert_eq!(requests.borrow().len(), 1);
    locked.disconnect(handler);
    locked.clear_button().emit_clicked();
    assert_eq!(*requests.borrow(), ["⌃F", ""]);
    let source = &stories::story_of("shortcut-recorder").unwrap().examples[1];
    let mut specimens = vec![];
    let root = scene::render_scene(source, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let disabled = specimens
        .iter()
        .filter_map(|(_, w)| w.downcast_ref::<gtk::Button>())
        .find(|w| !w.is_sensitive())
        .unwrap();
    disabled.emit_clicked();
    let disabled_root = disabled
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .downcast::<ShortcutRecorder>()
        .unwrap();
    assert!(!disabled_root.is_recording());
    let weak = locked.downgrade();
    drop(locked);
    settle();
    assert!(
        weak.upgrade().is_none(),
        "recording timers must not retain the control"
    );
}
fn tree_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    use std::{cell::RefCell, rc::Rc};
    let source = &stories::story_of("tree").unwrap().examples[0];
    let root = scene::render_scene(source, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let tree = find(&root, "dz-tree-content").downcast::<Tree>().unwrap();
    assert_eq!(tree.accessible_role(), gtk::AccessibleRole::Tree);
    assert_eq!(
        tree.visible_ids(),
        [
            "src",
            "components",
            "button.tsx",
            "tree.tsx",
            "styles",
            "index.ts",
            "package.json",
            "README.md"
        ]
    );
    assert_eq!(tree.expanded(), ["src", "components", "changed"]);
    assert_eq!(tree.selected().as_deref(), Some("tree.tsx"));
    let focused = |tree: &Tree, id: &str| {
        assert_eq!(
            gtk::prelude::GtkWindowExt::focus(window),
            tree.row(id).map(|r| r.upcast()),
            "focus {id}"
        )
    };
    tree.child_focus(gtk::DirectionType::TabForward);
    focused(&tree, "tree.tsx");
    assert!(
        !tree.child_focus(gtk::DirectionType::TabForward),
        "tree is one Tab stop"
    );
    key(&tree, Key::Left, M::empty());
    focused(&tree, "components");
    key(&tree, Key::Left, M::empty());
    settle();
    assert!(!tree.visible_ids().contains(&"tree.tsx".into()));
    key(&tree, Key::Right, M::empty());
    settle();
    focused(&tree, "components");
    key(&tree, Key::Right, M::empty());
    focused(&tree, "button.tsx");
    key(&tree, Key::Return, M::empty());
    settle();
    assert_eq!(tree.selected().as_deref(), Some("button.tsx"));
    let original = tree.row("tree.tsx").unwrap();
    tree.model().row(1).unwrap().set_expanded(false);
    settle();
    assert_eq!(
        tree.selected().as_deref(),
        Some("button.tsx"),
        "twisty never selects"
    );
    tree.model().row(1).unwrap().set_expanded(true);
    settle();
    assert_eq!(tree.row("tree.tsx").unwrap(), original);
    tree.rows()
        .emit_by_name::<()>("row-activated", &[&tree.row("components").unwrap()]);
    settle();
    assert_eq!(tree.selected().as_deref(), Some("components"));
    assert!(!tree.expanded().contains(&"components".into()));
    tree.focus_id("src");
    key(&tree, Key::r, M::empty());
    focused(&tree, "README.md");
    key(&tree, Key::e, M::empty());
    focused(&tree, "README.md");
    key(&tree, Key::p, M::CONTROL_MASK);
    focused(&tree, "README.md");
    // Prefixes persist for the shared 600ms window and restart afterwards.
    for _ in 0..7 {
        settle();
    }
    key(&tree, Key::p, M::empty());
    focused(&tree, "package.json");
    tree.set_expanded(&["src".into(), "components".into()]);
    settle();
    tree.focus_id("tree.tsx");
    tree.set_expanded(&[]);
    settle();
    gtk::prelude::GtkWindowExt::set_focus(window, None::<&gtk::Widget>);
    tree.child_focus(gtk::DirectionType::TabBackward);
    focused(&tree, "src");
    assert!(tree.row("src").unwrap().is_focusable());
    assert!(!tree.row("tree.tsx").unwrap().is_focusable());
    tree.set_direction(gtk::TextDirection::Rtl);
    settle();
    assert_eq!(
        tree.expander("src").unwrap().direction(),
        gtk::TextDirection::Rtl
    );
    key(&tree, Key::Right, M::empty());
    settle();
    assert!(
        tree.expanded().contains(&"src".into()),
        "source uses physical keys also in RTL"
    );
    let source = &stories::story_of("tree").unwrap().examples[1];
    let root = scene::render_scene(source, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let tree = find(&root, "dz-tree-content").downcast::<Tree>().unwrap();
    tree.focus_id("b");
    key(&tree, Key::Down, M::empty());
    focused(&tree, "b");
    key(&tree, Key::End, M::empty());
    focused(&tree, "c");
    key(&tree, Key::Return, M::empty());
    assert_eq!(tree.selected().as_deref(), Some("tree.tsx"));
    tree.rows()
        .emit_by_name::<()>("row-activated", &[&tree.row("c").unwrap()]);
    assert_eq!(tree.selected().as_deref(), Some("tree.tsx"));
    // Empty branches remain expandable; refused controlled changes leave the model intact.
    let input = gtk::Entry::new();
    input.set_text("retained");
    let mut branch = TreeNode::new("empty", &input, "Empty");
    branch.children = Some(vec![]);
    let tree = tree_content(
        vec![branch],
        TreeOptions {
            expanded: Some(vec![]),
            ..Default::default()
        },
    )
    .unwrap();
    let requests = Rc::new(RefCell::new(vec![]));
    let seen = requests.clone();
    tree.connect_expanded_changed(move |_, ids| seen.borrow_mut().push(ids.to_vec()));
    let selected = Rc::new(RefCell::new(vec![]));
    let seen = selected.clone();
    tree.connect_selected_changed(move |_, id| seen.borrow_mut().push(id.to_owned()));
    window.set_child(Some(&tree));
    settle();
    tree.focus_id("empty");
    assert!(tree.model().row(0).unwrap().is_expandable());
    key(&tree, Key::Right, M::empty());
    settle();
    assert_eq!(*requests.borrow(), vec![vec!["empty".to_owned()]]);
    assert!(!tree.model().row(0).unwrap().is_expanded());
    tree.model().row(0).unwrap().set_expanded(true);
    settle();
    assert!(!tree.model().row(0).unwrap().is_expanded());
    tree.set_expanded(&["empty".into()]);
    settle();
    assert!(tree.model().row(0).unwrap().is_expanded());
    key(&tree, Key::Return, M::empty());
    settle();
    assert_eq!(*selected.borrow(), ["empty"]);
    assert_eq!(tree.selected(), None);
    assert_eq!(input.text(), "retained");
    let mut branch = TreeNode::new("branch", &gtk::Label::new(Some("Branch")), "Branch");
    branch.children = Some(vec![TreeNode::new(
        "child",
        &gtk::Label::new(Some("Child")),
        "Child",
    )]);
    let controlled = tree_content(
        vec![branch],
        TreeOptions {
            expanded: Some(vec![]),
            ..Default::default()
        },
    )
    .unwrap();
    window.set_child(Some(&controlled));
    settle();
    controlled.model().row(0).unwrap().set_expanded(true);
    settle();
    assert_eq!(controlled.visible_ids(), ["branch"]);
    controlled.connect_expanded_changed(|tree, ids| tree.set_expanded(ids));
    controlled.model().row(0).unwrap().set_expanded(true);
    settle();
    assert_eq!(controlled.visible_ids(), ["branch", "child"]);
    controlled.set_item_disabled("branch", true);
    controlled.model().row(0).unwrap().set_expanded(false);
    settle();
    assert_eq!(
        controlled.visible_ids(),
        ["branch", "child"],
        "disabled twisty leaves model unchanged"
    );
    controlled.focus_id("branch");
    key(&controlled, Key::space, M::empty());
    settle();
    assert_eq!(
        controlled.visible_ids(),
        ["branch"],
        "keyboard activation follows source disabled-branch semantics"
    );
    controlled.set_expanded(&["branch".into(), "branch".into(), "unknown".into()]);
    assert_eq!(controlled.expanded(), ["branch", "unknown"]);
    // Duplicate ancestor ids fail cleanly without partially built object panics.
    let mut duplicate = TreeNode::new("same", &gtk::Label::new(Some("Same")), "Same");
    duplicate.children = Some(vec![TreeNode::new(
        "same",
        &gtk::Label::new(Some("Again")),
        "Again",
    )]);
    assert!(tree_content(vec![duplicate], TreeOptions::default()).is_err());
    let weak = tree.downgrade();
    window.set_child(None::<&gtk::Widget>);
    drop(tree);
    settle();
    assert!(
        weak.upgrade().is_none(),
        "tree bindings must not retain the widget"
    );
}
fn splitter_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    let source = &stories::story_of("splitter").unwrap().examples[0];
    let mut specimens = vec![];
    let root = scene::render_scene(source, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let view = find(&root, "dz-split-view")
        .downcast::<SplitView>()
        .unwrap();
    let assert_sizes = |expected: &[f64]| {
        for (actual, expected) in view.sizes().iter().zip(expected) {
            assert!(
                (actual - expected).abs() < 1e-9,
                "splitter shares {:?}, expected {:?}",
                view.sizes(),
                expected
            );
        }
    };
    assert_sizes(&[24., 52., 24.]);
    let first = view.paned(0).unwrap();
    let second = view.paned(1).unwrap();
    let first_handle = view.handle(0).unwrap();
    let second_handle = view.handle(1).unwrap();
    assert_eq!(
        first_handle.accessible_role(),
        gtk::AccessibleRole::Separator
    );
    first_handle.grab_focus();
    assert!(key(&first, Key::Right, M::empty()));
    settle();
    assert_sizes(&[25., 51., 24.]);
    key(&first, Key::Right, M::SHIFT_MASK);
    settle();
    assert_sizes(&[35., 41., 24.]);
    key(&first, Key::End, M::empty());
    settle();
    assert_sizes(&[40., 36., 24.]);
    key(&first, Key::Home, M::empty());
    settle();
    assert_sizes(&[15., 61., 24.]);
    second_handle.grab_focus();
    key(&second, Key::Home, M::empty());
    settle();
    assert_sizes(&[15., 45., 40.]);
    key(&second, Key::End, M::empty());
    settle();
    assert_sizes(&[15., 70., 15.]);
    let controllers = first.observe_controllers();
    let click = (0..controllers.n_items())
        .find_map(|i| controllers.item(i).and_downcast::<gtk::GestureClick>())
        .unwrap();
    assert!(
        click
            .group()
            .iter()
            .any(|gesture| gesture.is::<gtk::GestureDrag>()),
        "native drag capture must not cancel the double-click gesture"
    );
    let area = first_handle.compute_bounds(&first).unwrap();
    click.emit_by_name::<()>(
        "pressed",
        &[
            &2i32,
            &f64::from(area.x() + area.width() / 2.),
            &f64::from(area.y() + area.height() / 2.),
        ],
    );
    settle();
    assert_sizes(&[40., 45., 15.]);
    let gutter = Theme::of(&view).unwrap().dimension("spacing-2").unwrap();
    let run = f64::from(view.width()) - 2. * gutter;
    first.set_position((run * 0.35).round() as i32);
    settle();
    assert!((view.sizes()[0] - 35.).abs() < 100. / run);
    assert!((view.sizes()[0] + view.sizes()[1] - 85.).abs() < 1e-9);
    assert!(
        (view.sizes()[2] - 15.).abs() < 1e-9,
        "native dragging affects only the adjacent pair"
    );

    let input = text_field("", Size::Medium);
    input.set_text("Preserve this edit");
    let left = splitter_panel(&input);
    let middle = splitter_panel(&label("Middle", "body-medium"));
    let right = splitter_panel(&label("Right", "body-medium"));
    let mut a = SplitPane::new(&left);
    a.default_size = 1.;
    let mut b = SplitPane::new(&middle);
    b.default_size = 2.;
    let c = SplitPane::new(&right);
    let view = split_view(gtk::Orientation::Horizontal, vec![a, b, c]).unwrap();
    let changes = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
    let values = changes.clone();
    view.connect_sizes_changed(move |_, sizes| values.borrow_mut().push(sizes.to_vec()));
    window.set_child(Some(&view));
    window.set_default_size(800, 300);
    settle();
    assert_eq!(view.sizes(), [25., 50., 25.]);
    input.select_region(1, 4);
    input.grab_focus();
    assert!(
        !key(&view.paned(0).unwrap(), Key::Left, M::empty()),
        "seam keys do not consume field arrows"
    );
    assert!(view.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(gtk::prelude::GtkWindowExt::focus(window), view.handle(0));
    assert!(view.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(gtk::prelude::GtkWindowExt::focus(window), view.handle(1));
    assert!(view.child_focus(gtk::DirectionType::TabBackward));
    assert_eq!(gtk::prelude::GtkWindowExt::focus(window), view.handle(0));
    input.select_region(1, 4);
    view.resize(0, 35.);
    settle();
    let before = view.sizes();
    let count = changes.borrow().len();
    let width = view.width();
    window.set_default_size(1000, 300);
    settle();
    assert!(view.width() > width);
    assert_eq!(view.sizes(), before, "window resize preserves percentages");
    assert_eq!(
        changes.borrow().len(),
        count,
        "native allocation is not reported as a user drag"
    );
    assert_eq!(input.text(), "Preserve this edit");
    assert_eq!(input.selection_bounds(), Some((1, 4)));
    view.set_direction(gtk::TextDirection::Rtl);
    settle();
    assert!(left.compute_bounds(&view).unwrap().x() > right.compute_bounds(&view).unwrap().x());
    assert_eq!(view.sizes(), before);
    view.set_direction(gtk::TextDirection::Ltr);
    view.set_orientation(gtk::Orientation::Vertical);
    settle();
    assert_eq!(
        view.paned(0).unwrap().orientation(),
        gtk::Orientation::Vertical
    );
    assert!(left.compute_bounds(&view).unwrap().y() < right.compute_bounds(&view).unwrap().y());
    view.handle(0).unwrap().grab_focus();
    key(&view.paned(0).unwrap(), Key::Home, M::empty());
    settle();
    assert_eq!(
        view.sizes()[0],
        0.,
        "a zero floor may collapse the native panel"
    );
    assert!(
        view.handle(0).unwrap().is_mapped(),
        "the seam remains available to reopen a collapsed panel"
    );
    key(&view.paned(0).unwrap(), Key::Down, M::empty());
    settle();
    assert_eq!(view.sizes()[0], 1.);
    assert_eq!(input.text(), "Preserve this edit");
}
fn option_card_interactions(window: &gtk::Window) {
    let title = window.title();
    let original = (window.width(), window.height());
    window.set_title(Some("DazzUI isolated option card checkpoint"));
    window.set_default_size(800, 400);
    let root = box_(gtk::Orientation::Horizontal, "3");
    let first = option_card("First", "A short explanation.", None);
    let second = option_card(
        "Second",
        "A native explanation that wraps when this window becomes narrower.",
        Some(&first),
    );
    for card in [&first, &second] {
        card.set_valign(gtk::Align::Center);
        root.append(card);
    }
    first.set_active(true);
    window.set_child(Some(&root));
    settle();
    let wide = second.compute_bounds(window).unwrap().height();
    assert!(second.grab_focus());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(second.upcast_ref())
    );
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        let status = std::process::Command::new("python3")
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_key.py"))
            .args(["DazzUI isolated option card checkpoint", "space"])
            .status()
            .unwrap();
        assert!(status.success());
        settle();
        assert!(
            second.is_active() && !first.is_active(),
            "Space keeps native radio activation"
        );
        let click = |card: &gtk::CheckButton| {
            let r = card.compute_bounds(window).unwrap();
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_click.py"))
                .args([
                    "DazzUI isolated option card checkpoint",
                    &(r.x() + r.width() / 2.).round().to_string(),
                    &(r.y() + r.height() / 2.).round().to_string(),
                ])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
        };
        click(&first);
        assert!(
            first.is_active() && !second.is_active(),
            "the fractional border box remains clickable"
        );
        second.set_sensitive(false);
        click(&second);
        assert!(
            first.is_active() && !second.is_active(),
            "disabled cards keep native input exclusion"
        );
        second.set_sensitive(true);
        println!("X11 option card pointer, Space, grouping and disabled state passed");
    }
    window.set_default_size(400, 400);
    settle();
    let narrow = second.compute_bounds(window).unwrap().height();
    assert!(
        narrow > wide,
        "the fractional box follows native paragraph wrapping"
    );
    window.set_default_size(800, 400);
    settle();
    assert!(
        (second.compute_bounds(window).unwrap().height() - wide).abs() < 0.01,
        "widening restores the exact painted height"
    );
    window.set_title(title.as_deref());
    window.set_default_size(original.0, original.1);
}
fn table_interactions(window: &gtk::Window) {
    let source: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"table", "title":"Rich table state", "gaps":[], "state":{"selected":0,"interactive":true},
        "content":{"$el":"Col","props":{"width":"24rem"},"children":[
            {"$el":"Table","$at":0,"props":{"tint":"info","aria-label":"Services"},"children":[
                {"$el":"TableHead","$at":1,"children":[{"$el":"TableCell","$at":2,"props":{"head":true},"children":["Service"]},{"$el":"TableCell","props":{"head":true,"align":"end"},"children":["Status"]}]},
                {"$el":"TableRow","$at":3,"props":{"interactive":true,"active":{"$eq":[{"$state":"selected"},0]},"onClick":{"$do":[{"set":"selected","to":0}]}},"children":[{"$el":"TableCell","children":["web"]},{"$el":"TableCell","props":{"align":"end"},"children":[{"$el":"Badge","$at":4,"children":["Live"]}]}]},
                {"$el":"TableHead","children":[{"$el":"TableCell","props":{"head":true},"children":["More services"]}]},
                {"$el":"TableRow","$at":5,"props":{"interactive":{"$state":"interactive"},"active":{"$eq":[{"$state":"selected"},1]},"onClick":{"$do":[{"set":"selected","to":1}]}},"children":[{"$el":"TableCell","children":["api"]},{"$el":"TableCell","children":[{"$el":"TextField","$at":6,"props":{"defaultValue":"Keep this edit"}}]}]}
            ]},
            {"$el":"Text","children":[{"$concat":["Selected ",{"$state":"selected"}]}]},
            {"$el":"Button","$at":7,"props":{"onClick":{"$do":[{"set":"interactive","to":false}]}},"children":["Lock row"]}
        ]}
    })).unwrap();
    let mut specimens = vec![];
    let root = scene::render_scene(&source, &mut specimens).unwrap();
    window.set_child(Some(&root));
    settle();
    let at = |index| {
        specimens
            .iter()
            .find(|(i, _)| *i == index)
            .unwrap()
            .1
            .clone()
    };
    let table = at(0).downcast::<Table>().unwrap();
    let first = at(3).downcast::<gtk::ListBoxRow>().unwrap();
    let second = at(5).downcast::<gtk::ListBoxRow>().unwrap();
    let input = at(6).downcast::<gtk::Entry>().unwrap();
    assert_eq!(table.accessible_role(), gtk::AccessibleRole::Table);
    assert_eq!(at(1).accessible_role(), gtk::AccessibleRole::Row);
    assert_eq!(at(2).accessible_role(), gtk::AccessibleRole::ColumnHeader);
    assert!(
        !table.rows().row_at_index(0).unwrap().is_focusable(),
        "headings are not row tab stops"
    );
    assert!(first.is_focusable() && second.is_focusable());
    assert!(
        !first.is_selectable() && !second.is_selectable(),
        "host state owns selection"
    );
    assert!(first.has_css_class("dz-active") && !second.has_css_class("dz-active"));
    if std::env::var_os("DAZZUI_X11_INPUT").is_some() {
        let title = window.title();
        window.set_title(Some("DazzUI isolated table checkpoint"));
        settle();
        for row in [&second, &first] {
            let bounds = row.compute_bounds(window).unwrap();
            let status = std::process::Command::new("python3")
                .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tool/x11_click.py"))
                .args([
                    "DazzUI isolated table checkpoint",
                    &(bounds.x() + 5.).round().to_string(),
                    &(bounds.y() + bounds.height() / 2.).round().to_string(),
                ])
                .status()
                .unwrap();
            assert!(status.success());
            settle();
            assert!(
                row.has_css_class("dz-active"),
                "native hit testing follows the fractional row position"
            );
        }
        window.set_title(title.as_deref());
        println!("X11 table row activation across fractional positions passed");
    }
    input.set_text("Edited");
    input.select_region(1, 4);
    second.emit_by_name::<()>("activate", &[]);
    assert!(
        !first.has_css_class("dz-active") && second.has_css_class("dz-active"),
        "native row activation updates shared state"
    );
    assert_eq!(input.text(), "Edited");
    assert_eq!(input.selection_bounds(), Some((1, 4)));
    let value = table
        .next_sibling()
        .unwrap()
        .first_child()
        .and_downcast::<TextBox>()
        .unwrap()
        .label();
    assert_eq!(value.text(), "Selected 1");
    first.grab_focus();
    assert!(table.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(second.upcast_ref()),
        "Tab visits each interactive row and skips interleaved headings"
    );
    table.rows().emit_by_name::<()>("row-activated", &[&first]);
    assert_eq!(
        value.text(),
        "Selected 0",
        "the native pointer signal uses the same callback"
    );
    at(7).downcast::<gtk::Button>().unwrap().emit_clicked();
    assert!(!second.is_focusable() && !second.is_activatable());
    table.rows().emit_by_name::<()>("row-activated", &[&second]);
    assert_eq!(
        value.text(),
        "Selected 0",
        "a non-interactive row does not dispatch an action"
    );
    assert_eq!(
        at(6),
        input.upcast::<gtk::Widget>(),
        "row state preserves rich native cells"
    );
    table.set_direction(gtk::TextDirection::Rtl);
    settle();
    let cells = first.child().unwrap();
    assert!(
        cells
            .first_child()
            .unwrap()
            .compute_bounds(&cells)
            .unwrap()
            .x()
            > cells
                .last_child()
                .unwrap()
                .compute_bounds(&cells)
                .unwrap()
                .x()
    );
    assert_eq!(
        cells.last_child().unwrap().first_child().unwrap().halign(),
        gtk::Align::Start,
        "the end column stays right-aligned in RTL, as React's text-align:right does"
    );
}
fn menu_interactions(window: &gtk::Window) {
    use gtk::gdk::{Key, ModifierType as M};
    let source: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"menus", "title":"Menu state", "gaps":[], "state":{"current":0,"count":0,"locked":true},
        "content":{"$el":"Col","children":[
            {"$el":"Menu","props":{"trigger":{"$el":"Button","children":["Choose"]},"items":[
                {"label":"Alpha","checked":{"$eq":[{"$state":"current"},0]},"onSelect":{"$do":[{"set":"count","to":1}]}},
                {"label":"Alpine","checked":{"$eq":[{"$state":"current"},1]},"onSelect":{"$do":[{"set":"current","to":1},{"set":"count","to":2}]}},
                {"label":"Beta","disabled":{"$state":"locked"},"onSelect":{"$do":[{"set":"count","to":3}]}},
                {"label":"Delta","shortcut":"D"}
            ]}},
            {"$el":"Text","children":[{"$concat":["Selected ",{"$state":"current"}," / ",{"$state":"count"}]}]},
            {"$el":"Button","props":{"onClick":{"$do":[{"set":"locked","to":false}]}},"children":["Unlock"]}
        ]}
    })).unwrap();
    let root = scene::render_scene(&source, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let trigger = root.first_child().and_downcast::<gtk::Button>().unwrap();
    let popup = find(&trigger, "dz-menu")
        .downcast::<gtk::PopoverMenu>()
        .unwrap();
    let rows = menu_rows(&popup);
    let output = trigger
        .next_sibling()
        .unwrap()
        .first_child()
        .and_downcast::<TextBox>()
        .unwrap()
        .label();
    assert!(!rows[2].is_sensitive());
    assert!(key(&trigger, Key::Down, M::empty()));
    settle();
    assert!(popup.is_visible());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[0])
    );
    assert!(rows[0].property::<bool>("active") && !rows[1].property::<bool>("active"));
    assert!(
        rows[0].first_child().unwrap().is_visible(),
        "native check column survives delayed action binding"
    );
    key(&popup, Key::a, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[1])
    );
    key(&popup, Key::a, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[0])
    );
    key(&popup, Key::End, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[3])
    );
    key(&popup, Key::b, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[3]),
        "typeahead skips disabled items"
    );
    assert!(
        !key(&popup, Key::d, M::CONTROL_MASK),
        "hint text installs no accelerator"
    );
    rows[0].emit_by_name::<()>("clicked", &[]);
    settle();
    assert!(!popup.is_visible());
    assert_eq!(
        output.text(),
        "Selected 0 / 0",
        "an already selected radio item does not fire onValueChange"
    );
    trigger.emit_clicked();
    settle();
    rows[1].emit_by_name::<()>("clicked", &[]);
    settle();
    assert!(!popup.is_visible());
    assert_eq!(output.text(), "Selected 1 / 2");
    assert!(!rows[0].property::<bool>("active") && rows[1].property::<bool>("active"));
    root.last_child()
        .and_downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    assert!(rows[2].is_sensitive());
    key(&trigger, Key::Up, M::empty());
    settle();
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[3])
    );
    key(&popup, Key::b, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[2])
    );
    key(&popup, Key::Home, M::empty());
    key(&popup, Key::d, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[3])
    );
    std::thread::sleep(
        Theme::default()
            .duration("menu-typeahead-reset-duration")
            .unwrap()
            + std::time::Duration::from_millis(30),
    );
    key(&popup, Key::a, M::empty());
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(window).as_ref(),
        Some(&rows[0]),
        "a new search starts after the React search-session timeout"
    );
    popup.popdown();
    settle();

    let value = &stories::story_of("menu").unwrap().examples[1];
    let root = scene::render_scene(value, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let popup = find(&root, "dz-menu")
        .downcast::<gtk::PopoverMenu>()
        .unwrap();
    assert!(key(&root, Key::F10, M::SHIFT_MASK));
    settle();
    assert!(popup.is_visible());
    popup.popdown();
    settle();
    let controllers = root.observe_controllers();
    let right_click = (0..controllers.n_items())
        .find_map(|i| {
            controllers
                .item(i)
                .and_downcast::<gtk::GestureClick>()
                .filter(|g| g.button() == 3)
        })
        .unwrap();
    right_click.emit_by_name::<()>("pressed", &[&1i32, &24f64, &12f64]);
    settle();
    assert!(popup.is_visible());
    let (positioned, point) = popup.pointing_to();
    assert!(positioned);
    assert_eq!((point.x(), point.y()), (24, 12));
    popup.popdown();
    settle();

    let value = &stories::story_of("menu").unwrap().examples[2];
    let root = scene::render_scene(value, &mut vec![]).unwrap();
    assert!(
        root.is::<gtk::PopoverMenuBar>(),
        "one native menubar coordinates all menus"
    );
    window.set_child(Some(&root));
    settle();
    let rows = menu_rows(&root);
    assert_eq!(rows.len(), 7);
    let first = root.first_child().unwrap();
    let last = root.last_child().unwrap();
    assert!(first.compute_bounds(&root).unwrap().x() < last.compute_bounds(&root).unwrap().x());
    root.set_direction(gtk::TextDirection::Rtl);
    settle();
    assert!(
        first.compute_bounds(&root).unwrap().x() > last.compute_bounds(&root).unwrap().x(),
        "menubar geometry follows native text direction"
    );
    root.set_direction(gtk::TextDirection::Ltr);
    settle();
    assert!(rows[3].property::<bool>("active"));
    assert!(
        !rows[4].property::<bool>("active"),
        "React selects only the first checked radio item"
    );
    let popup = find(&root, "dz-menu")
        .downcast::<gtk::PopoverMenu>()
        .unwrap();
    popup.popup();
    settle();
    let row = rows[0].clone();
    window.set_child(None::<&gtk::Widget>);
    drop(rows);
    drop(root);
    drop(popup);
    drop(row);
    settle();
}
fn main() {
    gtk::init().expect("GTK display required");
    gtk::Settings::default()
        .unwrap()
        .set_gtk_enable_animations(false);
    let window = gtk::Window::builder()
        .default_width(800)
        .default_height(600)
        .focus_visible(false)
        .build();
    Theme::new("studio-light").unwrap().apply(&window);
    window.present();
    settle();
    toast_interactions(&window);
    if std::env::var_os("DAZZUI_TOAST_ONLY").is_some() {
        window.destroy();
        return;
    }
    drawer_interactions(&window);
    if std::env::var_os("DAZZUI_DRAWER_ONLY").is_some() {
        window.destroy();
        return;
    }
    button_group_interactions(&window);
    date_range_picker_interactions(&window);
    calendar_interactions(&window);
    sidebar_interactions(&window);
    form_interactions(&window);
    combobox_interactions(&window);
    menu_interactions(&window);
    table_interactions(&window);
    option_card_interactions(&window);
    splitter_interactions(&window);
    tree_interactions(&window);
    shortcut_interactions(&window);
    otp_interactions(&window);
    slider_interactions(&window);
    // Native collection coordination: one disclosure at a time, independent
    // selections in a multi-toggle group, and an empty single-toggle group.
    let value = &stories::story_of("accordion").unwrap().examples[0];
    let mut specimens = vec![];
    let root = scene::render_scene(value, &mut specimens).unwrap();
    window.set_child(Some(&root));
    window.set_default_size(800, 600);
    settle();
    let items: Vec<_> = specimens
        .iter()
        .filter_map(|(_, w)| w.clone().downcast::<gtk::Expander>().ok())
        .collect();
    assert!(items[0].is_expanded() && !items[1].is_expanded());
    items[1].set_expanded(true);
    assert!(!items[0].is_expanded() && items[1].is_expanded());
    let value = &stories::story_of("accordion").unwrap().examples[1];
    let mut specimens = vec![];
    let root = scene::render_scene(value, &mut specimens).unwrap();
    let items: Vec<_> = specimens
        .iter()
        .filter_map(|(_, w)| w.clone().downcast::<gtk::Expander>().ok())
        .collect();
    assert!(items[0].is_expanded() && items[1].is_expanded());
    window.set_child(Some(&root));
    settle();
    items[2].set_expanded(true);
    assert!(items.iter().all(|w| w.is_expanded()));
    for index in [1, 2] {
        let value = &stories::story_of("toggle").unwrap().examples[index];
        let root = scene::render_scene(value, &mut vec![]).unwrap();
        window.set_child(Some(&root));
        settle();
        let one = root
            .first_child()
            .unwrap()
            .downcast::<gtk::ToggleButton>()
            .unwrap();
        let two = one
            .next_sibling()
            .unwrap()
            .downcast::<gtk::ToggleButton>()
            .unwrap();
        assert!(one.is_active() && !two.is_active());
        two.set_active(true);
        assert!(two.is_active());
        assert_eq!(
            one.is_active(),
            index == 1,
            "single vs multiple toggle group"
        );
        if index == 2 {
            two.set_active(false);
            assert!(
                !one.is_active() && !two.is_active(),
                "a toggle group may be empty"
            );
        }
        let mut children = root.first_child();
        let mut stops = 0;
        while let Some(child) = children {
            children = child.next_sibling();
            if child.is_focusable() {
                stops += 1;
            }
        }
        assert_eq!(stops, 1, "a toggle group has one tab stop");
        if index == 1 {
            one.set_focusable(true);
            one.grab_focus();
            let controllers = root.observe_controllers();
            let keys = (0..controllers.n_items())
                .find_map(|i| {
                    controllers
                        .item(i)
                        .and_downcast::<gtk::EventControllerKey>()
                })
                .unwrap();
            let handled = keys.emit_by_name::<bool>(
                "key-pressed",
                &[
                    &gtk::gdk::Key::Right,
                    &0u32,
                    &gtk::gdk::ModifierType::empty(),
                ],
            );
            assert!(
                handled
                    && gtk::prelude::GtkWindowExt::focus(&window).as_ref()
                        == Some(two.upcast_ref()),
                "arrow navigation moves native focus inside the group"
            );
            two.set_visible(false);
            settle();
            assert!(one.is_focusable(), "a hidden tab stop moves to its sibling");
            two.set_visible(true);
            settle();
            assert_ne!(
                one.is_focusable(),
                two.is_focusable(),
                "showing a former tab stop does not add a second one"
            );
        }
    }

    let number: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"number", "title":"Numeric state", "gaps":[], "state":{"n":12},
        "content":{"$el":"Col","children":[
            {"$el":"NumberField","$at":0,"props":{"value":{"$state":"n"},"onValueChange":{"$do":[{"set":"n","to":{"$arg":0}}]},"min":0,"max":20,"step":2,"unit":"px"}},
            {"$el":"Text","children":[{"$concat":["Current: ",{"$state":"n"}]}]},
            {"$el":"Button","props":{"onClick":{"$do":[{"set":"n","to":4}]}},"children":["Reset"]}
        ]}
    })).unwrap();
    let root = scene::render_scene(&number, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    let field = root.first_child().unwrap();
    let input = field
        .first_child()
        .and_downcast::<gtk::SpinButton>()
        .unwrap();
    let stepper = field
        .last_child()
        .and_downcast::<gtk::SpinButton>()
        .unwrap();
    let output = field
        .next_sibling()
        .unwrap()
        .first_child()
        .and_downcast::<TextBox>()
        .unwrap()
        .label();
    assert_eq!(input.value(), 12.);
    stepper.spin(gtk::SpinType::StepForward, 0.);
    assert_eq!(output.text(), "Current: 14");
    input.set_text("18");
    input.update();
    assert_eq!(output.text(), "Current: 18");
    root.last_child()
        .and_downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    assert_eq!((input.value(), stepper.value()), (4., 4.));
    assert_eq!(output.text(), "Current: 4");
    assert_eq!(
        field.first_child(),
        Some(input.upcast()),
        "numeric state preserves the native input"
    );

    let inline = inline_code("pnpm install");
    let column = box_(gtk::Orientation::Vertical, "0");
    column.append(&inline);
    window.set_child(Some(&column));
    settle();
    let initial_height = inline.compute_bounds(&column).unwrap().height();
    let larger_code = gtk::CssProvider::new();
    larger_code.load_from_string(".dz-code.font-scale-fixture { font-size: 22px; }");
    gtk::style_context_add_provider_for_display(
        &gtk::prelude::WidgetExt::display(&window),
        &larger_code,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
    );
    inline.add_css_class("font-scale-fixture");
    settle();
    assert!(
        inline.compute_bounds(&column).unwrap().height() > initial_height,
        "normal code leading follows the actual font metrics"
    );
    inline.remove_css_class("font-scale-fixture");
    settle();
    assert_eq!(
        inline.compute_bounds(&column).unwrap().height(),
        initial_height
    );
    assert!(inline.label().is_selectable());
    gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
    assert!(window.child_focus(gtk::DirectionType::TabForward));
    assert_eq!(
        gtk::prelude::GtkWindowExt::focus(&window),
        Some(inline.label().upcast()),
        "inline code retains native keyboard selection"
    );

    let listing = code_block(
        "one\n two\n\n",
        None,
        None,
        None,
        CodeBlockOptions {
            line_numbers: true,
            copyable: true,
            ..Default::default()
        },
    );
    window.set_child(Some(&listing));
    settle();
    let text = find(&listing, "dz-code-pre")
        .downcast::<gtk::Label>()
        .unwrap();
    let gutter = find(&listing, "dz-code-gutter")
        .downcast::<gtk::Label>()
        .unwrap();
    assert_eq!(text.text(), "one\n two");
    assert_eq!(gutter.text(), "1\n2");
    assert!(text.is_selectable() && !gutter.is_selectable());
    let source = code_block_source(&listing);
    source.set_label("updated\nsource\nthird\n");
    assert_eq!(source.text(), "updated\nsource\nthird");
    assert_eq!(gutter.text(), "1\n2\n3");
    assert_eq!(
        source, text,
        "source updates retain the selectable native label"
    );
    // A private Xvfb display starts empty. Otherwise retain a local provider
    // exactly; an external clipboard is left untouched by the test suite.
    let clipboard = window.clipboard();
    let previous = clipboard.content();
    let empty_linux = ["GdkX11Display", "GdkWaylandDisplay"]
        .contains(&gtk::prelude::WidgetExt::display(&window).type_().name())
        && clipboard.formats().to_str().is_empty();
    if previous.is_some() || empty_linux {
        struct Restore(gtk::gdk::Clipboard, Option<gtk::gdk::ContentProvider>);
        impl Drop for Restore {
            fn drop(&mut self) {
                let _ = self.0.set_content(self.1.as_ref());
            }
        }
        let _restore = Restore(clipboard.clone(), previous);
        let copy = find(&listing, "dz-code-copy")
            .downcast::<gtk::Button>()
            .unwrap();
        copy.emit_clicked();
        let actual = gtk::glib::MainContext::default()
            .block_on(clipboard.read_text_future())
            .unwrap();
        assert!(
            actual.as_deref() == Some("updated\nsource\nthird"),
            "copy excludes the gutter and trailing newlines"
        );
        assert_eq!(copy.tooltip_text().as_deref(), Some("Copied"));
        for _ in 0..5 {
            settle();
        }
        copy.emit_clicked();
        for _ in 0..12 {
            settle();
        }
        assert_eq!(
            copy.tooltip_text().as_deref(),
            Some("Copy"),
            "a repeated copy does not extend the initial feedback lifetime"
        );
        copy.emit_clicked();
        window.set_child(None::<&gtk::Widget>);
        assert_eq!(copy.tooltip_text().as_deref(), Some("Copy"));
        for _ in 0..17 {
            settle();
        }
        assert_eq!(
            copy.tooltip_text().as_deref(),
            Some("Copy"),
            "unmapping cancels feedback timers"
        );
        let otp = otp_field(OtpOptions {
            length: 6,
            ..Default::default()
        })
        .unwrap();
        window.set_child(Some(&otp));
        settle();
        otp.focus_cell(0);
        clipboard.set_text(" 12x 3456 ");
        let first = otp.cells()[0]
            .delegate()
            .unwrap()
            .dynamic_cast::<gtk::Text>()
            .unwrap();
        first.emit_by_name::<()>("paste-clipboard", &[]);
        settle();
        assert_eq!(
            otp.value(),
            "123456",
            "native clipboard paste distributes normalized text"
        );
        let last = otp.cells()[5]
            .delegate()
            .unwrap()
            .dynamic_cast::<gtk::Text>()
            .unwrap();
        last.emit_by_name::<()>("cut-clipboard", &[]);
        settle();
        assert_eq!(otp.value(), "12345");
        assert_eq!(
            gtk::glib::MainContext::default()
                .block_on(clipboard.read_text_future())
                .unwrap()
                .as_deref(),
            Some("6")
        );
        println!("Clipboard payload, OTP paste/cut and copy feedback lifecycle passed");
    } else {
        println!("Clipboard interaction deferred: run under an empty Xvfb clipboard");
    }
    let wrapped: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"wrapped", "title":"Wrapped listing", "gaps":[], "state":{"source":"a_very_long_identifier_".repeat(10)},
        "content":{"$el":"Col","props":{"width":"12rem"},"children":[
            {"$el":"CodeBlock","props":{"code":{"$state":"source"},"lineNumbers":true,"wrap":true}},
            {"$el":"Button","props":{"onClick":{"$do":[{"set":"source","to":"now\nupdated\n"}]}},"children":["Update"]}
        ]}
    })).unwrap();
    let root = scene::render_scene(&wrapped, &mut vec![]).unwrap();
    let (fixed, frame) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&fixed));
    settle();
    let text = find(&root, "dz-code-pre").downcast::<gtk::Label>().unwrap();
    assert!(
        text.layout().line_count() > 1,
        "long code wraps inside a narrow listing"
    );
    assert!(
        find(&root, "dz-code-block")
            .compute_bounds(&frame)
            .unwrap()
            .height()
            > text.layout().pixel_size().1 as f32,
        "the listing grows rather than clipping its wrapped lines"
    );
    find(&root, "dz-button")
        .downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    assert_eq!(text.text(), "now\nupdated");
    assert_eq!(
        find(&root, "dz-code-gutter")
            .downcast::<gtk::Label>()
            .unwrap()
            .text(),
        "1\n2"
    );
    assert_eq!(
        find(&root, "dz-code-pre"),
        text.upcast::<gtk::Widget>(),
        "compiled source state preserves the native listing"
    );
    let opening: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"opening", "title":"Preference action", "gaps":[], "state":{"opened":false},
        "content":{"$el":"Col","children":[
            {"$el":"PreferenceRow","props":{"title":"Profile","onOpen":{"$do":[{"set":"opened","to":true}]}}},
            {"$el":"Text","children":[{"$concat":["Opened: ",{"$state":"opened"}]}]}
        ]}
    })).unwrap();
    let root = scene::render_scene(&opening, &mut vec![]).unwrap();
    window.set_child(Some(&root));
    settle();
    root.first_child()
        .and_downcast::<gtk::Button>()
        .unwrap()
        .emit_clicked();
    assert_eq!(
        root.last_child()
            .unwrap()
            .first_child()
            .and_downcast::<TextBox>()
            .unwrap()
            .text(),
        "Opened: true"
    );

    // Opening, closing and reopening use the same native controls, and the
    // modal restores the original parent tree after removing its scrim.
    let value = &stories::story_of("dialog").unwrap().preview;
    let mut specimens = vec![];
    let root = scene::render_scene(value, &mut specimens).unwrap();
    let (fixed, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&fixed));
    settle();
    let at = |index| {
        specimens
            .iter()
            .find(|(i, _)| *i == index)
            .unwrap()
            .1
            .clone()
    };
    let open = at(1).downcast::<gtk::Button>().unwrap();
    let input = at(4).downcast::<gtk::Entry>().unwrap();
    let close = at(7).downcast::<gtk::Button>().unwrap();
    assert!(!input.is_mapped());
    open.emit_clicked();
    settle();
    let modal = input.root().and_downcast::<gtk::Window>().unwrap();
    assert!(modal.is_modal() && modal.is_visible());
    assert_eq!(modal.transient_for(), Some(window.clone()));
    assert!(window.child().unwrap().is::<gtk::Overlay>());
    input.set_text("Edited name");
    close.emit_clicked();
    settle();
    assert!(!modal.is_visible());
    assert_eq!(window.child(), Some(fixed.clone().upcast()));
    open.emit_clicked();
    settle();
    assert_eq!(input.text(), "Edited name");
    close.emit_clicked();
    settle();

    let value = &stories::story_of("popover").unwrap().preview;
    let mut specimens = vec![];
    let root = scene::render_scene(value, &mut specimens).unwrap();
    let (fixed, _) = dazzui_gtk4_example::conformance::scene_frame(&root);
    window.set_child(Some(&fixed));
    settle();
    let open = specimens
        .iter()
        .find(|(i, _)| *i == 1)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::Button>()
        .unwrap();
    let input = specimens
        .iter()
        .find(|(i, _)| *i == 2)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::Entry>()
        .unwrap();
    let close = specimens
        .iter()
        .find(|(i, _)| *i == 4)
        .unwrap()
        .1
        .clone()
        .downcast::<gtk::Button>()
        .unwrap();
    open.emit_clicked();
    settle();
    assert!(input.is_mapped() && !input.is_editable());
    close.emit_clicked();
    settle();
    assert!(!input.is_mapped());
    // Focus-visible opens a hint immediately; activating dismisses it. A
    // cancelled hover must not reopen after its pending delay has elapsed.
    let value: dazzui_gtk4_example::support::story::Scene = serde_json::from_value(serde_json::json!({
        "id":"hint", "title":"Hint", "gaps":[],
        "content":{"$el":"Tooltip","props":{"label":"A hint","delay":600},"children":[{"$el":"Button","children":["Hint"]}]}
    })).unwrap();
    let hint = scene::render_scene(&value, &mut vec![]).unwrap();
    let (fixed, _) = dazzui_gtk4_example::conformance::scene_frame(&hint);
    window.set_focus_visible(false);
    window.set_child(Some(&fixed));
    settle();
    gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
    settle();
    let mut child = hint.first_child();
    let mut popup = None;
    while let Some(w) = child {
        child = w.next_sibling();
        if let Ok(p) = w.downcast::<gtk::Popover>() {
            popup = Some(p);
        }
    }
    let popup = popup.unwrap();
    assert!(!popup.is_visible());
    window.set_focus_visible(true);
    hint.grab_focus();
    settle();
    assert!(
        popup.is_visible(),
        "a keyboard hint does not wait for the hover timer"
    );
    hint.downcast_ref::<gtk::Button>().unwrap().emit_clicked();
    settle();
    assert!(!popup.is_visible(), "activation dismisses the hint");
    window.set_focus_visible(false);
    gtk::prelude::GtkWindowExt::set_focus(&window, None::<&gtk::Widget>);
    let controllers = hint.observe_controllers();
    let motion = (0..controllers.n_items())
        .find_map(|i| {
            controllers
                .item(i)
                .and_downcast::<gtk::EventControllerMotion>()
        })
        .unwrap();
    motion.emit_by_name::<()>("enter", &[&0f64, &0f64]);
    settle();
    assert!(!popup.is_visible(), "hover rest delay");
    motion.emit_by_name::<()>("leave", &[]);
    for _ in 0..7 {
        settle();
    }
    assert!(
        !popup.is_visible(),
        "leaving cancels pending hover presentation"
    );
    motion.emit_by_name::<()>("enter", &[&0f64, &0f64]);
    window.set_child(Some(&gtk::Box::new(gtk::Orientation::Vertical, 0)));
    for _ in 0..7 {
        settle();
    }
    assert!(
        !popup.is_visible(),
        "unmapping cancels pending hover presentation"
    );
    window.destroy();
    println!(
        "Form submission, reset and field validation, combobox filtering, suggestions and native popup focus, scalar/range slider geometry, keyboard and collision rules, OTP insertion, paste, focus and controlled completion, shortcut capture and cancellation, menus, tree navigation and controlled state, rich table actions and edits, splitter constraints, focus and resize, accordion exclusivity, toggles, modal ownership and popup lifecycle passed"
    );
}
