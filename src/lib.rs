#[doc = include_str!("../README.md")]
use std::collections::HashMap;

struct Node {
	node_number: u32,
	edges: HashMap<char, Option<Box<Node>>>,
}

struct Dfa {
	node_count: u32,
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
			node_count: 0,
			table: Vec::with_capacity(100),
		}
	}

	fn new_regex(&mut self, regex_string: &str) {
		let first_char = match regex_string.chars().nth(0) {
			Some(character) => character,
			None => panic!("Unable to parse string"),
		};
		let init_node: Node = Dfa::new_node(self.node_count + 1);
	}
}
