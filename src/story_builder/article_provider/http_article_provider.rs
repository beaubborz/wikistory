extern crate reqwest;
extern crate htmlstream;
use story_builder::article_provider::*;
use std::io::Read;
use self::htmlstream::HTMLTagState;

struct HTTPArticle {
    paragraphs: Vec<Paragraph>,
}

impl Article for HTTPArticle {
    fn get_paragraphs(&self) -> &Vec<Paragraph> {
        &self.paragraphs
    }
}

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

    /// Parse the body of the HTML article page and extract all paragraphs along with
    /// the topics found in them.
    fn extract_paragraphs_from_body(body: &str) -> Vec<Paragraph> {
        if body == "" {
            return vec![];
        }
        #[derive(Copy, Clone)]
        #[allow(non_camel_case_types)]
        enum State {
            SEARCH_FOR_P,
            READING_P,
            SKIPPING_SUP,
        }
        let mut state = State::SEARCH_FOR_P;
        let mut paragraphs: Vec<Paragraph> = vec![];
        let mut current_par: Option<Paragraph> = None;

        for (_, tag) in htmlstream::tag_iter(body) {
            match (state, tag.name.as_str()) {
                // If we are in state SEARCH_FOR_P and find a p:
                (State::SEARCH_FOR_P, "p") => {
                    // Create a new empty Paragraph:
                    current_par = Some(Paragraph {
                        text: String::from(""),
                        topics: vec![],
                    });
                    state = State::READING_P; // Switch to READING_P state.
                },
                // IF we are in READING_P state and finds text outside of a tag, append it to the current paragraph:
                (State::READING_P, "") => {
                    current_par.as_mut().unwrap().text.push_str(&tag.html);
                },
                // IF we are in READING_P state and finds an <a> tag, append it to the topic list of this paragraph:
                (State::READING_P, "a") => {
                    // Try to find the title of the reference:
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        if attr.name == "title" {
                            &current_par.as_mut().unwrap().topics.push(attr.value);
                        }
                    }
                },
                // IF we are in READING_P state and finds a <p> tag, then it is the end of the </p>;
                // Push it in the final vector and return to the initial state.
                (State::READING_P, "p") => {

                    let par = current_par.take().unwrap();
                    paragraphs.push(par);

                    state = State::SEARCH_FOR_P;
                },
                (State::READING_P, "sup") => {
                    // Enter SKIPPING_SUP state; we skip all tags until we found the corresponding </sup>
                    state = State::SKIPPING_SUP;
                },
                (State::SKIPPING_SUP, "sup") => {
                    // We are done skipping; back to READING_P
                    state = State::READING_P;
                },
                _ => (),
            };
        }
        paragraphs // Return the vector we built.
    }
}

impl ArticleProvider for HTTPArticleProvider {
    fn get(&self, topic: &str) -> Option<Box<Article + Send>> {
        if topic == "" {
            return None; // Do not even try if the topic is empty.
            }
        let mut uri = String::from(self.base_uri_for_get);
        uri.push_str(&HTTPArticleProvider::to_wiki_str(topic));
        let mut resp = reqwest::get(&uri).unwrap();
        // IF for whatever reason we do not get an OK from wikipedia,
        // consider as "Not found" and return None.
        if resp.status() != reqwest::StatusCode::Ok {
            return None;
        }

        let mut content = String::new();
        resp.read_to_string(&mut content).expect("Could not read content from HTTP response.");

        Some(Box::new(HTTPArticle {
            paragraphs: HTTPArticleProvider::extract_paragraphs_from_body(&content)
        }))
    }

    fn search(&self, topic: &str) -> Vec<String> {
        let mut uri = String::from(self.base_uri_for_search);
        uri.push_str(&HTTPArticleProvider::to_wiki_str(topic));
        let mut resp = reqwest::get(&uri).unwrap();
        assert!(resp.status().is_success());
        let mut content = String::new();
        resp.read_to_string(&mut content).expect("Could not read content from HTTP response.");
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

    #[test]
    fn get_from_empty_returns_none() {
        let provider = HTTPArticleProvider::new();
        match provider.get("") {
            None => (),
            _ => panic!("Expected None, got Some."),
        }
    }

    #[test]
    fn get_from_non_existing_returns_none() {
        let provider = HTTPArticleProvider::new();
        match provider.get("fsdafgnhtyunjfthdhtydfrt67yh65dgdtydtvydrgdrt") {
            None => (),
            _ => panic!("Expected None, got Some."),
        }
    }

    #[test]
    fn test_get_html_parser_empty_body_should_return_empty_vec() {
        assert!(HTTPArticleProvider::extract_results_from_search("").len() == 0);
    }

    #[test]
    fn test_get_html_parser_with_basic_p_format() {
        let mut body = String::new();
        body.push_str("junkhere......");
        body.push_str("<p>");
        body.push_str("Lynx's most notable <a href=\"/wiki/Deep_sky_object\" class=\"mw-redirect\" title=\"Deep sky object\">");
        body.push_str("deep sky object</a>");
        body.push_str(" is <a href=\"/wiki/NGC_2419\" title=\"NGC 2419\">");
        body.push_str("NGC 2419</a>");
        body.push_str(", also called the \"Intergalactic Wanderer\" as it was assumed to lie outside the Milky Way. At a distance of between 275,000 and 300,000 light-years from Earth, it is one of the most distant known <a href=\"/wiki/Globular_cluster\" title=\"Globular cluster\">");
        body.push_str("globular clusters</a>");
        body.push_str(" within our galaxy. NGC 2419 is likely in a highly elliptical orbit around the <a href=\"/wiki/Milky_Way\" title=\"Milky Way\">");
        body.push_str("Milky Way</a>");
        body.push_str(".<sup id=\"cite_ref-34\" class=\"reference\">");
        body.push_str("<a href=\"#cite_note-34\">");
        body.push_str("[30]</a>");
        body.push_str("</sup>");
        body.push_str(" It has a magnitude of 10.3 and is a <a href=\"/wiki/Shapley%E2%80%93Sawyer_Concentration_Class\" title=\"Shapley–Sawyer Concentration Class\">");
        body.push_str("Shapley class II</a>");
        body.push_str(" cluster; this classification indicates that it is extremely concentrated at its center. Originally thought to be a star, NGC 2419 was discovered to be a globular cluster by American astronomer <a href=\"/wiki/Carl_Otto_Lampland\" title=\"Carl Otto Lampland\">");
        body.push_str("Carl Lampland</a>");
        body.push_str(".<sup id=\"cite_ref-35\" class=\"reference\">");
        body.push_str("<a href=\"#cite_note-35\">");
        body.push_str("[31]</a>");
        body.push_str("</sup>");
        body.push_str("</p>");
body.push_str("junkafter...</html>");
        let result = HTTPArticleProvider::extract_paragraphs_from_body(&body);
        assert!(result.len() == 1);
        assert_eq!(result[0].text, "Lynx's most notable deep sky object is NGC 2419, also called the \"Intergalactic Wanderer\" as it was assumed to lie outside the Milky Way. At a distance of between 275,000 and 300,000 light-years from Earth, it is one of the most distant known globular clusters within our galaxy. NGC 2419 is likely in a highly elliptical orbit around the Milky Way. It has a magnitude of 10.3 and is a Shapley class II cluster; this classification indicates that it is extremely concentrated at its center. Originally thought to be a star, NGC 2419 was discovered to be a globular cluster by American astronomer Carl Lampland.");
    }
}