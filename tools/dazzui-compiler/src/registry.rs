use crate::ir::{ChildrenSpec, ComponentSpec, EnumSpec, EventSpec, PropSpec, PropType, Value};

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
        // SwiftUI spaces a stack by about 8 points of its own accord and
        // WinUI by none: the gap is the registry's to decide, not inherited
        // from whichever platform draws it.
        let spacing = PropSpec { default: Some(Value::Int(0)), ..prop("spacing", PropType::Int) };
        let disabled =
            PropSpec { default: Some(Value::Bool(false)), modifier: true, ..prop("disabled", PropType::Bool) };
        Self {
            components: vec![
                ComponentSpec {
                    name: "VStack".into(),
                    props: vec![spacing.clone()],
                    positional: vec![],
                    events: vec![],
                    children: ChildrenSpec::Many,
                },
                ComponentSpec {
                    name: "HStack".into(),
                    props: vec![spacing],
                    positional: vec![],
                    events: vec![],
                    children: ChildrenSpec::Many,
                },
                ComponentSpec {
                    name: "Text".into(),
                    props: vec![
                        PropSpec { required: true, ..prop("text", PropType::String) },
                        // No default: an unstyled text is the platform's own
                        // body text, which a style would only restate.
                        PropSpec { modifier: true, ..prop("style", enumeration("TextStyle", &["Title", "Body", "Caption"])) },
                    ],
                    positional: vec!["text".into()],
                    events: vec![],
                    children: ChildrenSpec::None,
                },
                ComponentSpec {
                    name: "Button".into(),
                    props: vec![
                        PropSpec { required: true, ..prop("title", PropType::String) },
                        PropSpec {
                            default: Some(Value::String("Secondary".into())),
                            modifier: true,
                            ..prop("style", enumeration("ButtonStyle", &["Primary", "Secondary"]))
                        },
                        disabled.clone(),
                    ],
                    positional: vec!["title".into()],
                    events: vec![EventSpec {
                        name: "click".into(),
                        modifier: "on_click".into(),
                        payload: None,
                        binds: None,
                    }],
                    children: ChildrenSpec::None,
                },
                ComponentSpec {
                    name: "Toggle".into(),
                    props: vec![PropSpec { required: true, bindable: true, ..prop("value", PropType::Bool) }, disabled],
                    positional: vec![],
                    events: vec![EventSpec {
                        name: "change".into(),
                        modifier: "on_change".into(),
                        payload: Some(PropType::Bool),
                        binds: Some("value".into()),
                    }],
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

/// An optional prop with no default, no modifier form and no binding;
/// each spec above spells out how it differs.
fn prop(name: &str, ty: PropType) -> PropSpec {
    PropSpec { name: name.into(), ty, required: false, default: None, modifier: false, bindable: false }
}

fn enumeration(name: &str, cases: &[&str]) -> PropType {
    PropType::Enum(EnumSpec { name: name.into(), cases: cases.iter().map(|c| (*c).into()).collect() })
}
