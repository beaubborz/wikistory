extern crate xml;

use std::io::{Read, Write, Seek, SeekFrom};
use self::xml::reader::*;

/// This function takes a Reader `data_source`, consumes it,
/// indexes it and outputs the result in the `index_out` Writer.
pub fn generate_index<R: Read + Seek>(data_source: R, index_out: &mut Write) {
    println!("Starting to index file...");
    let mut xml_reader = EventReader::new(data_source);
    // The XML file is made of lots of tags, but we only want to keep <page> events.
    while let Some(page_pos) = skip_until_page_start(&mut xml_reader) {
        if let Some(title) = extract_next_title(&mut xml_reader) {
            if let Some(ns) = extract_next_namespace(&mut xml_reader) {
                // Only consider namespace 0 (default articles)
                // Skip the others (files, templates, gadgets, etc..)
                if ns == "0" {
                    index_out.write(format!("{},{}\r\n", page_pos, title).as_bytes());
                }
            }
        }
    }
    println!("Done reading!");
}
/// Iterate over all XmlEvents in the `reader` until a <page> is found.
/// If an error occurs while reading, or the end of the file is reached,
/// `None` is returned.
fn skip_until_page_start<R: Read + Seek>(reader: &mut EventReader<R>) -> Option<u64> {
    let page_tag = "<page>";
    // Read all events as long as we find some:
    while let Ok(event) = reader.next() {
        // If the event is a start element:
        if let XmlEvent::StartElement{name, ..} = event {
            // And it is for page:
            if name.local_name == page_tag[1..page_tag.len() - 1] /* Test if the name is the inside of the tag */ {
                // Stop iterating, we found the page:
                // Go back right before the "page" tag and return that seek position:
                return Some(reader.source_mut().seek(SeekFrom::Current(-1 * page_tag.len() as i64)).unwrap());
            }
        }
    }
    // No more pages. Return nothing.
    return None;
}

fn extract_next_title<R: Read>(reader: &mut EventReader<R>) -> Option<String> {
    // Read all events until we get a title:
    while let Ok(event) = reader.next() {
        // If the event is a start element:
        if let XmlEvent::StartElement{name, ..} = event {
            // And it is for page:
            if name.local_name == "title" {
                // Read an extra tag; it is the content of the title:
                if let Ok(XmlEvent::Characters(title)) = reader.next() {
                    // Extract the title and return it:
                    return Some(title);
                } else {
                    // Did not get a title; unexpected !! Return nothing for now.
                    // TODO: Return proper errors.
                    return None;
                }
            }
        }
    }
    // Reached end of file (or got an error) while looking for <title>, return nothing:
    return None;
}

fn extract_next_namespace<R: Read>(reader: &mut EventReader<R>) -> Option<String> {
    // Read all events until we get a title:
    while let Ok(event) = reader.next() {
        // If the event is a start element:
        if let XmlEvent::StartElement{name, ..} = event {
            // And it is for page:
            if name.local_name == "ns" {
                // Read an extra tag; it is the content of the namespace tag:
                if let Ok(XmlEvent::Characters(ns)) = reader.next() {
                    // Extract the title and return it:
                    return Some(ns);
                } else {
                    // Did not get a title; unexpected !! Return nothing for now.
                    // TODO: Return proper errors.
                    return None;
                }
            }
        }
    }
    // Reached end of file (or got an error) while looking for <title>, return nothing:
    return None;
}

#[cfg(test)]
mod tests;
