use core::num;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use bincode::{config, encode_into_std_write, Decode, Encode};

use crate::{
    indexer::{self, BMap},
    parser,
};

pub fn test_posting() {
    println!("Run testing postings");
    // let post = load_single_tree("bono");
    // let post1 = load_single_tree("0");
    // serialize_posting(post, post1);
    let map = load_btree_postings();
    map_to_posting_file(map);
}

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

pub fn map_to_posting_file(map: BTreeMap<String, Vec<(u32, u32)>>) -> Vocab {
    let config = config::standard();
    let mut postings_file = File::create("./postings.bin").unwrap();
    let mut i = 0;
    let mut total_bytes = 0;
    let mut vocab_tree: BTreeMap<String, Info> = BTreeMap::new();
    for (key, value) in map.iter() {
        let bytes = bincode::encode_to_vec(value.clone(), config).unwrap();
        let num_bytes = postings_file.write(&bytes[..]).unwrap();
        println!("Key: {}, bytes: {}", key, num_bytes);
        vocab_tree.insert(
            key.to_string(),
            Info {
                length: value.len(),
                disk_bytes: num_bytes,
                disk_offset: total_bytes,
            },
        );
        total_bytes += num_bytes;
    }
    println!(
        "Vocab tree: len - {}, index 1 - {:?}",
        vocab_tree.len(),
        vocab_tree.get("tester")
    );
    let vocab = Vocab { btree: vocab_tree };
    return vocab;
    // println!("Create vocab file");
    // let config = config::standard();
    // let mut file_path = PathBuf::from("./vocab");
    // file_path.set_extension("tree");
    // let mut file = File::create(file_path).unwrap();
    // encode_into_std_write(vocab, &mut file, config).unwrap();
}

fn load_btree_postings() -> BTreeMap<String, Vec<(u32, u32)>> {
    let filepath = Path::new("./data/course_data/wsj.xml");
    let contents = read_file(filepath);
    let parsed_contents = parser::parse_words(&contents);
    let mut bmap = indexer::BMap::new();
    bmap.create_tree(&parsed_contents);
    bmap.encode_dgap();
    println!("btree of size: {}", bmap.btree.len());
    return bmap.btree;
}

fn read_file(filepath: &Path) -> String {
    let mut f = File::open(filepath).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf
}

//Testing functions

fn load_single_tree(node_name: &str) -> Vec<(u32, u32)> {
    let node_to_search = node_name;
    let mut file_path = PathBuf::from("./nodes/").join(node_to_search);
    file_path.set_extension("tree");

    let config = config::standard();
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);
    let decoded: BMap = bincode::decode_from_reader(file, config).unwrap();

    println!("loaded btree length: {}", decoded.btree.len());
    let test_term = decoded.btree.get(node_name).unwrap();
    println!("bono term length: {}", test_term.len());
    println!("bono contents: {:?}", test_term);
    return test_term.clone();
}

fn serialize_posting(post: Vec<(u32, u32)>, post1: Vec<(u32, u32)>) {
    println!("Vec 1: {:?}", post[0]);

    let config = config::standard();
    let bytes = bincode::encode_to_vec(post.clone(), config).unwrap();
    println!("Encoded {:?}, into {:?}", post, bytes);
    println!("There are {} bytes total", bytes.len());

    let mut file0 = File::create("test1.bin").unwrap();
    let num_buutes = file0.write(&bytes[..]).unwrap();
    println!("NUM BUTES: {}", num_buutes);

    let bytes2 = bincode::encode_to_vec(post1, config).unwrap();
    file0.write_all(&bytes2[..]).unwrap();
    // semi-manual slice testing
    {
        println!("bytes len: {}, bytes_2 len: {}", bytes.len(), bytes2.len());
        let mut reader = File::open("test1.bin").unwrap();
        reader.seek(SeekFrom::Start(bytes.len() as u64)).unwrap();
        let bytes_to_read = bytes2.len();
        let mut buf = vec![0u8; bytes_to_read];
        reader.read_exact(&mut buf).unwrap();
        println!("\nBuffer: {:?}, len: {}", buf, buf.len());
        let (decoded_from_exact, _): (Vec<(u32, u32)>, _) =
            bincode::decode_from_slice(&buf, config).unwrap();

        println!(
            "\nExact buffer: {:?}, \n\nlen:{}",
            decoded_from_exact,
            decoded_from_exact.len()
        );
    }

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

#[test]
fn read_doc_ref() {
    let offset: u64 = 2388;
    let bytes_to_read = 16;
    let mut reader = File::open("./data/course_data/wsj.xml").unwrap();
    reader.seek(SeekFrom::Start(offset)).unwrap();
    let mut buf = vec![0u8; bytes_to_read];
    reader.read_exact(&mut buf).unwrap();
    let s = std::str::from_utf8(&buf).unwrap();
    println!("Buf: {}", &s);
}

// Count: 13
// Count: 1166
// Count: 1677
// Count: 2087
// Count: 2388
