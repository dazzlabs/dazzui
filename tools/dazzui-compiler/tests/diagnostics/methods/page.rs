pub struct S {
    pub on: bool,
    pub name: String,
    pub count: u32,
    pub items: Vec<Item>,
    pub tags: Vec<Tag>,
}

pub struct Item {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

pub struct Tag {
    pub label: String,
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
            Button("b").on_click(Action::SetOn)
            Button("c").on_click(Action::Save(true))
            Button("d").on_click(Action::Pick(state.name))
            Toggle { value: state.on, on_change: Action::Rename }
        }
    }
}
