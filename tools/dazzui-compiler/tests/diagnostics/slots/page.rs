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

pub struct M;

impl ValueListenable for M {
    type Value = S;
}

#[page_model]
impl M {
    pub fn new() -> Self {
        M
    }

    pub fn save(&self) {}

    pub fn set_on(&self, on: bool) {}

    pub fn rename(&self, name: String) {}

    pub fn pick(&self, id: u32) {}
}

ui! {
    page P(model: M) {
        List(children: [
            ListRow(title: "a", icon: "x"),
            ListRow(title: "b", children: [Switch(checked: model.on), Text("y")]),
            Text("c"),
        ])
    }
}
