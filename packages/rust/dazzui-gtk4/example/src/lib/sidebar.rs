use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;
use std::cell::RefCell;
thread_local! { static PROVIDERS: RefCell<Vec<SidebarProvider>> = const { RefCell::new(vec![]) }; }
fn provider() -> Result<SidebarProvider, String> {
    PROVIDERS
        .with_borrow(|p| p.last().cloned())
        .ok_or_else(|| "Sidebar requires SidebarProvider".into())
}
fn nodes(raw: &Value) -> &[Value] {
    raw["children"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default()
}
fn caption(raw: &Value) -> String {
    if raw.is_number() {
        return raw.to_string();
    }
    if let Some(s) = raw.as_str() {
        return s.into();
    }
    nodes(raw).iter().map(caption).collect()
}
fn options(p: &Value) -> SidebarOptions {
    SidebarOptions {
        side: if p["side"] == "right" {
            SidebarSide::Right
        } else {
            SidebarSide::Left
        },
        variant: match p["variant"].as_str() {
            Some("floating") => SidebarVariant::Floating,
            Some("inset") => SidebarVariant::Inset,
            _ => SidebarVariant::Sidebar,
        },
        collapsible: match p["collapsible"].as_str() {
            Some("none") => SidebarCollapsible::None,
            Some("icon") => SidebarCollapsible::Icon,
            _ => SidebarCollapsible::Offcanvas,
        },
        collapsed: p["collapsed"].as_bool(),
        width: p["width"].as_str().and_then(|s| {
            s.strip_suffix("rem")
                .and_then(|s| s.parse::<f64>().ok())
                .map(|n| n * 16.)
                .or_else(|| s.strip_suffix("px").and_then(|s| s.parse().ok()))
        }),
    }
}
pub(super) fn render(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let kind = raw["$el"].as_str().unwrap();
    let p = ctx.resolve(&raw["props"], &[]);
    let append = |box_: &gtk::Box, specimens: &mut Vec<_>| -> Result<(), String> {
        for child in nodes(raw) {
            box_.append(&scene::render_with(child, specimens, ctx)?);
        }
        Ok(())
    };
    Ok(match kind {
        "SidebarProvider" => {
            let provider = SidebarProvider::new(p["defaultOpen"].as_bool().unwrap_or(true));
            let source = raw["props"].clone();
            let binding = source.clone();
            ctx.bind(&provider, move |provider, ctx| {
                provider.set_controlled_open(ctx.resolve(&binding, &[])["open"].as_bool())
            });
            let context = ctx.clone();
            provider.connect_open_change(move |_, open| {
                context.run(&source["onOpenChange"], &[Value::Bool(open)])
            });
            PROVIDERS.with_borrow_mut(|p| p.push(provider.clone()));
            let result = append(provider.upcast_ref(), specimens);
            PROVIDERS.with_borrow_mut(|p| {
                p.pop();
            });
            result?;
            let mut child = provider.first_child();
            while let Some(w) = child {
                child = w.next_sibling();
                if w.downcast_ref::<Sidebar>()
                    .is_some_and(|s| s.options().variant == SidebarVariant::Inset)
                {
                    provider.add_css_class("dz-sidebar-provider-inset");
                }
            }
            provider.upcast()
        }
        "Sidebar" => {
            let sidebar = Sidebar::new(&provider()?, options(&p));
            for child in nodes(raw) {
                sidebar.append(&scene::render_with(child, specimens, ctx)?);
            }
            let source = raw["props"].clone();
            ctx.bind(&sidebar, move |sidebar, ctx| {
                sidebar.set_options(options(&ctx.resolve(&source, &[])))
            });
            sidebar.upcast()
        }
        "SidebarHeader" => {
            let header = sidebar_header();
            let content = sidebar_box("header-content");
            append(&content, specimens)?;
            fn header_text(w: &gtk::Widget) {
                if let Some(label) = w.downcast_ref::<gtk::Label>() {
                    label.set_wrap_mode(gtk::pango::WrapMode::Word);
                }
                let mut child = w.first_child();
                while let Some(w) = child {
                    child = w.next_sibling();
                    header_text(&w);
                }
            }
            header_text(content.upcast_ref());
            header.set_child(&content);
            header.upcast()
        }
        "SidebarContent" => {
            let scroll = gtk::ScrolledWindow::new();
            scroll.add_css_class("dz-sidebar-content");
            scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
            scroll.set_vexpand(true);
            scroll.set_propagate_natural_height(false);
            let body = sidebar_box("content-body");
            append(&body, specimens)?;
            scroll.set_child(Some(&body));
            scroll.upcast()
        }
        "SidebarGroupLabel" => {
            let label = section_label(&caption(raw));
            label.add_css_class("dz-sidebar-group-label");
            label.upcast()
        }
        "SidebarTrigger" => {
            let button = sidebar_trigger_label(
                &provider()?,
                p["label"].as_str().unwrap_or("Toggle sidebar"),
            );
            if let Some(label) = p["label"].as_str() {
                button.set_dazz_label(label);
            }
            let source = raw["props"].clone();
            ctx.bind(&button, move |button, ctx| {
                button.set_sensitive(
                    !ctx.resolve(&source, &[])["disabled"]
                        .as_bool()
                        .unwrap_or(false),
                );
            });
            button.upcast()
        }
        "SidebarMenuButton" => {
            let icon = raw["props"]
                .get("icon")
                .map(|node| scene::render_with(node, specimens, ctx))
                .transpose()?;
            let button = sidebar_menu_button(
                &caption(&ctx.resolve(raw, &[])),
                icon.as_ref(),
                scene::variant(&p, Variant::Filled),
                scene::tint(&p, "primary"),
                scene::size(&p, "medium"),
            );
            let source = raw["props"].clone();
            let binding = source.clone();
            ctx.bind(&button, move |button, ctx| {
                let p = ctx.resolve(&binding, &[]);
                set_sidebar_active(button, p["active"].as_bool().unwrap_or(false));
                button.set_dazz_size(scene::size(&p, "medium"));
                button.set_dazz_variant(scene::variant(&p, Variant::Filled));
                button.set_dazz_tint(scene::tint(&p, "primary"));
                button.set_sensitive(!p["disabled"].as_bool().unwrap_or(false));
                if let Some(name) = p["aria-label"].as_str() {
                    button.set_dazz_label(name);
                }
            });
            let context = ctx.clone();
            button.connect_clicked(move |_| context.run(&source["onClick"], &[]));
            if let Some(text) = p["tooltip"].as_str()
                && let Ok(provider) = provider()
            {
                sidebar_menu_tooltip(&button, &provider, text);
            }
            button.upcast()
        }
        "SidebarMenuItem" => {
            let item = gtk::Overlay::new();
            item.add_css_class("dz-sidebar-menu-item");
            let body = sidebar_box("menu-item-body");
            item.set_child(Some(&body));
            let mut buttons = vec![];
            let mut trailing = false;
            for node in nodes(raw) {
                let child = scene::render_with(node, specimens, ctx)?;
                if child.has_css_class("dz-sidebar-badge") {
                    trailing = true;
                    child.set_halign(gtk::Align::End);
                    child.set_valign(gtk::Align::Center);
                    item.add_overlay(&child);
                    item.set_measure_overlay(&child, false);
                } else {
                    if let Some(button) = child.downcast_ref::<gtk::Button>() {
                        buttons.push(button.clone());
                    }
                    body.append(&child);
                }
            }
            for button in buttons {
                if trailing {
                    button.add_css_class("dz-has-trailing");
                }
                let update = |item: &gtk::Overlay, button: &gtk::Button| {
                    for class in [
                        "dz-active",
                        "dz-variant-filled",
                        "dz-variant-tinted",
                        "dz-variant-outlined",
                        "dz-variant-recessed",
                        "dz-variant-normal",
                        "dz-variant-plain",
                    ] {
                        if button.has_css_class(class) {
                            item.add_css_class(class);
                        } else {
                            item.remove_css_class(class);
                        }
                    }
                    if button.state_flags().contains(gtk::StateFlags::PRELIGHT)
                        && button.is_sensitive()
                    {
                        item.add_css_class("dz-hovered");
                    } else {
                        item.remove_css_class("dz-hovered");
                    }
                };
                update(&item, &button);
                let weak = item.downgrade();
                button.connect_css_classes_notify(move |button| {
                    if let Some(item) = weak.upgrade() {
                        update(&item, button);
                    }
                });
                let weak = item.downgrade();
                button.connect_state_flags_changed(move |button, _| {
                    if let Some(item) = weak.upgrade() {
                        update(&item, button);
                    }
                });
            }
            item.upcast()
        }
        "SidebarMenuBadge" => {
            let badge = sidebar_menu_badge(&caption(raw));
            badge.set_dazz_tint(scene::tint(&p, "primary"));
            let source = raw["props"].clone();
            ctx.bind(&badge, move |badge, ctx| {
                badge.set_dazz_tint(scene::tint(&ctx.resolve(&source, &[]), "primary"));
            });
            badge.set_can_target(false);
            badge.upcast()
        }
        _ => {
            let part = match kind {
                "SidebarInset" => "inset",
                "SidebarGroup" => "group",
                "SidebarGroupContent" => "group-content",
                "SidebarMenu" => "menu",
                "SidebarFooter" => "footer",
                _ => return Err(format!("Unmapped sidebar part {kind}")),
            };
            let box_ = sidebar_box(part);
            if kind == "SidebarInset" {
                box_.set_hexpand(true);
                box_.set_accessible_role(gtk::AccessibleRole::Main);
            }
            append(&box_, specimens)?;
            box_.upcast()
        }
    })
}
