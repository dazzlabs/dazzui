//! What an app links: the seam between a generated page and the model that
//! drives it, in Flutter's terms, and the macros that write both sides of it.
//!
//! A `ui!` page reads a state and calls a model's methods, as a Flutter
//! widget built by a `ValueListenableBuilder` reads its value and calls its
//! model. The model is any type with a `#[page_model] impl` — whose public
//! methods are what the page may call — that is also a [`ValueListenable`]:
//! a value to show, and a [`ChangeNotifier`] to say it has changed.
//!
//! How the model works inside is the app's to decide: a lock and a thread,
//! use cases behind it, or a [`Reducer`] for an app that wants Elm's shape.
//! Nothing here depends on that.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub use dazzui_compiler_macro::{page_model, selector, ui};

/// What `add_listener` returns, to remove that listener again.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ListenerId(pub u64);

type Listener = Arc<dyn Fn() + Send + Sync>;

/// Flutter's `ChangeNotifier`: listeners to add and remove, and
/// [`notify_listeners`](ChangeNotifier::notify_listeners) to call them.
#[derive(Default)]
pub struct ChangeNotifier {
    listeners: Mutex<Vec<(ListenerId, Listener)>>,
    next: AtomicU64,
}

impl ChangeNotifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_listener(&self, listener: impl Fn() + Send + Sync + 'static) -> ListenerId {
        let id = ListenerId(self.next.fetch_add(1, Ordering::Relaxed));
        self.listeners
            .lock()
            .unwrap()
            .push((id, Arc::new(listener)));
        id
    }

    pub fn remove_listener(&self, id: ListenerId) {
        self.listeners.lock().unwrap().retain(|(l, _)| *l != id);
    }

    /// Calls every listener, on this thread. No lock is held while they
    /// run, so a listener may read the value, or add or remove a listener.
    pub fn notify_listeners(&self) {
        let listeners: Vec<Listener> = self
            .listeners
            .lock()
            .unwrap()
            .iter()
            .map(|(_, l)| l.clone())
            .collect();
        for listener in listeners {
            listener();
        }
    }
}

impl std::fmt::Debug for ChangeNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.listeners.lock().map(|l| l.len()).unwrap_or(0);
        f.debug_struct("ChangeNotifier")
            .field("listeners", &n)
            .finish()
    }
}

/// Flutter's `ValueListenable`: a value, and whom to tell when it changes.
///
/// The value is a snapshot, read whole each time: across the bridge, one
/// call for the whole state is cheaper than one per field. A model may call
/// [`ChangeNotifier::notify_listeners`] from any thread; the native side
/// moves to its UI thread itself.
pub trait ValueListenable: Send + Sync + 'static {
    type Value;

    fn value(&self) -> Self::Value;

    fn notifier(&self) -> &ChangeNotifier;
}

/// Flutter's `ValueNotifier`: one value, set whole, that tells its
/// listeners when it is.
#[derive(Debug, Default)]
pub struct ValueNotifier<T> {
    value: Mutex<T>,
    notifier: ChangeNotifier,
}

impl<T> ValueNotifier<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Mutex::new(value),
            notifier: ChangeNotifier::new(),
        }
    }

    pub fn set(&self, value: T) {
        *self.value.lock().unwrap() = value;
        self.notifier.notify_listeners();
    }

    /// Changes the value in place, then tells the listeners.
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.value.lock().unwrap());
        self.notifier.notify_listeners();
    }
}

impl<T: Clone + Send + 'static> ValueListenable for ValueNotifier<T> {
    type Value = T;

    fn value(&self) -> T {
        self.value.lock().unwrap().clone()
    }

    fn notifier(&self) -> &ChangeNotifier {
        &self.notifier
    }
}

/// Elm's shape, for a model that wants it: a state, and one function that
/// changes it for an action. [`dispatch`](Reducer::dispatch) applies the
/// action and tells the listeners, on this thread. A model's methods
/// dispatch to it, and it is the model's value.
pub struct Reducer<S, A> {
    state: ValueNotifier<S>,
    update: fn(&mut S, A),
}

impl<S, A> Reducer<S, A> {
    pub fn new(initial: S, update: fn(&mut S, A)) -> Self {
        Self {
            state: ValueNotifier::new(initial),
            update,
        }
    }

    pub fn dispatch(&self, action: A) {
        self.state.update(|state| (self.update)(state, action));
    }
}

impl<S: Clone + Send + 'static, A: 'static> ValueListenable for Reducer<S, A> {
    type Value = S;

    fn value(&self) -> S {
        self.state.value()
    }

    fn notifier(&self) -> &ChangeNotifier {
        self.state.notifier()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;

    #[test]
    fn a_listener_hears_until_it_is_removed() {
        let notifier = ChangeNotifier::new();
        let heard = Arc::new(AtomicU32::new(0));
        let h = heard.clone();
        let id = notifier.add_listener(move || {
            h.fetch_add(1, Ordering::SeqCst);
        });
        notifier.notify_listeners();
        notifier.remove_listener(id);
        notifier.notify_listeners();
        assert_eq!(heard.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_reducer_applies_the_action_then_tells_its_listeners() {
        let model = Arc::new(Reducer::new(0u32, |n, by: u32| *n += by));
        let heard = Arc::new(AtomicU32::new(0));
        let (m, h) = (model.clone(), heard.clone());
        // A listener reads the value again, as the native side does.
        model
            .notifier()
            .add_listener(move || h.store(m.value(), Ordering::SeqCst));
        model.dispatch(2);
        model.dispatch(3);
        assert_eq!((model.value(), heard.load(Ordering::SeqCst)), (5, 5));
    }
}
