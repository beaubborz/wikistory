use super::*;

#[test]
/// For: fn extract_topics_from_args
fn extract_topics_from_args_with_empty_vec_should_err() {
    let empty_vec = vec![];
    assert_eq!(
        extract_topics_from_args_vec(&empty_vec),
        Err("Error reading arguments.")
    );
}

#[test]
/// For: fn extract_topics_from_args
fn extract_topics_from_args_with_only_one_item_should_err() {
    let vec = vec!["0".to_owned()];
    assert_eq!(
        extract_topics_from_args_vec(&vec),
        Err("Missing argument: Start topic")
    );
}

#[test]
/// For: fn extract_topics_from_args
fn extract_topics_from_args_with_only_two_item_should_err() {
    let vec = vec!["0".to_owned(), "1".to_owned()];
    assert_eq!(
        extract_topics_from_args_vec(&vec),
        Err("Missing argument: End topic")
    );
}

#[test]
/// For: fn extract_topics_from_args
fn extract_topics_from_args_with_three_items_should_ok() {
    let vec = vec!["0".to_owned(), "1".to_owned(), "2".to_owned()];
    assert_eq!(extract_topics_from_args_vec(&vec), Ok(("1", "2")));
}

#[test]
/// For: fn extract_topics_from_args
fn extract_topics_from_args_with_too_many_items_should_err() {
    let vec = vec![
        "0".to_owned(),
        "1".to_owned(),
        "2".to_owned(),
        "3".to_owned(),
        "4".to_owned(),
    ];
    assert_eq!(
        extract_topics_from_args_vec(&vec),
        Err("Too many arguments")
    );
}
