use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    time::Instant,
};

#[test]
fn chars_to_bytes() {
    let s = "There is something here\n Don't listen to him".to_string();
    let chars = s.chars();
    let byte_len = std::mem::size_of::<char>();
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    parse_words(&contents);
    assert_eq!(5, byte_len);
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
    let mut word_count = 0;
    let mut words = String::new();

    let mut count: u32 = 0;
    let mut doc_offsets = vec![];

    let now = Instant::now();
    while let Some(mut c) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            while c != '>' {
                tag.push(c);
                c = chars.next().unwrap();
                count += 1;
            }
            if tag == "<doc" {
                // println!("");
                words.push('\n');
            } else if tag == "<docno" {
                // if count < 3000 {
                //     println!("Count: {}", count + 1);
                // }
                doc_offsets.push(count + 1);
            }
        } else {
            if c == ' ' || c == '\n' || c == '-' {
                if !temp_word.is_empty() {
                    // println!("{}", &temp_word);
                    words.push('\n');
                    words.push_str(&temp_word);
                    word_count += 1;
                }
                temp_word.clear();
            } else if c.is_ascii_alphanumeric() {
                temp_word.push(c);
            }
        }
        count += 1;
    }
    let elapsed = now.elapsed();
    println!("Parse XML elapsed: {:.5?}", elapsed.as_secs_f64());
    println!("Word count: {}", word_count);
    create_doc_offset_index(doc_offsets);
    words
}

fn create_doc_offset_index(doc_offsets: Vec<u32>) {
    let mut file = File::create("doc_offsets.bin").unwrap();
    for offset in doc_offsets {
        let bytes = offset.to_be_bytes();
        let bytes_written = file.write(&bytes).unwrap();
    }
}
