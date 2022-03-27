use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = "data/course_data/wsj.small.xml";
    // let filename = "data/course_data/wsj.xml";
    let test_file = fs::read_to_string(filename)?;
    let lower_test_file = test_file.to_ascii_lowercase();
    parse_very_hack(&lower_test_file);
    Ok(())
}

// Current runtime: 103 seconds.
// Perhaps can be faster by no converting to chars, and istead just incrementing a pointer to the string slice.

fn parse_very_hack(contents: &str) {
    let mut chars = contents.chars();
    let mut tag_mode = true;

    let mut tag = String::new();
    let mut word = String::new();

    let mut dict: HashSet<String> = HashSet::new();
    let mut index: HashMap<String, Vec<u32>> = HashMap::new();
    let mut current_doc_num = 0;
    // println!("Chars: {:?}", chars);

    while let Some(c) = chars.next() {
        // let c = c.to_ascii_lowercase();
        if tag_mode {
            tag.push(c);
            if c == '>' {
                tag_mode = false;
                if tag == "<doc>" {
                    println!("New doc: {}", current_doc_num);
                    current_doc_num += 1;
                }
                tag.clear();
            }
        } else {
            if c == '<' {
                tag.push(c);
                tag_mode = true;
                if word != "" {
                    // println!("{}", word.clone());
                    dict.insert(word.clone());
                    if dict.contains(&word) {
                        index.get(&word).unwrap().push(current_doc_num);
                    } else {
                        index.insert(word.clone(), vec![current_doc_num]);
                    }
                    word.clear();
                }
                continue;
            }
            if c.is_ascii_alphanumeric() {
                word.push(c);
            }
            if c == ' ' || c == '\n' {
                if word != "" {
                    // println!("{}", word.clone());
                    dict.insert(word.clone());
                    if dict.contains(&word) {
                        index.get(&word.clone()).unwrap().push(current_doc_num);
                    } else {
                        index.insert(word.clone(), vec![current_doc_num]);
                    }
                    word.clear();
                }
            }
        }
    }

    // println!("Tags: {}", tag);
    println!("Words: {}", dict.len());
    println!("Index: {:?}", index);
}
