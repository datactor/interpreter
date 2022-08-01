use std::collections::VecDeque;

pub(crate) struct List {
    lane: VecDeque<Box<dyn Element>>,
}

impl List {
    pub(crate) fn new() -> Self {
        List { lane: VecDeque::new() }
    }

    fn append(&mut self, element: Box<dyn Element>) {
        self.lane.push_back(element);
    }

    fn pop(&mut self) {
        self.lane.pop_back().unwrap();
    }
}


pub trait Element {
    fn vals(&self) -> String;
}

pub(crate) struct I32 {
    pub val: String,
}

impl Element for I32 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

pub(crate) struct I8 {
    pub val: String,
}

impl Element for I8 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

pub(crate) struct I64 {
    pub val: String,
}

impl Element for I64 {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

pub(crate) struct Str {
    pub val: String,
}

impl Element for Str {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}

pub(crate) struct Bool {
    pub val: String,
}

impl Element for Bool {
    fn vals(&self) -> String {
        format!("{}", self.val)
    }
}