#![feature(arbitrary_self_types)]
mod support;
use dazzui_inset::*;
use inset_embedder::TextDirection;
use inset_widgets::{Center, Directionality, IntoWidget, SizedBox, Text};
use std::time::Duration;

#[test]
fn every_theme_renders_atoms() {
    for (_, vars) in THEMES {
        let widgets = vec![
            Button::new("Save").on_pressed(|_| {}).into_widget(),
            Badge::new("Ready").into_widget(),
            Card::new(Text::new("Card")).into_widget(),
            Checkbox::new(true)
                .label("Check")
                .on_changed(|_, _| {})
                .into_widget(),
            Radio::new(true)
                .label("Radio")
                .on_selected(|_| {})
                .into_widget(),
            Switch::new(true).on_changed(|_, _| {}).into_widget(),
        ];
        for widget in widgets {
            let cell = support::test_cell();
            support::build(
                &cell,
                Directionality::new(
                    TextDirection::Ltr,
                    Theme::new(vars, Center::new().child(widget)),
                )
                .into_widget(),
            );
            support::pump(&mut cell.borrow_mut(), Duration::from_millis(16));
        }
    }
}
#[test]
fn button_pointer_activates_once_and_disabled_never_activates() {
    use inset_embedder::{PointerChange, PointerData, PointerDataPacket, PointerDeviceKind};
    use inset_gestures::GestureBinding;
    use std::{cell::Cell, rc::Rc};
    for enabled in [false, true] {
        let clicks = Rc::new(Cell::new(0));
        let mut button = Button::new("Save");
        if enabled {
            let clicks = clicks.clone();
            button = button.on_pressed(move |_| clicks.set(clicks.get() + 1));
        }
        let cell = support::test_cell();
        support::build(
            &cell,
            Directionality::new(
                TextDirection::Ltr,
                Theme::new(
                    &STUDIO_LIGHT,
                    Center::new().child(SizedBox::new().width(100.0).child(button)),
                ),
            )
            .into_widget(),
        );
        for (i, change) in [PointerChange::Add, PointerChange::Down, PointerChange::Up]
            .into_iter()
            .enumerate()
        {
            let at = Duration::from_millis(20 + i as u64 * 20);
            let mut app = cell.borrow_mut();
            GestureBinding::instance(&mut app).handle_pointer_data_packet(
                &mut app,
                PointerDataPacket {
                    data: vec![PointerData {
                        change,
                        kind: PointerDeviceKind::Mouse,
                        physical_x: 400.0,
                        physical_y: 300.0,
                        time_stamp: at,
                        buttons: if change == PointerChange::Down { 1 } else { 0 },
                        ..Default::default()
                    }],
                },
            );
            support::pump(&mut app, at);
        }
        assert_eq!(clicks.get(), usize::from(enabled));
    }
}
#[test]
fn disabled_recipes_and_profiles_follow_tokens() {
    for (_, v) in THEMES {
        for size in ControlSize::ALL {
            assert!(v.metrics(size).size >= v.metrics(size).content.size);
        }
        for tint in Tint::ALL {
            for variant in ControlVariant::ALL {
                let c = v.control_colors(variant, tint, ControlState::Disabled);
                assert_eq!(c.content, v.color_content_faint);
                if variant != ControlVariant::Plain {
                    assert_eq!(c.surface, v.color_surface_sunken);
                }
                assert_eq!(c.border.a, 0.0);
            }
        }
    }
}

#[test]
fn slider_steps_are_relative_to_min_and_clamped_to_bounds() {
    let slider = Slider::new(12.0, 10.0, 23.0).step(3.0);
    for (input, expected) in [(9.0, 10.0), (12.0, 13.0), (18.0, 19.0), (25.0, 23.0)] {
        assert_eq!(slider.normalize(input), expected);
    }
}

#[test]
fn splitter_preserves_total_and_respects_both_neighbors() {
    let mut left = SplitterPanel::new(Text::new("Left"));
    left.default_size = 9.0;
    left.min_size = 20.0;
    left.max_size = 60.0;
    let mut right = SplitterPanel::new(Text::new("Right"));
    right.default_size = 1.0;
    right.min_size = 30.0;
    right.max_size = 80.0;
    let splitter = Splitter::new(vec![left, right]);
    let mut sizes = splitter.initial_sizes();
    assert_eq!(sizes, [60.0, 40.0]);
    splitter.resize_pair(&mut sizes, 0, -100.0);
    assert_eq!(sizes, [20.0, 80.0]);
    splitter.resize_pair(&mut sizes, 0, 200.0);
    assert_eq!(sizes, [60.0, 40.0]);
}

#[test]
fn toast_expiry_does_not_remove_persistent_or_newer_notifications() {
    let cell = support::test_cell();
    let manager = ToastManager::new(&mut cell.borrow_mut());
    let persistent = manager.show(
        &mut cell.borrow_mut(),
        "Persistent",
        "",
        Tint::Info,
        Duration::ZERO,
    );
    manager.show(
        &mut cell.borrow_mut(),
        "Short",
        "",
        Tint::Success,
        Duration::from_millis(20),
    );
    manager.show(
        &mut cell.borrow_mut(),
        "Long",
        "",
        Tint::Warning,
        Duration::from_millis(40),
    );
    cell.elapse(Duration::from_millis(20));
    assert_eq!(manager.len(&mut cell.borrow_mut()), 2);
    cell.elapse(Duration::from_millis(20));
    assert_eq!(manager.len(&mut cell.borrow_mut()), 1);
    manager.dismiss(&mut cell.borrow_mut(), persistent);
    assert!(manager.is_empty(&mut cell.borrow_mut()));
}
