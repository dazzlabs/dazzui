use dazzui_nsh::prelude::*;
use dazzui_nsh_example::{
    runtime::scene::{Scene, Story},
    stories::STORIES,
};
use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

#[derive(Clone, PartialEq, Props)]
struct SceneHostProps {
    scene: Prop<SharedString>,
    theme: Prop<SharedString>,
}
#[component]
fn SceneHost(p: SceneHostProps) -> impl IntoElement {
    let theme = Theme::new(&p.theme.get());
    provide_context(theme);
    let scene: Scene = serde_json::from_str(&p.scene.get()).unwrap();
    view()
        .p(px(24.0))
        .bg(theme.tokens().color_surface.hsla())
        .child(scene.render())
        .child(text("Scene ready"))
}
#[test]
fn every_supported_scene_renders_in_light_and_dark() {
    for story in STORIES {
        let raw: serde_json::Value = serde_json::from_str(story).unwrap();
        for scene in std::iter::once(&raw["preview"]).chain(raw["examples"].as_array().unwrap()) {
            if !scene["gaps"].as_array().unwrap().is_empty() {
                continue;
            }
            for name in ["bright-light", "studio-dark"] {
                let json = scene.to_string();
                let mut app = Headless::start(EngineConfig {
                    viewport: nsh_core::Size::new(px(640.0), px(640.0)),
                    ..desktop(Root::new::<SceneHost>(move || SceneHostProps {
                        scene: json.clone().into(),
                        theme: name.into(),
                    }))
                })
                .unwrap();
                app.wait_for("Scene ready")
                    .unwrap_or_else(|e| panic!("{} / {name}: {e}", scene["id"]));
            }
        }
    }
}

#[component]
fn SceneSequence() -> impl IntoElement {
    let scenes = std::rc::Rc::new(
        STORIES
            .iter()
            .map(|s| serde_json::from_str::<Story>(s).unwrap())
            .flat_map(|story| std::iter::once(story.preview).chain(story.examples))
            .collect::<Vec<_>>(),
    );
    let current = signal(0usize);
    let total = scenes.len();
    view()
        .child(
            button()
                .child(text("Next scene"))
                .on_press(move || current.update(|n| *n = (*n + 1) % total)),
        )
        .child(text(move || format!("Scene {}", current.get())))
        .child(dynamic(move || scenes[current.get()].render()))
}
#[test]
fn changing_scenes_disposes_old_providers_and_motion() {
    let mut app = Headless::start(desktop(Root::of::<SceneSequence>())).unwrap();
    let count = STORIES
        .iter()
        .map(|s| serde_json::from_str::<Story>(s).unwrap().examples.len() + 1)
        .sum::<usize>();
    for i in 0..count {
        app.wait_for(&format!("Scene {i}")).unwrap();
        app.wait(std::time::Duration::from_millis(160)).unwrap();
        app.press("Next scene").unwrap();
    }
    app.wait_for("Scene 0").unwrap();
}
#[test]
fn generated_registry_and_coverage_agree() {
    let coverage: serde_json::Value =
        serde_json::from_str(include_str!("../coverage.json")).unwrap();
    let stories: Vec<Story> = STORIES
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let scenes: Vec<_> = stories
        .iter()
        .flat_map(|s| std::iter::once(&s.preview).chain(s.examples.iter()))
        .collect();
    assert_eq!(scenes.len(), coverage.as_array().unwrap().len());
    for (scene, row) in scenes.iter().zip(coverage.as_array().unwrap()) {
        assert_eq!(scene.id, row["id"]);
        assert_eq!(scene.gaps.is_empty(), row["supported"].as_bool().unwrap());
        assert!(scene.gaps.is_empty(), "{}: {:?}", scene.id, scene.gaps);
    }
}

#[test]
fn mapped_overlays_and_toast_actions_reach_their_native_owners() {
    for (page, trigger, visible, close) in [
        ("dialog", "Rename…", "Rename project", "Cancel"),
        ("popover", "Share", "Share this project", "Done"),
        ("drawer", "Open filters", "Filters", "Done"),
        (
            "toast",
            "Save",
            "Saved to Downloads",
            "label:Dismiss notification",
        ),
    ] {
        let raw: serde_json::Value = STORIES
            .iter()
            .map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap())
            .find(|s| s["id"] == page)
            .unwrap();
        let json = raw["preview"].to_string();
        let mut app = Headless::start(desktop(Root::new::<SceneHost>(move || SceneHostProps {
            scene: json.clone().into(),
            theme: "bright-light".into(),
        })))
        .unwrap();
        app.wait_for(trigger).unwrap();
        app.press(trigger).unwrap();
        app.wait_for(visible)
            .unwrap_or_else(|e| panic!("{page}: {e}"));
        app.press(close).unwrap();
        app.wait_for_gone(visible, std::time::Duration::from_secs(2))
            .unwrap_or_else(|e| panic!("{page}: {e}"));
    }
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

#[test]
fn every_theme_renders_button_profiles_and_keeps_specimen_ids() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    let story: serde_json::Value = serde_json::from_str(
        STORIES
            .iter()
            .find(|s| serde_json::from_str::<serde_json::Value>(s).unwrap()["id"] == "button")
            .unwrap(),
    )
    .unwrap();
    let scene = story["examples"][2].clone();
    for theme in Theme::all() {
        let json = scene.to_string();
        let name = theme.name;
        let mut app = Headless::start(desktop(Root::new::<SceneHost>(move || SceneHostProps {
            scene: json.clone().into(),
            theme: name.into(),
        })))
        .unwrap();
        let frames = Arc::new(Mutex::new(HashMap::new()));
        let ids = Arc::new(Mutex::new(HashMap::new()));
        let seen_frames = frames.clone();
        let seen_ids = ids.clone();
        app.observe(move |_, batch| {
            for item in &batch.items {
                match item {
                    nsh_core::MountItem::Frame { id, frame } => {
                        seen_frames.lock().unwrap().insert(*id, *frame);
                    }
                    nsh_core::MountItem::ViewProps { id, props } => {
                        if let Some(address) = &props.test_id {
                            seen_ids.lock().unwrap().insert(address.to_string(), *id);
                        }
                    }
                    _ => {}
                }
            }
        });
        app.wait_for("Scene ready").unwrap();
        let frames = frames.lock().unwrap();
        let ids = ids.lock().unwrap();
        for (index, size) in [
            dazzui_nsh::Size::Tiny,
            dazzui_nsh::Size::Small,
            dazzui_nsh::Size::Medium,
            dazzui_nsh::Size::Large,
        ]
        .iter()
        .enumerate()
        {
            let address = format!("button/2/{index}");
            let id = ids
                .get(&address)
                .unwrap_or_else(|| panic!("missing {address} in {name}"));
            let height = f32::from(frames[id].size.height);
            assert!(
                (height - size.profile(theme.tokens).size).abs() <= 1.0,
                "{name} {address}: height {height}"
            );
        }
    }
}

#[test]
fn mapped_menu_opens_through_its_addressed_trigger() {
    let story: serde_json::Value = serde_json::from_str(
        STORIES
            .iter()
            .find(|s| serde_json::from_str::<serde_json::Value>(s).unwrap()["id"] == "menu")
            .unwrap(),
    )
    .unwrap();
    let json = story["examples"][0].to_string();
    let mut app = Headless::start(desktop(Root::new::<SceneHost>(move || SceneHostProps {
        scene: json.clone().into(),
        theme: "studio-dark".into(),
    })))
    .unwrap();
    app.wait_for("Actions").unwrap();
    app.press("Actions").unwrap();
    app.wait_for("Rename").unwrap();
    app.press_key("escape").unwrap();
    assert!(!app.screen().matches("Rename").unwrap());
}

#[test]
fn text_keeps_nested_icons_and_their_addresses() {
    let story: serde_json::Value = serde_json::from_str(
        STORIES
            .iter()
            .find(|s| serde_json::from_str::<serde_json::Value>(s).unwrap()["id"] == "icon")
            .unwrap(),
    )
    .unwrap();
    let json = story["examples"][2].to_string();
    let mut app = Headless::start(desktop(Root::new::<SceneHost>(move || SceneHostProps {
        scene: json.clone().into(),
        theme: "bright-light".into(),
    })))
    .unwrap();
    app.wait_for("Scene ready").unwrap();
    for index in 0..4 {
        assert_eq!(
            app.screen().count(&format!("id:icon/2/{index}")).unwrap(),
            1
        );
    }
    assert!(!app.screen().texts().iter().any(|text| text.contains("$el")));
}
