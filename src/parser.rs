use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    time::Instant,
};

#[test]
fn chars_to_bytes() {
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    parse_words(&contents);
    assert_eq!(5, 5);
}

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

pub fn parse_words(contents: &str) -> String {
    let lower_case_contents = contents.to_ascii_lowercase();
    let mut chars = lower_case_contents.chars();
    let mut temp_word = String::new();
    let mut total_word_count = 0;
    let mut words = String::new();

    let mut char_count: u32 = 0;
    let mut doc_offsets = vec![];

    let mut doc_word_count: u32 = 0;
    let mut doc_lengths: Vec<u32> = vec![];

    let now = Instant::now();
    while let Some(mut c) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            while c != '>' {
                tag.push(c);
                c = chars.next().unwrap();
                char_count += 1;
            }
            if tag == "<doc" {
                words.push('\n');
                if doc_word_count > 0 {
                    doc_lengths.push(doc_word_count);
                    doc_word_count = 0;
                }
            } else if tag == "<docno" {
                doc_offsets.push(char_count + 1);
            }
        } else {
            if c == ' ' || c == '\n' || c == '-' {
                if !temp_word.is_empty() {
                    words.push('\n');
                    words.push_str(&temp_word);
                    doc_word_count += 1;
                    total_word_count += 1;
                }
                temp_word.clear();
            } else if c.is_ascii_alphanumeric() {
                temp_word.push(c);
            }
        }
        char_count += 1;
    }
    // push last doc length (as it is normally pushed as start of next doc).
    doc_lengths.push(doc_word_count);
    let elapsed = now.elapsed();
    println!("Parse XML elapsed: {:.5?}", elapsed.as_secs_f64());
    println!("Word count: {}", total_word_count);
    create_doc_offset_index(doc_offsets);
    create_doc_length_index(doc_lengths);
    words
}

fn create_doc_offset_index(doc_offsets: Vec<u32>) {
    let mut file = File::create("doc_offsets.bin").unwrap();
    let mut total_bytes = 0;
    let len = doc_offsets.len();

    for offset in doc_offsets {
        let bytes = offset.to_be_bytes();
        let bytes_written = file.write(&bytes).unwrap();
        total_bytes += bytes_written;
    }
    println!("TOTAL BYTES: {}, for len {}", total_bytes, len);
}

fn create_doc_length_index(doc_lengths: Vec<u32>) {
    let num_docs = doc_lengths.len() as u32;
    let mut file = File::create("doc_lengths.bin").unwrap();
    let mut total_bytes = 0;

    //Write num docs as first bytes
    let bytes = num_docs.to_be_bytes();
    let bytes_written = file.write(&bytes).unwrap();
    println!("first bytes written: {}", bytes_written);
    total_bytes += bytes_written;

    for length in doc_lengths {
        let bytes = length.to_be_bytes();
        let bytes_written = file.write(&bytes).unwrap();
        total_bytes += bytes_written;
    }
    println!("TOTAL BYTES Lengths: {}", total_bytes);
    println!("Num DOCS: {}", num_docs);
}
