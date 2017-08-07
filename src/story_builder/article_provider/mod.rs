pub struct Reference {
    pub paragraph: String,
    pub topic: String,
}

pub trait Article {
    fn get_related_references(&self) -> &Vec<Reference>;
}

pub trait ArticleProvider {
    /// Returns a Some(Box<Article>) if the article is found, None otherwise.
    fn get(&self, topic: &str) -> Option<Box<Article>>;
    /// Returns a Vector of topics that might be related to the topic entered.
    fn search(&self, topic: &str) -> Vec<String>;
}
