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
