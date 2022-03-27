use std::process;
use std::time::Instant;
// Author: Taya Nicholas (5929161)

fn main() {
    let now = Instant::now();
    if let Err(e) = asgn1::run() {
        println!("Error: {}", e);
        process::exit(1);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed.as_secs_f64());
}
