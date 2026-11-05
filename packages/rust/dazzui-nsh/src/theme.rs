use crate::{
    generated::{THEMES, ThemeTokens},
    tokens::ThemeEntry,
};
use nsh_core::prelude::*;

/// A scope-local, reactive theme. Changing it updates styles without remounting
/// inputs or losing their editing, selection and focus state.
#[derive(Clone, Copy, PartialEq)]
pub struct Theme(Signal<usize>);

impl Theme {
    pub fn new(name: &str) -> Self {
        Self(signal(Self::index(name).unwrap_or(0)))
    }

    fn index(name: &str) -> Option<usize> {
        THEMES.iter().position(|entry| entry.name == name)
    }

    pub fn all() -> &'static [ThemeEntry] {
        THEMES
    }

    /// Returns false and leaves the theme unchanged for an unknown name.
    pub fn set(self, name: &str) -> bool {
        let Some(index) = Self::index(name) else {
            return false;
        };
        self.0.set(index);
        true
    }

    pub fn entry(self) -> &'static ThemeEntry {
        &THEMES[self.0.get()]
    }
    pub fn tokens(self) -> &'static ThemeTokens {
        self.entry().tokens
    }
    pub fn name(self) -> &'static str {
        self.entry().name
    }

    /// Call inside an NSH component or owner. Without a scope, Bright Light is
    /// installed locally, rather than a process-wide global shared by windows.
    pub fn current() -> Self {
        use_context::<Self>().unwrap_or_else(|| {
            let theme = Self::new("bright-light");
            provide_context(theme);
            theme
        })
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct ThemeScopeProps {
    pub name: Prop<SharedString>,
    pub children: Prop<Children>,
}

/// A subtree wearing a named theme; its name may be a signal.
#[component]
pub fn ThemeScope(props: ThemeScopeProps) -> impl IntoElement {
    let theme = Theme::new(&props.name.get());
    provide_context(theme);
    effect(move || {
        theme.set(&props.name.get());
    });
    view().child(props.children)
}
