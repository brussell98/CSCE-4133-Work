use std::time::Instant;

mod heap;
mod cows;

fn main() {
    let time = Instant::now();
    cows::main();
    println!("Execution took {:?}", time.elapsed());
}
