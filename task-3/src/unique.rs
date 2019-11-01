use std::io;
use std::io::{BufRead, Write};
// use std::time::Instant;
use self::rand::Rng;

extern crate rand;

const SEQ_LENGTH: u32 = 30_000;
const MAX_NUMBER: u32 = 1_000_000;
const QUERIES_LENGTH: u32 = 200_000;

/// Find number of distinct elements in sub-arrays
pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let mut lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	let sequence: Vec<usize> = lines[1].split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
	let qs = lines[2].parse::<usize>().unwrap();

//	let mut queries = Vec::with_capacity(qs);
//	let mut i = 0;
//	// Switched from removing element 0 to using an iterator, vastly increasing speed (it used all the time before even finishing taking in stdin)
//	for line in lines[3..].iter() {
//		// Parse start and end
//		let inputs: Vec<usize> = line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
//		let query = [inputs[0] - 1, inputs[1] - 1, i]; // 1-indexed to 0-indexed
//		queries.push(query);
//		i += 1;
//	}

	lines.remove(0);
	let mut rng = rand::thread_rng();
	let sequence: Vec<usize> = (0..SEQ_LENGTH).map(|_| {
		// inclusive to exclusive
		rng.gen_range(1, MAX_NUMBER + 1) as usize
	}).collect();
	lines.remove(0);
	lines.remove(0);
	let mut queries = Vec::with_capacity(QUERIES_LENGTH as usize);
	for i in 0..QUERIES_LENGTH {
		let left = rng.gen_range(0, SEQ_LENGTH);
		queries.push([left as usize, rng.gen_range(left, SEQ_LENGTH) as usize, i as usize])
	}

	// Sort queries by right bound
	// Rust sort_by uses an adaptive, iterative merge sort inspired by timsort
	queries.sort_by(|qa, qb| qa[1].cmp(&qb[1]));

	answer_queries(sequence, queries);
}

/// Update a BIT at index by val
fn update_bit(index: usize, val: i32, bit: &mut Vec<i32>) {
	let mut i = index;
	while i < bit.len() {
		bit[i] += val;

		// Increment index by the least significant bit
		i += ((i as isize) & -(i as isize)) as usize;
	}
}

/// Get the value in a BIT for the given index
fn query_bit(index: usize, bit: &Vec<i32>) -> i32 {
	let mut i = index;
	let mut sum = 0;
	while i > 0 {
		sum += bit[i];

		// Decrement index by the least significant bit
		i -= ((i as isize) & -(i as isize)) as usize;
	}

	sum
}

/// Given a sequence on numbers, and a right-bound sorted query set, find the number of distinct elements for each query
fn answer_queries(seq: Vec<usize>, queries: Vec<[usize; 3]>) {
	// let time1 = Instant::now();
	// Stores the last index a number was seen at, or -1
	let mut last_index = vec![-1; MAX_NUMBER as usize + 1];
	// Binary Index Tree representing TODO
	let mut bit: Vec<i32> = vec![0; seq.len() + 1];

	let mut q_count = 0; // The number of answered queries
	let mut answers = vec![0; queries.len()]; // Initialize and fill the array since it is being updated out of order

	for i in 0..seq.len() {
		// If the current element has occurred before TODO
		let el = seq[i] as usize;
		if last_index[el] != -1 {
			update_bit(last_index[el] as usize + 1, -1, &mut bit);
		}

		last_index[el] = i as i32; // Set last index for the number to this index
		update_bit(i + 1, 1, &mut bit); // TODO docs

		// If there is a query to answer where left is the current index
		while q_count < queries.len() && queries[q_count][1] == i {
			// Find the number of distinct elements using the BIT
			answers[queries[q_count][2]] = query_bit(queries[q_count][1] + 1, &bit) - query_bit(queries[q_count][0], &bit);
			q_count += 1;
		}
	}
	// let el = time1.elapsed();

	// Output all at once to avoid releasing and acquiring the lock, and flushing with every line
	println!("{}", answers.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n"));

	// println!("Answered queries in {:?}", el);
}
