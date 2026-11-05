use dazzui_nsh::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

#[component]
fn Controls() -> impl IntoElement {
    let theme = Theme::new("bright-light");
    provide_context(theme);
    let count = signal(0);
    let checked = signal(false);
    let value = signal(SharedString::from("draft"));
    view()
        .p(px(20.0))
        .gap(px(16.0))
        .child(
            button()
                .child(text("Add"))
                .on_press(move || count.update(|n| *n += 1)),
        )
        .child(
            button()
                .disabled(true)
                .child(text("Disabled"))
                .on_press(move || count.update(|n| *n += 100)),
        )
        .child(text(move || format!("Count: {}", count.get())))
        .child(
            checkbox()
                .accessibility_label("Accept")
                .checked(move || Some(checked.get()))
                .on_change(move |v: &bool| checked.set(*v))
                .child(text("Accept")),
        )
        .child(text(move || format!("Accepted: {}", checked.get())))
        .child(switch().accessibility_label("Notifications"))
        .child(
            text_field()
                .accessibility_label("Draft")
                .value(move || Some(value.get()))
                .on_change(move |v: &SharedString| value.set(v.clone())),
        )
        .child(text(move || format!("Draft: {}", value.get())))
        .child(button().child(text("Dark theme")).on_press(move || {
            theme.set("studio-dark");
        }))
        .child(button().child(text("Light theme")).on_press(move || {
            theme.set("bright-light");
        }))
        .child(text(move || format!("Theme: {}", theme.name())))
}
#[test]
fn native_controls_keep_state_when_the_theme_changes() {
    let mut app = Headless::start(desktop(Root::of::<Controls>())).unwrap();
    app.wait_for("Count: 0").unwrap();
    app.press("Add").unwrap();
    app.wait_for("Count: 1").unwrap();
    assert!(!app.screen().enabled("Disabled").unwrap());
    app.press("label:Accept").unwrap();
    app.wait_for("Accepted: true").unwrap();
    app.press("label:Notifications").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("label:Notifications")
            .unwrap()
            .checked,
        Some(nsh_core::Checked::True)
    );
    app.type_text("label:Draft", "edited").unwrap();
    let before = app
        .screen()
        .input_text("label:Draft")
        .unwrap()
        .unwrap()
        .to_owned();
    let id = app.screen().press_target("label:Accept").unwrap();
    app.press("Dark theme").unwrap();
    app.wait_for("Theme: studio-dark").unwrap();
    assert_eq!(
        app.screen().input_text("label:Draft").unwrap(),
        Some(before.as_str())
    );
    assert_eq!(app.screen().press_target("label:Accept").unwrap(), id);
    app.wait_for("Accepted: true").unwrap();
    app.press("Light theme").unwrap();
    app.wait_for("Theme: bright-light").unwrap();
    app.wait_for("Count: 1").unwrap();
}

#[component]
fn SelectControl() -> impl IntoElement {
    let value = signal(None::<SharedString>);
    view()
        .p(px(20.0))
        .child(
            select()
                .accessibility_label("Language")
                .placeholder("Choose language")
                .items(vec![
                    dazzui_nsh::SelectItem::new("en", "English"),
                    dazzui_nsh::SelectItem::new("zh", "中文"),
                    dazzui_nsh::SelectItem::new("xx", "Unavailable").disabled(true),
                ])
                .on_change(move |v: &Option<SharedString>| value.set(v.clone())),
        )
        .child(text(move || {
            format!("Selected: {}", value.get().unwrap_or_default())
        }))
}
#[test]
fn native_select_opens_chooses_and_dismisses() {
    let mut app = Headless::start(desktop(Root::of::<SelectControl>())).unwrap();
    app.wait_for("Choose language").unwrap();
    app.press("label:Language").unwrap();
    app.wait_for("English").unwrap();
    app.press("中文").unwrap();
    app.wait_for("Selected: zh").unwrap();
    app.press("label:Language").unwrap();
    app.press_key("escape").unwrap();
    app.wait_for("Selected: zh").unwrap();
}

fn desktop(root: Root) -> EngineConfig {
    let mut config = EngineConfig::new(root);
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

#[component]
fn KeyboardControls() -> impl IntoElement {
    let selected = signal(SharedString::from("one"));
    let selected_segment = signal(SharedString::from("one"));
    let count = signal(0);
    let choices = vec![
        dazzui_nsh::SelectItem::new("one", "First"),
        dazzui_nsh::SelectItem::new("skip", "Skipped").disabled(true),
        dazzui_nsh::SelectItem::new("two", "Last"),
    ];
    view()
        .gap(px(12.0))
        .child(
            radio_group()
                .options(choices.clone())
                .value(move || Some(selected.get()))
                .on_change(move |v: &SharedString| selected.set(v.clone())),
        )
        .child(text(move || format!("Radio: {}", selected.get())))
        .child(
            segmented_control()
                .items(choices)
                .value(move || Some(selected_segment.get()))
                .on_change(move |v: &SharedString| selected_segment.set(v.clone())),
        )
        .child(text(move || format!("Segment: {}", selected_segment.get())))
        .child(
            checkbox()
                .accessibility_label("Locked choice")
                .checked(false)
                .child(text("Locked choice")),
        )
        .child(
            form_field()
                .label("Project name")
                .hint("Shown in the sidebar")
                .child(text_field().default_value("Draft")),
        )
        .child(
            menu()
                .trigger(button().child(text("Actions")))
                .child(
                    menu_item()
                        .on_select(move || count.update(|n| *n += 1))
                        .child(text("Rename")),
                )
                .child(menu_item().disabled(true).child(text("Delete"))),
        )
        .child(text(move || format!("Actions: {}", count.get())))
}
#[test]
fn macos_headless_keyboard_navigation_skips_disabled_choices() {
    let mut app = Headless::start(keyboard_desktop(Root::of::<KeyboardControls>())).unwrap();
    app.wait_for("Radio: one").unwrap();
    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Radio: two").unwrap();
    app.press_key("tab").unwrap();
    app.press_key("right").unwrap();
    app.wait_for("Segment: two").unwrap();
}

#[test]
fn controlled_choices_and_menus_keep_native_behavior() {
    let mut app = Headless::start(desktop(Root::of::<KeyboardControls>())).unwrap();
    app.wait_for("Radio: one").unwrap();
    app.press("label:Locked choice").unwrap();
    assert_eq!(
        app.screen()
            .accessibility_state("label:Locked choice")
            .unwrap()
            .checked,
        Some(nsh_core::Checked::False)
    );
    assert_eq!(
        app.screen().input_text("label:Project name").unwrap(),
        Some("Draft")
    );
    app.press("Actions").unwrap();
    app.wait_for("Rename").unwrap();
    assert!(!app.screen().enabled("Delete").unwrap());
    app.press("Rename").unwrap();
    app.wait_for("Actions: 1").unwrap();
    assert!(!app.screen().matches("Rename").unwrap());
    app.press("Actions").unwrap();
    app.press_key("escape").unwrap();
    assert!(!app.screen().matches("Rename").unwrap());
}

#[component]
fn ThemeLabel() -> impl IntoElement {
    let theme = Theme::current();
    text(move || theme.name()).text_color(move || theme.tokens().color_content.hsla())
}
#[component]
fn ScopedThemes() -> impl IntoElement {
    let theme = Theme::new("bright-light");
    provide_context(theme);
    view()
        .child(theme_label())
        .child(theme_scope().name("studio-dark").child(theme_label()))
        .child(button().child(text("Unknown theme")).on_press(move || {
            assert!(!theme.set("missing"));
        }))
}
#[test]
fn nested_theme_does_not_replace_the_parent_and_unknown_names_are_rejected() {
    let mut app = Headless::start(desktop(Root::of::<ScopedThemes>())).unwrap();
    app.wait_for("studio-dark").unwrap();
    app.wait_for("bright-light").unwrap();
    app.press("Unknown theme").unwrap();
    app.wait_for("bright-light").unwrap();
    assert_ne!(
        app.screen().span_style("studio-dark").unwrap().color,
        app.screen().span_style("bright-light").unwrap().color
    );
}

#[component]
fn LabelledSelect() -> impl IntoElement {
    form_field()
        .label("Language")
        .child(select().default_value("en").items(vec![
            dazzui_nsh::SelectItem::new("en", "English"),
            dazzui_nsh::SelectItem::new("zh", "Chinese"),
        ]))
}
#[test]
fn macos_headless_form_label_focuses_the_select_trigger() {
    let mut app = Headless::start(keyboard_desktop(Root::of::<LabelledSelect>())).unwrap();
    app.wait_for("Language").unwrap();
    app.press("text:Language").unwrap();
    app.press_key("space").unwrap();
    app.wait_for("Chinese").unwrap();
}

fn keyboard_desktop(root: Root) -> EngineConfig {
    // nsh-testing 0.4.0 headless/keyboard.rs enables view focus only for
    // MacOs and Web. Exercise that explicit simulation on every build host;
    // these two tests do not verify Windows/Linux native keyboard behavior.
    let mut config = desktop(root);
    config.environment.os = nsh_core::platform::Os::MacOs;
    config
}
