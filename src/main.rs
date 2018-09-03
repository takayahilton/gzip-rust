extern crate bit_vec;

use std::fs::File;
use std::io::{BufReader, Read};

mod huffman_tree;
use huffman_tree::HuffmanTree;


fn main() {
    let mut f = BufReader::new(File::open("test.txt").unwrap());
    let mut raw_text: String = String::new();
    f.read_to_string(& mut raw_text).unwrap();

    let ht = HuffmanTree::new(&raw_text);

    let encoded = ht.encode(&raw_text);

    println!("{:?}", &encoded);
    println!("encoded length {:?}", encoded.to_bytes().len());
    println!("raw str length {:?}", raw_text.len());

    let decoded = ht.decode(&encoded);
    println!("{}", &decoded);

    println!("{}", &raw_text);

}
