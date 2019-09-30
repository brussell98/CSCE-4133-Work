use std::time::Instant;

mod heap;

fn main() {
    let time = Instant::now();
    heap::main();
    println!("Execution took {:?}", time.elapsed());
}
