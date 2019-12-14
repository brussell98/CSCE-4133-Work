use std::io;
use std::io::BufRead;

/// Find the maximum number of tasks that can be completed for each schedule
pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	let mut tasks = Vec::with_capacity(100_000);

	// Parse input into tasks vector, then run get_answers
	for line in lines[2..].iter() {
		if !line.contains(' ') {
			get_answer(&mut tasks);
			tasks.clear(); // Clear tasks vector for next test case
		} else {
			// Parse start and end time
			let input: Vec<u32> = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
			tasks.push((input[0], input[1]));
		}
	}

	get_answer(&mut tasks);
}

/// Find the maximum number of tasks that can be completed
fn get_answer(tasks: &mut Vec<(u32, u32)>) {
	// Sort tasks by end time
	// Implementation based on Pattern-Defeating Quicksort https://github.com/orlp/pdqsort
	tasks.sort_unstable_by(|a, b| a.1.cmp(&b.1));

	let mut time = 0;
	let mut completed = 0;

	// Find the task that ends first from the possible tasks, then set the time to its end time
	for task in tasks {
		if task.0 >= time {
			time = task.1;
			completed += 1;
		}
	}

	println!("{}", completed);
}
