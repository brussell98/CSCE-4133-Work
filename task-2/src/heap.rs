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

// Array-based heap
//                 v  v children at 5 and 6
// [1, 2, 3, 4, 5, 6, 7, 8]
//        ^ parent at 2
// Children are index * 2 + (1 or 2)
// Parent is (index - 1) / 2
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
	// Build a heap in-place from an unsorted tree
	fn heapify(&mut self) {
		let length = self.array.len() - 1;
		let mut start = Heap::parent_of(length); // Start at last tree
		loop {
			self.sift_down(start, length);
			if start == 0 { // If top of heap
				return;
			}
			start -= 1;
		}
	}
	// Make sure the tree is a max heap by moving the root node down if smaller than its children
	fn sift_down(&mut self, start: usize, end: usize) {
		let mut root = start;
		let mut child = Heap::child_of(root, true);

		while child <= end { // While the current node has child nodes
			let mut swap = root; // Larger node to swap with

			if self.array[swap] < self.array[child] { // If left child is greater, swap with it
				swap = child;
			}
			if child + 1 <= end && self.array[swap] < self.array[child + 1] { // If right child exists and is greatest swap with it
				swap = child + 1;
			}

			if swap == root { // If both children are smaller it is a max heap. They are known heaps because bottom-up so exit.
				return;
			} else { // Move the larger node up and continue down the tree form where the root was swapped to
				self.array.swap(root, swap);
				root = swap;
			}

			child = Heap::child_of(root, true); // Update the child index
		}
	}
	fn sort(&mut self) {
		self.heapify(); // Make sure the the max heap conditions are satisfied

		// Pop the top of the heap until the array is sorted
		let mut end = self.array.len() - 1;
		while end > 0 { // Swap the root (greatest) element to the end, and sift the swapped element down to rebuild the heap
			self.array.swap(end, 0);
			end -= 1; // Exclude the sorted element
			self.sift_down(0, end); // Bring the largest node to the top
		}
	}
}
