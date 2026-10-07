/// One file a backend writes, at a path relative to the output directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedFile {
    pub path: String,
    pub contents: String,
}

impl GeneratedFile {
    pub fn new(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

/// `SettingsModel` → `Settings`: the stem a model's companion types are
/// named from — `SettingsStore`, `SettingsActions` — so that they are not
/// `SettingsModelStore`.
pub fn model_stem(model: &str) -> &str {
    model
        .strip_suffix("Model")
        .filter(|s| !s.is_empty())
        .unwrap_or(model)
}

/// `SetAutoUpdate` → `set_auto_update`; `launch_at_startup` stays as it is.
pub fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// `launch_at_startup` → `launchAtStartup`.
pub fn camel_case(snake: &str) -> String {
    let pascal = pascal_case(snake);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// `launch_at_startup` → `LaunchAtStartup`; `SetAutoUpdate` stays as it is.
pub fn pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}
