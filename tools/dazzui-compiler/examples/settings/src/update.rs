use dazzui_compiler_macro::app;

use crate::{SettingsAction, SettingsState};

/// The app's one piece of logic: how an action changes the state.
#[app]
pub fn update(state: &mut SettingsState, action: SettingsAction) {
    match action {
        SettingsAction::SetLaunchAtStartup(on) => {
            state.launch_at_startup = on;
            changed(state);
        }
        SettingsAction::SetAutoUpdate(on) => {
            state.auto_update = on;
            changed(state);
        }
        SettingsAction::Save => {
            state.saves += 1;
            state.saved = true;
            state.status = match state.saves {
                1 => "Saved".into(),
                n => format!("Saved {n} times"),
            };
        }
        SettingsAction::Reset => *state = SettingsState::default(),
        SettingsAction::SetShowDisabledPlugins(on) => state.show_disabled_plugins = on,
        SettingsAction::TogglePlugin(id) => {
            if let Some(plugin) = state.plugins.iter_mut().find(|p| p.id == id) {
                plugin.enabled = !plugin.enabled;
                changed(state);
            }
        }
    }
}

fn changed(state: &mut SettingsState) {
    state.saved = false;
    state.status = "Unsaved changes".into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_and_resets() {
        let mut state = SettingsState::default();
        update(&mut state, SettingsAction::SetAutoUpdate(true));
        assert!(state.auto_update && !state.saved);
        update(&mut state, SettingsAction::Save);
        assert_eq!((state.saves, state.saved, state.status.as_str()), (1, true, "Saved"));
        update(&mut state, SettingsAction::Reset);
        assert_eq!(state, SettingsState::default());
    }

    #[test]
    fn hides_disabled_plugins_until_asked() {
        let mut state = SettingsState::default();
        let names = |s: &SettingsState| s.visible_plugins().into_iter().map(|p| p.name).collect::<Vec<_>>();
        assert_eq!(names(&state), ["Spell check", "Translate"]);
        assert!(state.has_hidden_plugins());
        update(&mut state, SettingsAction::SetShowDisabledPlugins(true));
        assert_eq!(names(&state), ["Spell check", "Grammar", "Translate"]);
        update(&mut state, SettingsAction::SetShowDisabledPlugins(false));
        update(&mut state, SettingsAction::TogglePlugin(2));
        assert!(!state.has_hidden_plugins() && !state.saved);
    }

    /// The page's path to Rust: what the native side calls, through the
    /// ABI `#[app]` emitted.
    #[test]
    fn the_bridge_runs_update_and_calls_back() {
        use crate::update::__dazzui_bridge_settings::*;

        unsafe extern "C" fn count(context: *mut core::ffi::c_void) {
            unsafe { *(context as *mut u32) += 1 };
        }

        let mut calls = 0u32;
        unsafe {
            let app = settings_create();
            settings_subscribe(app, count, &mut calls as *mut u32 as *mut _);
            let set = SettingsActionDto { tag: 1, payload: SettingsActionPayload { set_auto_update: true } };
            settings_send(app, set);
            settings_send(app, SettingsActionDto { tag: 2, payload: SettingsActionPayload { set_auto_update: false } });
            let state = settings_get_state(app);
            assert!(state.auto_update && state.saved);
            assert_eq!(state.saves, 1);
            let status = core::slice::from_raw_parts(state.status.ptr, state.status.len);
            assert_eq!(status, b"Saved");
            // A selector crosses as a field, its list owned by Rust until freed.
            let visible = core::slice::from_raw_parts(state.visible_plugins.ptr, state.visible_plugins.len);
            let ids: Vec<u32> = visible.iter().map(|p| p.id).collect();
            assert_eq!(ids, [1, 3]);
            assert!(state.has_hidden_plugins);
            settings_state_free(state);
            settings_destroy(app);
        }
        assert_eq!(calls, 2);
    }
}
