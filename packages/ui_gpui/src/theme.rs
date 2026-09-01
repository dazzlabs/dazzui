use std::cell::RefCell;
use std::panic::Location;

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, Global, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Window,
};

use crate::generated::theme_variables::{self, ThemeVariables};

/// The four themes, as they are named in `terrazzo.config.mjs`.
///
/// Studio Light is the default and carries no attribute in the React package
/// at all — selecting it means removing `data-theme` rather than writing
/// `studio-light`. Here it is simply the variables nothing overrides, which
/// is also what an app that never sets a theme gets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeName {
    /// White card, light grey canvas, violet accent. **The baseline.**
    #[default]
    StudioLight,
    /// The same skeleton over a near-black canvas.
    StudioDark,
    /// Warm paper, ink navy, acid-green marker, pill controls.
    BrightLight,
    /// Ink-navy canvas, with acid green as the accent itself.
    BrightDark,
}

impl ThemeName {
    pub const ALL: [ThemeName; 4] = [
        ThemeName::StudioLight,
        ThemeName::StudioDark,
        ThemeName::BrightLight,
        ThemeName::BrightDark,
    ];

    /// What `data-theme` would carry on the web.
    pub const fn attribute(self) -> &'static str {
        match self {
            ThemeName::StudioLight => "studio-light",
            ThemeName::StudioDark => "studio-dark",
            ThemeName::BrightLight => "bright-light",
            ThemeName::BrightDark => "bright-dark",
        }
    }

    /// The name a theme menu prints.
    pub const fn label(self) -> &'static str {
        match self {
            ThemeName::StudioLight => "Studio Light",
            ThemeName::StudioDark => "Studio Dark",
            ThemeName::BrightLight => "Bright Light",
            ThemeName::BrightDark => "Bright Dark",
        }
    }

    /// Is this one of the two drawn on a dark canvas?
    pub const fn is_dark(self) -> bool {
        matches!(self, ThemeName::StudioDark | ThemeName::BrightDark)
    }

    /// The token set this theme resolved to.
    pub fn variables(self) -> &'static ThemeVariables {
        match self {
            ThemeName::StudioLight => &theme_variables::STUDIO_LIGHT,
            ThemeName::StudioDark => &theme_variables::STUDIO_DARK,
            ThemeName::BrightLight => &theme_variables::BRIGHT_LIGHT,
            ThemeName::BrightDark => &theme_variables::BRIGHT_DARK,
        }
    }
}

/// The active theme: which one it is, and the variables it resolved to.
///
/// A theme is two words wide — a name and a reference to a `static` — so a
/// component copies it rather than borrowing the app, and nothing is
/// allocated or recomputed per frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub name: ThemeName,
    pub vars: &'static ThemeVariables,
}

impl Theme {
    pub fn new(name: ThemeName) -> Self {
        Theme {
            name,
            vars: name.variables(),
        }
    }

    /// Set the theme for the whole app.
    pub fn set(name: ThemeName, cx: &mut App) {
        cx.set_global(Theme::new(name));
        cx.refresh_windows();
    }
}

impl Default for Theme {
    /// The same baseline `:root` holds.
    fn default() -> Self {
        Theme::new(ThemeName::StudioLight)
    }
}

impl Global for Theme {}

/// The theme a component draws in.
///
/// Reads the scope first, so a subtree wrapped in [`themed`] resolves against
/// its own variables; then the app-wide global; then Studio Light, which is
/// the same thing a page with no `data-theme` attribute gets.
pub trait ActiveTheme {
    fn theme(&self) -> Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> Theme {
        scoped_theme().unwrap_or_else(|| self.try_global::<Theme>().copied().unwrap_or_default())
    }
}

thread_local! {
    /// The stack [`Themed`] pushes onto while its subtree is being built.
    static SCOPE: RefCell<Vec<Theme>> = const { RefCell::new(Vec::new()) };
}

fn scoped_theme() -> Option<Theme> {
    SCOPE.with(|scope| scope.borrow().last().copied())
}

fn with_scope<R>(theme: Theme, f: impl FnOnce() -> R) -> R {
    SCOPE.with(|scope| scope.borrow_mut().push(theme));
    let result = f();
    SCOPE.with(|scope| {
        scope.borrow_mut().pop();
    });
    result
}

/// Scopes a theme to a subtree, the way React's `data-theme` attribute does.
///
/// Themes are per-subtree, so several can sit on screen at once — which is
/// what the playground's Themes page is: the same view rendered four times,
/// each resolving against its own variables.
///
/// A component reads the theme while its element is being built, and GPUI
/// builds a subtree inside its parent's layout pass, so scoping it means
/// holding the theme for the length of that pass rather than passing it down
/// by hand through every component in between.
pub fn themed(name: ThemeName, child: impl IntoElement) -> Themed {
    Themed {
        theme: Theme::new(name),
        child: child.into_any_element(),
    }
}

/// The element [`themed`] builds.
pub struct Themed {
    theme: Theme,
    child: AnyElement,
}

impl Element for Themed {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let theme = self.theme;
        let child = &mut self.child;
        (with_scope(theme, || child.request_layout(window, cx)), ())
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
        let theme = self.theme;
        let child = &mut self.child;
        with_scope(theme, || child.prepaint(window, cx));
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
        let theme = self.theme;
        let child = &mut self.child;
        with_scope(theme, || child.paint(window, cx));
    }
}

impl IntoElement for Themed {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
