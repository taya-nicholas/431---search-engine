mod btree;
mod index;
mod indexer;
mod parser;

// TODO - split index into multiple files.
// TODO - display doc num for returned search.
// TODO - add and sort by TF.IDF.

use std::{fs::File, io::Read, path::Path};

pub fn run_build() {
    println!("Running program");
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    let parsed_contents = parser::parse_words(&contents);
    let mut bmap = indexer::BMap::new();
    bmap.create_tree(&parsed_contents);
    btree::create_persistent_btree(bmap);
}

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn run_load() {
    println!("Loading files");
}
