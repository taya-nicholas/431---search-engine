use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
    time::Instant,
};

use bincode::config;

use crate::inverted_index::Vocab;

pub struct Search {
    root: Vec<String>,
    multi_postings: Vec<Vec<(u32, f64)>>,
    merged_postings: Vec<(u32, f64)>,
}

pub fn new() -> Search {
    Search {
        root: vec![],
        multi_postings: vec![],
        merged_postings: vec![],
    }
}

impl Search {
    pub fn search(&mut self, query: String) {
        if self.root.is_empty() {
            self.load_root();
        }
        let leaf = self.find_leaf_index(&query);
        let leaf_tree = self.load_index(leaf);
        match leaf_tree.btree.get(&query) {
            Some(info) => {
                println!("Found: {:?}", info);
                let postings = self.seek_read_posting(info.disk_bytes, info.disk_offset);
                let index_postings = self.decode_dgap(postings);
                let relevance_postings = self.rank_postings(index_postings);
                self.multi_postings.push(relevance_postings);
            }
            None => {
                println!("Nothing found");
            }
        }
    }

    fn find_leaf_index(&mut self, search_term: &str) -> String {
        let node_to_search: String;
        match self.root.binary_search(&search_term.to_string()) {
            Ok(i) => {
                println!("Found: {}, at index {}", search_term, i);
                node_to_search = self.root[i].clone();
                return node_to_search;
            }
            Err(i) => {
                println!(
                    "Didn't find: {}. Look in block at index {} - ({})",
                    search_term,
                    i - 1,
                    self.root[i - 1]
                );
                node_to_search = self.root[i - 1].clone();
                return node_to_search;
            }
        }
    }

    fn decode_dgap(&self, mut list: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        let now = Instant::now();

        let mut pre_doc = 0;
        for posting in list.iter_mut() {
            posting.0 += pre_doc;
            pre_doc = posting.0;
        }

        let elapsed = now.elapsed();
        println!("Decode dgap elapsed: {:.5?}", elapsed.as_secs_f64());

        return list;
    }
}

// file loads
impl Search {
    fn load_root(&mut self) {
        let now = Instant::now();

        let config = config::standard();
        let file = File::open("./root.tree").unwrap();
        let file = BufReader::new(file);
        let decoded: Vec<String> = bincode::decode_from_reader(file, config).unwrap();
        self.root = decoded;

        let elapsed = now.elapsed();
        println!("Load root elapsed: {:.5?}", elapsed.as_secs_f64());
    }

    fn load_index(&self, leaf_name: String) -> Vocab {
        let mut file_path = PathBuf::from("./nodes/").join(leaf_name);
        file_path.set_extension("tree");
        let now = Instant::now();

        let config = config::standard();
        let file = File::open(file_path).unwrap();
        let file = BufReader::new(file);
        let decoded: Vocab = bincode::decode_from_reader(file, config).unwrap();

        let elapsed = now.elapsed();
        println!("Load Node elapsed: {:.5?}", elapsed.as_secs_f64());
        return decoded;
    }

    fn seek_read_posting(&self, disk_length: usize, disk_offset: usize) -> Vec<(u32, u32)> {
        let now = Instant::now();

        let config = config::standard();
        let mut reader = File::open("postings.bin").unwrap();
        reader.seek(SeekFrom::Start(disk_offset as u64)).unwrap();
        let bytes_to_read = disk_length;
        let mut buf = vec![0u8; bytes_to_read];
        reader.read_exact(&mut buf).unwrap();
        let (decoded_from_exact, _): (Vec<(u32, u32)>, _) =
            bincode::decode_from_slice(&buf, config).unwrap();

        let elapsed = now.elapsed();
        println!(
            "Seek reading posting elapsed: {:.5?}",
            elapsed.as_secs_f64()
        );

        return decoded_from_exact;
    }
}

impl Search {
    fn rank_postings(&self, posting_list: Vec<(u32, u32)>) -> Vec<(u32, f64)> {
        // TF.IDF = tf/doc_len * num_of_docs / num_of_docs_with_term

        // Attempting rank based on single postings list.
        // With have to change things when merging, as postings_list.len() will
        // no longer represent num_of_docs_with_term.
        let mut file = File::open("./doc_lengths.bin").unwrap();

        let mut buf: [u8; 4] = [0u8; 4];
        file.read_exact(&mut buf).unwrap();
        let collection_length: u32 = u32::from_be_bytes(buf);
        println!("Collection len: {}", collection_length);

        let posting_length: u32 = posting_list.len() as u32;
        let mut doc_lengths = vec![];

        let mut relevance_postings = vec![];
        for (doc_id, tf) in posting_list {
            // Read doc length in doc_lengths.bin file by seek_read, based on doc_id number.
            let length_offset = (doc_id) * 4; // -1 to start at 0, +1 because first element is collection length
            file.seek(SeekFrom::Start(length_offset as u64)).unwrap();
            let mut buf: [u8; 4] = [0u8; 4];
            file.read_exact(&mut buf).unwrap();
            let doc_length: u32 = u32::from_be_bytes(buf);
            doc_lengths.push(doc_length);

            let score: f64 = (f64::from(tf) / f64::from(doc_length))
                * (f64::from(collection_length) / f64::from(posting_length));
            relevance_postings.push((doc_id, score));
        }
        // In order to sort by float (second element in tuple) - do this after merge
        // relevance_postings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        return relevance_postings;
    }
}

impl Search {
    pub fn merge_postings(&mut self) -> Option<Vec<(u32, f64)>> {
        match self.multi_postings.len() {
            0 => None,
            1 => {
                let mut merged_postings = self.multi_postings.remove(0);
                merged_postings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                return Some(merged_postings);
            }
            _ => {
                let now = Instant::now();
                let mut postings = self.multi_postings.to_owned();
                let num_merges = postings.len();
                let mut flatten: Vec<(u32, f64)> = postings.into_iter().flatten().collect();

                flatten.sort_by(|a, b| a.0.cmp(&b.0));
                let mut merged_list = vec![];
                let mut doc_count = 0;
                let mut doc_target = 0;
                let mut running_score: f64 = 0.0;
                for (doc_id, score) in flatten {
                    if doc_target == doc_id {
                        doc_count += 1;
                        running_score += score;
                        if doc_count == num_merges {
                            let combined_posting = (doc_target, running_score);
                            merged_list.push(combined_posting)
                        }
                    } else {
                        doc_target = doc_id;
                        doc_count = 1;
                        running_score = score;
                    }
                }

                merged_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                let elapsed = now.elapsed();
                println!("Merge postings elapsed {:.5?}", elapsed.as_secs_f64());

                return Some(merged_list);
                // let mut tree: BTreeMap<u32, f64> = BTreeMap::new();
                // for vec in self.multi_postings.to_owned() {
                //     for posting in vec {
                //         match tree.get_mut(&posting.0) {
                //             Some(score) => {
                //                 // val present - update
                //                 *score = *score + posting.1;
                //             }
                //             None => {
                //                 // val not present, add
                //                 tree.insert(posting.0, posting.1);
                //             }
                //         }
                //     }
                // }
                // let mut merged_postings = vec![];
                // for (key, value) in tree.into_iter() {
                //     merged_postings.push((key, value));
                // }
                // merged_postings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                // // self.merged_postings = merged_postings;
                // return Some(merged_postings);
            }
        }
    }
}

impl Search {
    pub fn display_postings(&self, posting_list: Vec<(u32, f64)>) {
        let len = posting_list.len();

        let mut file = File::open("./doc_offsets.bin").unwrap();
        let mut doc_file = File::open("./data/course_data/wsj.xml").unwrap();

        // let mut output = String::new();

        for (doc_id, score) in posting_list {
            let offset_offset = (doc_id - 1) * 4; // doc_id as index (from 0). Each offset is 4 bytes, so must be shifted.
            file.seek(SeekFrom::Start(offset_offset as u64)).unwrap();
            // let mut buf = vec![0u8; 4]; // read 4 bytes
            let mut buf: [u8; 4] = [0u8; 4];
            // println!("Seek from : {}", offset_offset as u64);
            file.read_exact(&mut buf).unwrap();
            let doc_offset: u32 = u32::from_be_bytes(buf);

            doc_file.seek(SeekFrom::Start(doc_offset as u64)).unwrap();

            let mut label_buf = vec![0u8; 16]; // journal tags, including spaces are all 16 characters
            doc_file.read_exact(&mut label_buf).unwrap();
            let s = std::str::from_utf8(&label_buf).unwrap();
            println!("{} - {:.4?}", s.trim(), score);
        }

        println!("For {} documents", len);
    }
}
