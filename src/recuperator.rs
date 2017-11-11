use tomorrow_core::Result;
use tomorrow_http::Builder;
use tomorrow_http::raw::*;
use tomorrow_recuperator::Recuperator;

use select::document::Document;
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
        let names = self.extract_names(&document);
        let descriptions = self.extract_descriptions(&document);
        let links = self.extract_links(&document);

        names.iter()
             .zip(descriptions.iter())
             .zip(links.iter())
             .map(|((name, description), url)| SearchResult::new(name.as_ref(), description.as_ref(), url.as_ref()))
             .collect()
    }

    fn extract_names(&self, document: &Document) -> Vec<String> {
        document.find(Class("g"))
            .map(|g| g.find(Name("h3")).next())
            .filter(Option::is_some).map(Option::unwrap)
            .map(|node| node.text())
            .collect()
    }

    fn extract_descriptions(&self, document: &Document) -> Vec<String> {
        document.find(Class("g"))
            .map(|g| g.find(Class("st")).next())
            .filter(Option::is_some).map(Option::unwrap)
            .map(|node| node.text())
            .collect()
    }

    fn extract_links(&self, document: &Document) -> Vec<String> {
        document.find(Class("g"))
            .map(|g| g.find(Name("h3")).next())
            .filter(Option::is_some).map(Option::unwrap)
            .map(|node| node.find(Name("a")).next())
            .filter(Option::is_some).map(Option::unwrap)
            .map(|a| a.attr("href"))
            .filter(Option::is_some).map(Option::unwrap)
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