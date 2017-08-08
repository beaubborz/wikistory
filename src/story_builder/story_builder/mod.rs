use story_builder::article_provider::ArticleProvider;

pub struct StoryBuilder<'a> {
    article_provider: &'a ArticleProvider,
    max_depth: u8,
}

impl <'a> StoryBuilder<'a>  {
    pub fn new(article_provider: &'a ArticleProvider) -> StoryBuilder<'a> {
        StoryBuilder {
            article_provider,
            max_depth: 5, // default value for now
        }
    }

    fn build_story(&self, start_topic: &str , end_topic: &str) -> Result<String, String> {
        // If one of the topics is an empty string, do not try to make a story out of it.
        if start_topic == "" {
            return Err("Missing start topic.".to_owned());
        }
        if end_topic == "" {
            return Err("Missing end topic.".to_owned());
        }

        // If both topics are the same,
        // there is no point in trying to figure out the story.
        if start_topic == end_topic {
            return Err("No story to build; same start and end topics.".to_owned());
        }

        // Load the first article
        let start_article = match self.article_provider.get(start_topic) {
            Some(start_article) => start_article,
            None => {return Err(self.build_suggestions_msg(start_topic));},
        };

        // Load the end article
        let end_article = match self.article_provider.get(end_topic) {
            Some(end_topic) => end_topic,
            None => {return Err(self.build_suggestions_msg(end_topic));},
        };

        // To build a story, we need to build a tree starting at the start_article
        // node and going down in a "breadth-first" way; that way, once we find
        // the end note, we know it is the shortest path to it. Also, going depth-first
        // will be impossibly long to complete sincethe depth of the wikipedia
        // article tree is almost infinite.

        // We look for a paragraph that holds a reference to our end topic:
        if let Some(paragraph) = start_article.get_paragraphs().iter().find(|par| {
            // if any of the topics in the paragraph is <end>, return it.
            par.topics.iter().any(|topic| {topic == end_topic})
            }) {
                // We found the paragraph; return it directly.
            return Ok(format!("{}\r\n", &paragraph.text));
        }

        Err(format!("Reached depth of <{}> without finding <{}>. Stopping search.", self.max_depth, end_topic).to_owned())
    }

    fn build_suggestions_msg(&self, topic: &str) -> String {
        let mut msg = String::from(format!("Cannot find wikipedia article for <{}>, try one of the following suggestions:\r\n", topic));
        for sugg in self.article_provider.search(topic) {
        msg.push_str(&format!("- {}\r\n", &sugg));
        }

        msg
    }
}

struct TreeNode<T> {
    data: T,
}

impl <T> TreeNode<T> {
    fn new(data: T) -> TreeNode<T> {
        TreeNode {
            data
        }
    }
}

#[cfg(test)]
mod tests;
