use tomorrow_core::Result;
use tomorrow_http::Builder;
use tomorrow_http::raw::*;
use tomorrow_recuperator::Recuperator;

use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

use ::{GoogleRequest, GoogleResponse};
use ::models::SearchResult;

const API_URL: &'static str = "https://encrypted.google.com";

pub struct GoogleRecuperator<T> where T: Requester {
    requester: T
}

impl <T> GoogleRecuperator<T> where T: Requester {
    
    pub fn new(requester: T) -> Self {
        GoogleRecuperator {
            requester: requester
        }
    }

    fn extract_results(&self, document: Document) -> Vec<SearchResult> {
        document.find(Class("g"))
            .map(|g| self.map_to_search_result(g))
            .filter(|search_result| search_result.name.len() > 0)
            .collect()
    }

    fn map_to_search_result(&self, node: Node) -> SearchResult {
        let name = self.extract_name(&node);
        let description = self.extract_description(&node);
        let url = self.extract_link(&node);

        SearchResult::new(name.as_ref(), description.as_ref(), url.as_ref())
    }

    fn extract_name(&self, node: &Node) -> String {
        node.find(Name("h3"))
            .map(|h3| h3.text())
            .collect()
    }

    fn extract_description(&self, node: &Node) -> String {
        node.find(Class("st"))
            .map(|st| st.text())
            .collect()
    }

    fn extract_link(&self, node: &Node) -> String {
        node.find(Name("h3"))
            .map(|h3| h3.find(Name("a")).next())
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|a| a.attr("href"))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|url| self.skip_url_prefix(url))
            .collect()
    }

    fn skip_url_prefix(&self, url: &str) -> String {
        url.chars().skip(7).collect::<String>()
    }
}

impl <T> Recuperator<GoogleRequest, GoogleResponse> for GoogleRecuperator<T> where T: Requester {

    fn compute(&self, request: GoogleRequest) -> Result<GoogleResponse> {
        let html = self.requester.request(&request.query)?;
        let document = Document::from(html.as_ref());

        let results = self.extract_results(document);
        let response = GoogleResponse::new(results);

        Ok(response)
    }
}

impl Default for GoogleRecuperator<Client> {

    fn default() -> Self {
        let client: Client = Builder::https(API_URL).into();
        GoogleRecuperator::new(client)
    }
}