use gtk::{gdk, prelude::*};
use std::{cell::RefCell, fmt};
include!("generated/themes.rs");
pub const TOKENS_CSS: &str = include_str!("generated/tokens.css");
pub const COMPONENTS_CSS: &str = include_str!("components.css");
thread_local! { static DISPLAYS: RefCell<Vec<gtk::glib::WeakRef<gtk::gdk::Display>>> = const { RefCell::new(Vec::new()) }; }

/// Install the embedded sheets once per display, on the GTK main thread.
pub fn init(display: &gdk::Display) {
    static ICONS: std::sync::Once = std::sync::Once::new();
    ICONS.call_once(|| {
        gtk::gio::resources_register_include!("icons.gresource")
            .expect("The embedded Fluent icons are valid resources");
    });
    DISPLAYS.with(|displays| {
        let mut displays = displays.borrow_mut();
        displays.retain(|d| d.upgrade().is_some());
        if displays
            .iter()
            .any(|d| d.upgrade().as_ref() == Some(display))
        {
            return;
        }
        for css in [TOKENS_CSS, COMPONENTS_CSS] {
            let provider = gtk::CssProvider::new();
            provider.load_from_string(css);
            gtk::style_context_add_provider_for_display(
                display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        gtk::IconTheme::for_display(display).add_resource_path("/dev/dazzlabs/dazzui/icons");
        displays.push(display.downgrade());
    });
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme(&'static str);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeError(pub String);
impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown DazzUI theme: {}", self.0)
    }
}
impl std::error::Error for ThemeError {}
impl Default for Theme {
    fn default() -> Self {
        Self("bright-light")
    }
}
impl Theme {
    pub fn new(name: &str) -> Result<Self, ThemeError> {
        THEMES
            .iter()
            .find(|&&n| n == name)
            .map(|&n| Self(n))
            .ok_or_else(|| ThemeError(name.into()))
    }
    pub fn name(self) -> &'static str {
        self.0
    }
    pub fn is_dark(self) -> bool {
        self.0.ends_with("-dark")
    }
    /// A resolved ds length in logical pixels, for native layout properties
    /// that cannot read GTK CSS variables (for example FlowBox's row spacing).
    pub fn dimension(self, name: &str) -> Option<f64> {
        use std::{collections::HashMap, sync::LazyLock};
        static DIMENSIONS: LazyLock<HashMap<&str, HashMap<&str, f64>>> = LazyLock::new(|| {
            TOKENS_CSS
                .split(".dz-theme-")
                .skip(1)
                .filter_map(|block| {
                    let (theme, block) = block.split_once(" {\n")?;
                    let block = block.split_once("\n}")?.0;
                    let values = block
                        .lines()
                        .filter_map(|line| {
                            let (key, value) = line.trim().strip_prefix("--")?.split_once(": ")?;
                            Some((key, value.strip_suffix("px;")?.parse().ok()?))
                        })
                        .collect();
                    Some((theme, values))
                })
                .collect()
        });
        DIMENSIONS.get(self.0)?.get(name).copied()
    }
    /// Resolved ds timing for native event controllers and hover timers.
    pub fn duration(self, name: &str) -> Option<std::time::Duration> {
        let selector = format!(".dz-theme-{} {{\n", self.0);
        let block = TOKENS_CSS.split_once(&selector)?.1.split_once("\n}")?.0;
        let key = format!("--{name}: ");
        block.lines().find_map(|line| {
            let ms = line
                .trim()
                .strip_prefix(&key)?
                .strip_suffix("ms;")?
                .parse()
                .ok()?;
            Some(std::time::Duration::from_millis(ms))
        })
    }
    /// A resolved ds colour for native text attributes that cannot read CSS.
    pub fn color(self, name: &str) -> Option<gdk::RGBA> {
        let selector = format!(".dz-theme-{} {{\n", self.0);
        let block = TOKENS_CSS.split_once(&selector)?.1.split_once("\n}")?.0;
        let key = format!("--{name}: ");
        block.lines().find_map(|line| {
            gdk::RGBA::parse(line.trim().strip_prefix(&key)?.strip_suffix(';')?).ok()
        })
    }
    /// A resolved unitless ds value for native layout proportions.
    pub fn number(self, name: &str) -> Option<f64> {
        let selector = format!(".dz-theme-{} {{\n", self.0);
        let block = TOKENS_CSS.split_once(&selector)?.1.split_once("\n}")?.0;
        let key = format!("--{name}: ");
        block.lines().find_map(|line| {
            line.trim()
                .strip_prefix(&key)?
                .strip_suffix(';')?
                .parse()
                .ok()
        })
    }
    /// Apply a complete palette and metrics to this subtree. Nested themes
    /// reset all tokens, and changing a theme retains the controls' state.
    pub fn apply(self, scope: &impl IsA<gtk::Widget>) {
        init(&scope.display());
        for name in THEMES {
            scope.remove_css_class(&format!("dz-theme-{name}"));
        }
        scope.add_css_class("dz-scope");
        scope.add_css_class(&format!("dz-theme-{}", self.0));
    }
    pub fn of(widget: &impl IsA<gtk::Widget>) -> Option<Self> {
        let mut current = Some(widget.as_ref().clone());
        while let Some(w) = current {
            if let Some(&name) = THEMES
                .iter()
                .find(|n| w.has_css_class(&format!("dz-theme-{n}")))
            {
                return Some(Self(name));
            }
            current = w.parent();
        }
        None
    }
}
