use std::path::Path;
use std::time::Instant;
use std::{fs, process};
// Author: Taya Nicholas (5929161)

fn main() {
    let now = Instant::now();
    // if let Err(e) = asgn1::run() {
    //     println!("Error: {}", e);
    //     process::exit(1);
    // }
    // asgn1::run_build();
    asgn1::run_load();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed.as_secs_f64());
}
