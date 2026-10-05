//! A counter in Elm's shape: the runtime's `Reducer` holds the state and
//! runs `update`, and `#[page_model]` carries it to the native side.

use dazzui_runtime::{Reducer, page_model, ui};

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

#[page_model]
pub fn counter() -> Reducer<CounterState, CounterAction> {
    Reducer::new(CounterState::default(), update)
}

ui! {
    page CounterPage(state: CounterState, action: CounterAction) {
        HStack(spacing: 8) {
            Button("−").on_click(Action::Decrement)
            Text(state.label)
            Button("+").on_click(Action::Increment)
            Button("Reset").on_click(Action::Reset)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::__dazzui_bridge_counter::*;

    struct Count(AtomicU32);
    impl CounterListener for Count {
        fn state_changed(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn the_bridge_runs_the_reducer() {
        let source = CounterSource::new();
        let heard = Arc::new(Count(AtomicU32::new(0)));
        source.subscribe(heard.clone());
        source.send(CounterAction::Increment);
        source.send(CounterAction::Increment);
        source.send(CounterAction::Decrement);
        assert_eq!(
            source.state(),
            CounterState {
                count: 1,
                label: "Count: 1".into()
            }
        );
        assert_eq!(heard.0.load(Ordering::SeqCst), 3);
    }
}
