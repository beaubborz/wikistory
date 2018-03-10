extern crate wikistory;
use wikistory::xml_wiki_parser::generate_index as generate_index;
use std::fs::File;
use std::fs::OpenOptions;

fn main() {
    // 1. Open XML data file to read from:
    let xml_file = File::open("./data/enwiki-20170820-pages-articles.xml").expect("File not found.");

    // 2: Open the index output file:
    let index_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true) // Or create a new file if it does not exist
                        .open("./data/index.csv")
                        .expect("Unable to create index file.");
    // 3. Index the file:
    generate_index(xml_file, index_file);
}
