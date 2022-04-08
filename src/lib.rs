use std::{fs::File, io::Read, path::Path};

mod inverted_index;
mod parser;

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
