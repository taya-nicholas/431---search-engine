use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use bincode::{config, encode_into_std_write, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Vocab {
    pub btree: BTreeMap<String, Info>,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Info {
    pub length: usize,
    pub disk_bytes: usize,
    pub disk_offset: usize,
}

pub struct Index {
    btree: BTreeMap<String, Vec<(u32, u32)>>,
    vocab_tree: BTreeMap<String, Info>,
}

pub fn new() -> Index {
    Index {
        btree: BTreeMap::new(),
        vocab_tree: BTreeMap::new(),
    }
}

// Create basic b-tree
impl Index {
    pub fn create_tree_from_parsed_contents(&mut self, words: &str) {
        let mut doc_num = 0;
        for word in words.strip_prefix("\n").unwrap().lines() {
            if word.is_empty() {
                doc_num += 1;
            } else {
                self.add_word(word, doc_num);
            }
        }
    }

    fn add_word(&mut self, word: &str, doc_num: u32) {
        match self.btree.get_mut(word) {
            Some(vec) => {
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

    pub fn encode_dgap(&mut self) {
        for (_key, value) in self.btree.iter_mut() {
            let mut prev_doc = 0;
            for posting in value {
                let temp_doc = posting.0.clone();
                posting.0 = posting.0 - prev_doc;
                prev_doc = temp_doc;
            }
        }
    }
}

// Convert to vocab file
impl Index {
    pub fn create_postings_and_vocab(&mut self) {
        let config = config::standard();
        let mut postings_file = File::create("./index/postings.bin").unwrap();
        let mut total_bytes = 0;

        for (key, value) in self.btree.iter() {
            let bytes = bincode::encode_to_vec(value.clone(), config).unwrap();
            let num_bytes = postings_file.write(&bytes[..]).unwrap();
            self.vocab_tree.insert(
                key.to_string(),
                Info {
                    length: value.len(),
                    disk_bytes: num_bytes,
                    disk_offset: total_bytes,
                },
            );
            total_bytes += num_bytes;
        }
    }
}

// Create btree entry index and nodes
impl Index {
    pub fn create_persistent_btree(&self) {
        let config = config::standard();
        let mut map_copy = self.vocab_tree.clone();
        if !Path::new("./index/nodes").exists() {
            fs::create_dir("./index/nodes").unwrap();
        }

        let length = self.vocab_tree.len();
        let block_size = (length as f64).sqrt() as usize + 1;

        let mut temp_vec: Vec<String> = vec![];
        for (i, (key, _value)) in self.vocab_tree.iter().enumerate().rev() {
            if i % block_size == 0 {
                let mut file_path = PathBuf::from("./index/nodes/").join(key);
                file_path.set_extension("tree");
                // println!("i: {}, word: {}", i, key);

                temp_vec.push(key.clone());
                let mut file = File::create(file_path).unwrap();
                let tree_split = map_copy.split_off(key);
                let btree_split = Vocab { btree: tree_split };
                encode_into_std_write(btree_split, &mut file, config).unwrap();
            }
        }
        self.create_entry_index(temp_vec);
    }

    fn create_entry_index(&self, mut word_samples: Vec<String>) {
        let config = config::standard();
        let mut file_path = PathBuf::from("./index/root");
        file_path.set_extension("tree");
        let mut file = File::create(file_path).unwrap();
        word_samples.sort();
        encode_into_std_write(word_samples, &mut file, config).unwrap();
    }
}
