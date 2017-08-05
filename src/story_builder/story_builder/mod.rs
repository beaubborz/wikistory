use story_builder::article_provider::ArticleProvider;

struct StoryBuilder<'a> {
    article_provider: &'a ArticleProvider,
}

impl <'a> StoryBuilder<'a> {
    fn new(article_provider: &'a ArticleProvider) -> StoryBuilder {
        StoryBuilder {
            article_provider
        }
    }

    fn build_story(&self, start_topic: &str , end_topic: &str) -> Result<String, String> {
        let start_article = self.article_provider.get(start_topic).unwrap_or({
            return Err(self.build_suggestions_msg(start_topic));
        });


        Err("TODO".to_owned())
    }

    fn build_suggestions_msg(&self, topic: &str) -> String {
        let mut msg = String::from(format!("Cannot find wikipedia article for <{}>, try one of the following suggestions:\r\n", topic));
        for sugg in self.article_provider.search(topic) {
        msg.push_str(&format!("- {}\r\n", &sugg));
        }

        msg
    }
}

#[cfg(test)]
mod tests;
