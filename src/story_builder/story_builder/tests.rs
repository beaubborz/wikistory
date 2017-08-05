use story_builder::article_provider::*;
use story_builder::story_builder::StoryBuilder;

struct test_Article {}

impl Article for test_Article {
    fn get_related(&self) -> Vec<String> {
        vec![]
    }
}

struct test_ArticleProvider {}

impl test_ArticleProvider {
    fn expected_suggestion(&self) -> String {
        let mut expected = String::from("Cannot find wikipedia article for <not-found>, try one of the following suggestions:\r\n");
        expected.push_str("- Suggestion 1\r\n");
        expected.push_str("- Suggestion 2\r\n");
        expected.push_str("- Suggestion 3\r\n");
        expected
    }
}

impl ArticleProvider for test_ArticleProvider {
    /// Returns a Some(Box<Article>) if the article is found, None otherwise.
    fn get(&self, topic: &str) -> Option<Box<Article>> {
        match topic {
            "found" => Some(Box::new(test_Article {})),
            _ => None,
        }
    }
    /// Returns a Vector of topics that might be related to the topic entered.
    fn search(&self, topic: &str) -> Vec<String> {
        vec!["Suggestion 1".to_owned(),
             "Suggestion 2".to_owned(),
             "Suggestion 3".to_owned()]
    }
}

#[test]
/// For: build_suggestions_msg
fn build_suggestions_msg_test() {
    let provider = test_ArticleProvider {};
    let story_builder = StoryBuilder::new(&provider);

    assert_eq!(story_builder.build_suggestions_msg("not-found"), provider.expected_suggestion());
}

#[test]
/// For: build_story
fn build_story_cannot_find_first_article_suggest() {
    let provider = test_ArticleProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("not-found", "not-found"), Err(provider.expected_suggestion()));
}
