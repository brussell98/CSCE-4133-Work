use std::io;
use std::io::BufRead;
use std::cmp::Ordering;

/// Insert and find the index of elements in a binary search tree
pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let queries: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	let mut tree = BST::new();

	for query in queries[1..].iter() {
		// Parse start and end
		let input: Vec<u128> = query.split_whitespace().map(|n| n.parse::<u128>().unwrap()).collect();
		let insert = if input[0] == 1 { true } else { false };
		let key = input[1];

		if insert {
			tree.insert(key);
		} else {
			let index = tree.get_index(key);
			if index.is_some() {
				println!("{}", index.unwrap());
			} else {
				println!("Data tidak ada"); // Translation: Missing data
			}
		}
	}
}

#[derive(Debug)]
struct Node<T> {
	key: T,
	index: u64,
	left: Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>
}

impl<T: Ord> Node<T> {
	fn new(t: T) -> Node<T> {
		Node {
			key: t,
			index: 1,
			left: None,
			right: None
		}
	}
	fn insert(&mut self, n: Node<T>) {
		match n.key.cmp(&self.key) {
			Ordering::Less => {
				match self.left {
					None => self.left = Some(Box::new(n)), // If no node, insert here
					Some(ref mut left) => left.insert(n), // Else recurse
				}
			},
			Ordering::Greater => {
				self.index += 1; // If inserting into the right subtree, increment the index of the current node
				match self.right {
					None => self.right = Some(Box::new(n)),
					Some(ref mut right) => right.insert(n),
				}
			},
			_ => {} // Don't insert duplicate keys
		}
	}
	fn get_index(&self, key: T, sum: &mut u64) -> Option<u64> {
		match key.cmp(&self.key) {
			// If found, return the sum + the found node's index
			Ordering::Equal => {
				*sum += self.index;
				Some(*sum)
			},
			// If traversing left, add the right subtree length to the index sum (all these elements are greater)
			Ordering::Less => {
				*sum += self.index;
				if let Some(ref left) = self.left {
					if left.get_index(key, sum).is_some() {
						Some(*sum)
					} else {
						None
					}
				} else {
					None
				}
			},
			Ordering::Greater => {
				if let Some(ref right) = self.right {
					if right.get_index(key, sum).is_some() {
						Some(*sum)
					} else {
						None
					}
				} else {
					None
				}
			}
		}
	}
}

#[derive(Debug)]
struct BST<T> {
	root: Option<Box<Node<T>>>
}
impl<T: Ord> BST<T> {
	fn new() -> BST<T> {
		BST {
			root: None
		}
	}
	fn insert(&mut self, t: T) {
		match self.root {
			// If there is no root, insert as root
			None => self.root = Some(Box::new(Node::new(t))),
			Some(ref mut root) => root.insert(Node::new(t))
		}
	}

	fn get_index(&self, t: T) -> Option<u64> {
		match self.root {
			None => None,
			Some(ref root) => root.get_index(t, &mut 0)
		}
	}
}
