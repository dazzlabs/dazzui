#![feature(arbitrary_self_types)]
#[path = "../../tests/support/mod.rs"]
mod support;
use dazzui_inset::{STUDIO_LIGHT, Theme};
use dazzui_inset_playground::{generated_stories, host, scene_state::SceneHandle};
use inset::*;
use inset_embedder::{PointerChange, PointerData, PointerDataPacket, PointerDeviceKind};
use inset_gestures::GestureBinding;
use inset_services::{TextEditingValue, TextInputClient};
use std::{cell::RefCell, rc::Rc, time::Duration};

fn text(app: &App, element: BuildContext) -> String {
    if let Some(text) = element.widget(app).as_any().downcast_ref::<Text>() {
        return text.data.clone().unwrap_or_default();
    }
    let mut result = String::new();
    element.visit_child_elements(app, &mut |child| result.push_str(&text(app, child)));
    result
}
fn find_editable(app: &App, element: BuildContext) -> Option<Handle<EditableTextState>> {
    if let Some(element) = element.downcast::<StatefulElement<EditableText>>(app) {
        return Some(element.state(app));
    }
    let mut result = None;
    element.visit_child_elements(app, &mut |child| {
        if result.is_none() {
            result = find_editable(app, child);
        }
    });
    result
}
fn disabled(app: &App, element: BuildContext) -> Option<bool> {
    if let Some(button) = element
        .widget(app)
        .as_any()
        .downcast_ref::<dazzui_inset::Button>()
    {
        return Some(button.on_pressed.is_none());
    }
    let mut result = None;
    element.visit_child_elements(app, &mut |child| {
        if result.is_none() {
            result = disabled(app, child);
        }
    });
    result
}
fn same_value(actual: &serde_json::Value, expected: &serde_json::Value) -> bool {
    match (actual.as_f64(), expected.as_f64()) {
        (Some(a), Some(b)) => a == b,
        _ => actual == expected,
    }
}
/// Every checkpoint starts from a fresh mount, as the React runner does.
#[test]
fn shared_checkpoints_drive_native_controls_timers_and_dialogs() {
    let scenes: serde_json::Value = serde_json::from_str(generated_stories::INTERACTIONS).unwrap();
    assert!(!scenes.as_object().unwrap().is_empty());
    for (id, checkpoints) in scenes.as_object().unwrap() {
        let cell = support::test_cell();
        let observed = Rc::new(RefCell::new(None::<SceneHandle>));
        let slot = cell
            .borrow_mut()
            .new_entity(|_| SizedBox::shrink().into_widget());
        let render_slot = slot.clone();
        support::build(
            &cell,
            host(Theme::new(
                &STUDIO_LIGHT,
                Center::new().child(Builder::new(move |app, _| app.read(&render_slot).clone())),
            ))
            .into_widget(),
        );
        let mut tick = 20;
        let mut previous: Option<SceneHandle> = None;
        for checkpoint in checkpoints.as_array().unwrap() {
            // Unmount the previous scene so its state, controllers and timers go.
            cell.borrow_mut().update(&slot, |child, cx| {
                *child = SizedBox::shrink().into_widget();
                cx.notify();
            });
            tick += 20;
            support::pump(&mut cell.borrow_mut(), Duration::from_millis(tick));
            if let Some(old) = previous.take() {
                if let Some(first) = checkpoint["steps"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find_map(|s| s["target"].as_str())
                {
                    assert!(
                        old.key(first)
                            .current_context(&mut cell.borrow_mut())
                            .is_none(),
                        "{id}: unmounted"
                    );
                }
            }
            let observe = observed.clone();
            let widget = generated_stories::build_observed(
                id,
                Some(Rc::new(move |_, state| *observe.borrow_mut() = Some(state))),
            )
            .unwrap();
            cell.borrow_mut().update(&slot, |child, cx| {
                *child = widget;
                cx.notify();
            });
            tick += 20;
            support::pump(&mut cell.borrow_mut(), Duration::from_millis(tick));
            let state = observed.borrow().as_ref().unwrap().clone();
            for step in checkpoint["steps"].as_array().unwrap() {
                let mut app = cell.borrow_mut();
                match step["action"].as_str().unwrap() {
                    "advance" => {
                        tick += step["milliseconds"].as_u64().unwrap();
                        support::pump(&mut app, Duration::from_millis(tick));
                    }
                    "fill" => {
                        let target = step["target"].as_str().unwrap();
                        let element = state.key(target).current_context(&mut app).expect(target);
                        let editable = find_editable(&app, element).expect("native editable text");
                        let value = TextEditingValue::default()
                            .copy_with()
                            .text(step["value"].as_str().unwrap());
                        editable.update_editing_value(&mut app, value);
                        tick += 20;
                        support::pump(&mut app, Duration::from_millis(tick));
                    }
                    "click" => {
                        let target = step["target"].as_str().unwrap();
                        let render = state
                            .key(target)
                            .current_context(&mut app)
                            .expect(target)
                            .find_render_object(&app)
                            .unwrap()
                            .as_box()
                            .unwrap();
                        let size = render.size(&app);
                        let position = render.local_to_global(
                            &app,
                            Offset::new(size.width() / 2.0, size.height() / 2.0),
                            None,
                        );
                        for change in [
                            PointerChange::Add,
                            PointerChange::Down,
                            PointerChange::Up,
                            PointerChange::Remove,
                        ] {
                            tick += 20;
                            let at = Duration::from_millis(tick);
                            GestureBinding::instance(&mut app).handle_pointer_data_packet(
                                &mut app,
                                PointerDataPacket {
                                    data: vec![PointerData {
                                        change,
                                        kind: PointerDeviceKind::Mouse,
                                        physical_x: position.dx() * 2.0,
                                        physical_y: position.dy() * 2.0,
                                        time_stamp: at,
                                        buttons: if change == PointerChange::Down { 1 } else { 0 },
                                        ..Default::default()
                                    }],
                                },
                            );
                            support::pump(&mut app, at);
                        }
                    }
                    other => panic!("unsupported native checkpoint action {other}"),
                }
            }
            let mut app = cell.borrow_mut();
            for (key, expected) in checkpoint["expect"].as_object().into_iter().flatten() {
                let actual = app.read(&state.values)[key].clone();
                assert!(
                    same_value(&actual, expected),
                    "{id}/{}: {key} = {actual}, expected {expected}",
                    checkpoint["id"]
                );
            }
            for (key, expected) in checkpoint["expectDisabled"]
                .as_object()
                .into_iter()
                .flatten()
            {
                let element = state.key(key).current_context(&mut app).unwrap();
                assert_eq!(
                    disabled(&app, element),
                    expected.as_bool(),
                    "{id}: disabled {key}"
                );
            }
            for (key, expected) in checkpoint["expectText"].as_object().into_iter().flatten() {
                let element = state.key(key).current_context(&mut app).unwrap();
                assert_eq!(
                    text(&app, element),
                    expected.as_str().unwrap(),
                    "{id}/{}: {key}",
                    checkpoint["id"]
                );
            }
            drop(app);
            previous = Some(state);
        }
    }
}

#[test]
fn assignment_updates_editing_controller_and_navigation_disposes_it() {
    use dazzui_inset_playground::scene_state::{self, Scene};
    let cell = support::test_cell();
    let observed = Rc::new(RefCell::new(None::<SceneHandle>));
    let make = |id: &str, initial: &str| {
        let observe = observed.clone();
        Scene::new(
            id.to_string(),
            [("name".to_string(), serde_json::json!(initial))]
                .into_iter()
                .collect(),
            Some(Rc::new(move |_, state| *observe.borrow_mut() = Some(state))),
            |app, _, state| {
                dazzui_inset::TextField::new()
                    .controller(state.controller(app, "name"))
                    .into_widget()
            },
        )
        .into_widget()
    };
    let slot = cell.borrow_mut().new_entity(|_| make("first", "Before"));
    let render_slot = slot.clone();
    support::build(
        &cell,
        host(Theme::new(
            &STUDIO_LIGHT,
            Center::new().child(
                SizedBox::new()
                    .width(300.0)
                    .child(Builder::new(move |app, _| app.read(&render_slot).clone())),
            ),
        ))
        .into_widget(),
    );
    let first = observed.borrow().as_ref().unwrap().clone();
    let controller = first.controller(&mut cell.borrow_mut(), "name");
    scene_state::set(&mut cell.borrow_mut(), &first, "name", "After");
    assert_eq!(controller.text_value(&cell.borrow()), "After");
    let next = make("second", "Fresh");
    cell.borrow_mut().update(&slot, |child, cx| {
        *child = next;
        cx.notify();
    });
    support::pump(&mut cell.borrow_mut(), Duration::from_millis(20));
    assert!(
        !cell.borrow().contains(controller),
        "old controller destroyed on navigation"
    );
    let second = observed.borrow().as_ref().unwrap().clone();
    assert_eq!(cell.borrow().read(&second.values)["name"], "Fresh");
    let controller = second.controller(&mut cell.borrow_mut(), "name");
    support::build(&cell, SizedBox::shrink().into_widget());
    assert!(
        !cell.borrow().contains(controller),
        "controller destroyed on unmount"
    );
}

#[test]
fn unmounting_a_timed_scene_stops_its_ticker() {
    let cell = support::test_cell();
    let observed = Rc::new(RefCell::new(None::<SceneHandle>));
    let observe = observed.clone();
    let widget = generated_stories::build_observed(
        "components-progress--live",
        Some(Rc::new(move |_, state| *observe.borrow_mut() = Some(state))),
    )
    .unwrap();
    let slot = cell.borrow_mut().new_entity(|_| widget);
    let render_slot = slot.clone();
    support::build(
        &cell,
        host(Theme::new(
            &STUDIO_LIGHT,
            Center::new().child(Builder::new(move |app, _| app.read(&render_slot).clone())),
        ))
        .into_widget(),
    );
    let state = observed.borrow().as_ref().unwrap().clone();
    support::pump(&mut cell.borrow_mut(), Duration::from_millis(800));
    let running = cell.borrow().read(&state.values)["value"].as_f64().unwrap();
    assert_eq!(running, 16.0);
    cell.borrow_mut().update(&slot, |child, cx| {
        *child = SizedBox::shrink().into_widget();
        cx.notify();
    });
    support::pump(&mut cell.borrow_mut(), Duration::from_millis(820));
    support::pump(&mut cell.borrow_mut(), Duration::from_millis(5000));
    assert_eq!(
        cell.borrow().read(&state.values)["value"].as_f64().unwrap(),
        running
    );
}
