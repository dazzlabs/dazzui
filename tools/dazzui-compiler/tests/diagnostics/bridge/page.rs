pub struct S {
    pub on: bool,
    pub opened: std::time::Instant,
}

pub enum A {
    Save,
}

#[page_model]
pub fn model() -> Reducer<S, A> {
    todo!()
}

pub struct M;

#[page_model]
pub fn other() -> M {
    M
}

pub struct U {
    pub on: bool,
}

#[page_model]
pub fn first() -> Reducer<U, A> {
    todo!()
}

#[page_model]
pub fn second() -> Reducer<U, A> {
    todo!()
}

// A page needs no model: a view of a state is a page whoever holds it.
ui! {
    page P(state: U, action: A) {
        Text("a")
    }
}
