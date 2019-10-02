use std::io;
use std::io::BufRead;

pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let mut lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	lines.remove(0); // First input is useless to me

	while lines.len() > 0 { // While there are more test cases
		let length = lines[0].parse::<i32>().unwrap() as usize; // Parse first line for number of words
		lines.remove(0); // Remove it from the vector

		// Remove this test case's words
		let words: Vec<String> = lines.drain(..length).collect();

		// Heap sort the words
		let mut word_heap = Heap::new(words);
		word_heap.sort();
		for word in word_heap.array {
			println!("{}", word);
		}
	}
}

struct Heap {
	array: Vec<String>
}

impl Heap {
	fn new(array: Vec<String>) -> Heap {
		Heap {
			array
		}
	}
	fn parent_of(i: usize) -> usize {
		(i - 1) / 2 // Floored by data type
	}
	fn child_of(i: usize, left: bool) -> usize {
		i * 2 + if left { 1 } else { 2 }
	}
	fn heapify(&mut self) {
		let length = self.array.len() - 1; // Fix borrow error caused by outdated rust compiler on spoj
		let mut start = Heap::parent_of(self.array.len() - 1);
		loop {
			self.sift_down(start, length);
			if start == 0 {
				return;
			}
			start -= 1;
		}
	}
	fn sift_down(&mut self, start: usize, end: usize) {
		let mut root = start;
		while Heap::child_of(root, true) <= end {
			let child = Heap::child_of(root, true);
			let mut swap = root;

			if self.array[swap] < self.array[child] {
				swap = child;
			}
			if child + 1 <= end && self.array[swap] < self.array[child + 1] {
				swap = child + 1;
			}
			if swap == root {
				return;
			} else {
				self.array.swap(root, swap);
				root = swap;
			}
		}
	}
	fn sort(&mut self) {
		self.heapify();

		let mut end = self.array.len() - 1;
		while end > 0 {
			self.array.swap(end, 0);
			end -= 1;
			self.sift_down(0, end);
		}
	}
}
