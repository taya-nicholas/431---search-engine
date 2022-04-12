use std::{
    fs::{self, File},
    io::{stdin, Read},
    path::Path,
};

mod inverted_index;
mod parser;
mod search;

pub const WSJ_PATH: &str = "./data/course_data/wsj.xml";

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn create_index() {
    if !Path::new("./index").exists() {
        fs::create_dir("./index").unwrap();
    }

    let filepath = Path::new(WSJ_PATH);
    let doc_collection_contents = read_file(filepath);

    let mut p = parser::new();
    p.parse(&doc_collection_contents);
    p.create_doc_offset_file();
    p.create_doc_length_file();
    let parsed_contents = p.get_parsed_contents();

    let mut t = inverted_index::new();
    t.create_tree_from_parsed_contents(parsed_contents);
    t.encode_dgap();
    t.create_postings_and_vocab();
    t.create_persistent_btree();
}

pub fn start_search() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    let input = input.trim().to_ascii_lowercase();
    let input: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();
    let query: Vec<&str> = input.split_whitespace().collect();
    let mut s = search::new();

    for term in query {
        s.search(term.to_string());
    }
    let merged = s.merge_postings();
    match merged {
        Some(list) => s.display_postings(list),
        None => (),
    }
}
