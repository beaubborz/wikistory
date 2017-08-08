use story_builder::story_builder::*;
use story_builder::article_provider::*;
use std::collections::HashMap;
use std::rc::Rc;

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
        topics: Vec<Paragraph>
    }
    impl Article for TestArticle {
        fn get_paragraphs(&self) -> &Vec<Paragraph> {&self.topics}
    }
    struct TestProvider {}
    impl ArticleProvider for TestProvider {
        fn get(&self, topic: &str) -> Option<Box<Article>> {
            if topic == "found" {
                Some(Box::new(TestArticle {topics: vec![]}))
            }
            else {
                None
            }
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
        topics: Vec<Paragraph>
    }
    impl Article for TestArticle {
        fn get_paragraphs(&self) -> &Vec<Paragraph> {&self.topics}
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


#[test]
/// For: build_story
fn build_story_end_topic_found_in_start_article() {
    let mut prebuilt_rels = Rc::new(HashMap::new());
    Rc::get_mut(&mut prebuilt_rels).unwrap().insert("start", vec![Paragraph {text: "Paragraph 1".to_owned(), topics: vec!["topic 1".to_owned(), "topic 2".to_owned(), "topic 3".to_owned()]},
                                                                  Paragraph {text: "Paragraph 2".to_owned(), topics: vec!["topic 3".to_owned(), "end".to_owned(), "topic 5".to_owned()]},
                                                                  Paragraph {text: "Paragraph 3".to_owned(), topics: vec!["topic 3".to_owned(), "topic 1".to_owned(), "topic 5".to_owned()]}]);
    struct TestArticle {
        topic: String,
        prebuilt_rels: Rc<HashMap<&'static str, Vec<Paragraph>>>,
    }

    impl TestArticle {
        fn new(topic: String, prebuilt_rels: Rc<HashMap<&'static str, Vec<Paragraph>>>) -> TestArticle {
            TestArticle {topic, prebuilt_rels}
        }
    }

    impl Article for TestArticle {
            fn get_paragraphs(&self) -> &Vec<Paragraph> {
            self.prebuilt_rels.get::<str>(&self.topic).expect("Tried to access a node that doesn't exist!")
        }
    }
    struct TestProvider {
        prebuilt_rels: Rc<HashMap<&'static str, Vec<Paragraph>>>
    }
    impl TestProvider {
        fn new(prebuilt_rels: Rc<HashMap<&'static str, Vec<Paragraph>>>) -> TestProvider {
            TestProvider {prebuilt_rels}
        }
    }
    impl ArticleProvider for TestProvider {
        fn get(&self, topic: &str) -> Option<Box<Article>> {
                let new_rels: Rc<HashMap<&'static str, Vec<Paragraph>>> = self.prebuilt_rels.clone();
                Some(Box::new(TestArticle::new(topic.to_owned(), new_rels)))
        }
        fn search(&self, topic: &str) -> Vec<String> {panic!("search() should not be called in this test.");}
    }
    let provider = TestProvider::new(prebuilt_rels.clone());
    let story_builder = StoryBuilder::new(&provider);
    assert_eq!(story_builder.build_story("start", "end"), Ok("Paragraph for end.\r\n".to_owned()));
}
