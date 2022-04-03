use bincode::{config, encode_into_std_write, Decode, Encode};
use core::num;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env::temp_dir;
use std::error::Error;
use std::io::BufReader;
use std::io::{prelude::*, SeekFrom};
use std::time::Instant;
use std::{fs, vec};
use std::{fs::File, io::stdin, io::Read};

// TODO - split index into multiple files.
// TODO - display doc num for returned search.
// TODO - add and sort by TF.IDF.

pub fn testing() {
    println!("Testing start");
    let filename = "data/course_data/wsj.xml";

    let now = Instant::now();
    let mut f = File::open(filename).unwrap();
    let mut buffer = [0; 14];
    // f.read(&mut buffer).unwrap();
    // let test_file = fs::read(filename);
    f.seek(SeekFrom::Start(14)).unwrap();
    f.read_exact(&mut buffer).unwrap();

    println!("Buff: {:?}", buffer);

    let x = String::from_utf8(buffer.to_vec()).unwrap();
    println!("X: {}", x);

    let elapsed = now.elapsed();
    println!("Read file elapsed: {:.5?}", elapsed.as_secs_f64());
}

pub fn run() {
    // create_index();
    let now = Instant::now();
    let index = load_index();
    let elapsed = now.elapsed();
    println!("Load index elapsed: {:.5?}", elapsed.as_secs_f64());

    println!("Enter search term");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    println!("Searching for: {}", s);

    let now = Instant::now();
    index.search(s.trim());
    let elapsed = now.elapsed();
    println!("Search elapsed: {:.5?}", elapsed.as_secs_f64());
}

fn create_index() {
    let filename = "data/course_data/wsj.xml";
    let now = Instant::now();
    let test_file = fs::read_to_string(filename).unwrap();
    let elapsed = now.elapsed();
    println!("Read file elapsed: {:.5?}", elapsed.as_secs_f64());

    let now = Instant::now();
    let lower_test_file = test_file.to_ascii_lowercase();
    let elapsed = now.elapsed();
    println!("Convert lowercase elapsed: {:.5?}", elapsed.as_secs_f64());

    let mut index = Index::new();
    let words = index.parse_words(&lower_test_file);

    let now = Instant::now();
    index.create_tree(&words);
    index.encode_to_d_gap();
    let elapsed = now.elapsed();
    println!("Make tree elapsed: {:.5?}", elapsed.as_secs_f64());

    let now = Instant::now();
    index.save_file();
    let elapsed = now.elapsed();
    println!("Save file elapsed: {:.5?}", elapsed.as_secs_f64());
}

fn load_index() -> Index {
    let config = config::standard();
    let file = File::open("index.bin").unwrap();
    let file = BufReader::new(file);
    let decoded: Index = bincode::decode_from_reader(file, config).unwrap();
    return decoded;
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Index {
    btree: BTreeMap<String, Vec<(u32, u32)>>,
}

impl Index {
    fn new() -> Index {
        return Index {
            btree: BTreeMap::new(),
        };
    }

    fn parse_words(&self, contents: &str) -> String {
        let mut chars = contents.chars();
        let mut temp_word = String::new();
        let mut word_count = 0;
        let mut words = String::new();

        let now = Instant::now();
        while let Some(mut c) = chars.next() {
            if c == '<' {
                let mut tag = String::new();
                while c != '>' {
                    tag.push(c);
                    c = chars.next().unwrap();
                }
                if tag == "<doc" {
                    // println!("");
                    words.push('\n');
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
        }
        let elapsed = now.elapsed();
        println!("Parse XML elapsed: {:.5?}", elapsed.as_secs_f64());
        println!("Word count: {}", word_count);
        words
    }

    fn create_tree(&mut self, words: &str) {
        let mut doc_num = 0;
        for line in words.strip_prefix("\n").unwrap().lines() {
            if line.is_empty() {
                doc_num += 1;
            } else {
                self.add_word(line, doc_num);
            }
        }
        let num_documents = self.btree.len();
        self.btree
            .insert("@Lengths".to_string(), vec![(num_documents as u32, 0)]);
    }

    fn add_word(&mut self, word: &str, doc_num: u32) {
        match self.btree.get_mut(word) {
            Some(vec) => {
                // if most recent posting has current doc_id, then increment word count, else add new posting.
                // Change to struct for readability if it doesn't decrease performance too much.
                if vec.last().unwrap().0 == doc_num {
                    let mut temp_vec = vec.pop().unwrap();
                    temp_vec.1 = temp_vec.1 + 1;
                    vec.push(temp_vec);
                } else {
                    vec.push((doc_num, 1));
                }
            }
            None => {
                let vec = vec![(doc_num, 1)];
                self.btree.insert(word.to_string(), vec);
            }
        }
    }

    fn encode_to_d_gap(&mut self) {
        for (key, value) in self.btree.iter_mut() {
            let mut prev_doc = 0;
            for posting in value {
                let temp_doc = posting.0.clone();
                posting.0 = posting.0 - prev_doc;
                prev_doc = temp_doc;
            }
        }
    }

    fn decode_from_d_gap(&self, mut list: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        let mut new_list: Vec<(u32, u32)> = vec![];
        let mut pre_doc = 0;
        for posting in list.iter_mut() {
            posting.0 += pre_doc;
            pre_doc = posting.0;
        }

        return list;
    }

    fn convert_to_tfidf(&self, list: Vec<(u32, u32)>) {
        let docs_containing_t = list.len();
        let num_documents = self.btree.get("@Lengths").unwrap()[0].0;
        println!("nt: {}, N: {}", docs_containing_t, num_documents);
        let idf: f32 = num_documents as f32 / docs_containing_t as f32;
        println!("idf: {}", idf);
    }

    fn search(self, search_term: &str) {
        match self.btree.get(search_term) {
            Some(vec) => {
                println!("Found: {:?}", vec);
                let decode_vec = self.decode_from_d_gap(vec.clone());
                println!("Found decoded: {:?}", decode_vec);
                self.convert_to_tfidf(decode_vec);
            }
            None => {
                println!("No results found for: {}", search_term);
                println!("Chars: {:?}", search_term.chars());
            }
        }
    }

    fn save_file(self) {
        let index_serial = self.btree;
        let config = config::standard();
        let mut file = File::create("index.bin").unwrap();
        encode_into_std_write(index_serial, &mut file, config).unwrap();
    }

    fn parse_contents(&mut self, contents: &str) {
        let mut chars = contents.chars();
        let mut temp_word = String::new();
        // let mut index: BTreeMap<String, Vec<(u32, u32)>> = BTreeMap::new();
        let mut doc_id = 0;

        let now = Instant::now();
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
                        match self.btree.get_mut(&temp_word) {
                            Some(vec) => {
                                // vec.push(doc_id);
                                // if most recent posting has current doc_id, then increment word count, else add new posting.
                                if vec.last().unwrap().0 == doc_id {
                                    let mut temp_vec = vec.pop().unwrap();
                                    temp_vec.1 = temp_vec.1 + 1;
                                    vec.push(temp_vec);
                                } else {
                                    vec.push((doc_id, 1));
                                }
                            }
                            None => {
                                let vec = vec![(doc_id, 1)];
                                self.btree.insert(temp_word.clone(), vec);
                            }
                        }
                    }
                    temp_word.clear();
                } else {
                    temp_word.push(c);
                }
            }
        }
        let elapsed = now.elapsed();
        println!("Parse XML elapsed: {:.5?}", elapsed.as_secs_f64());

        // println!("Index: {:?}", index);
        println!("Len: {:?}", self.btree.len());
        println!("Max doc id: {}", &doc_id);

        // let index_serial = Index { ii: index };
        // let config = config::standard();

        // let mut file = File::create("index.bin").unwrap();

        // let now = Instant::now();
        // encode_into_std_write(index_serial, &mut file, config).unwrap();

        // let elapsed = now.elapsed();
        // println!("Store file elapsed: {:.5?}", elapsed.as_secs_f64());
        // return index_serial;
    }
}
