use dazzui_compiler_macro::ui;

use crate::{SettingsAction, SettingsState};

ui! {
    page SettingsPage(state: SettingsState, action: SettingsAction) {
        VStack(spacing: 16) {
            Text("General").style(TextStyle::Title)
            HStack(spacing: 8) {
                Toggle {
                    value: bind(state.launch_at_startup),
                    on_change: Action::SetLaunchAtStartup,
                }
                Text("Launch at startup")
            }
            HStack(spacing: 8) {
                Toggle {
                    value: bind(state.auto_update),
                    on_change: Action::SetAutoUpdate,
                }
                Text("Update automatically")
            }

            Text("Plugins").style(TextStyle::Title)
            HStack(spacing: 8) {
                Toggle {
                    value: bind(state.show_disabled_plugins),
                    on_change: Action::SetShowDisabledPlugins,
                }
                Text("Show disabled plugins")
            }
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

            Text(state.status).style(TextStyle::Caption)
            HStack(spacing: 8) {
                Button("Save")
                    .style(ButtonStyle::Primary)
                    .disabled(state.saved)
                    .on_click(Action::Save)
                Button { title: "Reset to defaults" }
                    .on_click(Action::Reset)
            }
        }
    }
}
