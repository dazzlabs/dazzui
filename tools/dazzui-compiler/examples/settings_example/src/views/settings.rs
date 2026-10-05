use dazzui_runtime::ui;

use crate::{SettingsAction, SettingsState};

ui! {
    page SettingsPage(state: SettingsState, action: SettingsAction) {
        VStack(spacing: 24) {
            Settings {
                SettingsSection("General") {
                    SettingsRow {
                        title: "Launch at startup",
                        trailing: Toggle {
                            value: bind(state.launch_at_startup),
                            on_change: Action::SetLaunchAtStartup,
                        }
                    }
                    SettingsRow {
                        title: "Update automatically",
                        subtitle: "Download new versions in the background",
                        trailing: Toggle {
                            value: bind(state.auto_update),
                            on_change: Action::SetAutoUpdate,
                        }
                    }
                }
                SettingsSection("Plugins", footer: "Plugins run in a sandbox and can be turned off at any time.") {
                    SettingsRow {
                        title: "Show disabled plugins",
                        trailing: Toggle {
                            value: bind(state.show_disabled_plugins),
                            on_change: Action::SetShowDisabledPlugins,
                        }
                    }
                }
            }

            VStack(spacing: 8) {
                for plugin in state.visible_plugins {
                    HStack(spacing: 8) {
                        Text(plugin.name)
                        if plugin.enabled {
                            Button("Disable").on_click(Action::TogglePlugin(plugin.id))
                        } else {
                            Button("Enable")
                                .style(ButtonStyle::Primary)
                                .on_click(Action::TogglePlugin(plugin.id))
                        }
                    }
                }
                if state.has_hidden_plugins {
                    Text("Disabled plugins are hidden").style(TextStyle::Caption)
                }
            }

            Text(state.status).style(TextStyle::Caption)
            HStack(spacing: 8) {
                Button("Save")
                    .style(ButtonStyle::Primary)
                    .disabled(state.save_disabled)
                    .on_click(Action::Save)
                Button { title: "Reset to defaults" }
                    .on_click(Action::Reset)
            }
        }
    }
}
