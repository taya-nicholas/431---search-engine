use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf, time::Instant};

use bincode::config;

use crate::indexer::BMap;

pub fn load_root(filename: String) -> Vec<String> {
    let now = Instant::now();

    let config = config::standard();
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let decoded: Vec<String> = bincode::decode_from_reader(file, config).unwrap();
    let elapsed = now.elapsed();
    println!("Load root elapsed: {:.5?}", elapsed.as_secs_f64());
    return decoded;
}

pub fn load_index(filename: PathBuf) -> BMap {
    let now = Instant::now();

    let config = config::standard();
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let decoded: BMap = bincode::decode_from_reader(file, config).unwrap();

    let elapsed = now.elapsed();
    println!("Load Node elapsed: {:.5?}", elapsed.as_secs_f64());
    return decoded;
}

pub fn search_node(
    map: BTreeMap<String, Vec<(u32, u32)>>,
    search_term: &str,
) -> Option<Vec<(u32, u32)>> {
    let now = Instant::now();
    {
        match map.get(search_term) {
            Some(val) => {
                println!("Found {}: {:?}", search_term, val.len());
                let elapsed = now.elapsed();
                println!("Node search elapsed: {:.5?}", elapsed.as_secs_f64());
                return Some(val.clone());
            }
            None => {
                println!("Found nothing for {}", search_term);
                let elapsed = now.elapsed();
                println!("Node search elapsed: {:.5?}", elapsed.as_secs_f64());
                return None;
            }
        }
    }
}

pub fn decode_dgap(mut list: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let now = Instant::now();

    let mut new_list: Vec<(u32, u32)> = vec![];
    let mut pre_doc = 0;
    for posting in list.iter_mut() {
        posting.0 += pre_doc;
        pre_doc = posting.0;
    }

    let elapsed = now.elapsed();
    println!("Decode dgap elapsed: {:.5?}", elapsed.as_secs_f64());

    return list;
}
