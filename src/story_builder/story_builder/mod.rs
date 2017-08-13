use story_builder::article_provider::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::ops::Deref;

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

    pub fn build_story(&self, start_topic: &str , end_topic: &str) -> Result<String, String> {
        let end_topic = &end_topic.to_lowercase();
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

                   /* To build a story, we need to build a tree starting at the start_article
                      node and going down in a "breadth-first" way; that way, once we find
                      the end note, we know it is the shortest path to it. Also, going depth-first
                      will be impossibly long to complete sincethe depth of the wikipedia
                      article tree is almost infinite. */
                   /* We look for a paragraph that holds a reference to our end topic
                      somewhere in the last level we fetched: */
        let mut last_level: Vec<Rc<ArticleNode>> = vec![Rc::new(ArticleNode::new(start_article))]; // starts with start article
        for i in 0..self.max_depth { // To prevent overloading the system, stop after X level deep
                       // Start by loading the next level of articles:
           if i > 0 {
               // Any other iteration: go one level deeper:
               let mut current_level = vec![];
               for rc_article_node in last_level.iter() {
                       /* Iterate on each paragraph of this article, then map on it to
                          get the article for each related topic in it. */
               let rc_local = rc_article_node.clone();
               let article = rc_local.deref();
               for paragraph in article.get_paragraphs().iter() {
                   for topic in paragraph.topics.iter() {
                       let mut new_node = ArticleNode::new((&self).article_provider.get(topic).unwrap());
                       new_node.attach_to(rc_local.clone(), &paragraph.text);
                       current_level.push(Rc::new(new_node));
                       }
                   }
               }
               last_level = current_level; // Replace the previous level with this level.
           }

           for article_node in last_level.iter() {
               if let Some(text) = StoryBuilder::find_text_for_topic_in_article(article_node.deref().deref().borrow(), end_topic) {
                   // Found the topic. Format and return.
                   return Ok(StoryBuilder::build_final_text(article_node.clone(), text));
               }
           }
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

    fn find_text_for_topic_in_article<'b>(article: &'b Article, topic: &str) -> Option<&'b str> {
        if let Some(paragraph) = article.get_paragraphs().iter().find(|par| {
                   // if any of the topics in the paragraph is <end>, return it.
            par.topics.iter().any(|t| {&t.to_lowercase() == topic})
            }) {
                   // We found the paragraph; return it directly.
            return Some(&paragraph.text);
        } else {
            None
        }
    }

    fn build_final_text(article_node: Rc<ArticleNode>, final_text: &str) -> String {
        let mut text = String::new();
        {
            let n: &ArticleNode = article_node.borrow();
            if let Some(n_text) = n.text() {
                text.push_str(&format!("{}\r\n", n_text));
            }
        }

        let mut node = article_node;
        loop {
            node = match node.parent() {
                Some(rc) => {
                        {
                        let n: &ArticleNode = rc.borrow();
                        if let Some(n_text) = n.text() {
                            text.push_str(&format!("{}\r\n", n_text));
                        }
                        }
                    rc
                    },
                _ => break,
            }
        }
        text.push_str(&format!("{}\r\n", final_text));
        text
    }
}

struct ArticleNode {
    data: Box<Article>,
    parent: Option<Rc<ArticleNode>>,
    text: Option<String>,
}

impl ArticleNode {
    fn new(data: Box<Article>) -> ArticleNode {
        ArticleNode {
            data,
            parent: None,
            text: None,
        }
    }
    fn attach_to(&mut self, parent: Rc<ArticleNode>, paragraph_text: &str) {
        self.parent = Some(parent);
        self.text = Some(String::from(paragraph_text));
    }
    fn parent(&self) -> Option<Rc<ArticleNode>> {
        self.parent.clone()
    }
    fn text(&self) -> Option<String> {
        self.text.clone()
    }
}

impl <'n> Deref for ArticleNode {
    type Target = Box<Article>;

    fn deref(&self) -> &Box<Article> {
        &self.data
    }
}

#[cfg(test)]
mod tests;
