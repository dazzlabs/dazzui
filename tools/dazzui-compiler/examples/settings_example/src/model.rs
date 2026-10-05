use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use dazzui_runtime::{Notifier, PageModel, page_model};

use crate::{SettingsAction, SettingsState};

/// What drives the settings page. Every action but `Save` changes the state
/// before `send` returns; a save takes a while, as writing to disk would,
/// and finishes on a thread of its own, which tells the page when it is done.
pub struct SettingsModel {
    state: Arc<Mutex<SettingsState>>,
    notifier: Arc<Notifier>,
    save_takes: Duration,
}

/// The model the native side makes when it opens the settings page.
#[page_model]
pub fn settings() -> SettingsModel {
    SettingsModel::new(Duration::from_millis(200))
}

impl SettingsModel {
    pub fn new(save_takes: Duration) -> Self {
        Self {
            state: Arc::default(),
            notifier: Arc::default(),
            save_takes,
        }
    }

    fn change(&self, f: impl FnOnce(&mut SettingsState)) {
        f(&mut self.state.lock().unwrap());
        self.notifier.changed();
    }

    fn save(&self) {
        self.change(|state| {
            state.saving = true;
            state.status = "Saving…".into();
        });
        let (state, notifier, takes) = (self.state.clone(), self.notifier.clone(), self.save_takes);
        thread::spawn(move || {
            thread::sleep(takes);
            let mut state = state.lock().unwrap();
            state.saving = false;
            state.saves += 1;
            state.saved = true;
            state.status = match state.saves {
                1 => "Saved".into(),
                n => format!("Saved {n} times"),
            };
            drop(state);
            notifier.changed();
        });
    }
}

impl PageModel for SettingsModel {
    type State = SettingsState;
    type Action = SettingsAction;

    fn state(&self) -> SettingsState {
        self.state.lock().unwrap().clone()
    }

    fn send(&self, action: SettingsAction) {
        match action {
            SettingsAction::SetLaunchAtStartup(on) => self.change(|s| {
                s.launch_at_startup = on;
                edited(s);
            }),
            SettingsAction::SetAutoUpdate(on) => self.change(|s| {
                s.auto_update = on;
                edited(s);
            }),
            SettingsAction::Save => self.save(),
            SettingsAction::Reset => self.change(|s| *s = SettingsState::default()),
            SettingsAction::SetShowDisabledPlugins(on) => {
                self.change(|s| s.show_disabled_plugins = on)
            }
            SettingsAction::TogglePlugin(id) => self.change(|s| {
                if let Some(plugin) = s.plugins.iter_mut().find(|p| p.id == id) {
                    plugin.enabled = !plugin.enabled;
                    edited(s);
                }
            }),
        }
    }

    fn notifier(&self) -> &Notifier {
        &self.notifier
    }
}

fn edited(state: &mut SettingsState) {
    state.saved = false;
    state.status = "Unsaved changes".into();
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    /// A model whose saves are instant, and a channel that hears each change.
    fn model() -> (SettingsModel, mpsc::Receiver<()>) {
        let model = SettingsModel::new(Duration::ZERO);
        let (tx, rx) = mpsc::channel();
        model.notifier().subscribe(move || {
            let _ = tx.send(());
        });
        (model, rx)
    }

    #[test]
    fn a_save_finishes_on_its_own_thread() {
        let (model, changes) = model();
        model.send(SettingsAction::SetAutoUpdate(true));
        assert!(model.state().auto_update && !model.state().saved);
        model.send(SettingsAction::Save);
        assert!(model.state().saving);
        // The edit, the start of the save, and its end.
        for _ in 0..3 {
            changes.recv_timeout(Duration::from_secs(5)).unwrap();
        }
        let state = model.state();
        assert_eq!(
            (
                state.saving,
                state.saved,
                state.saves,
                state.status.as_str()
            ),
            (false, true, 1, "Saved")
        );
        model.send(SettingsAction::Reset);
        assert_eq!(model.state(), SettingsState::default());
    }

    #[test]
    fn hides_disabled_plugins_until_asked() {
        let (model, _) = model();
        let names = |m: &SettingsModel| {
            m.state()
                .visible_plugins()
                .into_iter()
                .map(|p| p.name)
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&model), ["Spell check", "Translate"]);
        assert!(model.state().has_hidden_plugins());
        model.send(SettingsAction::SetShowDisabledPlugins(true));
        assert_eq!(names(&model), ["Spell check", "Grammar", "Translate"]);
        model.send(SettingsAction::SetShowDisabledPlugins(false));
        model.send(SettingsAction::TogglePlugin(2));
        assert!(!model.state().has_hidden_plugins() && !model.state().saved);
    }

    /// The page's path to Rust: the UniFFI object `#[page_model]` emitted,
    /// as the native side drives it.
    #[test]
    fn the_bridge_reaches_the_model_and_calls_back() {
        use std::sync::atomic::{AtomicU32, Ordering};

        use crate::model::__dazzui_bridge_settings::*;

        struct Count(AtomicU32);
        impl SettingsListener for Count {
            fn state_changed(&self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let source = SettingsSource::with_model(SettingsModel::new(Duration::ZERO));
        let count = Arc::new(Count(AtomicU32::new(0)));
        source.subscribe(count.clone());
        source.send(SettingsAction::SetAutoUpdate(true));
        source.send(SettingsAction::TogglePlugin(2));
        let state = source.state();
        assert!(state.auto_update && !state.saved);
        // A selector crosses as a field of the snapshot.
        assert_eq!(
            state
                .visible_plugins
                .iter()
                .map(|p| p.id)
                .collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert!(!state.has_hidden_plugins);
        assert_eq!(count.0.load(Ordering::SeqCst), 2);
        // The constructor the native side calls makes the marked function's model.
        assert_eq!(SettingsSource::new().state().status, "No changes");
    }
}
