use anyhow::{Result, Context};
use reqwest::Url;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

pub struct ScrapingConfig {
    api_key: String,
}

impl ScrapingConfig {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("SCRAPING_BEE_API_KEY")
            .context("SCRAPING_BEE_API_KEY must be set in environment")?;
        Ok(Self { api_key })
    }

    pub fn with_key(api_key: String) -> Self {
        Self { api_key } 
    }
}

#[derive(Deserialize)]
struct ScrapingBeeResponse {
    body: String,
}

pub async fn scrape_url(url: &str, config: &ScrapingConfig) -> Result<String> {
    let client = reqwest::Client::new();
    
    // Build ScrapingBee API URL with parameters
    let api_url = Url::parse_with_params(
        "https://app.scrapingbee.com/api/v1/", 
        &[
            ("api_key", &config.api_key),
            ("url", &url.to_string()),
            ("render_js", &String::from("true")), // Disable JS rendering to save credits
            ("block_ads", &String::from("true")),  // Block ads  
            ("block_resources", &String::from("true")), // Block images/CSS to speed up request
        ]
    ).context("Failed to build API URL")?;

    // Make request to ScrapingBee API
    let response = client.get(api_url)
        .send()
        .await
        .context("Failed to send request to ScrapingBee")?;

    // Check if request was successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "ScrapingBee API error: {} - {}", 
            response.status(),
            response.text().await.unwrap_or_default()
        ));
    }

    // Parse response
    let scraped = response
        .json::<ScrapingBeeResponse>()
        .await
        .context("Failed to parse ScrapingBee response")?;

    Ok(scraped.body)
}

pub async fn scrape_urls_from_file(path: &str, config: &ScrapingConfig) -> Result<Vec<(String, String)>> {
    let content = tokio::fs::read_to_string(path).await?;
    let mut results = Vec::new();
    
    for url in content.lines() {
        if !url.trim().is_empty() {
            let content = scrape_url(url, config).await?;
            results.push((url.to_string(), content));
        }
    }
    
    Ok(results)
}
