use std::time::Instant;

mod busyman;
mod mst;

fn main() {
    let time = Instant::now();
    mst::main();
    println!("Execution took {:?}", time.elapsed());
}
