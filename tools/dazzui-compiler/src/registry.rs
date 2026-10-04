use crate::ir::{ChildrenSpec, ComponentSpec, EventSpec, PropSpec, PropType, Value};

/// The components a page may use, by semantic name.
///
/// Phase 1 carries the built-in set in code. Phase 4 loads the same specs
/// from TOML — the built-ins and an app's own `dazzui-compiler.toml` — without the
/// compiler changing.
#[derive(Clone, Debug)]
pub struct Registry {
    components: Vec<ComponentSpec>,
}

impl Registry {
    pub fn builtin() -> Self {
        Self {
            components: vec![
                ComponentSpec {
                    name: "VStack".into(),
                    // SwiftUI spaces a stack by about 8 points of its own accord and
                    // WinUI by none: the gap is the registry's to decide, not
                    // inherited from whichever platform draws it.
                    props: vec![defaulted("spacing", PropType::Int, Value::Int(0))],
                    positional: vec![],
                    events: vec![],
                    children: ChildrenSpec::Many,
                },
                ComponentSpec {
                    name: "Text".into(),
                    props: vec![required("text", PropType::String)],
                    positional: vec!["text".into()],
                    events: vec![],
                    children: ChildrenSpec::None,
                },
                ComponentSpec {
                    name: "Button".into(),
                    props: vec![required("title", PropType::String)],
                    positional: vec!["title".into()],
                    events: vec![EventSpec { name: "click".into(), modifier: "on_click".into() }],
                    children: ChildrenSpec::None,
                },
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&ComponentSpec> {
        self.components.iter().find(|c| c.name == name)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|c| c.name.as_str())
    }
}

fn required(name: &str, ty: PropType) -> PropSpec {
    PropSpec { name: name.into(), ty, required: true, default: None }
}

fn defaulted(name: &str, ty: PropType, default: Value) -> PropSpec {
    PropSpec { name: name.into(), ty, required: false, default: Some(default) }
}
