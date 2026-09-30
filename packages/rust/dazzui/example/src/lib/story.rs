//! A story, as the window reads it: the React example's `Story`, compiled
//! into data by codegen and parsed here once.

use std::sync::OnceLock;

use serde_json::{Map, Value};

use crate::stories::SOURCES;

/// One scene: the slots its `useState`s start at, and the tree it draws.
#[derive(Debug)]
pub struct Scene {
    /// Unique across every story: `button/preview`, `button/2`.
    pub key: String,
    pub state: Map<String, Value>,
    pub content: Value,
}

#[derive(Debug)]
pub struct Example {
    pub title: String,
    pub note: Option<String>,
    pub demo: Scene,
}

/// A component, as the shell reads it. `preview` is the one scene that
/// stands for it on the overview; `examples` are what its page shows.
#[derive(Debug)]
pub struct Story {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub preview: Scene,
    pub examples: Vec<Example>,
}

fn scene(key: String, value: &Value) -> Scene {
    Scene {
        key,
        state: value["state"].as_object().cloned().unwrap_or_default(),
        content: value["content"].clone(),
    }
}

fn parse(source: &str) -> Story {
    let json: Value = serde_json::from_str(source).expect("a story codegen wrote");
    let id = json["id"].as_str().unwrap_or_default().to_string();
    let text = |value: &Value| value.as_str().unwrap_or_default().to_string();
    Story {
        preview: scene(format!("{id}/preview"), &json["preview"]),
        examples: json["examples"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(index, example)| Example {
                title: text(&example["title"]),
                note: example["note"].as_str().map(str::to_string),
                demo: scene(format!("{id}/{index}"), &example["demo"]),
            })
            .collect(),
        name: text(&json["name"]),
        summary: text(&json["summary"]),
        id,
    }
}

/// Every story, in the order the React example lists them.
pub fn stories() -> &'static [Story] {
    static STORIES: OnceLock<Vec<Story>> = OnceLock::new();
    STORIES.get_or_init(|| SOURCES.iter().map(|source| parse(source)).collect())
}

pub fn story(id: &str) -> Option<&'static Story> {
    stories().iter().find(|story| story.id == id)
}

/// The stories a query leaves, matched on name and summary.
pub fn matching(query: &str) -> Vec<&'static Story> {
    let needle = query.trim().to_lowercase();
    stories()
        .iter()
        .filter(|story| {
            needle.is_empty()
                || story.name.to_lowercase().contains(&needle)
                || story.summary.to_lowercase().contains(&needle)
        })
        .collect()
}
