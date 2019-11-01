use std::io;
use std::io::BufRead;

pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let mut lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	lines.remove(0); // First input is useless to me

	while lines.len() > 0 { // While there are more test cases
		// Parse input N and C ([0] = stalls, [1] = cows)
		let inputs: Vec<usize> = lines.remove(0).split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
		let stalls = inputs[0];
		let cows = inputs[1];

		// Get the stall positions
		let positions: Vec<usize> = lines.drain(..stalls).map(|s| s.parse::<usize>().unwrap()).collect();

		// Heap sort the stall positions
		let mut stall_heap = Heap::new(positions);
		stall_heap.sort();

		// Binary search between lowest and highest possible distance
		let mut low: usize = 0;
		let mut high: usize = stall_heap.array.last().unwrap() - stall_heap.array[0] + 1;
		let mut answer = 0;
		while low < high {
			let mid = (low + high) / 2;
			if distance_is_possible(&mid, &cows, &stall_heap.array) {
				answer = mid;
				low = mid + 1;
			} else {
				high = mid;
			}
		}

		println!("{}", answer);
	}
}

fn distance_is_possible(dist: &usize, cows: &usize, positions: &Vec<usize>) -> bool {
	// Auto-fill the first stall
	let mut cows_remain = *cows - 1;
	let mut current_min = positions[0] + *dist;

	// Search through every stall and see if the distance is far enough to place a cow there
	for i in 1..positions.len() {
		if positions[i] >= current_min {
			cows_remain -= 1;
			current_min = positions[i] + *dist;
		}

		if cows_remain == 0 { // If all cows are placed the it is possible
			return true;
		}
	}

	false
}

struct Heap {
	array: Vec<usize>
}

// Array-based max heap (read top to bottom, left to right)
//                 v  v children at 5 and 6
// [1, 2, 3, 4, 5, 6, 7, 8]
//        ^ parent at 2
// Children are index * 2 + (1 or 2)
// Parent is (index - 1) / 2
impl Heap {
	fn new(array: Vec<usize>) -> Heap {
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
	// Bottom up heapify
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
	// Make sure the tree is a max heap by moving the root node down if needed
	fn sift_down(&mut self, start: usize, end: usize) {
		let mut root = start;
		let mut child = Heap::child_of(root, true);

		while child <= end {
			// Node to swap to
			let mut swap = root;

			if self.array[swap] < self.array[child] { // If left child is greater, swap with it
				swap = child;
			}
			if child + 1 <= end && self.array[swap] < self.array[child + 1] { // If right child exists and  is greatest swap with it
				swap = child + 1;
			}

			if swap == root { // If both children are smaller it is a max heap
				return;
			} else { // Move the larger node up and continue down the tree
				self.array.swap(root, swap);
				root = swap;
			}

			child = Heap::child_of(root, true);
		}
	}
	fn sort(&mut self) {
		self.heapify(); // Make sure the the max heap conditions are satisfied

		let mut end = self.array.len() - 1;
		while end > 0 { // Swap the root (greatest) element to the end, and sift the swapped element down to rebuild the heap
			self.array.swap(end, 0);
			end -= 1; // Exclude the sorted element
			self.sift_down(0, end);
		}
	}
}
