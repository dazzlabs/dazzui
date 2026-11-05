use dazzui_nsh::{ButtonAction, DateRangeValue, SelectItem, ToastHandle, ToastNotice, prelude::*};
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;
use std::time::Duration;

fn desktop(root: Root) -> EngineConfig {
    let mut config = EngineConfig::new(root);
    config.viewport = nsh_core::Size::new(px(1200.0), px(900.0));
    config.environment.os = if cfg!(target_os = "windows") {
        nsh_core::platform::Os::Windows
    } else if cfg!(target_os = "linux") {
        nsh_core::platform::Os::Linux
    } else {
        nsh_core::platform::Os::MacOs
    };
    config.environment.idiom = nsh_core::platform::Idiom::Desktop;
    config
}
fn app<T: nsh_core::element::Component>() -> Headless
where
    T::Props: Default,
{
    Headless::start(desktop(Root::of::<T>())).unwrap()
}
#[component]
fn RequiredForm() -> impl IntoElement {
    let count = signal(0);
    form()
        .on_submit(move || count.update(|n| *n += 1))
        .child(
            text_field()
                .accessibility_label("Email")
                .required(true)
                .email(true),
        )
        .child(button().action(ButtonAction::Submit).child(text("Save")))
        .child(button().action(ButtonAction::Reset).child(text("Reset")))
        .child(text(move || format!("Saved: {}", count.get())))
}
#[test]
fn form_validates_before_submitting_and_resets_editors() {
    let mut app = app::<RequiredForm>();
    app.wait_for("Save").unwrap();
    app.press("Save").unwrap();
    app.wait_for("Saved: 0").unwrap();
    app.type_text("label:Email", "invalid").unwrap();
    app.press("Save").unwrap();
    app.wait_for("Saved: 0").unwrap();
    app.clear("label:Email").unwrap();
    app.type_text("label:Email", "ada@example.com").unwrap();
    app.press("Save").unwrap();
    app.wait_for("Saved: 1").unwrap();
    app.press("Reset").unwrap();
    assert_eq!(app.screen().input_text("label:Email").unwrap(), Some(""));
    app.press("Save").unwrap();
    app.wait_for("Saved: 1").unwrap();
}
#[component]
fn NumericAndOtp() -> impl IntoElement {
    let number = signal(1.0);
    let otp = signal(SharedString::default());
    view()
        .gap(px(16.0))
        .child(
            number_field()
                .accessibility_label("Quantity")
                .default_value(1.0)
                .min(0.0)
                .max(2.0)
                .on_change(move |v: &Option<f64>| number.set(v.unwrap_or_default())),
        )
        .child(text(move || format!("Quantity: {}", number.get())))
        .child(
            otp_field()
                .accessibility_label("Verification code")
                .length(6)
                .groups(2)
                .on_complete(move |v: &SharedString| otp.set(v.clone())),
        )
        .child(text(move || format!("Complete: {}", otp.get())))
}
#[test]
fn number_bounds_and_otp_paste_use_native_editors() {
    let mut app = app::<NumericAndOtp>();
    app.wait_for("Quantity: 1").unwrap();
    app.type_text("label:Quantity", "").unwrap();
    app.press_key("up").unwrap();
    app.wait_for("Quantity: 2").unwrap();
    app.type_text("label:Quantity", "").unwrap();
    app.press_key("up").unwrap();
    app.wait_for("Quantity: 2").unwrap();
    let mut paste = nsh_core::PasteEvent::default();
    paste.text = "123456".into();
    app.paste("label:Verification code", &paste).unwrap();
    app.wait_for("Complete: 123456").unwrap();
}
#[component]
fn OverlayControls() -> impl IntoElement {
    view()
        .gap(px(12.0))
        .child(
            dazzui_nsh::dialog()
                .trigger(button().child(text("Open dialog")))
                .child(dialog_header().title("Dialog title"))
                .child(dialog_body().child(text_field().accessibility_label("Dialog editor"))),
        )
        .child(
            popover_panel()
                .title("Popover title")
                .trigger(button().child(text("Open popover")))
                .child(dialog_close().content(button().child(text("Close popover")))),
        )
        .child(
            drawer()
                .title("Drawer title")
                .trigger(button().child(text("Open drawer")))
                .child(dialog_close().content(button().child(text("Close drawer")))),
        )
}
#[test]
fn overlays_open_close_and_restore_focus_without_losing_triggers() {
    let mut app = app::<OverlayControls>();
    app.wait_for("Open dialog").unwrap();
    app.press("Open dialog").unwrap();
    app.wait_for("Dialog title").unwrap();
    app.type_text("label:Dialog editor", "draft").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for_gone("Dialog title", Duration::from_secs(2))
        .unwrap();
    app.press("Open popover").unwrap();
    app.wait_for("Popover title").unwrap();
    app.press("Close popover").unwrap();
    app.wait_for_gone("Popover title", Duration::from_secs(2))
        .unwrap();
    app.press("Open drawer").unwrap();
    app.wait_for("Drawer title").unwrap();
    app.press("Close drawer").unwrap();
    app.wait_for_gone("Drawer title", Duration::from_secs(2))
        .unwrap();
}
#[component]
fn DateEditor() -> impl IntoElement {
    let count = signal(0);
    let value = signal(DateRangeValue::new(
        "2026-10-16T14:30",
        "2026-10-16T15:30",
        false,
    ));
    view()
        .p(px(24.0))
        .child(
            date_range_picker()
                .accessibility_label("When")
                .today("2026-10-16")
                .value(move || Some(value.get()))
                .on_change(move |v: &DateRangeValue| {
                    value.set(v.clone());
                    count.update(|n| *n += 1);
                }),
        )
        .child(text(move || {
            format!(
                "Committed: {} / {} / {}",
                count.get(),
                value.get().start,
                value.get().all_day
            )
        }))
}
#[test]
fn date_editor_commits_only_done_and_discards_dismissed_drafts() {
    let mut app = app::<DateEditor>();
    app.wait_for("label:When").unwrap();
    app.press("label:When").unwrap();
    app.wait_for("Tomorrow").unwrap();
    app.press("Tomorrow").unwrap();
    app.press("All day").unwrap();
    app.press("Cancel").unwrap();
    app.wait_for("Committed: 0 / 2026-10-16T14:30 / false")
        .unwrap();
    app.press("label:When").unwrap();
    app.press("Tomorrow").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for("Committed: 0 / 2026-10-16T14:30 / false")
        .unwrap();
    app.press("label:When").unwrap();
    app.press("Tomorrow").unwrap();
    app.press("All day").unwrap();
    app.press("Done").unwrap();
    app.wait_for("Committed: 1 / 2026-10-17T14:30 / true")
        .unwrap();
}
#[component]
fn ToastActions() -> impl IntoElement {
    let handle = ToastHandle::current().unwrap();
    let count = signal(0);
    view()
        .child(button().child(text("Notify")).on_press(move || {
            let mut notice = ToastNotice::new("Project archived");
            notice.action = Some((
                "Undo".into(),
                std::rc::Rc::new(move || count.update(|n| *n += 1)),
            ));
            handle.push(notice);
        }))
        .child(text(move || format!("Undone: {}", count.get())))
        .child(button().child(text("Temporary notice")).on_press(move || {
            handle.push(ToastNotice::new("Saved to Downloads"));
        }))
        .child(toast_viewport())
}
#[component]
fn ToastHost() -> impl IntoElement {
    toast_provider().child(toast_actions())
}
#[test]
fn toast_action_dismissal_and_timeout_are_live() {
    let mut app = app::<ToastHost>();
    app.wait_for("Notify").unwrap();
    app.press("Notify").unwrap();
    app.wait_for("Project archived").unwrap();
    app.press("Undo").unwrap();
    app.wait_for("Undone: 1").unwrap();
    app.wait_for_gone("Project archived", Duration::from_secs(2))
        .unwrap();
    app.press("Temporary notice").unwrap();
    app.wait_for("Saved to Downloads").unwrap();
    app.wait_for_gone("Saved to Downloads", Duration::from_secs(8))
        .unwrap();
}
#[component]
fn ChoiceControls() -> impl IntoElement {
    let value = signal(SharedString::default());
    let query = signal(SharedString::default());
    view()
        .gap(px(12.0))
        .child(
            combobox()
                .accessibility_label("Language")
                .items(vec![
                    SelectItem::new("rust", "Rust").group("Native"),
                    SelectItem::new("swift", "Swift").group("Native"),
                ])
                .on_change(move |v: &Vec<SharedString>| {
                    value.set(v.first().cloned().unwrap_or_default())
                }),
        )
        .child(text(move || format!("Language: {}", value.get())))
        .child(
            autocomplete()
                .accessibility_label("Custom language")
                .items(vec!["Rust".into(), "Swift".into()])
                .on_change(move |v: &SharedString| query.set(v.clone())),
        )
        .child(text(move || format!("Query: {}", query.get())))
}
#[test]
fn combobox_filters_and_autocomplete_keeps_custom_values() {
    let mut app = app::<ChoiceControls>();
    app.wait_for("label:Language").unwrap();
    app.type_text("label:Language", "rus").unwrap();
    app.wait_for("Rust").unwrap();
    app.press("Rust").unwrap();
    app.wait_for("Language: rust").unwrap();
    app.type_text("label:Custom language", "Elm").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for("Query: Elm").unwrap();
}
#[component]
fn MenuControls() -> impl IntoElement {
    let action = signal(SharedString::default());
    view()
        .child(
            menubar()
                .child(
                    menubar_menu().value("file").label("File").child(
                        menu_item()
                            .child(text("New document"))
                            .on_select(move || action.set("new".into())),
                    ),
                )
                .child(
                    menubar_menu().value("edit").label("Edit").child(
                        menu_item()
                            .child(text("Undo edit"))
                            .on_select(move || action.set("undo".into())),
                    ),
                ),
        )
        .child(text(move || format!("Action: {}", action.get())))
}
#[test]
fn menubar_switches_open_menus_and_dispatches_selection() {
    let mut app = app::<MenuControls>();
    app.wait_for("File").unwrap();
    app.press("File").unwrap();
    app.wait_for("New document").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Undo edit").unwrap();
    app.press("Undo edit").unwrap();
    app.wait_for("Action: undo").unwrap();
}

#[component]
fn StructuralControls() -> impl IntoElement {
    let selected = signal(SharedString::default());
    view()
        .gap(px(16.0))
        .child(
            accordion().multiple(true).child(
                accordion_item()
                    .value("one")
                    .title("First section")
                    .child(text_field().accessibility_label("Retained draft")),
            ),
        )
        .child(
            tree()
                .items(vec![
                    dazzui_nsh::TreeItem::new("src", "Sources")
                        .child(dazzui_nsh::TreeItem::new("main", "main.rs")),
                ])
                .on_selected_change(move |v: &SharedString| selected.set(v.clone())),
        )
        .child(text(move || format!("File: {}", selected.get())))
        .child(
            view().h(px(200.0)).child(
                sidebar_provider()
                    .child(
                        sidebar()
                            .collapsible(dazzui_nsh::SidebarCollapse::Icon)
                            .child(sidebar_header().child(text("Workspace")))
                            .child(
                                sidebar_menu().child(
                                    sidebar_menu_button()
                                        .tooltip("Documents")
                                        .icon(icon("Document20Regular"))
                                        .child(text("Documents")),
                                ),
                            ),
                    )
                    .child(
                        sidebar_inset()
                            .child(sidebar_trigger())
                            .child(text_field().accessibility_label("Main editor")),
                    ),
            ),
        )
}
#[test]
fn accordion_and_sidebar_preserve_editors_and_tree_expands() {
    let mut app = app::<StructuralControls>();
    app.wait_for("First section").unwrap();
    app.press("First section").unwrap();
    app.type_text("label:Retained draft", "keep").unwrap();
    app.press("First section").unwrap();
    app.press("First section").unwrap();
    assert_eq!(
        app.screen().input_text("label:Retained draft").unwrap(),
        Some("keep")
    );
    app.press("Sources").unwrap();
    app.wait_for("main.rs").unwrap();
    app.press("main.rs").unwrap();
    app.wait_for("File: main").unwrap();
    app.type_text("label:Main editor", "still here").unwrap();
    app.press("label:Toggle sidebar").unwrap();
    app.press("label:Toggle sidebar").unwrap();
    assert_eq!(
        app.screen().input_text("label:Main editor").unwrap(),
        Some("still here")
    );
}
#[component]
fn Recording() -> impl IntoElement {
    let value = signal(SharedString::default());
    shortcut_recorder()
        .value(value)
        .accessibility_label("Shortcut")
        .on_change(move |v: &SharedString| value.set(v.clone()))
        .on_clear(move || value.set(SharedString::default()))
}
#[test]
fn shortcut_recording_cancel_and_clear_are_distinct() {
    let mut app = app::<Recording>();
    app.wait_for("label:Shortcut").unwrap();
    app.press("label:Shortcut").unwrap();
    app.press_key("ctrl+alt+k").unwrap();
    app.wait_for("⌃⌥K").unwrap();
    app.press("label:Shortcut").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for("⌃⌥K").unwrap();
    app.press("label:Clear shortcut").unwrap();
    app.wait_for("Record shortcut").unwrap();
}
#[component]
fn Panes() -> impl IntoElement {
    let width = signal(0.0);
    view()
        .w(px(600.0))
        .h(px(240.0))
        .child(
            splitter()
                .child(
                    splitter_panel()
                        .default_size(35.0)
                        .min_size(20.0)
                        .max_size(60.0)
                        .child(
                            view()
                                .w_full()
                                .h_full()
                                .label("Left pane")
                                .on_layout(move |b: &nsh_core::Bounds<nsh_core::Points>| {
                                    width.set(b.size.width.0)
                                })
                                .child(text("List")),
                        ),
                )
                .child(
                    splitter_panel().default_size(65.0).min_size(40.0).child(
                        view()
                            .w_full()
                            .h_full()
                            .label("Right pane")
                            .child(text("Detail")),
                    ),
                ),
        )
        .child(text(move || format!("Left width: {}", width.get())))
}
#[test]
fn splitter_drag_respects_panel_bounds() {
    let mut app = app::<Panes>();
    app.wait_for("List").unwrap();
    let before = app
        .screen()
        .texts()
        .iter()
        .find_map(|t| {
            t.strip_prefix("Left width: ")
                .and_then(|s| s.parse::<f32>().ok())
        })
        .unwrap();
    app.drag(
        "label:Resize panes",
        nsh_core::Point::new(px(600.0), px(0.0)),
    )
    .unwrap();
    let after = app
        .screen()
        .texts()
        .iter()
        .find_map(|t| {
            t.strip_prefix("Left width: ")
                .and_then(|s| s.parse::<f32>().ok())
        })
        .unwrap();
    assert!(after > before, "{before} -> {after}");
    assert!(after <= 360.0, "max is 60%: {after}");
}

#[component]
fn BoundedCalendar() -> impl IntoElement {
    let selected = signal(SharedString::default());
    view()
        .child(
            calendar()
                .default_value(dazzui_nsh::CalendarDate::Single(dazzui_nsh::parse_date(
                    "2026-10-16",
                )))
                .today("2026-10-16")
                .min("2026-10-12")
                .max("2026-10-23")
                .on_change(move |v: &dazzui_nsh::CalendarDate| {
                    selected.set(v.start().unwrap().to_string().into())
                }),
        )
        .child(text(move || format!("Day: {}", selected.get())))
}
#[test]
fn calendar_enforces_date_bounds() {
    let mut app = app::<BoundedCalendar>();
    app.wait_for("label:2026-10-16").unwrap();
    assert!(!app.screen().enabled("label:2026-10-11").unwrap());
    assert!(!app.screen().enabled("label:2026-10-24").unwrap());
    app.press("label:2026-10-22").unwrap();
    app.wait_for("Day: 2026-10-22").unwrap();
}

#[component]
fn RangeCalendar() -> impl IntoElement {
    let value = signal(dazzui_nsh::CalendarDate::Range(
        dazzui_nsh::parse_date("2026-10-16"),
        dazzui_nsh::parse_date("2026-10-20"),
    ));
    view()
        .child(
            calendar()
                .range(true)
                .value(move || value.get())
                .on_change(move |v: &dazzui_nsh::CalendarDate| value.set(*v)),
        )
        .child(text(move || {
            format!(
                "Range: {} / {}",
                value
                    .get()
                    .start()
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
                value.get().end().map(|d| d.to_string()).unwrap_or_default()
            )
        }))
}
#[test]
fn calendar_reports_pending_range_and_accepts_reverse_order() {
    let mut app = app::<RangeCalendar>();
    app.wait_for("label:2026-10-22").unwrap();
    app.press("label:2026-10-22").unwrap();
    app.wait_for("Range: 2026-10-22 / ").unwrap();
    app.hover("label:2026-10-18").unwrap();
    app.wait_for("Range: 2026-10-22 / ").unwrap();
    app.press("label:2026-10-18").unwrap();
    app.wait_for("Range: 2026-10-18 / 2026-10-22").unwrap();
}

#[component]
fn NavigationChoices() -> impl IntoElement {
    let tab = signal(SharedString::from("a"));
    let pressed = signal(Vec::<SharedString>::new());
    view()
        .child(
            pill_tabs()
                .items(vec![
                    SelectItem::new("a", "Alpha"),
                    SelectItem::new("b", "Blocked").disabled(true),
                    SelectItem::new("c", "Charlie"),
                ])
                .value(move || Some(tab.get()))
                .on_change(move |v: &SharedString| tab.set(v.clone())),
        )
        .child(text(move || format!("Tab: {}", tab.get())))
        .child(
            toggle_group()
                .items(vec![
                    SelectItem::new("bold", "Bold"),
                    SelectItem::new("italic", "Italic"),
                ])
                .value(move || Some(pressed.get()))
                .on_change(move |v: &Vec<SharedString>| pressed.set(v.clone())),
        )
        .child(text(move || {
            format!(
                "Toggles: {}",
                pressed
                    .get()
                    .iter()
                    .map(|v| v.as_ref())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }))
}
#[test]
fn toggle_group_defaults_to_independent_multiple_selections() {
    let mut app = app::<NavigationChoices>();
    app.wait_for("Bold").unwrap();
    app.press("Bold").unwrap();
    app.press("Italic").unwrap();
    app.wait_for("Toggles: bold,italic").unwrap();
    app.press("Bold").unwrap();
    app.wait_for("Toggles: italic").unwrap();
}
#[test]
fn macos_headless_pill_tabs_skip_disabled_and_activate_with_arrow_keys() {
    let mut config = desktop(Root::of::<NavigationChoices>());
    config.environment.os = nsh_core::platform::Os::MacOs;
    let mut app = Headless::start(config).unwrap();
    app.wait_for("Alpha").unwrap();
    app.press("Alpha").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Tab: c").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Tab: a").unwrap();
    app.press_key("end").unwrap();
    app.wait_for("Tab: c").unwrap();
    assert!(!app.screen().enabled("Blocked").unwrap());
}

#[test]
fn calendar_keyboard_navigation_keeps_bounds_and_announces_selection() {
    let mut app = app::<BoundedCalendar>();
    app.wait_for("label:2026-10-16").unwrap();
    app.press("label:2026-10-16").unwrap();
    app.press_key("right").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Day: 2026-10-17").unwrap();
    assert!(app.screen().selected("label:2026-10-17").unwrap());
    app.press_key("end").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Day: 2026-10-18").unwrap();
    app.press_key("pagedown").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Day: 2026-10-23").unwrap();
    app.press_key("right").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Day: 2026-10-23").unwrap();
}

#[test]
fn calendar_keyboard_crosses_months_without_losing_focus_or_pending_range() {
    let mut app = app::<RangeCalendar>();
    app.wait_for("label:2026-10-22").unwrap();
    app.press("label:2026-10-22").unwrap();
    app.press_key("pagedown").unwrap();
    app.press_key("enter").unwrap();
    app.wait_for("Range: 2026-10-22 / 2026-11-22").unwrap();
    assert!(app.screen().selected("label:2026-11-15").unwrap());
}
