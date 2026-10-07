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
        Col(children: [
            Buton(child: Text("Save")),
            Button(varient: ButtonVariant::Filled, child: Text("a")),
            Switch(checked: model.on, checked: false),
            Button("b", on_click: model.save),
            PreferenceRow(title: "c", children: [Text("d")]),
        ])
    }
}
