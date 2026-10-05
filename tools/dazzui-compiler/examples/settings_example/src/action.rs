/// Everything the settings page can ask of the app.
#[derive(Clone, Debug, PartialEq)]
pub enum SettingsAction {
    SetLaunchAtStartup(bool),
    SetAutoUpdate(bool),
    Save,
    Reset,
    SetShowDisabledPlugins(bool),
    /// Turns the plugin with this id on or off.
    TogglePlugin(u32),
}
