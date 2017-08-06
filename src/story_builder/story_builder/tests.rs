use story_builder::article_provider::*;
use story_builder::story_builder::StoryBuilder;

struct TestArticle {}

impl Article for TestArticle {
    fn get_related(&self) -> Vec<String> {
        vec![]
    }
}

struct TestArticleProvider {}

impl TestArticleProvider {
    fn expected_suggestion(&self) -> String {
        let mut expected = String::from("Cannot find wikipedia article for <not-found>, try one of the following suggestions:\r\n");
        expected.push_str("- Suggestion 1\r\n");
        expected.push_str("- Suggestion 2\r\n");
        expected.push_str("- Suggestion 3\r\n");
        expected
    }
    fn expected_story(&self) -> String {
        let mut expected = String::from("Paragraph of topic 1\r\n");
        expected.push_str("Paragraph of topic 2\r\n");
        expected.push_str("Paragraph of topic 3\r\n");
        expected.push_str("Paragraph of topic 4\r\n");
        expected.push_str("Paragraph of topic 5\r\n");
        expected.push_str("Paragraph of topic 6\r\n");
        expected
    }
}

impl ArticleProvider for TestArticleProvider {
    /// Returns a Some(Box<Article>) if the article is found, None otherwise.
    fn get(&self, topic: &str) -> Option<Box<Article>> {
        match topic {
            "found" => Some(Box::new(TestArticle {})),
            "not-found" => None,
            _ => {
                // Build a fake article and return:
                Some(Box::new (TestArticle {

                }))
                },
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
        let provider = TestArticleProvider {};
        let story_builder = StoryBuilder::new(&provider);

        assert_eq!(story_builder.build_suggestions_msg("not-found"), provider.expected_suggestion());
    }

    #[test]
    /// For: build_story
    fn build_story_cannot_find_first_article_suggest() {
        let provider = TestArticleProvider {};
        let story_builder = StoryBuilder::new(&provider);
        assert_eq!(story_builder.build_story("not-found", "found"), Err(provider.expected_suggestion()));
    }

    #[test]
    /// For: build_story
    fn build_story_cannot_find_second_article_suggest() {
        let provider = TestArticleProvider {};
        let story_builder = StoryBuilder::new(&provider);
        assert_eq!(story_builder.build_story("found", "not-found"), Err(provider.expected_suggestion()));
    }
