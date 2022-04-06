mod btree;
mod index;
mod indexer;
mod parser;
mod posting;
mod search;

// TODO - split index into multiple files.
// TODO - display doc num for returned search.
// TODO - add and sort by TF.IDF.

use std::{
    fs::File,
    io::{stdin, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use crate::{
    indexer::BMap,
    posting::{map_to_posting_file, Vocab},
    search::seek_read_posting,
};

pub fn run_postings() {
    posting::test_posting();
}

pub fn run_build() {
    println!("Running program");
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    let parsed_contents = parser::parse_words(&contents);
    let mut bmap = indexer::BMap::new();
    bmap.create_tree(&parsed_contents);
    bmap.encode_dgap();
    let vocab = map_to_posting_file(bmap.btree);
    btree::create_persistent_btree(vocab);
}

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn run_load() {
    println!("Enter search term");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    let mut search_term = s.trim();

    // println!("Searching for: {:?}", search_term.chars());

    let now = Instant::now();

    let root = search::load_root("./root.tree".to_string());
    let mut node_to_search: String;
    match root.binary_search(&search_term.to_string()) {
        Ok(i) => {
            println!("Found: {}, at index {}", search_term, i);
            node_to_search = root[i].clone();
        }
        Err(i) => {
            println!(
                "Didn't find: {}. Look in block at index {} - ({})",
                search_term,
                i - 1,
                root[i - 1]
            );
            node_to_search = root[i - 1].clone();
        }
    }

    let mut file_path = PathBuf::from("./nodes/").join(node_to_search);
    file_path.set_extension("tree");

    // let filename: String = String::from("./nodes/testifies.tree");
    let map: Vocab = search::load_index(file_path);
    println!("Map: {}", map.btree.len());

    // Search node:
    match search::search_node(map.btree, search_term) {
        Some(info) => {
            let postings = seek_read_posting(info.disk_bytes, info.disk_offset);
            let index_postings = search::decode_dgap(postings);
            println!("Posting: {:?}", index_postings.last());
        }
        None => {
            println!("No result");
        }
    }
    let elapsed = now.elapsed();
    println!("Search time: {:.5?}", elapsed.as_secs_f64());
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
