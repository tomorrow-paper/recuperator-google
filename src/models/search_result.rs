#[derive(
    Serialize, Deserialize,
    Eq, PartialEq, Ord, PartialOrd,
    Debug, Clone
)]
pub struct SearchResult {
    pub name: String,
    pub description: String,
    pub url: String
}

impl SearchResult {

    pub fn new(name: &str, description: &str, url: &str) -> Self {
        SearchResult {
            name: String::from(name),
            description: String::from(description),
            url: String::from(url)
        }
    }
}