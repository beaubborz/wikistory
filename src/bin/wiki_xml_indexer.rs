extern crate wikistory;
use wikistory::xml_wiki_parser::generate_index as generate_index;
use std::fs::File;
use std::io::{BufReader};

fn main() {
    // 1. Open XML data file to read from:
    let file_in = File::open("./data/enwiki-20170820-pages-articles.xml").expect("File not found.");

    // 2. Create file to write to:
    let mut file_out = File::create("./data/index.csv").expect("Unable to create index file.");

    // 3. Index the file:
    generate_index(BufReader::new(file_in), &mut file_out);
}
