use std::collections::BTreeMap;

use bincode::{Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BMap {
    pub btree: BTreeMap<String, Vec<(u32, u32)>>,
}

impl BMap {
    pub fn new() -> BMap {
        return BMap {
            btree: BTreeMap::new(),
        };
    }

    pub fn create_tree(&mut self, words: &str) {
        let mut doc_num = 0;
        let mut words_in_document = 0;
        for word in words.strip_prefix("\n").unwrap().lines() {
            if word.is_empty() {
                doc_num += 1;
            } else {
                words_in_document += 1;
                self.add_word(word, doc_num);
            }
        }
        // let num_documents = self.btree.len();
        // self.btree
        //     .insert("@Lengths".to_string(), vec![(num_documents as u32, 0)]);
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
}
