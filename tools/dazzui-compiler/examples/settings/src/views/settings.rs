use dazzui_compiler_macro::ui;

use crate::{SettingsAction, SettingsState};

ui! {
    page SettingsPage(state: SettingsState, action: SettingsAction) {
        VStack(spacing: 16) {
            Text("General")
            Text("Changes apply the next time the app starts.")
            VStack(spacing: 8) {
                Button("Save").on_click(Action::Save)
                Button { title: "Reset to defaults" }
                    .on_click(Action::Reset)
            }
        }
    }
}
