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
    let input = input.trim().to_ascii_lowercase();
    let query: Vec<&str> = input.split_whitespace().collect();

    let now = Instant::now();
    let mut s = search::new();

    let s_time = Instant::now();
    for term in query {
        s.search(term.to_string());
    }
    let s_elapsed = s_time.elapsed();
    let m_time = Instant::now();
    let merged = s.merge_postings();
    let m_elapsed = m_time.elapsed();
    let d_time = Instant::now();
    match merged {
        Some(list) => s.display_postings(list),
        None => println!("No postings to display"),
    }
    let d_elapsed = d_time.elapsed();

    let elapsed = now.elapsed();
    println!("Simple search time: {:.5?}", s_elapsed.as_secs_f64());
    println!("Merge time: {:.5?}", m_elapsed.as_secs_f64());
    println!("Display time: {:.5?}", d_elapsed.as_secs_f64());
    println!("Search time: {:.5?}", elapsed.as_secs_f64());
}
