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

	fn look_up_where_to_insert_node(self, node_to_insert: Node) -> u32 {
		todo!("Function should tell us where to insert the node")
	}

	pub fn new(self, init_string: &str) -> Self {
		Self {
			node_count: 0,
			table: Vec::with_capacity(100),
		}
	}

	pub fn new_regex(&mut self, regex_string: &str) {
		let first_char = match regex_string.chars().nth(0) {
			Some(character) => character,
			None => panic!("Unable to parse string"),
		};
		let init_node: Node = Dfa::new_node(self.node_count + 1);
		todo!("we should check where to insert our node, insert it then do the same for the rest of character");
	}
}
