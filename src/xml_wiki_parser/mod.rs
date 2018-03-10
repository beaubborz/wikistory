extern crate xml;

use std::io::{BufReader, Read, Write, Seek, SeekFrom};
use self::xml::reader::*;

/// This function takes a Reader `data_source`, consumes it,
/// indexes it and outputs the result in the `index_out` Writer.
pub fn generate_index<R: Read + Seek, W: Read + Write + Seek>(mut data_source: R, mut index_out: W) {
    /* To allow resuming indexation process, read the last indexed position from the index and
       seek to it: */
    let seek_resume = get_last_index_position(&mut index_out);
    let mut must_skip_one = false;
    if seek_resume > 0 {
        println!("Seeking to {}", seek_resume);
        must_skip_one = true;
    }
    data_source.seek(SeekFrom::Start(seek_resume));
    println!("Indexing...");
    let mut xml_reader = EventReader::new(BufReader::new(data_source));
    // Skip the next article since it was already in the index
    if (must_skip_one) {
        skip_until_page_start(&mut xml_reader);
        xml_reader.next();
    }
    // The XML file is made of lots of tags, but we only want to keep <page> events.
    // Skip to the next article (<page>) start:
    while let Some(page_pos) = skip_until_page_start(&mut xml_reader) {
        // Then, read the title of that article:
        if let Some(title) = extract_next_title(&mut xml_reader) {
            // Also extract the namespace:
            if let Some(ns) = extract_next_namespace(&mut xml_reader) {
                // Only consider namespace 0 (default articles)
                // Skip the others (files, templates, gadgets, etc..)
                if ns == "0" {
                    index_out.write(format!("{},{}\r\n", page_pos, title).as_bytes()).expect("Cannot write to index file. Aborting.");
                }
            }
        }
    }
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

fn get_last_index_position<W: Read + Write + Seek>(index_in: &mut W) -> u64 {
    // Go back 4k characters and read the lines:
    const CHUNK_SIZE: usize = 4096;
    let start_pos = index_in.seek(SeekFrom::End(-1*CHUNK_SIZE as i64)).expect("Cannot seek in index file. Aborting.");
    let mut buf = String::with_capacity(CHUNK_SIZE);
    index_in.read_to_string(&mut buf).expect("Unable to read index file. Aborting.");
    let lines: Vec<&str> = buf.split("\r\n").filter(|x| x.len() > 0 /* Ignore empty lines */).collect();
    let line_count = lines.len();
    if line_count == 0 {
        return 0; // There was no content in the file, return 0.
        // If we have at least 2 lines, it means the first line might be mangled but the second (last) line is complete.
        // Or, if the start position was the start of the file, and we read a line, then we have the only line in the file.
        // In this case, we can process it.
    } else if (line_count >= 2) || (line_count == 1 && start_pos == 0) {
        // The lines are built as: SeekIndex,ArticleName
        // Split on comma, parse the first part and return it.
            println!("{:?}", lines[line_count - 1]);
        return lines[line_count - 1]
                .split(',')
                .next().expect("Index file does not respect the correct file format (Must be comma-separated.)")
                .parse().expect("Cannot parse index as a valid number.");
    } else {
        panic!("We did not get a full line by reading the last chunk of the index file (TODO: Implement this!). Aborting.");
    }
}

#[cfg(test)]
mod tests;
