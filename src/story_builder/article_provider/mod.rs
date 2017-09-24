pub type ThreadedAP = (ArticleProvider + Send + Sync);
pub type ThreadedArticle = (Article + Send + Sync);

pub struct Paragraph {
    pub text: String,
    pub topics: Vec<String>,
}

pub trait Article {
    fn get_paragraphs(&self) -> &Vec<Paragraph>;
    fn get_topic(&self) -> &str;
}

pub trait ArticleProvider {
    /// Returns a Some(Box<Article>) if the article is found, None otherwise.
    fn get(&self, topic: &str) -> Option<Box<Article + Send + Sync>>;
    /// Returns a Vector of topics that might be related to the topic entered.
    fn search(&self, topic: &str) -> Vec<String>;
}

pub mod http_article_provider;
