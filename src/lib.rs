mod btree;
mod index;
mod indexer;
mod parser;
mod posting;
mod search;

// TODO - add and sort by TF.IDF.

use std::{
    fs::File,
    io::{stdin, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    time::Instant,
    vec,
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

            let relevance_postings = rank_postings(index_postings);
            display_postings(relevance_postings);
            // println!("Posting: {:?}", index_postings.first());
            // println!("Posting: {:?}", index_postings.last());
        }
        None => {
            println!("No result");
        }
    }
    let elapsed = now.elapsed();
    println!("Search time: {:.5?}", elapsed.as_secs_f64());
}

#[test]
fn test_retieve_doc_lengths() {
    let mock_postings = vec![(1, 5)];
    let len_list = rank_postings(mock_postings);
    println!("Len: {:?}", len_list);
}

// TF.IDF = tf/doc_len * num_of_docs / num_of_docs_with_term

fn rank_postings(posting_list: Vec<(u32, u32)>) -> Vec<(u32, f64)> {
    // Attempting rank based on single postings list.
    // With have to change things when merging, as postings_list.len() will
    // no longer represent num_of_docs_with_term.
    let mut file = File::open("./doc_lengths.bin").unwrap();

    let mut buf: [u8; 4] = [0u8; 4];
    file.read_exact(&mut buf).unwrap();
    let collection_length: u32 = u32::from_be_bytes(buf);
    println!("Collection len: {}", collection_length);

    let posting_length: u32 = posting_list.len() as u32;
    let mut doc_lengths = vec![];

    let mut relevance_postings = vec![];
    for (doc_id, tf) in posting_list {
        // Read doc length in doc_lengths.bin file by seek_read, based on doc_id number.
        let length_offset = (doc_id) * 4; // -1 to start at 0, +1 because first element is collection length
        file.seek(SeekFrom::Start(length_offset as u64)).unwrap();
        let mut buf: [u8; 4] = [0u8; 4];
        file.read_exact(&mut buf).unwrap();
        let doc_length: u32 = u32::from_be_bytes(buf);
        doc_lengths.push(doc_length);

        let score: f64 = (f64::from(tf) / f64::from(doc_length))
            * (f64::from(collection_length) / f64::from(posting_length));
        relevance_postings.push((doc_id, score));
    }
    // In order to sort by float (second element in tuple)
    relevance_postings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    return relevance_postings;
}

fn display_postings(posting_list: Vec<(u32, f64)>) {
    let now = Instant::now();
    let len = posting_list.len();

    let mut file = File::open("./doc_offsets.bin").unwrap();
    let mut doc_file = File::open("./data/course_data/wsj.xml").unwrap();

    // let mut output = String::new();

    for (doc_id, score) in posting_list {
        let offset_offset = (doc_id - 1) * 4; // doc_id as index (from 0). Each offset is 4 bytes, so must be shifted.
        file.seek(SeekFrom::Start(offset_offset as u64)).unwrap();
        // let mut buf = vec![0u8; 4]; // read 4 bytes
        let mut buf: [u8; 4] = [0u8; 4];
        // println!("Seek from : {}", offset_offset as u64);
        file.read_exact(&mut buf).unwrap();
        let doc_offset: u32 = u32::from_be_bytes(buf);

        doc_file.seek(SeekFrom::Start(doc_offset as u64)).unwrap();

        let mut label_buf = vec![0u8; 16]; // journal tags, including spaces are all 16 characters
        doc_file.read_exact(&mut label_buf).unwrap();
        let s = std::str::from_utf8(&label_buf).unwrap();
        println!("{} - {}", s.trim(), score);
    }

    // println!("Output: {}", output);

    let elapsed = now.elapsed();
    println!("For {} documents", len);
    println!("Display postings elapsed: {:.5?}", elapsed.as_secs_f64());
}
