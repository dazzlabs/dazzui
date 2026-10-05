use dazzui_compiler_macro::ui;

pub struct S {
    pub on: bool,
    pub name: String,
    pub items: Vec<Item>,
}

pub struct Item {
    pub id: u32,
    pub title: String,
}

pub enum A {
    Save,
    SetOn(bool),
    Rename(String),
    Pick(u32),
}

ui! {
    page P(state: S, action: A) {
        VStack(spacing: 8) {
            Text(state.name).style(TextStyle::Title)
            Toggle { value: bind(state.on), on_change: Action::SetOn }
            if !state.on { Text("Off") } else { Text("On") }
            for item in state.items {
                Button(item.title).on_click(Action::Pick(item.id))
            }
            Button("Save").style(ButtonStyle::Primary).disabled(state.on).on_click(Action::Save)
        }
    }
}

fn main() {}
