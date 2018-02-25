extern crate wikistory;
extern crate clap;
use wikistory::story_builder::article_provider::http_article_provider::HTTPArticleProvider;
use wikistory::story_builder::story_builder::StoryBuilder;
use std::sync::Arc;
use clap::{App, Arg};

/// The main entry point for WikiStory. It is tasked with reading user input to
/// choose a starting and ending topic, as well as printing out results.
fn main() {
    let args = App::new("Wikistory")
               .author("GCouvrette")
               .about("Builds a story from one topic to another using links in wikipedia articles.")
               .arg(Arg::with_name("Starting topic")
                   .required(true))
               .arg(Arg::with_name("Final topic")
                   .required(true))
               .get_matches();

    let first_topic = args.value_of("Starting topic").unwrap();
    let end_topic = args.value_of("Final topic").unwrap();

    println!("Wikistory will now try to generate a story from <{}> to <{}>: ",
             first_topic, end_topic);

    let provider = HTTPArticleProvider::new();
    let mut sb = StoryBuilder::new(Arc::new(provider));
    match sb.build_story(&first_topic, &end_topic) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{}", err),
    };
}
