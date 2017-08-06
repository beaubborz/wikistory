use story_builder::article_provider::ArticleProvider;

pub struct StoryBuilder<'a> {
    article_provider: &'a ArticleProvider,
}

impl <'a> StoryBuilder<'a> {
    pub fn new(article_provider: &'a ArticleProvider) -> StoryBuilder {
        StoryBuilder {
            article_provider
        }
    }

    fn build_story(&self, start_topic: &str , end_topic: &str) -> Result<String, String> {
        let start_article = match self.article_provider.get(start_topic) {
            Some(start_article) => start_article,
            None => {return Err(self.build_suggestions_msg(start_topic));},
        };

        let end_article = match self.article_provider.get(end_topic) {
            Some(end_topic) => end_topic,
            None => {return Err(self.build_suggestions_msg(end_topic));},
        };

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
