#[doc = include_str!("../README.md")]

struct Dfa {
	character: char,
	next: Vec<Option<Box<Dfa>>>,
}

impl Dfa {
	fn new_node(dfa_character: char) -> Self {
		Self {
			character: dfa_character,
			next: Vec::with_capacity(1)
		}
	}

	fn new(&mut self, init_string: &str) {
		let head: Box<Dfa> = Box::new(Dfa::new_node(match init_string.chars().nth(0) {
			Some(char) => char,
			None => panic!("Unable to parse init string"),
		}));

        //Todo: for each following character in our string
        //create a next node
        for c in init_string.chars().skip(1) {
            match self.next[0] {
                Some( ref mut x ) => println!("fdasf"),
                None => println!("fdasf")
            }
        }
        /*
            match self.head {
            None => self.head = Some(new_node),
            Some(ref mut node) => {
                let mut current = node;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
        }
        */

	}
}






/*
    let temp = head;
    while temp.next != null {
        temp = temp.next
    }
*/
