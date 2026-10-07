use std::sync::Arc;
use std::thread;
use std::time::Duration;

use dazzui_runtime::{ChangeNotifier, ValueListenable, ValueNotifier, page_model};

use crate::SettingsState;

/// What drives the settings page, as a Flutter `ChangeNotifier` would: its
/// public methods are what the page calls, and its value is what the page
/// shows. Every method but `save` changes the value before it returns; a
/// save takes a while, as writing to disk would, and finishes on a thread of
/// its own, which tells the page when it is done.
pub struct SettingsModel {
    state: Arc<ValueNotifier<SettingsState>>,
    save_takes: Duration,
}

#[page_model]
impl SettingsModel {
    /// The model the native side makes when it opens the settings page.
    pub fn new() -> Self {
        Self::saving_for(Duration::from_millis(200))
    }

    pub fn set_launch_at_startup(&self, on: bool) {
        self.edit(|s| s.launch_at_startup = on);
    }

    pub fn set_auto_update(&self, on: bool) {
        self.edit(|s| s.auto_update = on);
    }

    pub fn set_show_disabled_plugins(&self, on: bool) {
        self.state.update(|s| s.show_disabled_plugins = on);
    }

    /// Turns the plugin with this id on or off.
    pub fn toggle_plugin(&self, id: u32) {
        self.edit(|s| {
            if let Some(plugin) = s.plugins.iter_mut().find(|p| p.id == id) {
                plugin.enabled = !plugin.enabled;
            }
        });
    }

    pub fn save(&self) {
        self.state.update(|s| {
            s.saving = true;
            s.status = "Saving…".into();
        });
        let (state, takes) = (self.state.clone(), self.save_takes);
        thread::spawn(move || {
            thread::sleep(takes);
            state.update(|s| {
                s.saving = false;
                s.saves += 1;
                s.saved = true;
                s.status = match s.saves {
                    1 => "Saved".into(),
                    n => format!("Saved {n} times"),
                };
            });
        });
    }

    pub fn reset(&self) {
        self.state.set(SettingsState::default());
    }
}

impl Default for SettingsModel {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsModel {
    fn saving_for(save_takes: Duration) -> Self {
        Self {
            state: Arc::new(ValueNotifier::new(SettingsState::default())),
            save_takes,
        }
    }

    /// Changes what is shown away from what was saved.
    fn edit(&self, f: impl FnOnce(&mut SettingsState)) {
        self.state.update(|s| {
            f(s);
            s.saved = false;
            s.status = "Unsaved changes".into();
        });
    }
}

impl ValueListenable for SettingsModel {
    type Value = SettingsState;

    fn value(&self) -> SettingsState {
        self.state.value()
    }

    fn notifier(&self) -> &ChangeNotifier {
        self.state.notifier()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    /// A model whose saves are instant, and a channel that hears each change.
    fn model() -> (SettingsModel, mpsc::Receiver<()>) {
        let model = SettingsModel::saving_for(Duration::ZERO);
        let (tx, rx) = mpsc::channel();
        model.notifier().add_listener(move || {
            let _ = tx.send(());
        });
        (model, rx)
    }

    #[test]
    fn a_save_finishes_on_its_own_thread() {
        let (model, changes) = model();
        model.set_auto_update(true);
        assert!(model.value().auto_update && !model.value().saved);
        model.save();
        assert!(model.value().saving);
        // The edit, the start of the save, and its end.
        for _ in 0..3 {
            changes.recv_timeout(Duration::from_secs(5)).unwrap();
        }
        let state = model.value();
        assert_eq!(
            (
                state.saving,
                state.saved,
                state.saves,
                state.status.as_str()
            ),
            (false, true, 1, "Saved")
        );
        model.reset();
        assert_eq!(model.value(), SettingsState::default());
    }

    #[test]
    fn hides_disabled_plugins_until_asked() {
        let (model, _) = model();
        let names = |m: &SettingsModel| {
            m.value()
                .visible_plugins()
                .into_iter()
                .map(|p| p.name)
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&model), ["Spell check", "Translate"]);
        assert!(model.value().has_hidden_plugins());
        model.set_show_disabled_plugins(true);
        assert_eq!(names(&model), ["Spell check", "Grammar", "Translate"]);
        model.set_show_disabled_plugins(false);
        model.toggle_plugin(2);
        assert!(!model.value().has_hidden_plugins() && !model.value().saved);
    }

    /// The page's path to Rust: the UniFFI object `#[page_model]` emitted,
    /// as the native side drives it.
    #[test]
    fn the_bridge_forwards_to_the_model_and_calls_back() {
        use std::sync::atomic::{AtomicU32, Ordering};

        use crate::model::__dazzui_bridge_settings as bridge;

        struct Count(AtomicU32);
        impl bridge::SettingsListener for Count {
            fn changed(&self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }

        let native = bridge::SettingsModel::with_model(SettingsModel::saving_for(Duration::ZERO));
        let count = Arc::new(Count(AtomicU32::new(0)));
        let id = native.add_listener(count.clone());
        native.set_auto_update(true);
        native.toggle_plugin(2);
        let state = native.value();
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
        native.remove_listener(id);
        native.reset();
        assert_eq!(count.0.load(Ordering::SeqCst), 2);
        // The constructor the native side calls makes the model `new` does.
        assert_eq!(bridge::SettingsModel::new().value().status, "No changes");
    }
}
