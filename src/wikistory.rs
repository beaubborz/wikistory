extern crate wikistory;
use wikistory::story_builder::article_provider::http_article_provider::HTTPArticleProvider;
use wikistory::story_builder::story_builder::StoryBuilder;
use std::sync::Arc;

/// The main entry point for WikiStory. It is tasked with reading user input to
/// choose a starting and ending topic, as well as printing out results.
fn main() {
    let args_vec: Vec<String> = std::env::args().collect();
    let (first_topic, end_topic) = match extract_topics_from_args_vec(&args_vec) {
        Ok(items) => items,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    println!(
        "Wikistory will now try to generate a story from <{}> to <{}>: ",
        first_topic,
        end_topic
    );
    let provider = HTTPArticleProvider::new();
    let mut sb = StoryBuilder::new(Arc::new(provider));
    match sb.build_story(&first_topic, &end_topic) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{}", err),
    };
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
mod tests;
