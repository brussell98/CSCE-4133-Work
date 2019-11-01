use std::time::Instant;

mod expression;
mod unique;

fn main() {
	let time = Instant::now();
	unique::main();
	println!("Execution took {:?}", time.elapsed());
}
