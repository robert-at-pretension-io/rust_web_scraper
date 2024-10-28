use anyhow::{Result, Context};
use reqwest::Url;
use crate::search::models::{SearchResult, SerpApiResponse};

pub struct SerpApiClient {
    api_key: String,
    client: reqwest::Client,
}

impl SerpApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let api_url = Url::parse_with_params(
            "https://serpapi.com/search.json",
            &[
                ("api_key", &self.api_key),
                ("engine", &"google".to_string()),
                ("q", &query.to_string()),
                ("num", &"100".to_string()), // Request maximum results
                ("start", &"0".to_string()),
            ],
        ).context("Failed to build API URL")?;

        let response = self.client.get(api_url)
            .send()
            .await
            .context("Failed to send request to SerpApi")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("SerpApi error: {} - {}", status, error_text));
        }

        let search_response: SerpApiResponse = response.json().await
            .context("Failed to parse SerpApi response")?;

        Ok(search_response.organic_results)
    }
}
