use std::path::Path;

use toml::{Table, Value as Toml};

use crate::ir::{
    ChildrenSpec, Diagnostic, SourceLocation, ComponentSpec, EnumSpec, EventSpec, PropSpec, PropType, SlotSpec, Source, SwiftArg, SwiftUIMapping,
    Trailing, Value, WinAttr, WinChildren, WinEvent, WinUIMapping,
};

/// The file an app's own components are in, at its crate root.
pub const FILE: &str = "dazzui-compiler.toml";

pub(crate) const BUILTIN: &str = include_str!("registry.toml");

/// Components each backend lowers itself rather than from a mapping.
pub const BACKEND_LOWERED: &[&str] = &["VStack", "HStack", "Text"];

/// A registry file that does not load: what is wrong, in which file, and
/// where in it when that is known.
#[derive(Clone, Debug, PartialEq)]
pub struct RegistryError {
    pub file: String,
    /// The component it is about, whose table the diagnostic points at.
    pub component: Option<String>,
    pub message: String,
    /// The byte range a TOML syntax error is at.
    pub span: Option<std::ops::Range<usize>>,
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.component {
            Some(c) => write!(f, "{}: component `{c}`: {}", self.file, self.message),
            None => write!(f, "{}: {}", self.file, self.message),
        }
    }
}

impl RegistryError {
    fn new(file: &str, component: Option<&str>, message: impl Into<String>) -> Self {
        Self { file: file.into(), component: component.map(Into::into), message: message.into(), span: None }
    }

    /// A rustc-style diagnostic, pointing into `text`, the file's contents:
    /// at a syntax error, or at the component's table.
    pub fn to_diagnostic(&self, text: &str) -> Diagnostic {
        let message = match &self.component {
            Some(c) => format!("component `{c}`: {}", self.message),
            None => self.message.clone(),
        };
        let range = self.span.clone().or_else(|| {
            let c = self.component.as_ref()?;
            [format!("[component.{c}]"), format!("[component.{c}."), format!("component.{c}")]
                .iter()
                .find_map(|needle| text.find(needle.as_str()).map(|at| at..at + needle.len()))
        });
        let mut d = Diagnostic::error(message);
        if let Some(range) = range {
            let before = &text[..range.start.min(text.len())];
            let line = before.matches('\n').count() as u32 + 1;
            let column = (range.start - before.rfind('\n').map_or(0, |i| i + 1)) as u32 + 1;
            let line_end = text[range.start..].find('\n').map_or(text.len(), |i| range.start + i);
            let width = (range.end.min(line_end).saturating_sub(range.start)) as u32;
            d = d.at(SourceLocation::new(&self.file, line, column), width);
        }
        d
    }
}

/// The components a page may use, by semantic name: the built-in ones,
/// and an app's own from its `dazzui-compiler.toml`, which may also replace
/// a built-in one. A new component is a change to a registry, never to the
/// compiler.
#[derive(Clone, Debug)]
pub struct Registry {
    components: Vec<ComponentSpec>,
}

impl Registry {
    pub fn builtin() -> Self {
        Self::from_tables(&[(BUILTIN, "registry.toml")]).expect("the built-in registry is valid")
    }

    /// The built-in registry and, if `dir` has one, its `dazzui-compiler.toml`.
    pub fn load(dir: &Path) -> Result<Self, RegistryError> {
        let path = dir.join(FILE);
        match std::fs::read_to_string(&path) {
            Ok(text) => Self::from_tables(&[(BUILTIN, "registry.toml"), (&text, FILE)]),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::builtin()),
            Err(e) => Err(RegistryError::new(FILE, None, format!("cannot read {}: {e}", path.display()))),
        }
    }

    /// Later files add components and enums, and replace those of the same
    /// name.
    pub(crate) fn from_tables(files: &[(&str, &str)]) -> Result<Self, RegistryError> {
        let mut enums: Vec<EnumSpec> = Vec::new();
        let mut raw: Vec<(String, Table, &str)> = Vec::new();
        for (text, file) in files {
            let table: Table = text.parse().map_err(|e: toml::de::Error| RegistryError {
                span: e.span(),
                ..RegistryError::new(file, None, e.message().trim().replace('\n', "; "))
            })?;
            for key in table.keys() {
                if key != "enum" && key != "component" {
                    return Err(RegistryError::new(file, None, format!("unknown table `{key}`; expected `enum` or `component`")));
                }
            }
            for (name, value) in table.get("enum").and_then(Toml::as_table).into_iter().flatten() {
                let at = format!("enum `{name}`");
                let cases = strings(value.get("cases"), &at, "cases")
                    .and_then(|c| c.ok_or(format!("{at}: needs `cases`")))
                    .map_err(|e| RegistryError::new(file, None, e))?;
                enums.retain(|e| &e.name != name);
                enums.push(EnumSpec { name: name.clone(), cases });
            }
            for (name, value) in table.get("component").and_then(Toml::as_table).into_iter().flatten() {
                let body = value.as_table().ok_or(RegistryError::new(file, Some(name), "is not a table"))?;
                raw.retain(|(n, _, _)| n != name);
                raw.push((name.clone(), body.clone(), file));
            }
        }
        let components = raw
            .iter()
            .map(|(name, body, file)| component(name, body, &enums).map_err(|e| RegistryError::new(file, Some(name), e)))
            .collect::<Result<Vec<_>, _>>()?;
        let registry = Registry { components };
        for (spec, (_, _, file)) in registry.components.iter().zip(&raw) {
            registry.validate(spec).map_err(|e| RegistryError::new(file, Some(&spec.name), e))?;
        }
        Ok(registry)
    }

    pub fn get(&self, name: &str) -> Option<&ComponentSpec> {
        self.components.iter().find(|c| c.name == name)
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|c| c.name.as_str())
    }

    /// What a spec's mappings and lists name must exist on it.
    fn validate(&self, spec: &ComponentSpec) -> Result<(), String> {
        for name in &spec.positional {
            spec.prop(name).ok_or(format!("positional `{name}` is not a prop"))?;
        }
        for slot in &spec.slots {
            if spec.prop(&slot.name).is_some() {
                return Err(format!("`{}` is both a prop and a slot", slot.name));
            }
        }
        for event in &spec.events {
            if let Some(binds) = &event.binds
                && !spec.prop(binds).is_some_and(|p| p.bindable)
            {
                return Err(format!("event `{}` binds `{binds}`, which is not a bindable prop", event.name));
            }
        }
        if let ChildrenSpec::Many { allow: Some(allow) } = &spec.children {
            for name in allow {
                self.get(name).ok_or(format!("children allow `{name}`, which is not a component"))?;
            }
        }
        for prop in spec.props.iter().filter(|p| p.bindable) {
            if !spec.events.iter().any(|e| e.binds.as_deref() == Some(&prop.name)) {
                return Err(format!("prop `{}` is bindable, but no event `binds` it to write the change back", prop.name));
            }
        }
        let backend_lowered = BACKEND_LOWERED.contains(&spec.name.as_str());
        if backend_lowered && (spec.swiftui.is_some() != spec.winui.is_some()) {
            return Err(
                "replaces a component the backends lower themselves, so it needs both a `swiftui` and a `winui` table".into()
            );
        }
        if spec.name == "Text"
            && spec.swiftui.is_none()
            && let Some(PropType::Enum(e)) = spec.prop("style").map(|p| &p.ty)
            && let Some(case) = e.cases.iter().find(|c| !["Title", "Body", "Caption"].contains(&c.as_str()))
        {
            return Err(format!(
                "`TextStyle::{case}` has no type token in the backends; Text without platform tables takes Title, Body and Caption"
            ));
        }
        // Every case of an enum prop a mapping writes needs its platform code.
        let check_cases = |source: &Source, platform: &str| -> Result<(), String> {
            if let Source::Prop { prop, cases, .. } = source
                && let Some(PropType::Enum(e)) = spec.prop(prop).map(|p| &p.ty)
            {
                for case in &e.cases {
                    if !cases.iter().any(|(c, _)| c == case) {
                        return Err(format!("{platform}: prop `{prop}` maps no code for `{}::{case}`", e.name));
                    }
                }
            }
            Ok(())
        };
        if let Some(m) = &spec.swiftui {
            m.args.iter().try_for_each(|a| check_cases(&a.source, "swiftui"))?;
        }
        if let Some(m) = &spec.winui {
            m.attrs.iter().try_for_each(|a| check_cases(&a.source, "winui"))?;
            for e in &m.events {
                if spec.event(&e.name).is_some_and(|spec| spec.payload.is_some()) && e.value.is_none() {
                    return Err(format!("winui: event `{}` carries a value, so its mapping needs `value`, the C# that reads it", e.name));
                }
            }
        }
        let check_source = |source: &Source| match source {
            Source::Prop { prop, .. } => spec.prop(prop).map(|_| ()).ok_or(format!("maps prop `{prop}`, which it has not")),
            Source::Event(e) => spec.event(e).map(|_| ()).ok_or(format!("maps event `{e}`, which it has not")),
            Source::Slot(s) => spec.slot(s).map(|_| ()).ok_or(format!("maps slot `{s}`, which it has not")),
        };
        match &spec.swiftui {
            Some(m) => {
                m.args.iter().try_for_each(|a| check_source(&a.source))?;
                match &m.trailing {
                    Some(Trailing::Prop { prop, .. }) => check_source(&Source::Prop {
                        prop: prop.clone(),
                        cases: vec![],
                        invert: false,
                        omit: None,
                    })?,
                    Some(Trailing::Slot(s)) => check_source(&Source::Slot(s.clone()))?,
                    Some(Trailing::Children) | None => {}
                }
            }
            None if !backend_lowered => return Err("has no `swiftui` table".into()),
            None => {}
        }
        match &spec.winui {
            Some(m) => {
                m.attrs.iter().try_for_each(|a| check_source(&a.source))?;
                for e in &m.events {
                    check_source(&Source::Event(e.name.clone()))?;
                }
                for (s, _) in &m.slots {
                    check_source(&Source::Slot(s.clone()))?;
                }
            }
            None if !backend_lowered => return Err("has no `winui` table".into()),
            None => {}
        }
        Ok(())
    }
}

fn component(name: &str, body: &Table, enums: &[EnumSpec]) -> Result<ComponentSpec, String> {
    known(body, &["positional", "children", "props", "events", "slots", "swiftui", "winui"], "")?;
    let props = entries(body.get("props"), "props")?
        .into_iter()
        .map(|(name, t)| prop(&name, t, enums).map_err(|e| format!("prop `{name}`: {e}")))
        .collect::<Result<Vec<_>, _>>()?;
    let events = entries(body.get("events"), "events")?
        .into_iter()
        .map(|(name, t)| {
            known(t, &["modifier", "payload", "binds"], &format!("event `{name}`: "))?;
            let payload = string(t.get("payload"), "payload")?.map(|ty| prop_type(&ty, enums)).transpose()?;
            Ok(EventSpec {
                modifier: string(t.get("modifier"), "modifier")?.ok_or(format!("event `{name}` needs `modifier`"))?,
                payload,
                binds: string(t.get("binds"), "binds")?,
                name,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let slots = entries(body.get("slots"), "slots")?
        .into_iter()
        .map(|(name, t)| {
            known(t, &["required"], &format!("slot `{name}`: "))?;
            Ok(SlotSpec { required: boolean(t.get("required"), "required")?.unwrap_or(false), name })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let children = match body.get("children") {
        None => ChildrenSpec::None,
        Some(Toml::String(s)) if s == "none" => ChildrenSpec::None,
        Some(Toml::String(s)) if s == "many" => ChildrenSpec::Many { allow: None },
        Some(Toml::Table(t)) => {
            known(t, &["allow"], "children: ")?;
            ChildrenSpec::Many { allow: strings(t.get("allow"), "children", "allow")? }
        }
        Some(_) => return Err("`children` is \"none\", \"many\" or { allow = [...] }".into()),
    };
    Ok(ComponentSpec {
        name: name.into(),
        props,
        positional: strings(body.get("positional"), "", "positional")?.unwrap_or_default(),
        events,
        slots,
        children,
        swiftui: body.get("swiftui").map(swiftui).transpose().map_err(|e| format!("swiftui: {e}"))?,
        winui: body.get("winui").map(winui).transpose().map_err(|e| format!("winui: {e}"))?,
    })
}

fn prop(name: &str, t: &Table, enums: &[EnumSpec]) -> Result<PropSpec, String> {
    known(t, &["type", "required", "default", "modifier", "bindable"], "")?;
    let ty = prop_type(&string(t.get("type"), "type")?.ok_or("needs `type`")?, enums)?;
    let default = t.get("default").map(|v| value(v, &ty)).transpose()?;
    Ok(PropSpec {
        name: name.into(),
        required: boolean(t.get("required"), "required")?.unwrap_or(false),
        default,
        modifier: boolean(t.get("modifier"), "modifier")?.unwrap_or(false),
        bindable: boolean(t.get("bindable"), "bindable")?.unwrap_or(false),
        ty,
    })
}

fn prop_type(name: &str, enums: &[EnumSpec]) -> Result<PropType, String> {
    match name {
        "string" => Ok(PropType::String),
        "bool" => Ok(PropType::Bool),
        "int" => Ok(PropType::Int),
        _ => match name.strip_prefix("enum:") {
            Some(e) => enums
                .iter()
                .find(|spec| spec.name == e)
                .cloned()
                .map(PropType::Enum)
                .ok_or(format!("no enum `{e}`; declare it under [enum.{e}]")),
            None => Err(format!("unknown type `{name}`; expected string, bool, int or enum:Name")),
        },
    }
}

/// A literal of type `ty`: an enum case is its name, as a string.
fn value(v: &Toml, ty: &PropType) -> Result<Value, String> {
    match (v, ty) {
        (Toml::String(s), PropType::String) => Ok(Value::String(s.clone())),
        (Toml::String(s), PropType::Enum(e)) if e.cases.contains(s) => Ok(Value::String(s.clone())),
        (Toml::Boolean(b), PropType::Bool) => Ok(Value::Bool(*b)),
        (Toml::Integer(n), PropType::Int) => Ok(Value::Int(*n)),
        _ => Err(format!("`{v:?}` is not a {}", ty.name())),
    }
}

fn swiftui(v: &Toml) -> Result<SwiftUIMapping, String> {
    let t = v.as_table().ok_or("is not a table")?;
    known(t, &["call", "args", "trailing"], "")?;
    let args = match t.get("args") {
        None => Vec::new(),
        Some(Toml::Array(items)) => items
            .iter()
            .map(|item| {
                let a = item.as_table().ok_or("an arg is a table")?;
                known(a, &["arg", "prop", "event", "slot", "enum", "invert", "omit", "fallback"], "arg: ")?;
                Ok(SwiftArg {
                    arg: string(a.get("arg"), "arg")?.ok_or("an arg needs `arg`")?,
                    source: source(a)?,
                    fallback: string(a.get("fallback"), "fallback")?,
                })
            })
            .collect::<Result<_, String>>()?,
        Some(_) => return Err("`args` is an array".into()),
    };
    let trailing = match t.get("trailing") {
        None => None,
        Some(Toml::Table(tr)) => {
            known(tr, &["children", "prop", "view", "slot"], "trailing: ")?;
            Some(if boolean(tr.get("children"), "children")? == Some(true) {
                Trailing::Children
            } else if let Some(slot) = string(tr.get("slot"), "slot")? {
                Trailing::Slot(slot)
            } else {
                Trailing::Prop {
                    prop: string(tr.get("prop"), "prop")?.ok_or("`trailing` takes children, slot or prop")?,
                    view: string(tr.get("view"), "view")?.unwrap_or_else(|| "Text".into()),
                }
            })
        }
        Some(_) => return Err("`trailing` is a table".into()),
    };
    Ok(SwiftUIMapping { call: string(t.get("call"), "call")?.ok_or("needs `call`")?, args, trailing })
}

fn winui(v: &Toml) -> Result<WinUIMapping, String> {
    let t = v.as_table().ok_or("is not a table")?;
    known(t, &["element", "attrs", "events", "slots", "children"], "")?;
    let attrs = match t.get("attrs") {
        None => Vec::new(),
        Some(Toml::Array(items)) => items
            .iter()
            .map(|item| {
                let a = item.as_table().ok_or("an attr is a table")?;
                known(a, &["attr", "prop", "event", "slot", "enum", "invert", "omit"], "attr: ")?;
                Ok(WinAttr { attr: string(a.get("attr"), "attr")?.ok_or("an attr needs `attr`")?, source: source(a)? })
            })
            .collect::<Result<_, String>>()?,
        Some(_) => return Err("`attrs` is an array".into()),
    };
    let events = entries(t.get("events"), "events")?
        .into_iter()
        .map(|(name, e)| {
            known(e, &["event", "args", "value", "using"], &format!("event `{name}`: "))?;
            Ok(WinEvent {
                event: string(e.get("event"), "event")?.ok_or(format!("event `{name}` needs `event`"))?,
                args: string(e.get("args"), "args")?.unwrap_or_else(|| "RoutedEventArgs".into()),
                value: string(e.get("value"), "value")?,
                using: string(e.get("using"), "using")?,
                name,
            })
        })
        .collect::<Result<_, String>>()?;
    let slots = entries(t.get("slots"), "slots")?
        .into_iter()
        .map(|(name, s)| {
            known(s, &["property"], &format!("slot `{name}`: "))?;
            Ok((name.clone(), string(s.get("property"), "property")?.ok_or(format!("slot `{name}` needs `property`"))?))
        })
        .collect::<Result<_, String>>()?;
    let children = match t.get("children") {
        None => None,
        Some(Toml::Table(c)) => {
            known(c, &["property", "wrap", "wrap_property"], "children: ")?;
            let wrap = string(c.get("wrap"), "wrap")?;
            Some(WinChildren {
                property: string(c.get("property"), "property")?,
                wrap: wrap.map(|w| Ok::<_, String>((w, string(c.get("wrap_property"), "wrap_property")?))).transpose()?,
            })
        }
        Some(_) => return Err("`children` is a table".into()),
    };
    Ok(WinUIMapping {
        element: string(t.get("element"), "element")?.ok_or("needs `element`")?,
        attrs,
        events,
        slots,
        children,
    })
}

/// `prop = …` with its `enum`, `invert` and `omit`; or `event = …`; or
/// `slot = …`.
fn source(t: &Table) -> Result<Source, String> {
    if let Some(event) = string(t.get("event"), "event")? {
        return Ok(Source::Event(event));
    }
    if let Some(slot) = string(t.get("slot"), "slot")? {
        return Ok(Source::Slot(slot));
    }
    let prop = string(t.get("prop"), "prop")?.ok_or("names one of `prop`, `event` or `slot`")?;
    let cases = match t.get("enum") {
        None => Vec::new(),
        Some(Toml::Table(cases)) => cases
            .iter()
            .map(|(k, v)| Ok((k.clone(), v.as_str().ok_or("an enum case maps to a string")?.to_string())))
            .collect::<Result<_, String>>()?,
        Some(_) => return Err("`enum` is a table of case = code".into()),
    };
    let omit = match t.get("omit") {
        None => None,
        Some(Toml::String(s)) => Some(Value::String(s.clone())),
        Some(Toml::Boolean(b)) => Some(Value::Bool(*b)),
        Some(Toml::Integer(n)) => Some(Value::Int(*n)),
        Some(_) => return Err("`omit` is a string, bool or integer".into()),
    };
    Ok(Source::Prop { prop, cases, invert: boolean(t.get("invert"), "invert")?.unwrap_or(false), omit })
}

/// The tables under `props`, `events` or `slots`, by name, in file order.
fn entries<'a>(v: Option<&'a Toml>, what: &str) -> Result<Vec<(String, &'a Table)>, String> {
    let Some(v) = v else { return Ok(Vec::new()) };
    let t = v.as_table().ok_or(format!("`{what}` is a table"))?;
    t.iter()
        .map(|(k, v)| Ok((k.clone(), v.as_table().ok_or(format!("`{what}.{k}` is a table"))?)))
        .collect()
}

fn known(t: &Table, keys: &[&str], at: &str) -> Result<(), String> {
    match t.keys().find(|k| !keys.contains(&k.as_str())) {
        Some(k) => Err(format!("{at}unknown key `{k}`; expected one of {}", keys.join(", "))),
        None => Ok(()),
    }
}

fn string(v: Option<&Toml>, key: &str) -> Result<Option<String>, String> {
    match v {
        None => Ok(None),
        Some(Toml::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(format!("`{key}` is a string")),
    }
}

fn boolean(v: Option<&Toml>, key: &str) -> Result<Option<bool>, String> {
    match v {
        None => Ok(None),
        Some(Toml::Boolean(b)) => Ok(Some(*b)),
        Some(_) => Err(format!("`{key}` is true or false")),
    }
}

fn strings(v: Option<&Toml>, at: &str, key: &str) -> Result<Option<Vec<String>>, String> {
    match v {
        None => Ok(None),
        Some(Toml::Array(items)) => items
            .iter()
            .map(|i| i.as_str().map(str::to_owned).ok_or(format!("{at}: `{key}` holds strings")))
            .collect::<Result<_, _>>()
            .map(Some),
        Some(_) => Err(format!("{at}: `{key}` is an array of strings")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_builtin_registry() {
        let r = Registry::builtin();
        let button = r.get("Button").unwrap();
        assert_eq!(button.positional, ["title"]);
        assert_eq!(button.prop("style").unwrap().default, Some(Value::String("Secondary".into())));
        assert_eq!(button.swiftui.as_ref().unwrap().args[0].arg, "variant");
        let toggle = r.get("Toggle").unwrap();
        assert_eq!(toggle.event("change").unwrap().binds.as_deref(), Some("value"));
        assert!(r.get("VStack").unwrap().swiftui.is_none());
    }

    #[test]
    fn an_app_adds_and_replaces_components() {
        let app = r#"
            [component.Row]
            positional = ["title"]
            props.title = { type = "string", required = true }
            slots.trailing = {}
            swiftui = { call = "Row", args = [{ arg = "title", prop = "title" }, { arg = "trailing", slot = "trailing" }] }
            winui = { element = "Row", attrs = [{ attr = "Label", prop = "title" }], slots.trailing = { property = "Control" } }

            [component.Text]
            positional = ["text"]
            props.text = { type = "string", required = true }
            swiftui = { call = "Label", args = [{ arg = "text", prop = "text" }] }
            winui = { element = "Label", attrs = [{ attr = "Text", prop = "text" }] }
        "#;
        let r = Registry::from_tables(&[(BUILTIN, "registry.toml"), (app, FILE)]).unwrap();
        assert_eq!(r.get("Row").unwrap().slots[0].name, "trailing");
        assert_eq!(r.get("Text").unwrap().swiftui.as_ref().unwrap().call, "Label");
        assert!(r.get("Button").is_some());
    }

    #[test]
    fn rejects_what_does_not_line_up() {
        let err = |app: &str| Registry::from_tables(&[(BUILTIN, "registry.toml"), (app, FILE)]).unwrap_err().to_string();
        assert_eq!(
            err("[component.X]\nprops.a = { type = \"strng\" }"),
            "dazzui-compiler.toml: component `X`: prop `a`: unknown type `strng`; expected string, bool, int or enum:Name"
        );
        assert_eq!(err("[component.X]\nprops.a = { type = \"string\" }"), "dazzui-compiler.toml: component `X`: has no `swiftui` table");
        assert_eq!(
            err("[component.X]\nswiftui = { call = \"X\", args = [{ arg = \"a\", prop = \"b\" }] }\nwinui = { element = \"X\" }"),
            "dazzui-compiler.toml: component `X`: maps prop `b`, which it has not"
        );
        assert_eq!(
            err("[component.X]\ncolour = 1"),
            "dazzui-compiler.toml: component `X`: unknown key `colour`; expected one of positional, children, props, events, slots, swiftui, winui"
        );
        assert_eq!(
            err("[component.X]\nprops.s = { type = \"enum:ButtonStyle\" }\nswiftui = { call = \"X\", args = [{ arg = \"s\", prop = \"s\", enum = { Primary = \".p\" } }] }\nwinui = { element = \"X\" }"),
            "dazzui-compiler.toml: component `X`: swiftui: prop `s` maps no code for `ButtonStyle::Secondary`"
        );
        assert_eq!(
            err("[component.X]\nprops.v = { type = \"bool\", bindable = true }\nswiftui = { call = \"X\" }\nwinui = { element = \"X\" }"),
            "dazzui-compiler.toml: component `X`: prop `v` is bindable, but no event `binds` it to write the change back"
        );
    }

    #[test]
    fn points_at_the_component_or_the_syntax_error() {
        let app = "# mine\n\n[component.X]\ncolour = 1\n";
        let e = Registry::from_tables(&[(BUILTIN, "registry.toml"), (app, FILE)]).unwrap_err();
        let d = e.to_diagnostic(app);
        assert_eq!(d.location, Some(SourceLocation::new(FILE, 3, 1)));
        assert_eq!(d.width, 13);

        let app = "[component.X]\nprops = {\n";
        let e = Registry::from_tables(&[(BUILTIN, "registry.toml"), (app, FILE)]).unwrap_err();
        assert_eq!(e.to_diagnostic(app).location.map(|l| l.line), Some(2));
    }
}
