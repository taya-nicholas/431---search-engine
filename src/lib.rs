mod btree;
mod index;
mod indexer;
mod parser;
mod search;

// TODO - split index into multiple files.
// TODO - display doc num for returned search.
// TODO - add and sort by TF.IDF.

use std::{fs::File, io::Read, path::Path};

use crate::indexer::BMap;

pub fn run_build() {
    println!("Running program");
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    let parsed_contents = parser::parse_words(&contents);
    let mut bmap = indexer::BMap::new();
    bmap.create_tree(&parsed_contents);
    bmap.encode_dgap();
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
    let filename: String = String::from("./nodes/testifies.tree");
    let map: BMap = search::load_index(filename);
    println!("Map: {}", map.btree.len());

    // Search node:
    let search_term = "testing";
    match search::search_node(map.btree, search_term) {
        Some(result) => {
            let postings = search::decode_dgap(result);
            println!("Posting: {:?}", postings.last());
        }
        None => {
            println!("No result");
        }
    }
}

// Loading files
// Load Node elapsed: 0.00698
// Map: 592
// Found testing: 3106
// Node search elapsed: 0.00010
// Decode dgap elapsed: 0.00000
// Posting: Some((273860375, 1))
// Elapsed: 0.00837

// 2.7 MB max (testifies.tree)

//After encoding:
// 1 MB max (testifies.tree)

// Loading files
// Load Node elapsed: 0.00536
// Map: 592
// Found testing: 3106
// Node search elapsed: 0.00014
// Decode dgap elapsed: 0.00000
// Posting: Some((173109, 1))
// Elapsed: 0.00687
