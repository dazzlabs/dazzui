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

#[app]
pub fn update(state: &mut S, action: A) {}

ui! {
    page P(state: S, action: A) {
        VStack {
            Toggle { value: bind(state.on) }
            Button { title: bind(state.name) }
        }
    }
}
