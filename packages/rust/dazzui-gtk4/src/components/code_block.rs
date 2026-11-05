//! Readable source with native selection, scrolling and clipboard ownership.
use crate::{Size, Theme, Tint, Variant, box_, icon_button, prelude::*};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, Debug, Default)]
pub struct CodeBlockOptions {
    pub line_numbers: bool,
    pub wrap: bool,
    pub copyable: bool,
}

pub fn code_block(
    code: &str,
    title: Option<&gtk::Widget>,
    language: Option<&str>,
    actions: Option<&gtk::Widget>,
    options: CodeBlockOptions,
) -> gtk::Box {
    let body = code.trim_end_matches('\n');
    let root = box_(gtk::Orientation::Vertical, "0");
    root.add_css_class("dz-code-block");
    root.set_dazz_tint(Tint::Neutral);
    root.set_overflow(gtk::Overflow::Hidden);
    let listing = gtk::Label::new(Some(body));
    listing.add_css_class("dz-code-pre");
    listing.set_selectable(true);
    listing.set_xalign(0.);
    listing.set_yalign(0.);
    listing.set_wrap(options.wrap);
    listing.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    listing.set_visible(!body.is_empty());
    listing.connect_label_notify(|label| {
        let text = label.text();
        let body = text.trim_end_matches('\n');
        if text.as_str() != body {
            label.set_label(body);
        }
        label.set_visible(!body.is_empty());
    });
    if title.is_some()
        || language.is_some_and(|s| !s.is_empty())
        || actions.is_some()
        || options.copyable
    {
        let head = box_(gtk::Orientation::Horizontal, "2");
        head.add_css_class("dz-code-head");
        if let Some(title) = title {
            title.add_css_class("dz-code-title");
            if let Some(label) = title.downcast_ref::<gtk::Label>() {
                label.set_wrap(false);
                label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            }
            head.append(title);
        }
        if let Some(language) = language.filter(|s| !s.is_empty()) {
            let label = gtk::Label::new(Some(&language.to_uppercase()));
            label.add_css_class("dz-code-language");
            head.append(&label);
        }
        let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        head.append(&spacer);
        let controls = box_(gtk::Orientation::Horizontal, "0-5");
        controls.set_valign(gtk::Align::Center);
        if let Some(actions) = actions {
            controls.append(actions);
        }
        if options.copyable {
            let copy = icon_button(
                "dazzui-Copy16Regular-symbolic",
                "Copy",
                Variant::Plain,
                Tint::Neutral,
                Size::Tiny,
            );
            copy.add_css_class("dz-code-copy");
            let pending: Rc<RefCell<Option<gtk::glib::SourceId>>> = Rc::new(RefCell::new(None));
            let timer = pending.clone();
            let source = listing.downgrade();
            copy.connect_clicked(move |button| {
                let Some(source) = source.upgrade() else {
                    return;
                };
                button
                    .clipboard()
                    .set_text(source.text().trim_end_matches('\n'));
                // React's copied state keeps its original timeout when
                // another successful copy occurs before it expires.
                if timer.borrow().is_some() {
                    return;
                }
                copy_feedback(button, true);
                let weak = button.downgrade();
                let timer_ref = timer.clone();
                let delay = Theme::of(button)
                    .unwrap_or_default()
                    .duration("code-block-copy-feedback-duration")
                    .unwrap();
                *timer.borrow_mut() = Some(gtk::glib::timeout_add_local_once(delay, move || {
                    timer_ref.borrow_mut().take();
                    if let Some(button) = weak.upgrade() {
                        copy_feedback(&button, false);
                    }
                }));
            });
            let timer = pending.clone();
            copy.connect_unmap(move |button| {
                if let Some(timer) = timer.borrow_mut().take() {
                    timer.remove();
                }
                copy_feedback(button, false);
            });
            copy.connect_destroy(move |_| {
                if let Some(timer) = pending.borrow_mut().take() {
                    timer.remove();
                }
            });
            controls.append(&copy);
        }
        head.append(&controls);
        root.append(&head);
    }
    let row = box_(gtk::Orientation::Horizontal, "0");
    row.add_css_class("dz-code-scroll");
    if options.line_numbers {
        let lines = body
            .split('\n')
            .enumerate()
            .map(|(i, _)| (i + 1).to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let gutter = gtk::Label::new(Some(&lines));
        gutter.add_css_class("dz-code-gutter");
        gutter.set_xalign(1.);
        gutter.set_valign(gtk::Align::Start);
        gutter.set_can_target(false);
        gutter.set_accessible_role(gtk::AccessibleRole::Presentation);
        gutter.update_state(&[gtk::accessible::State::Hidden(true)]);
        let weak = gutter.downgrade();
        listing.connect_label_notify(move |source| {
            if let Some(gutter) = weak.upgrade() {
                gutter.set_label(
                    &source
                        .text()
                        .trim_end_matches('\n')
                        .split('\n')
                        .enumerate()
                        .map(|(i, _)| (i + 1).to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
            }
        });
        row.append(&gutter);
    }
    if options.line_numbers {
        listing.add_css_class("dz-code-numbered");
    }
    let scroll = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(if options.wrap {
            gtk::PolicyType::Never
        } else {
            gtk::PolicyType::Automatic
        })
        .vscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .hexpand(true)
        .child(&listing)
        .build();
    row.append(&scroll);
    root.append(&row);
    root
}

/// The native source label. Updating it also updates the gutter and the
/// payload offered by the copy action, without rebuilding the panel.
pub fn code_block_source(block: &gtk::Box) -> gtk::Label {
    fn source(widget: &gtk::Widget) -> Option<gtk::Label> {
        if widget.has_css_class("dz-code-pre") {
            return widget.clone().downcast().ok();
        }
        let mut child = widget.first_child();
        while let Some(node) = child {
            child = node.next_sibling();
            if let Some(found) = source(&node) {
                return Some(found);
            }
        }
        None
    }
    source(block.upcast_ref()).expect("A CodeBlock contains its native source label")
}

fn copy_feedback(button: &gtk::Button, copied: bool) {
    if let Some(image) = button.child().and_downcast::<gtk::Image>() {
        image.set_icon_name(Some(if copied {
            "dazzui-Checkmark16Regular-symbolic"
        } else {
            "dazzui-Copy16Regular-symbolic"
        }));
    }
    let label = if copied { "Copied" } else { "Copy" };
    button.set_dazz_label(label);
    button.set_tooltip_text(Some(label));
}
