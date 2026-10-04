use crate::{SettingsAction, SettingsState};

/// The app's one piece of logic: how an action changes the state.
pub fn update(state: &mut SettingsState, action: SettingsAction) {
    match action {
        SettingsAction::SetLaunchAtStartup(on) => state.launch_at_startup = on,
        SettingsAction::SetAutoUpdate(on) => state.auto_update = on,
        SettingsAction::Save => state.saves += 1,
        SettingsAction::Reset => *state = SettingsState::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_resets() {
        let mut state = SettingsState::default();
        update(&mut state, SettingsAction::SetAutoUpdate(true));
        update(&mut state, SettingsAction::Save);
        assert!(state.auto_update);
        assert_eq!(state.saves, 1);
        update(&mut state, SettingsAction::Reset);
        assert_eq!(state, SettingsState::default());
    }
}
