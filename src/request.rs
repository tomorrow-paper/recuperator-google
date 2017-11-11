use tomorrow_recuperator::Request;

pub struct GoogleRequest {
    pub query: String
}

impl GoogleRequest {

    pub fn new(query: &str) -> Self {
        GoogleRequest {
            query: format!("search?q={}", query)
        }
    }
}

impl Request for GoogleRequest {}

