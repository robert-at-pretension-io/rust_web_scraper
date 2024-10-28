use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub link: String,
    pub snippet: Option<String>,
    pub position: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerpApiResponse {
    pub search_metadata: SearchMetadata,
    pub organic_results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchMetadata {
    pub status: String,
    pub total_time_taken: f32,
}
