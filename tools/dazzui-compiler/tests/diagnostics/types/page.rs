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
        VStack(spacing: "wide") {
            Text(state.on)
            Button("a").style(ButtonStyle::Primry)
            Button("b").style(TextStyle::Title)
            HStack(spacing: state.count) { Text("c") }
        }
    }
}
