extern crate reqwest;
extern crate htmlstream;
use story_builder::article_provider::*;
use std::io::Read;
use self::htmlstream::HTMLTagState;

pub struct HTTPArticleProvider {
    base_uri_for_get: &'static str,
    base_uri_for_search: &'static str,
}

impl HTTPArticleProvider {
    pub fn new() -> HTTPArticleProvider {
        HTTPArticleProvider{
            base_uri_for_get: "https://en.wikipedia.org/wiki/",
            base_uri_for_search: "https://en.wikipedia.org/w/index.php?title=Special:Search&fulltext=1&search=",
        }
    }
    /// This function takes a `topic` and replaces all spaces with underscores
    fn to_wiki_str(topic: &str) -> String {
        topic.replace(" ", "_")
    }

    /// Parse the body of the HTML search results page and extract all topics found in it.
    fn extract_results_from_search(body: &str) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        // iterate over each tag in the body
        for (_, tag) in htmlstream::tag_iter(body) {
            // if we find an opening <a> tag:
            if (tag.name == "a") && (tag.state == HTMLTagState::Opening) {
                // Extract the title and href of the tag:
                let mut title: Option<String> = None;
                let mut href: Option<String> = None;
                let mut has_data_serp_pos: bool = false;
                for(_, attr) in htmlstream::attr_iter(&tag.attributes) {
                    match attr.name.as_str() {
                        "title" => title = Some(attr.value),
                        "href" => href = Some(attr.value),
                        "data-serp-pos" => has_data_serp_pos = true,
                        _ => (),
                    };
                }
                // check if the <a> was valid:
                if has_data_serp_pos {
                    match (title, href) {
                        (Some(title), Some(href)) => {
                            if &href[..6] == "/wiki/" {
                                results.push(title);
                            }
                        },
                        _ => (),
                    };
                }
            }
        }
        results
    }
}

impl ArticleProvider for HTTPArticleProvider {
    fn get(&self, topic: &str) -> Option<Box<Article>> {
    None
    }

    fn search(&self, topic: &str) -> Vec<String> {
        let mut uri = String::from(self.base_uri_for_search);
        uri.push_str(topic);
        let mut resp = reqwest::get(&uri).unwrap();
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content);
        HTTPArticleProvider::extract_results_from_search(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_wiki_str_is_working() {
        assert_eq!(HTTPArticleProvider::to_wiki_str(""), "");
        assert_eq!(HTTPArticleProvider::to_wiki_str(" test"), "_test");
        assert_eq!(HTTPArticleProvider::to_wiki_str("test test"), "test_test");
        assert_eq!(HTTPArticleProvider::to_wiki_str("test_test"), "test_test");
        assert_eq!(HTTPArticleProvider::to_wiki_str("  __  __"), "________");
    }
    #[test]
    // This test actually fetches search results from Wikipedia. It may FAIL due to changes in the search
    // results themselves rather than a code issue. FIXME test on locally hosted version of wikipedia
    fn search_results_works() {
    let provider = HTTPArticleProvider::new();
    let mut first_3_results = provider.search("test1234");
    first_3_results.resize(3, "".to_owned());
    let search_results_for_test1234 =  vec!["German submarine U-1234".to_owned(),
                                            "2,3,3,3-Tetrafluoropropene".to_owned(),
                                            "Unit testing".to_owned()];
    assert_eq!(first_3_results, search_results_for_test1234);
    }

    #[test]
    fn extract_results_from_empty_search() {
        let empty_vec: Vec<String> = vec![];
        assert_eq!(HTTPArticleProvider::extract_results_from_search(""), empty_vec);
    }

    #[test]
    fn extract_results_from_test1234() {
        let empty_vec: Vec<String> = vec![];
        let search_results_for_test1234 =  vec!["German submarine U-1234".to_owned(),
                                                "2,3,3,3-Tetrafluoropropene".to_owned(),
                                                "Unit testing".to_owned()];
        assert_eq!(HTTPArticleProvider::extract_results_from_search(
                                    "junkblablabla\
                                    <ul class=\"mw-search-results\">\
                                    <li>\
                                    <div class=\"mw-search-result-heading\">\
                                    <a href=\"/wiki/German_submarine_U-1234\" title=\"German submarine U-1234\" data-serp-pos=\"0\">German submarine U-1234\
                                    </a>    \
                                    </div>\
                                    <div class=\"searchresult\">German submarine U-1234 was a Type IXC/40 U-boat of Nazi Germany's Kriegsmarine built during World War II for service in the Battle of the Atlantic. U-1234\
                                    </div> \
                                    <div class=\"mw-search-result-data\">9 KB (893 words) - 19:19, 17 June 2017\
                                    </div>\
                                    </li>\
                                    <li>\
                                    <div class=\"mw-search-result-heading\">\
                                    <a href=\"/wiki/2,3,3,3-Tetrafluoropropene\" title=\"2,3,3,3-Tetrafluoropropene\" data-serp-pos=\"1\">2,3,3,3-Tetrafluoropropene\
                                    </a>    \
                                    </div>\
                                    <div class=\"searchresult\">2,3,3,3-Tetrafluoropropene, or HFO-1234yf, is a hydrofluoroolefin (HFO) with the formula CH2=CFCF3. This colorless gas has been proposed as a replacement\
                                    </div> \
                                    <div class=\"mw-search-result-data\">12 KB (1,291 words) - 17:02, 26 June 2017\
                                    </div>\
                                    </li>\
                                    <li>\
                                    <div class=\"mw-search-result-heading\">\
                                    <a href=\"/wiki/Unit_testing\" title=\"Unit testing\" data-serp-pos=\"2\">Unit testing\
                                    </a>    \
                                    </div>\
                                    <div class=\"searchresult\">In computer programming, unit testing is a software testing method by which individual units of source code, sets of one or more computer program modules\
                                    </div> \
                                    <div class=\"mw-search-result-data\">28 KB (3,455 words) - 18:46, 5 August 2017\
                                    </div>")
, search_results_for_test1234);
    }
}