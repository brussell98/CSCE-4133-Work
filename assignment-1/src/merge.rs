use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let mut lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	lines.remove(0); // First input is useless to me

	while lines.len() > 0 { // While there are more test cases
		let length = lines[0].parse::<i32>().unwrap() as usize; // Parse first line for number of words
		lines.remove(0); // Remove it from the vector

        // Remove this test case's words and convert them to &str
        let tmp_words: Vec<String> = lines.drain(..length).collect();
        let mut words = Vec::with_capacity(length);
        for i in 0..length {
            words.push(tmp_words[i].as_str());
        }

		// Sort them
        let sorted = merge_sort(&words);
        for word in sorted {
            println!("{}", word);
        }
	}
}

fn merge(array1: &Vec<&str>, array2: &Vec<&str>) -> Vec<String> {
    let mut i1: usize = 0;
    let l1 = array1.len();
    let mut i2: usize = 0;
    let l2 = array2.len();
    let mut out = Vec::with_capacity(l1 + l2);

    // While both arrays still have elements left, compare them and push the lowest
    while i1 < l1 && i2 < l2 {
        if array1[i1] < array2[i2] {
            out.push(String::from(array1[i1]));
            i1 = i1 + 1;
        } else {
            out.push(String::from(array2[i2]));
            i2 = i2 + 1;
        }
    }

    // Push the remaining elements of whichever array still has some
    for i in i1..l1 {
        out.push(String::from(array1[i]));
    }

    for i in i2..l2 {
        out.push(String::from(array2[i]));
    }

    out
}

fn merge_sort(array: &[&str]) -> Vec<String> {
    let length = array.len();
    if length < 2 { // If there is nothing to sort just return
        return Vec::from(array).iter().map(|s| String::from(*s)).collect();
    }

    // Split the array into two by the midpoint
    // Note: In rust I can't use arrays because the length must be known at compile-time
    let mid = length / 2;
    let array1 = Vec::from_iter(array[0..mid].iter().cloned()); // Not including mid
    let array2 = Vec::from_iter(array[mid..length].iter().cloned());

    // Sort the two vectors
    let array1 = merge_sort(&array1);
    let array1 = array1.iter().map(AsRef::as_ref).collect();
    let array2 = merge_sort(&array2);
    let array2 = array2.iter().map(AsRef::as_ref).collect();
    // Merge them
    merge(&array1, &array2)
}
