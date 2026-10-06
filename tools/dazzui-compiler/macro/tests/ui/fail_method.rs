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
        VStack {
            Button("a").on_click(Action::Sav)
            Toggle { value: bind(state.on), on_change: Action::Rename }
            Button("b").on_click(Action::Pick(state.name))
        }
    }
}

fn main() {}
