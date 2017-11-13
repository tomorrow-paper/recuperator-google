use tomorrow_recuperator::Response;

use ::models::SearchResult;

#[derive(Debug)]
pub struct GoogleResponse {
    pub results: Vec<SearchResult>
}

impl GoogleResponse {

    pub fn new(results: Vec<SearchResult>) -> Self {
        GoogleResponse {
            results: results
        }
    }
}

impl Response for GoogleResponse {}