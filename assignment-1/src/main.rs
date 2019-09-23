use std::time::Instant;

mod merge;
mod quick;

fn main() {
	let time = Instant::now();
	quick::main();
	println!("Execution took {:?}", time.elapsed());
}
