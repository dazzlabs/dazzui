/// Everything the settings page shows.
#[derive(Clone, Debug, PartialEq)]
pub struct SettingsState {
    pub launch_at_startup: bool,
    pub auto_update: bool,
    /// Whether what is shown is what was last saved.
    pub saved: bool,
    pub saves: u32,
    pub status: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self { launch_at_startup: false, auto_update: false, saved: true, saves: 0, status: "No changes".into() }
    }
}
