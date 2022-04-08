use std::{
    fs::File,
    io::{stdin, Read},
    path::Path,
    time::Instant,
};

mod inverted_index;
mod parser;
mod search;

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn create_index() {
    println!("create index here");
    let filepath = Path::new("./data/course_data/wsj.xml");
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
    println!("Enter search term");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    let query = input.trim();

    let now = Instant::now();
    let mut s = search::new();
    s.search(query.to_string());
    let elapsed = now.elapsed();
    println!("Search time: {:.5?}", elapsed.as_secs_f64())
}
