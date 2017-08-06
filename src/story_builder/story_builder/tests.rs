use story_builder::story_builder::*;
use story_builder::article_provider::*;

static EXPECTED_SUGGESTION: &'static str = "Cannot find wikipedia article for <not-found>, try one of the following suggestions:\r\n\
- Suggestion 1\r\n\
- Suggestion 2\r\n\
- Suggestion 3\r\n";

#[test]
/// For: build_suggestions_msg
fn build_suggestions_msg_is_working() {
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        #[allow(unused_variables)]
        fn get(&self, topic: &str) -> Option<Box<Article>> {
            panic!("get() should never be called in this test.");
        }
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {
            vec!["Suggestion 1".to_owned(),
            "Suggestion 2".to_owned(),
            "Suggestion 3".to_owned()]
        }
    }
    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);

    assert_eq!(story_builder.build_suggestions_msg("not-found"), EXPECTED_SUGGESTION);
}

#[test]
/// For: build_story
fn build_story_cannot_find_first_article_suggest() {
    struct TestArticle {
        topics: Vec<String>
    }
    impl Article for TestArticle {
        fn get_related_topics(&self) -> &Vec<String> {&self.topics}
    }
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        fn get(&self, topic: &str) -> Option<Box<Article>> {
            if topic == "found"
                {Some(Box::new(TestArticle {topics: vec![]}))}
            else
                {None}
        }
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {
            vec!["Suggestion 1".to_owned(),
            "Suggestion 2".to_owned(),
            "Suggestion 3".to_owned()]
        }
    }

    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("not-found", "found"), Err(EXPECTED_SUGGESTION.to_owned()));
}

#[test]
/// For: build_story
fn build_story_cannot_find_second_article_suggest() {
    struct TestArticle {
        topics: Vec<String>
    }
    impl Article for TestArticle {
        fn get_related_topics(&self) -> &Vec<String> {&self.topics}
    }
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        fn get(&self, topic: &str) -> Option<Box<Article>> {
            if topic == "found"
                {Some(Box::new(TestArticle {topics: vec![]}))}
            else
                {None}
        }
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {
            vec!["Suggestion 1".to_owned(),
            "Suggestion 2".to_owned(),
            "Suggestion 3".to_owned()]
        }
    }

    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("found", "not-found"), Err(EXPECTED_SUGGESTION.to_owned()));
}

#[test]
/// For: build_story
fn build_story_from_same_start_and_end_should_err() {
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        #[allow(unused_variables)]
        fn get(&self, topic: &str) -> Option<Box<Article>> {panic!("get() should not get called in this test.")}
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {panic!("get() should not get called in this test.")}
    }
    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("similar topic", "similar topic"),
               Err("No story to build; same start and end topics.".to_owned()));
}

#[test]
/// For: build_story
fn build_story_empty_start_topic_should_err() {
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        #[allow(unused_variables)]
        fn get(&self, topic: &str) -> Option<Box<Article>> {panic!("get() should not get called in this test.")}
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {panic!("get() should not get called in this test.")}
    }
    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("", "Other topic"),
               Err("Missing start topic.".to_owned()));
}

#[test]
/// For: build_story
fn build_story_empty_end_topic_should_err() {
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        #[allow(unused_variables)]
        fn get(&self, topic: &str) -> Option<Box<Article>> {panic!("get() should not get called in this test.")}
        #[allow(unused_variables)]
        fn search(&self, topic: &str) -> Vec<String> {panic!("get() should not get called in this test.")}
    }
    let provider = TestProvider {};
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("First topic", ""),
               Err("Missing end topic.".to_owned()));
}
