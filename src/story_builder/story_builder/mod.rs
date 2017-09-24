use story_builder::article_provider::*;
use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::collections::HashSet;

pub struct StoryBuilder {
    article_provider: Arc<ThreadedAP>,
    max_depth: u8,
    visited_nodes: HashSet<String>,
}

impl StoryBuilder {
    pub fn new(article_provider: Arc<ThreadedAP>) -> StoryBuilder {
        StoryBuilder {
            article_provider,
            max_depth: 5, // default value for now
            visited_nodes: HashSet::new(),
        }
    }

    pub fn build_story(&mut self, start_topic: &str, end_topic: &str) -> Result<String, String> {
        let start_topic = start_topic.to_lowercase();
        let end_topic = end_topic.to_lowercase();
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
        let start_article = match self.article_provider.get(&start_topic) {
            Some(start_article) => start_article,
            None => {
                return Err(self.build_suggestions_msg(&start_topic));
            }
        };
        self.visited_nodes.insert(start_topic);
        // Load the end article
        match self.article_provider.get(&end_topic) {
            Some(end_topic) => end_topic,
            None => {
                return Err(self.build_suggestions_msg(&end_topic));
            }
        };

        /* To build a story, we need to build a tree starting at the start_article
           node and going down in a "breadth-first" way; that way, once we find
           the end note, we know it is the shortest path to it. Also, going depth-first
           will be impossibly long to complete sincethe depth of the wikipedia
           article tree is almost infinite. */
        /* We look for a paragraph that holds a reference to our end topic
           somewhere in the last level we fetched: */

        let mut last_level: Vec<Arc<ArticleNode>> = vec![Arc::new(ArticleNode::new(start_article))]; // starts with start article
        for i in 0..self.max_depth {
            // To prevent overloading the system, stop after X level deep
            // Start by loading the next level of articles:
            if i > 0 {
                // Any other iteration: go one level deeper:
                let mut current_level = vec![];
                // First: spawn all threads first
                let mut threads: Vec<
                    JoinHandle<(Arc<ArticleNode>, String, Option<Box<ThreadedArticle>>)>,
                > = Vec::new();
                for arc_article_node in last_level.iter() {
                    /* Iterate on each paragraph of this article, then map on it to
                       get the article for each related topic in it. */
                    for paragraph in arc_article_node.deref().get_paragraphs().iter() {
                        for topic in paragraph.topics.iter() {
                            let topic = topic.to_lowercase();
                            // Do not access the same article more than once!!
                            if !self.visited_nodes.contains(&topic) {
                                let article_provider = self.article_provider.clone();
                                let topic_for_thread = topic.clone();
                                let par_text = paragraph.text.clone();
                                let parent_article = arc_article_node.clone();
                                threads.push(thread::spawn(move || {
                                    (
                                        parent_article,
                                        par_text,
                                        article_provider.get(&topic_for_thread),
                                    )
                                }));
                                self.visited_nodes.insert(topic);
                            }
                        }
                    }
                }
                // Then, join them up one at a time.
                for t in threads.into_iter() {
                    match t.join().unwrap() {
                        (parent_article, source_paragraph, Some(a)) => {
                            let mut new_node = ArticleNode::new(a);
                            new_node.attach_to(parent_article, source_paragraph);
                            current_level.push(Arc::new(new_node));
                        }
                        _ => (),
                    }
                }

                last_level = current_level;
            }
            // Replace the previous level with this level.

            for article_node in last_level.iter() {
                if let Some(text) = StoryBuilder::find_text_for_topic_in_article(
                    article_node.deref().deref().borrow(),
                    &end_topic,
                ) {
                    // Found the topic. Format and return.
                    return Ok(StoryBuilder::build_final_text(
                        article_node.clone(),
                        text,
                        &end_topic,
                    ));
                }
            }
        }


        Err(
            format!(
                "Reached depth of <{}> without finding <{}>. Stopping search.",
                self.max_depth,
                end_topic
            ).to_owned(),
        )
    }

    fn build_suggestions_msg(&self, topic: &str) -> String {
        let mut msg = String::from(format!(
            "Cannot find wikipedia article for <{}>, try one of the following suggestions:\r\n",
            topic
        ));
        for sugg in self.article_provider.search(topic) {
            msg.push_str(&format!("- {}\r\n", &sugg));
        }

        msg
    }

    fn find_text_for_topic_in_article<'b>(
        article: &'b (ThreadedArticle),
        topic: &str,
    ) -> Option<&'b str> {
        if let Some(paragraph) = article.get_paragraphs().iter().find(|par| {
            // if any of the topics in the paragraph is <end>, return it.
            par.topics.iter().any(|t| &t.to_lowercase() == topic)
        }) {
            // We found the paragraph; return it directly.
            return Some(&paragraph.text);
        } else {
            None
        }
    }

    fn build_final_text(
        article_node: Arc<ArticleNode>,
        final_text: &str,
        final_topic: &str,
    ) -> String {
        let mut texts: Vec<String> = Vec::new();
        let mut last_topic = final_topic.to_owned();
        texts.push(format!("{}\r\n", final_text));
        {
            let n: &ArticleNode = article_node.borrow();
            if let Some(n_text) = n.text() {
                let new_topic = n.get_topic().to_owned();
                texts.push(format!("-> ({} to {})\r\n", &new_topic, last_topic));
                last_topic = new_topic;
                texts.push(format!("{}\r\n", n_text));
            }
        }

        let mut node = article_node;
        loop {
            node = match node.parent() {
                Some(arc) => {
                    {
                        let n: &ArticleNode = arc.borrow();
                        if let Some(n_text) = n.text() {
                            let new_topic = n.get_topic().to_owned();
                            texts.push(format!("-> ({} to {})\r\n", &new_topic, last_topic));
                            last_topic = new_topic;
                            texts.push(format!("{}\r\n", n_text));
                        }
                    }
                    arc
                }
                _ => {
                    let new_topic = node.get_topic().to_owned();
                    texts.push(format!("-> ({} to {})\r\n", &new_topic, last_topic));
                    return texts.into_iter().rev().collect();
                }
            }
        }
    }
}

struct ArticleNode {
    data: Box<ThreadedArticle>,
    parent: Option<Arc<ArticleNode>>,
    text: Option<String>,
}

impl ArticleNode {
    fn new(data: Box<ThreadedArticle>) -> ArticleNode {
        ArticleNode {
            data,
            parent: None,
            text: None,
        }
    }
    fn attach_to(&mut self, parent: Arc<ArticleNode>, paragraph_text: String) {
        self.parent = Some(parent);
        self.text = Some(paragraph_text);
    }
    fn parent(&self) -> Option<Arc<ArticleNode>> {
        self.parent.clone()
    }
    fn text(&self) -> Option<String> {
        self.text.clone()
    }
}

impl<'n> Deref for ArticleNode {
    type Target = Box<ThreadedArticle>;

    fn deref(&self) -> &Box<ThreadedArticle> {
        &self.data
    }
}

#[cfg(test)]
mod tests;
