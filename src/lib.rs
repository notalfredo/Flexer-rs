#[doc = include_str!("../README.md")]

struct Node {
    character: char,
    next: Vec<Option<Box<Dfa>>>,
}

struct Dfa {
    head: Vec<Option<Box<Dfa>>>,
}

