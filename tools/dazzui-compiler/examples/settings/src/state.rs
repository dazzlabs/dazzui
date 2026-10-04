/// Everything the settings page shows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SettingsState {
    pub launch_at_startup: bool,
    pub auto_update: bool,
    pub saving: bool,
    pub saves: u32,
}
