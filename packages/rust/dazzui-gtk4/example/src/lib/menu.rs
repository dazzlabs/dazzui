//! The shared menu data feeds GTK models and actions; row slots keep the
//! compiled specimen addresses and the original trigger widgets.
use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;

fn action_name(set: usize, item: usize) -> String {
    format!("m{set}i{item}")
}
fn selected(items: &Value) -> String {
    items
        .as_array()
        .and_then(|v| {
            v.iter()
                .position(|i| i["checked"].as_bool().unwrap_or(false))
        })
        .map(|i| i.to_string())
        .unwrap_or_else(|| "none".into())
}
fn model(
    raw: &Value,
    set: usize,
    group: &gtk::gio::SimpleActionGroup,
    ctx: &Context,
) -> Result<gtk::gio::Menu, String> {
    let nodes = raw
        .as_array()
        .ok_or("Menu items must be a committed array")?;
    let resolved = ctx.resolve(raw, &[]);
    let items = resolved
        .as_array()
        .ok_or("Menu items must resolve to an array")?;
    let value = selected(&resolved);
    let model = gtk::gio::Menu::new();
    for (index, (node, item)) in nodes.iter().zip(items).enumerate() {
        let label = item["label"]
            .as_str()
            .ok_or("Menu labels must resolve to strings")?;
        let name = action_name(set, index);
        let checkable = item.get("checked").is_some();
        let action = if checkable {
            gtk::gio::SimpleAction::new_stateful(
                &name,
                Some(gtk::glib::VariantTy::STRING),
                &value.to_variant(),
            )
        } else {
            gtk::gio::SimpleAction::new(&name, None)
        };
        action.set_enabled(!item["disabled"].as_bool().unwrap_or(false));
        let ctx = ctx.clone();
        let callback = node["onSelect"].clone();
        let source = raw.clone();
        action.connect_activate(move |_, _| {
            if checkable && selected(&ctx.resolve(&source, &[])) == index.to_string() {
                return;
            }
            ctx.run(&callback, &[]);
        });
        group.add_action(&action);
        let entry = gtk::gio::MenuItem::new(Some(&label.replace('_', "__")), None);
        let target = checkable.then(|| index.to_string().to_variant());
        entry.set_action_and_target_value(Some(&format!("dazzmenu.{name}")), target.as_ref());
        model.append_item(&entry);
    }
    Ok(model)
}
fn bind_models(
    widget: &gtk::Widget,
    sets: Vec<Value>,
    group: &gtk::gio::SimpleActionGroup,
    ctx: &Context,
) {
    let weak = group.downgrade();
    ctx.bind(widget, move |_, ctx| {
        let Some(group) = weak.upgrade() else {
            return;
        };
        for (set, raw) in sets.iter().enumerate() {
            let items = ctx.resolve(raw, &[]);
            let checked = selected(&items);
            for (index, item) in items.as_array().into_iter().flatten().enumerate() {
                if let Some(action) = group
                    .lookup_action(&action_name(set, index))
                    .and_downcast::<gtk::gio::SimpleAction>()
                {
                    action.set_enabled(!item["disabled"].as_bool().unwrap_or(false));
                    if item.get("checked").is_some() {
                        action.set_state(&checked.to_variant());
                    }
                }
            }
        }
    });
}
fn decorate(
    root: &gtk::Widget,
    sets: &[Value],
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<(), String> {
    let nodes: Vec<_> = sets
        .iter()
        .flat_map(|items| items.as_array().into_iter().flatten())
        .collect();
    let rows = menu_rows(root);
    if rows.len() != nodes.len() {
        return Err(format!(
            "GTK created {} menu rows for {} items",
            rows.len(),
            nodes.len()
        ));
    }
    for (row, raw) in rows.iter().zip(nodes) {
        let slot = |name: &str,
                    specimens: &mut Vec<(usize, gtk::Widget)>|
         -> Result<Option<gtk::Widget>, String> {
            let Some(node) = raw.get(name) else {
                return Ok(None);
            };
            let value = ctx.resolve(node, &[]);
            if value.is_null() || value == false || value == "" {
                return Ok(None);
            }
            let child = scene::render_with(node, specimens, ctx)?;
            if let Some(label) = child.downcast_ref::<gtk::Label>() {
                label.set_wrap(false);
            }
            if let Some(text) = child.downcast_ref::<TextBox>() {
                text.label().set_wrap(false);
            }
            let slot = box_(gtk::Orientation::Horizontal, "0");
            slot.append(&child);
            Ok(Some(slot.upcast()))
        };
        let leading = slot("icon", specimens)?;
        let trailing = slot("shortcut", specimens)?;
        menu_row_content(row, leading.as_ref(), trailing.as_ref());
        let label = raw["label"].clone();
        ctx.bind(row, move |row, ctx| {
            if let Some(label) = ctx.resolve(&label, &[]).as_str()
                && row.property::<String>("text") != label
            {
                row.set_property("text", label.replace('_', "__"));
            }
        });
    }
    Ok(())
}
fn tint(p: &Value) -> Tint {
    match p["tint"].as_str().unwrap_or("neutral") {
        "primary" => Tint::Primary,
        "info" => Tint::Info,
        "success" => Tint::Success,
        "warning" => Tint::Warning,
        "danger" => Tint::Danger,
        _ => Tint::Neutral,
    }
}
fn style_popups(widget: &gtk::Widget, tint: Tint) {
    if let Some(popup) = widget.downcast_ref::<gtk::PopoverMenu>() {
        popup.set_dazz_tint(tint);
    }
    let mut child = widget.first_child();
    while let Some(w) = child {
        child = w.next_sibling();
        style_popups(&w, tint);
    }
}
pub fn render(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let node = ctx.resolve(raw, &[]);
    let p = &node["props"];
    let props = &raw["props"];
    let group = gtk::gio::SimpleActionGroup::new();
    if node["$el"] == "Menubar" {
        let menus = props["menus"]
            .as_array()
            .ok_or("Menubar needs a committed menu array")?;
        let model_root = gtk::gio::Menu::new();
        let mut sets = vec![];
        for (index, menu) in menus.iter().enumerate() {
            let resolved = ctx.resolve(menu, &[]);
            let items = menu["items"].clone();
            let section = model(&items, index, &group, ctx)?;
            model_root.append_submenu(
                resolved["label"]
                    .as_str()
                    .map(|s| s.replace('_', "__"))
                    .as_deref(),
                &section,
            );
            sets.push(items);
        }
        let bar = menubar(&model_root);
        bar.insert_action_group("dazzmenu", Some(&group));
        style_popups(bar.upcast_ref(), tint(p));
        decorate(bar.upcast_ref(), &sets, specimens, ctx)?;
        bind_models(bar.upcast_ref(), sets, &group, ctx);
        let source = props["menus"].clone();
        let disabled = props["disabled"].clone();
        ctx.bind(&bar, move |bar, ctx| {
            bar.set_sensitive(!ctx.resolve(&disabled, &[]).as_bool().unwrap_or(false));
            let value = ctx.resolve(&source, &[]);
            let mut child = bar.first_child();
            for menu in value.as_array().into_iter().flatten() {
                let Some(item) = child else {
                    break;
                };
                child = item.next_sibling();
                item.set_sensitive(!menu["disabled"].as_bool().unwrap_or(false));
            }
        });
        return Ok(bar.upcast());
    }
    let context = node["$el"] == "ContextMenu";
    let trigger = scene::render_with(
        if context {
            raw["children"]
                .as_array()
                .and_then(|v| v.first())
                .ok_or("ContextMenu needs a region")?
        } else {
            &props["trigger"]
        },
        specimens,
        ctx,
    )?;
    let model = model(&props["items"], 0, &group, ctx)?;
    let popup = menu_popup(&model);
    popup.insert_action_group("dazzmenu", Some(&group));
    popup.set_dazz_tint(tint(p));
    let side = match p["side"].as_str().unwrap_or("bottom") {
        "top" => gtk::PositionType::Top,
        "left" => gtk::PositionType::Left,
        "right" => gtk::PositionType::Right,
        _ => gtk::PositionType::Bottom,
    };
    let align = match p["align"]
        .as_str()
        .unwrap_or(if context { "start" } else { "end" })
    {
        "start" => gtk::Align::Start,
        "center" => gtk::Align::Center,
        _ => gtk::Align::End,
    };
    attach_popover(&trigger, &popup, side, align);
    decorate(
        popup.upcast_ref(),
        &[props["items"].clone()],
        specimens,
        ctx,
    )?;
    bind_models(
        popup.upcast_ref(),
        vec![props["items"].clone()],
        &group,
        ctx,
    );
    trigger.update_property(&[gtk::accessible::Property::HasPopup(true)]);
    trigger.update_state(&[gtk::accessible::State::Expanded(Some(false))]);
    let weak = trigger.downgrade();
    popup.connect_visible_notify(move |p| {
        if let Some(trigger) = weak.upgrade() {
            trigger.update_state(&[gtk::accessible::State::Expanded(Some(p.is_visible()))]);
        }
    });
    if context {
        trigger.set_focusable(true);
        let click = gtk::GestureClick::new();
        click.set_button(3);
        click.set_propagation_phase(gtk::PropagationPhase::Capture);
        let weak = popup.downgrade();
        click.connect_pressed(move |gesture, _, x, y| {
            if let Some(popup) = weak.upgrade() {
                popup.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
                popup.popup();
                gesture.set_state(gtk::EventSequenceState::Claimed);
            }
        });
        trigger.add_controller(click);
        let press = gtk::GestureLongPress::new();
        press.set_touch_only(true);
        let weak = popup.downgrade();
        press.connect_pressed(move |_, x, y| {
            if let Some(popup) = weak.upgrade() {
                popup.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 1, 1)));
                popup.popup();
            }
        });
        trigger.add_controller(press);
    } else {
        scene::popup_trigger(&trigger, popup.upcast_ref());
    }
    let keys = gtk::EventControllerKey::new();
    keys.set_propagation_phase(gtk::PropagationPhase::Capture);
    keys.set_propagation_limit(gtk::PropagationLimit::SameNative);
    let weak = popup.downgrade();
    keys.connect_key_pressed(move |controller, key, _, mods| {
        use gtk::{
            gdk::{Key, ModifierType as M},
            glib::Propagation,
        };
        let show = if context {
            key == Key::Menu || (key == Key::F10 && mods == M::SHIFT_MASK)
        } else {
            (key == Key::Down || key == Key::Up) && mods.is_empty()
        };
        if !show {
            return Propagation::Proceed;
        }
        if let Some(popup) = weak.upgrade() {
            if context && let Some(w) = controller.widget() {
                popup.set_pointing_to(Some(&gtk::gdk::Rectangle::new(
                    w.width() / 2,
                    w.height() / 2,
                    1,
                    1,
                )));
            }
            popup.popup();
            popup.remove_css_class("dz-menu-initial");
            let rows: Vec<_> = menu_rows(&popup)
                .into_iter()
                .filter(|w| w.is_sensitive())
                .collect();
            if let Some(row) = if key == Key::Up {
                rows.last()
            } else {
                rows.first()
            } {
                row.grab_focus();
            }
        }
        Propagation::Stop
    });
    trigger.add_controller(keys);
    Ok(trigger)
}
