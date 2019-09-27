use std::time::Instant;

mod merge;
mod quick;
mod distribute;

fn main() {
	let time = Instant::now();
	merge::main();
	println!("Execution took {:?}", time.elapsed());
}
