//! The theme an app is drawn in, and the way a subtree wears another.
//!
//! React selects a theme with `data-theme` on any element, and every
//! component below it reads the variables that attribute binds. GPUI has no
//! cascade, but it has the next best thing: a component's `render` runs while
//! its parent lays out. So the theme is a global holding a stack — the app's
//! theme at the bottom, and a [`Themed`] element pushes its own for exactly as
//! long as its child lays out — and a component reads the top of it with
//! [`ActiveTheme::theme`].

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Window,
};

use crate::generated::{STUDIO_LIGHT, THEMES, ThemeTokens};
use crate::tokens::ThemeEntry;

/// The theme global: the app's theme and any subtree's pushed over it.
pub struct Theme {
    base: &'static ThemeEntry,
    stack: Vec<&'static ThemeEntry>,
}

impl gpui::Global for Theme {}

impl Theme {
    /// Every theme, by the name React selects it with.
    pub fn all() -> &'static [ThemeEntry] {
        THEMES
    }

    /// A theme by its `data-theme` name. Studio Light is `studio-light`,
    /// though React writes no attribute for it.
    pub fn named(name: &str) -> Option<&'static ThemeEntry> {
        THEMES.iter().find(|entry| entry.name == name)
    }

    /// Sets the app's theme, by name. An unknown name is Studio Light, as a
    /// `data-theme` no block matches is on the web.
    pub fn set(name: &str, cx: &mut App) {
        let entry = Theme::named(name).unwrap_or(&THEMES[0]);
        Theme::set_entry(entry, cx);
    }

    pub fn set_entry(entry: &'static ThemeEntry, cx: &mut App) {
        if cx.has_global::<Theme>() {
            cx.global_mut::<Theme>().base = entry;
        } else {
            cx.set_global(Theme {
                base: entry,
                stack: Vec::new(),
            });
        }
        cx.refresh_windows();
    }

    /// The theme in force where this is read.
    pub fn entry(cx: &App) -> &'static ThemeEntry {
        match cx.try_global::<Theme>() {
            Some(theme) => theme.stack.last().copied().unwrap_or(theme.base),
            None => &THEMES[0],
        }
    }
}

/// Reads the theme in force: `cx.theme().color_surface`.
pub trait ActiveTheme {
    fn theme(&self) -> &'static ThemeTokens;
    /// Whether the theme in force is a dark one.
    fn is_dark(&self) -> bool;
}

impl ActiveTheme for App {
    fn theme(&self) -> &'static ThemeTokens {
        if self.has_global::<Theme>() {
            Theme::entry(self).tokens
        } else {
            &STUDIO_LIGHT
        }
    }

    fn is_dark(&self) -> bool {
        Theme::entry(self).dark
    }
}

/// A subtree drawn in a theme of its own — React's `data-theme` on an
/// element rather than on the root.
pub fn themed(entry: &'static ThemeEntry, child: impl IntoElement) -> Themed {
    Themed {
        entry,
        child: child.into_any_element(),
    }
}

pub struct Themed {
    entry: &'static ThemeEntry,
    child: AnyElement,
}

impl Themed {
    fn with<R>(entry: &'static ThemeEntry, cx: &mut App, f: impl FnOnce(&mut App) -> R) -> R {
        if !cx.has_global::<Theme>() {
            Theme::set_entry(&THEMES[0], cx);
        }
        cx.global_mut::<Theme>().stack.push(entry);
        let result = f(cx);
        cx.global_mut::<Theme>().stack.pop();
        result
    }
}

impl IntoElement for Themed {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Themed {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let entry = self.entry;
        let child = &mut self.child;
        let layout = Themed::with(entry, cx, |cx| child.request_layout(window, cx));
        (layout, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let entry = self.entry;
        let child = &mut self.child;
        Themed::with(entry, cx, |cx| child.prepaint(window, cx));
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let entry = self.entry;
        let child = &mut self.child;
        Themed::with(entry, cx, |cx| child.paint(window, cx));
    }
}
