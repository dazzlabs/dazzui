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

pub enum A {
    Save,
    SetOn(bool),
    Rename(String),
    Pick(u32),
}

use dazzui_compiler_macro::{page_model, selector};

#[page_model]
pub fn model(state: S) -> u32 {
    0
}

impl S {
    #[selector]
    pub fn count(self) -> u32 {
        0
    }
}

fn main() {}
