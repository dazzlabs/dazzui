#![allow(unused)]

use dazzui_compiler_macro::{page_model, selector};

pub struct S {
    pub on: bool,
}

pub struct M;

#[page_model]
impl M {
    pub fn new() -> Self {
        M
    }

    pub fn set(&self, a: u32, b: u32) {}
}

impl S {
    #[selector]
    pub fn count(self) -> u32 {
        0
    }
}

fn main() {}
