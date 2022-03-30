use bincode::{config, encode_into_std_write, Decode, Encode};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::{fs::File, io::Read};

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "data/course_data/wsj.xml";
    // let filename = "data/course_data/wsj.xml";
    let test_file = fs::read_to_string(filename)?;
    let lower_test_file = test_file.to_ascii_lowercase();
    parse_very_hack(&lower_test_file);
    Ok(())
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Index {
    ii: HashMap<String, Vec<u32>>,
}

fn parse_very_hack(contents: &str) {
    let mut chars = contents.chars();
    let mut temp_word = String::new();
    let mut index: HashMap<String, Vec<u32>> = HashMap::new();
    let mut temp_vec = vec![];
    let mut doc_id = 0;

    while let Some(mut c) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            while c != '>' {
                tag.push(c);
                c = chars.next().unwrap();
            }
            if tag == "<doc" {
                doc_id += 1;
            }
        } else {
            if !c.is_ascii_alphanumeric() {
                if !temp_word.is_empty() {
                    // Complete word found - do something here.
                    match index.get_mut(&temp_word) {
                        Some(vec) => {
                            vec.push(doc_id);
                        }
                        None => {
                            let vec = vec![doc_id];
                            index.insert(temp_word.clone(), vec);
                        }
                    }
                    temp_vec.push(temp_word.clone()); // just for testing, remove in final
                }
                temp_word.clear();
            } else {
                temp_word.push(c);
            }
        }
    }
    // println!("Index: {:?}", index);
    println!("Len: {:?}", index.len());

    let index_serial = Index { ii: index };
    let config = config::standard();

    let mut file = File::create("index.bin").unwrap();
    encode_into_std_write(index_serial, &mut file, config).unwrap();
}
