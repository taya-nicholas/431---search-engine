use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
    time::Instant,
};

use bincode::config;

use crate::{
    indexer::BMap,
    posting::{Info, Vocab},
};

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

pub fn load_index(filename: PathBuf) -> Vocab {
    let now = Instant::now();

    let config = config::standard();
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let decoded: Vocab = bincode::decode_from_reader(file, config).unwrap();

    let elapsed = now.elapsed();
    println!("Load Node elapsed: {:.5?}", elapsed.as_secs_f64());
    return decoded;
}

pub fn search_node(map: BTreeMap<String, Info>, search_term: &str) -> Option<Info> {
    let now = Instant::now();
    {
        match map.get(search_term) {
            Some(val) => {
                let elapsed = now.elapsed();
                println!("Node search elapsed: {:.5?}", elapsed.as_secs_f64());
                println!("Found {}: {:?}", search_term, val);
                return Some(val.clone());
            }
            None => {
                let elapsed = now.elapsed();
                println!("Node search elapsed: {:.5?}", elapsed.as_secs_f64());
                println!("Found nothing for {}", search_term);
                return None;
            }
        }
    }
}

pub fn seek_read_posting(disk_length: usize, disk_offset: usize) -> Vec<(u32, u32)> {
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
