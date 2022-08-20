use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryResponse {
    pub titles: Titles,
    pub description: String,
    pub content_urls: ContentUrls,
    pub extract: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentUrls {
    pub desktop: URL,
    pub mobile: URL,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URL {
    pub page: String,
    pub revisions: String,
    pub edit: String,
    pub talk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Titles {
    pub canonical: String,
    pub normalized: String,
    pub display: String,
}
