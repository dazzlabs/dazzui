pub struct S {
    pub on: bool,
    pub opened: std::time::Instant,
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
}

pub struct T {
    pub on: bool,
}

pub struct N;

#[page_model]
impl N {
    pub fn new() -> Self {
        N
    }
}

pub struct U {
    pub on: bool,
}

pub struct O;

impl ValueListenable for O {
    type Value = U;
}

#[page_model]
impl O {
    pub fn new() -> Self {
        O
    }

    pub fn at(&self, when: std::time::Instant) {}
}

pub struct PModel;

impl ValueListenable for PModel {
    type Value = U;
}

#[page_model]
impl PModel {
    pub fn new() -> Self {
        PModel
    }
}

pub struct P;

impl ValueListenable for P {
    type Value = U;
}

#[page_model]
impl P {
    pub fn new() -> Self {
        P
    }
}
