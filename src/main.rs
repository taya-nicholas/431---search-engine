use std::path::Path;
use std::time::Instant;
use std::{fs, process};

enum Mode {
    Indexer,
    Searcher,
}

const MODE: Mode = Mode::Indexer;

fn main() {
    match MODE {
        Mode::Indexer => {
            println!("Indexer running");
            asgn1::create_index();
        }
        Mode::Searcher => {
            println!("searcher running");
        }
    }

    // let now = Instant::now();
    // // if let Err(e) = asgn1::run() {
    // //     println!("Error: {}", e);
    // //     process::exit(1);
    // // }
    // // asgn1::run_build();
    // asgn1::run_load();
    // // asgn1::run_postings();
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.5?}", elapsed.as_secs_f64());
}
