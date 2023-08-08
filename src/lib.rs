#[doc = include_str!("../README.md")]
use std::collections::HashMap;

struct Node {
	node_number: u32,
	edges: HashMap<char, Option<Box<Node>>>,
}

struct Dfa {
	table: Vec<Option<Box<Node>>>,
}

impl Dfa {
	fn new_node(num: u32) -> Node {
		Node {
			node_number: num,
			edges: HashMap::new(),
		}
	}

	fn new(self, init_string: &str) -> Self {
		Self {
			table: Vec::with_capacity(100),
		}
	}

	fn new_regex(regex_string: &str) {}
}
