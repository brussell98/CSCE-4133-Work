use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

const NONE: u32 = 1_000_100; // A value greater than all possible values

/// Find the weight of the minimum spanning tree
pub fn main() {
	let stdin = io::stdin();
	// Vector of stdin lines
	let lines: Vec<String> = stdin.lock().lines().map(|el| el.unwrap()).collect();
	// [nodes <= 10,000, edges <= 100,000]
	let bounds: Vec<u32> = lines[0].split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
	// source = [(target, weight)]
	let mut adj_list: HashMap<u32, Vec<(u32, u32)>> = HashMap::with_capacity(bounds[0] as usize);
	// source = (target, weight)
	let mut least_weight: Vec<(u32, u32)> = Vec::with_capacity(bounds[0] as usize);

	// Build an adjacency list for the graph
	// All vertices are converted to 0-index
	for line in lines[1..].iter() {
		// [vertex from, vertex to, weight]
		let input: Vec<u32> = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect();
		if adj_list.contains_key(&(input[0] - 1)) {
			adj_list.get_mut(&(input[0] - 1)).unwrap().push((input[1] - 1, input[2]));
		} else {
			adj_list.insert(input[0] - 1, vec![(input[1] - 1, input[2])]);
		}
		if adj_list.contains_key(&(input[1] - 1)) {
			adj_list.get_mut(&(input[1] - 1)).unwrap().push((input[0] - 1, input[2]));
		} else {
			adj_list.insert(input[1] - 1, vec![(input[0] - 1, input[2])]);
		}
	}

	// Initialize the least weight edge to each vertex as none
	for _ in 0..bounds[0] {
		least_weight.push((NONE, NONE));
	}

	// Start at vertex 0, set least weight to each adjacent vertex as the weight from it
	for edge in adj_list.get(&0).unwrap() {
		least_weight[edge.0 as usize] = (edge.0, edge.1);
	}

	let mut total_weight: u64 = 0; // Stores the weight of the MST
	// Place all vertices, except 0, into a set marking them as not included in the MST
	let mut not_included = HashSet::with_capacity(bounds[0] as usize - 1);
	for i in 1..bounds[0] {
		not_included.insert(i);
	}

	// While not all vertices are included, add the one with the least weight
	while !not_included.is_empty() {
		// Find the edge from an included vertex with the least weight to a non-included vertex
		let mut min = (NONE, NONE); // (target, weight)
		for vert in &not_included {
			if least_weight[*vert as usize].1 < min.1 {
				min = (*vert, least_weight[*vert as usize].1);
			}
		}

		if min.1 == NONE {
			panic!("Not all nodes are reachable");
		}

		// Mark the vertex as included and add the weight of the edge to the total weight of the MST
		not_included.remove(&min.0);
		total_weight += min.1 as u64;
		// Check the edges from the vertex, and if the weight to a non-included node is less than its current least weight, update it
		// (target, weight)
		for edge in adj_list.get(&min.0).unwrap() {
			if edge.1 < least_weight[edge.0 as usize].1 {
				least_weight[edge.0 as usize] = (min.0, edge.1);
			}
		}
	}

	println!("{}", total_weight);
}
