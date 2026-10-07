#![allow(unused)]

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

pub struct M;

impl dazzui_runtime::ValueListenable for M {
    type Value = S;

    fn value(&self) -> S {
        unimplemented!()
    }

    fn notifier(&self) -> &dazzui_runtime::ChangeNotifier {
        unimplemented!()
    }
}

impl M {
    pub fn save(&self) {}

    pub fn set_on(&self, on: bool) {}

    pub fn rename(&self, name: String) {}

    pub fn pick(&self, id: u32) {}
}

ui! {
    page P(model: M) {
        Col(children: [
            for item in model.items {
                Text(item.titel)
            },
        ])
    }
}

fn main() {}
