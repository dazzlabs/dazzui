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
            if state.name { Text("a") }
            for item in state.count { Text("b") }
            for tag in state.tags { Text(tag.label) }
            for item in state.items {
                Text(state.name)
                for other in state.items { Text("c") }
            }
        }
    }
}
