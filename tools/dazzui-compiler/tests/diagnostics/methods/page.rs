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
            Button(on_click: model.sav, child: Text("a")),
            Button(on_click: model.set_on, child: Text("b")),
            Button(on_click: model.save(true), child: Text("c")),
            Button(on_click: model.pick(model.name), child: Text("d")),
            Switch(checked: model.on, on_checked_change: model.rename),
            Button(on_click: state.save, child: Text("e")),
        ])
    }
}
