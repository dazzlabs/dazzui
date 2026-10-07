//! A counter in Elm's shape: a `Reducer` holds the state and runs
//! `update`, and the model's methods — what the page calls — dispatch to it.

use dazzui_runtime::{ChangeNotifier, Reducer, ValueListenable, page_model, ui};

uniffi::setup_scaffolding!();

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CounterState {
    pub count: i32,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CounterAction {
    Increment,
    Decrement,
    Reset,
}

pub fn update(state: &mut CounterState, action: CounterAction) {
    match action {
        CounterAction::Increment => state.count += 1,
        CounterAction::Decrement => state.count -= 1,
        CounterAction::Reset => state.count = 0,
    }
    state.label = format!("Count: {}", state.count);
}

pub struct CounterModel {
    store: Reducer<CounterState, CounterAction>,
}

#[page_model]
impl CounterModel {
    pub fn new() -> Self {
        Self {
            store: Reducer::new(CounterState::default(), update),
        }
    }

    pub fn increment(&self) {
        self.store.dispatch(CounterAction::Increment);
    }

    pub fn decrement(&self) {
        self.store.dispatch(CounterAction::Decrement);
    }

    pub fn reset(&self) {
        self.store.dispatch(CounterAction::Reset);
    }
}

impl Default for CounterModel {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueListenable for CounterModel {
    type Value = CounterState;

    fn value(&self) -> CounterState {
        self.store.value()
    }

    fn notifier(&self) -> &ChangeNotifier {
        self.store.notifier()
    }
}

ui! {
    page CounterPage(model: CounterModel) {
        Row(gap: "2", children: [
            Button(on_click: model.decrement, child: Text("−")),
            Text(model.label, strong: true),
            Button(on_click: model.increment, child: Text("+")),
            Button(variant: ButtonVariant::Plain, on_click: model.reset, child: Text("Reset")),
        ])
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::__dazzui_bridge_counter as bridge;
    use super::CounterState;

    struct Count(AtomicU32);
    impl bridge::CounterListener for Count {
        fn changed(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn the_bridge_runs_the_reducer() {
        let native = bridge::CounterModel::new();
        let heard = Arc::new(Count(AtomicU32::new(0)));
        native.add_listener(heard.clone());
        native.increment();
        native.increment();
        native.decrement();
        let value = native.value();
        assert_eq!((value.count, value.label.as_str()), (1, "Count: 1"));
        assert_eq!(heard.0.load(Ordering::SeqCst), 3);
        let _ = CounterState::default();
    }
}
