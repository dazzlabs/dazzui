pub mod gallery;
pub mod stage;
use crate::Demo;
use inset::{App, WidgetRef};
pub struct StoryPage {
    pub name: &'static str,
    pub description: &'static str,
    pub render: fn(&mut App, &Demo) -> WidgetRef,
}
pub fn story_id(page: &str, story: &str) -> String {
    fn sanitize(s: &str) -> String {
        let mut out = String::new();
        for c in s.to_lowercase().chars() {
            if c.is_ascii_alphanumeric() {
                out.push(c);
            } else if !out.ends_with('-') && !out.is_empty() {
                out.push('-');
            }
        }
        out.trim_end_matches('-').into()
    }
    format!("components-{}--{}", sanitize(page), sanitize(story))
}
