//! The built-in registry speaks React's language: every component, prop
//! and event it declares — or lists as not drawn yet — is a name the React
//! kit or the React example's stage has, in snake case, and every enum's
//! cases are a React union's values, in PascalCase. Read from the `.tsx`
//! sources themselves, so a name made up here fails, and so does a React
//! rename the registry has not followed.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use dazzui_compiler::Registry;
use dazzui_compiler::ir::{ComponentSpec, PropType, pascal_case, snake_case};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &str) -> String {
    let path = repo().join(path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Every `name:` and `name?:` a source declares: a superset of its props,
/// which is all a check of the registry's names against it needs.
fn declared(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let bytes = source.as_bytes();
    for (i, _) in source.match_indices(':') {
        let mut end = i;
        if end > 0 && bytes[end - 1] == b'?' {
            end -= 1;
        }
        let start = source[..end]
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |p| p + 1);
        if start < end {
            names.insert(source[start..end].to_string());
        }
    }
    names
}

/// The quoted values of the union or array that starts at `marker`.
fn union(source: &str, marker: &str) -> Vec<String> {
    let start = source
        .find(marker)
        .unwrap_or_else(|| panic!("no `{marker}`"))
        + marker.len();
    let rest = &source[start..];
    // A union on one line ends with it; one over several, at a blank line.
    let line = rest.find('\n').unwrap_or(rest.len());
    let end = if rest[..line].contains('\'') {
        line
    } else {
        rest.find("\n\n").unwrap_or(rest.len())
    };
    rest[..end]
        .split('\'')
        .skip(1)
        .step_by(2)
        .map(|v| pascal_case(&v.replace('-', "_")))
        .collect()
}

/// Where each component the registry draws is written in React, and the
/// props it takes from what it extends rather than declares.
fn sources() -> Vec<(&'static str, &'static str, &'static [&'static str])> {
    const BUTTON: &[&str] = &["disabled", "onClick", "type"];
    const SWITCH: &[&str] = &["checked", "defaultChecked", "onCheckedChange", "disabled"];
    const STAGE: &str = "packages/js/dazzui/example/src/components/stage.tsx";
    vec![
        (
            "Button",
            "packages/js/dazzui/src/components/button.tsx",
            BUTTON,
        ),
        (
            "Switch",
            "packages/js/dazzui/src/components/switch.tsx",
            SWITCH,
        ),
        (
            "Preferences",
            "packages/js/dazzui/src/components/preferences.tsx",
            &[],
        ),
        (
            "PreferenceSection",
            "packages/js/dazzui/src/components/preferences.tsx",
            &[],
        ),
        (
            "PreferenceRow",
            "packages/js/dazzui/src/components/preferences.tsx",
            &[],
        ),
        ("Row", STAGE, &[]),
        ("Col", STAGE, &[]),
        ("Text", STAGE, &[]),
    ]
}

/// Every name a page may write on `spec`, or is told is not drawn yet.
fn names(spec: &ComponentSpec) -> Vec<String> {
    spec.names()
        .map(str::to_owned)
        .chain(spec.unsupported.iter().cloned())
        .collect()
}

#[test]
fn every_component_and_prop_is_react_s() {
    let registry = Registry::builtin();
    let sources = sources();
    let mut wrong = Vec::new();
    for name in registry.names() {
        let Some((_, path, inherited)) = sources.iter().find(|(c, _, _)| *c == name) else {
            wrong.push(format!(
                "`{name}` is not a component the registry knows React draws"
            ));
            continue;
        };
        let source = read(path);
        if !source.contains(&format!("function {name}(")) {
            wrong.push(format!("`{name}` is not a component in {path}"));
        }
        let react: BTreeSet<String> = declared(&source)
            .into_iter()
            .chain(inherited.iter().map(|s| s.to_string()))
            .map(|n| snake_case(&n))
            .collect();
        for prop in names(registry.get(name).unwrap()) {
            if !react.contains(&prop) {
                wrong.push(format!("`{name}({prop}: …)` is not a React prop in {path}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_enum_is_a_react_union() {
    let registry = Registry::builtin();
    let enum_of =
        |component: &str, prop: &str| match registry.get(component).and_then(|c| c.prop(prop)) {
            Some(p) => match &p.ty {
                PropType::Enum(e) => e.clone(),
                other => panic!("`{component}.{prop}` is a {}", other.name()),
            },
            None => panic!("no `{component}.{prop}`"),
        };
    let button = read("packages/js/dazzui/src/components/button.tsx");
    let switch = read("packages/js/dazzui/src/components/switch.tsx");
    let tint = read("packages/js/dazzui/src/lib/tint.ts");
    let stage = read("packages/js/dazzui/example/src/components/stage.tsx");
    let cases = [
        (
            "Button",
            "variant",
            union(&button, "export type ButtonVariant ="),
        ),
        ("Button", "size", union(&button, "export type ButtonSize =")),
        ("Button", "tint", union(&tint, "export const TINTS = [")),
        ("Switch", "size", union(&switch, "export type SwitchSize =")),
        ("Row", "align", union(&stage, "type Align =")),
        ("Text", "tone", union(&stage, "tone?:")),
    ];
    for (component, prop, react) in cases {
        let spec = enum_of(component, prop);
        assert_eq!(
            spec.cases, react,
            "`{component}.{prop}` (`{}`) is not React's union",
            spec.name
        );
    }
}
