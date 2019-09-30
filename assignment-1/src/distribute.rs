use std::io;
use std::io::BufRead;
use std::cmp::min;

pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let mut lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	lines.remove(0); // First input is useless to me

	while lines.len() > 0 { // While there are more test cases
		// Parse input N and K ([0] = boxes, [1] = people)
		let inputs: Vec<usize> = lines.remove(0).split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();

		// Create vector of boxes
		let mut boxes: Vec<u64> = lines.remove(0).split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();

		// Sort boxes by increasing capacity
		quick_sort(&mut boxes, 0, inputs[0] - 1);

		let answer_subset_len = min(boxes.len(), inputs[1]);

		// If impossible, print 0 and skip wasting resources
		let sum = boxes.iter().cloned().sum::<u64>();
		if answer_subset_len != inputs[1] && sum < inputs[1] as u64 {
			println!("0");
			continue;
		}

		let mut answer = if answer_subset_len != inputs[1] {
			0 // More people than boxes, start at 0
		} else {
			boxes[boxes.len() - answer_subset_len] // The smallest possible amount that can be given (first box in set)
		};

		let mut subset: Vec<u64> = Vec::with_capacity(answer_subset_len);
		subset.extend_from_slice(&boxes[(boxes.len() - answer_subset_len)..]);

		let mut higher_bound = boxes[boxes.len() - 1]; // Largest amount in a box
		let mut lower_bound = answer; // Lowest possible amount per person
		// Right-most binary search
		while lower_bound < higher_bound {
			let mid = (lower_bound + higher_bound) / 2;
			// If possible, set answer and try one higher
			if test_candies(&mid, &inputs[1], &mut subset.clone()) {
				answer = mid;
				lower_bound = mid + 1;
			} else { // Otherwise lower the higher bound
				higher_bound = mid;
			}
		}

		println!("{}", answer);
	}
}

fn test_candies(candies: &u64, persons: &usize, boxes: &mut Vec<u64>) -> bool {
	let mut persons_left = persons.clone() as i64;
	let mut b = boxes.len() - 1;

	loop {
		if persons_left <= 0 { // If all people got candies return possible
			return true;
		}

		// If a box has enough candies, take them out
		if boxes[b] >= *candies {
			// Optimize by taking out for as many people as possible
			persons_left -= (boxes[b] / *candies) as i64;
			boxes[b] = 0; // Empty the box and advance to next one
			if b > 0 {
				b -= 1;
			}
		} else { // If no more boxes return impossible
			return false;
		}
	};
}

fn partition(array: &mut Vec<u64>, low: usize, high: usize) -> usize {
	let pivot = low;
	let mut i = (low + 1) as usize; // +1 since low was used for the pivot
	let mut j = high as usize;

	loop {
		// Find the next element from the left that is greater than the pivot
		while i < j && array[i] <= array[pivot] {
			i += 1;
		}
		// Find the next element from the right that is less than the pivot
		while j >= i && array[j] >= array[pivot] {
			j -= 1;
		}
		// If not all elements have been checked, swap i and j and continue
		if i < j {
			array.swap(i, j);
			i += 1;
			j -= 1;
		} else { // Otherwise swap the pivot and first and return where the pivot is
			array.swap(pivot, j);
			return j;
		}
	}
}

fn quick_sort(array: &mut Vec<u64>, low: usize, high: usize) {
	if high - low < 1 { // If only one element is given return with no modifications
		return;
	}

	let p = partition(array, low, high);
	// Recursively sort the two halves
	if p > 0 {
		quick_sort(array, low, p);
	}
	if p < high {
		quick_sort(array, p + 1, high);
	}
}
