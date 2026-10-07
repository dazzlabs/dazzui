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
        Col(gap: "2", children: [
            Text(model.name, strong: true),
            Switch(checked: model.on, on_checked_change: model.set_on),
            if !model.on { Text("Off", tone: TextTone::Secondary) } else { Text("On") },
            for item in model.items {
                Button(on_click: model.pick(item.id), child: Text(item.title))
            },
            Button(variant: ButtonVariant::Filled, disabled: model.on, on_click: model.save, child: Text("Save")),
            Button(on_click: model.rename("x"), child: Text("Rename")),
        ])
    }
}

fn main() {}
