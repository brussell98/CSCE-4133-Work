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

		// Remove this test case's words and convert them to &str
		let mut words: Vec<String> = lines.drain(..length).collect();

		// Sort them
		quick_sort(&mut words, 0, length - 1);
		for word in words {
			println!("{}", word);
		}
	}
}

fn partition(array: &mut Vec<String>, low: usize, high: usize) -> usize {
	let pivot = low;
	let mut i = (low + 1) as usize; // +1 since low was used for the pivot
	let mut j = high as usize;

	loop {
		// Find the next element from the left that is greater than the pivot
		while i < j && array[i] <= array[pivot] {
			i = i + 1;
		}
		// Find the next element from the right that is less than the pivot
		while j >= i && array[j] >= array[pivot] {
			j = j - 1;
		}
		// If not all elements have been checked, swap i and j and continue
		if i < j {
			array.swap(i, j);
			i = i + 1;
			j = j - 1;
		} else { // Otherwise swap the pivot and first and return where the pivot is
			array.swap(pivot, j);
			return j;
		}
	}
}

fn quick_sort(array: &mut Vec<String>, low: usize, high: usize) {
	if high - low < 2 { // If only one element is given return with no modifications
		return;
	}

	let p = partition(array, low, high);
	// Recursively sort the two halves
	quick_sort(array, low, p);
	quick_sort(array, p + 1, high);
}
