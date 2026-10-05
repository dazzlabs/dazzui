// A page and nothing to drive it: its host brings the state and takes the
// actions, from Rust or not.
pub struct S {
    pub name: String,
    pub on: bool,
}

pub enum A {
    Save,
    SetOn(bool),
}

ui! {
    page P(state: S, action: A) {
        VStack(spacing: 8) {
            Text(state.name)
            Toggle { value: bind(state.on), on_change: Action::SetOn }
            Button("Save").on_click(Action::Save)
        }
    }
}
