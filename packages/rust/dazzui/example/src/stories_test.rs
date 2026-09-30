//! Every story's preview and every example, drawn in each of the twelve
//! drawn themes — the Flutter example's `stories_test.dart`. A component a
//! story uses that nothing maps fails here by name, before anyone sees a
//! gap in the window.

use dazzui::Theme;
use gpui::TestAppContext;
use serde_json::Value;

use crate::lib::mapping;
use crate::lib::scene::SceneView;
use crate::lib::story::stories;
use crate::lib::themes::THEME_FAMILY_GROUPS;

fn components(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Array(items) => items.iter().for_each(|item| components(item, found)),
        Value::Object(object) => {
            if let Some(name) = object.get("$el").and_then(Value::as_str) {
                found.push(name.to_string());
            }
            object.values().for_each(|item| components(item, found));
        }
        _ => {}
    }
}

#[test]
fn every_component_a_story_uses_is_mapped() {
    let mut missing = Vec::new();
    for story in stories() {
        let mut found = Vec::new();
        components(&story.preview.content, &mut found);
        for example in &story.examples {
            components(&example.demo.content, &mut found);
        }
        for name in found {
            if !mapping::maps(&name) && !missing.contains(&name) {
                missing.push(name);
            }
        }
    }
    assert!(missing.is_empty(), "no GPUI mapping for {missing:?}");
}

#[gpui::test]
fn every_scene_draws_in_every_drawn_theme(cx: &mut TestAppContext) {
    cx.update(|cx| {
        dazzui::init(cx);
    });
    let themes: Vec<String> = THEME_FAMILY_GROUPS[0]
        .families
        .iter()
        .flat_map(|family| {
            [
                format!("{}-light", family.value),
                format!("{}-dark", family.value),
            ]
        })
        .collect();
    for theme in &themes {
        cx.update(|cx| Theme::set(theme, cx));
        for story in stories() {
            let scenes = std::iter::once(&story.preview)
                .chain(story.examples.iter().map(|example| &example.demo));
            for scene in scenes {
                let (_, window) = cx.add_window_view(|_, _| SceneView::of(scene));
                window.run_until_parked();
            }
        }
    }
}
