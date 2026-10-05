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

    /// The page's path to Rust: the UniFFI object `#[app]` emitted, as the
    /// native side drives it.
    #[test]
    fn the_bridge_runs_update_and_calls_back() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        use crate::update::__dazzui_bridge_settings::*;

        struct Count(AtomicU32);
        impl SettingsListener for Count {
            fn state_changed(&self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let app = SettingsApp::new();
        let count = Arc::new(Count(AtomicU32::new(0)));
        app.subscribe(count.clone());
        app.send(SettingsAction::SetAutoUpdate(true));
        app.send(SettingsAction::Save);
        let state = app.state();
        assert!(state.auto_update && state.saved);
        assert_eq!((state.saves, state.status.as_str()), (1, "Saved"));
        // A selector crosses as a field of the snapshot.
        assert_eq!(state.visible_plugins.iter().map(|p| p.id).collect::<Vec<_>>(), [1, 3]);
        assert!(state.has_hidden_plugins);
        assert_eq!(count.0.load(Ordering::SeqCst), 2);
    }
}
