use std::time::Instant;

mod bst;

fn main() {
    let time = Instant::now();
    bst::main();
    println!("Execution took {:?}", time.elapsed());
}
