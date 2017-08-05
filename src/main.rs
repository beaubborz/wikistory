
/// The main entry point for WikiStory. It is tasked with reading user input to
/// choose a starting and ending topic, as well as printing out results.
fn main() {
    let args_vec: Vec<String> = std::env::args().collect();
    let (first_topic, end_topic) = match extract_topics_from_args_vec(&args_vec) {
        Ok(items) => items,
        Err(msg) => {println!("{}", msg); return;},
    };

println!("Wikistory will now try to generate a story from <{}> to <{}>: ", first_topic, end_topic);

}

/// This function extracts topics from a vector and expect the following items:
/// 1. Command name (this item will be skipped)
/// 2. First topic
/// 3. End topic
///
/// If the vector does not have enough or has too many items, an error will
/// be returned with a message describing which item is missing.
/// # Examples
/// ```
/// let args_vec: Vec<String> = std::env::args().collect();
/// let (start, end) = extract_topics_from_args_vec();
/// ```
fn extract_topics_from_args_vec(args: &Vec<String>) -> Result<(&str, &str), &str> {
    match args.len() {
        0 => Err("Error reading arguments."),
        1 => Err("Missing argument: Start topic"),
        2 => Err("Missing argument: End topic"),
        3 => Ok((&args[1], &args[2])),
        _ => Err("Too many arguments"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_topics_from_args_with_empty_vec_should_err() {
        let empty_vec = vec![];
        assert_eq!(extract_topics_from_args_vec(&empty_vec), Err("Error reading arguments."));
    }

    #[test]
    fn extract_topics_from_args_with_only_one_item_should_err() {
        let vec = vec!["0".to_owned()];
        assert_eq!(extract_topics_from_args_vec(&vec), Err("Missing argument: Start topic"));
    }

    #[test]
    fn extract_topics_from_args_with_only_two_item_should_err() {
        let vec = vec!["0".to_owned(), "1".to_owned()];
        assert_eq!(extract_topics_from_args_vec(&vec), Err("Missing argument: End topic"));
    }

    #[test]
    fn extract_topics_from_args_with_three_items_should_ok() {
        let vec = vec!["0".to_owned(), "1".to_owned(), "2".to_owned()];
        assert_eq!(extract_topics_from_args_vec(&vec), Ok(("1", "2")));
    }

    #[test]
    fn extract_topics_from_args_with_too_many_items_should_err() {
        let vec = vec!["0".to_owned(), "1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
        assert_eq!(extract_topics_from_args_vec(&vec), Err("Too many arguments"));
    }
}
