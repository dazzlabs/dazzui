pub struct S {
    pub on: bool,
    pub opened: std::time::Instant,
}

pub enum A {
    Save,
}

#[app]
pub fn update(state: &mut S, action: A) {}

pub struct T {
    pub on: bool,
}

ui! {
    page P(state: S, action: A) {
        Text("a")
    }
}

ui! {
    page Q(state: T, action: A) {
        Text("b")
    }
}
