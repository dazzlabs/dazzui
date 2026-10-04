use dazzui_compiler_macro::selector;

/// Everything the settings page shows.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsState {
    pub launch_at_startup: bool,
    pub auto_update: bool,
    /// Whether what is shown is what was last saved.
    pub saved: bool,
    pub saves: u32,
    pub status: String,
    pub show_disabled_plugins: bool,
    pub plugins: Vec<Plugin>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plugin {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
}

impl SettingsState {
    /// The plugins the list shows: the enabled ones, and the rest only
    /// when asked for.
    #[selector]
    pub fn visible_plugins(&self) -> Vec<Plugin> {
        self.plugins.iter().filter(|p| p.enabled || self.show_disabled_plugins).cloned().collect()
    }

    #[selector]
    pub fn has_hidden_plugins(&self) -> bool {
        !self.show_disabled_plugins && self.plugins.iter().any(|p| !p.enabled)
    }
}

impl Default for SettingsState {
    fn default() -> Self {
        let plugin = |id, name: &str, enabled| Plugin { id, name: name.into(), enabled };
        Self {
            launch_at_startup: false,
            auto_update: false,
            saved: true,
            saves: 0,
            status: "No changes".into(),
            show_disabled_plugins: false,
            plugins: vec![plugin(1, "Spell check", true), plugin(2, "Grammar", false), plugin(3, "Translate", true)],
        }
    }
}
