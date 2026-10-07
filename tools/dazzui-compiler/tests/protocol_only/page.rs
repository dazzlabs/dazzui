// A page and the model it calls. The page holds the model only as the
// protocol of its methods, so a host may hand it something else entirely.
pub struct S {
    pub name: String,
    pub on: bool,
}

pub struct NotesModel;

impl ValueListenable for NotesModel {
    type Value = S;
}

#[page_model]
impl NotesModel {
    pub fn new() -> Self {
        NotesModel
    }

    pub fn save(&self) {}

    pub fn set_on(&self, on: bool) {}
}

ui! {
    page P(model: NotesModel) {
        Col(children: [
            Text(model.name),
            Switch(checked: model.on, on_checked_change: model.set_on),
            Button(on_click: model.save, child: Text("Save")),
        ])
    }
}
