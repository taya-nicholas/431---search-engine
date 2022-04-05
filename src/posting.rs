use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use bincode::config;

use crate::indexer::BMap;

pub fn test_posting() {
    println!("Run testing postings");
    let post = load_single_tree();
    serialize_posting(post);
}

fn load_single_tree() -> Vec<(u32, u32)> {
    let node_to_search = "bono";
    let mut file_path = PathBuf::from("./nodes/").join(node_to_search);
    file_path.set_extension("tree");

    let config = config::standard();
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);
    let decoded: BMap = bincode::decode_from_reader(file, config).unwrap();

    println!("loaded btree length: {}", decoded.btree.len());
    let test_term = decoded.btree.get("bono").unwrap();
    println!("bono term length: {}", test_term.len());
    println!("bono contents: {:?}", test_term);
    return test_term.clone();
}

fn serialize_posting(post: Vec<(u32, u32)>) {
    println!("Vec 1: {:?}", post[0]);

    let config = config::standard();
    let bytes = bincode::encode_to_vec(post.clone(), config).unwrap();
    println!("Encoded {:?}, into {:?}", post, bytes);
    println!("There are {} bytes total", bytes.len());

    let mut file = File::create("test.bin").unwrap();
    bincode::encode_into_std_write(post.clone(), &mut file, config).unwrap();

    // let decoded: <Vec(u32, u32)> =
    //     bincode::decode_from_slice(&bytes[..], config).unwrap();

    // println!("Decoded: {:?}, len {}", decoded, len);

    let reader = File::open("test.bin").unwrap();
    let reader = BufReader::new(reader);

    let decoded_from_file: Vec<(u32, u32)> = bincode::decode_from_reader(reader, config).unwrap();
    println!("Decoded from file: {:?}", &decoded_from_file);

    let mut reader = File::open("test.bin").unwrap();
    let bytes_to_read = bytes.len();
    let mut buf = vec![0u8; bytes_to_read];
    reader.read_exact(&mut buf).unwrap();
    println!("buf: {:?}, len: {}", buf, buf.len());
    let (decoded_from_exact, _): (Vec<(u32, u32)>, _) =
        bincode::decode_from_slice(&buf, config).unwrap();

    println!(
        "Exact: {:?}, \n\nlen:{}",
        decoded_from_exact,
        decoded_from_exact.len()
    );
}
