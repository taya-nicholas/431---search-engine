use crate::indexer::BMap;
use bincode::{config, encode_into_std_write, Decode, Encode};
use std::{
    borrow::Borrow,
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

pub fn create_persistent_btree(map: BMap) {
    let config = config::standard();
    let mut map_copy = map.btree.clone();
    if !Path::new("./nodes").exists() {
        fs::create_dir("./nodes").unwrap();
    }
    let length = map.btree.len();
    let block_size = (length as f64).sqrt() as usize + 1;
    println!("Map length: {}, blocksize: {}", length, block_size);

    let mut temp_vec: Vec<String> = vec![];
    for (i, (key, value)) in map.btree.iter().enumerate().rev() {
        if i % block_size == 0 {
            let mut file_path = PathBuf::from("./nodes/").join(key);
            file_path.set_extension("tree");
            println!("i: {}, word: {}", i, key);
            temp_vec.push(key.clone());
            let mut file = File::create(file_path).unwrap();
            let tree_split = map_copy.split_off(key);
            let btree_split = BMap { btree: tree_split };
            encode_into_std_write(btree_split, &mut file, config).unwrap();
        }
    }
    println!("Vec length: {}", temp_vec.len());
}
