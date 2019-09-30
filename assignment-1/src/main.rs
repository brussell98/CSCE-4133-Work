use std::time::Instant;

mod merge;
mod quick;
mod distribute;

fn main() {
	let time = Instant::now();
	distribute::main();
	println!("Execution took {:?}", time.elapsed());
}
