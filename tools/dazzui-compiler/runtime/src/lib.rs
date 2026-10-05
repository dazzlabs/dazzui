//! What an app links: the seam between a generated page and whatever drives
//! it, and the macros that write both sides of it.
//!
//! A `ui!` page is a function of a state and a way to send actions; it knows
//! nothing of where either comes from. [`PageModel`] is the one thing the
//! compiler asks of the Rust side that drives a page: a snapshot of the
//! state, a way to take an action, and a [`Notifier`] to say the state has
//! changed. Mark a function that makes one with `#[page_model]` and the
//! bridge carries it to the native side.
//!
//! How the model works inside is the app's to decide — a reducer, a view
//! model calling use cases, a presenter over a store. [`Reducer`] is one
//! such model, for an app that wants Elm's shape and nothing more; nothing
//! else here depends on it.

use std::sync::{Arc, Mutex};

pub use dazzui_compiler_macro::{page_model, selector, ui};

/// What drives a page: a state to show and somewhere to send actions.
///
/// `send` returns as soon as the model has taken the action. A model that
/// changes its state then — or later, on another thread, when some work is
/// done — calls [`Notifier::changed`], and the native side reads [`state`]
/// again. The native side moves to its UI thread itself, so `changed` may be
/// called from any thread.
///
/// [`state`]: PageModel::state
pub trait PageModel: Send + Sync + 'static {
    type State;
    type Action;

    /// A snapshot of the state as the page shows it.
    fn state(&self) -> Self::State;

    fn send(&self, action: Self::Action);

    /// Whom to tell when the state changes.
    fn notifier(&self) -> &Notifier;
}

type Listener = Arc<dyn Fn() + Send + Sync>;

/// The listeners of a [`PageModel`]: the native side subscribes, the model
/// calls [`changed`](Notifier::changed).
#[derive(Default)]
pub struct Notifier {
    listeners: Mutex<Vec<Listener>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self, listener: impl Fn() + Send + Sync + 'static) {
        self.listeners.lock().unwrap().push(Arc::new(listener));
    }

    /// Calls every listener, on this thread. No lock is held while they
    /// run, so a listener may read the state, or subscribe another.
    pub fn changed(&self) {
        let listeners = self.listeners.lock().unwrap().clone();
        for listener in listeners {
            listener();
        }
    }
}

impl std::fmt::Debug for Notifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let n = self.listeners.lock().map(|l| l.len()).unwrap_or(0);
        f.debug_struct("Notifier").field("listeners", &n).finish()
    }
}

/// A [`PageModel`] in Elm's shape: a state, and a function that changes it
/// for an action. The action is applied before `send` returns, and the
/// listeners are told on the same thread.
pub struct Reducer<S, A> {
    state: Mutex<S>,
    update: fn(&mut S, A),
    notifier: Notifier,
}

impl<S, A> Reducer<S, A> {
    pub fn new(initial: S, update: fn(&mut S, A)) -> Self {
        Self {
            state: Mutex::new(initial),
            update,
            notifier: Notifier::new(),
        }
    }
}

impl<S, A> PageModel for Reducer<S, A>
where
    S: Clone + Send + 'static,
    A: Send + 'static,
{
    type State = S;
    type Action = A;

    fn state(&self) -> S {
        self.state.lock().unwrap().clone()
    }

    fn send(&self, action: A) {
        (self.update)(&mut self.state.lock().unwrap(), action);
        self.notifier.changed();
    }

    fn notifier(&self) -> &Notifier {
        &self.notifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn a_reducer_applies_the_action_then_tells_its_listeners() {
        let model = Arc::new(Reducer::new(0u32, |n, by: u32| *n += by));
        let heard = Arc::new(AtomicU32::new(0));
        let (m, h) = (model.clone(), heard.clone());
        // A listener reads the state again, as the native side does.
        model
            .notifier()
            .subscribe(move || h.store(m.state(), Ordering::SeqCst));
        model.send(2);
        model.send(3);
        assert_eq!((model.state(), heard.load(Ordering::SeqCst)), (5, 5));
    }
}
