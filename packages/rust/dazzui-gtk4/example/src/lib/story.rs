use serde::Deserialize;
use serde_json::Value;

/// Same document model as React: Preview belongs to Overview, not examples.
#[derive(Debug, Deserialize)]
pub struct Story {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub preview: Scene,
    pub examples: Vec<Scene>,
}
#[derive(Debug, Deserialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub note: Option<String>,
    pub content: Value,
    #[serde(default)]
    pub state: Value,
    pub gaps: Vec<String>,
}
impl Scene {
    pub fn is_supported(&self) -> bool {
        self.gaps.is_empty()
    }
}
impl Story {
    pub fn scenes(&self) -> impl Iterator<Item = &Scene> {
        std::iter::once(&self.preview).chain(self.examples.iter())
    }
}
