use anyhow::{Result, Context};
use reqwest::Url;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use crate::logging::{self, LogLevel};

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
    logging::log(LogLevel::Info, &format!("Starting to scrape URL: {}", url)).await?;
    
    let client = reqwest::Client::new();
    
    logging::log(LogLevel::Info, "Building ScrapingBee API URL").await?;
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

    logging::log(LogLevel::Info, "Sending request to ScrapingBee API").await?;
    let response = client.get(api_url)
        .send()
        .await
        .context("Failed to send request to ScrapingBee")?;

    let status = response.status();
    let text = response.text().await.context("Failed to get response text")?;

    if !status.is_success() {
        let error_msg = format!(
            "ScrapingBee API error: {} - {}", 
            status, text
        );
        logging::log(LogLevel::Error, &error_msg).await?;
        return Err(anyhow::anyhow!(error_msg));
    }

    logging::log(LogLevel::Info, "Successfully received response from ScrapingBee").await?;

    // Try to parse as JSON first
    match serde_json::from_str::<ScrapingBeeResponse>(&text) {
        Ok(scraped) => Ok(scraped.body),
        Err(e) => {
            // If JSON parsing fails, check if we got HTML directly
            if text.trim().starts_with("<") {
                Ok(text)
            } else {
                logging::log(LogLevel::Error, &format!("Failed to parse response: {}", e)).await?;
                Err(anyhow::anyhow!("Failed to parse ScrapingBee response"))
            }
        }
    }
}

pub async fn read_unprocessed_urls(urls_path: &str, processed_path: &str) -> Result<Vec<String>> {
    logging::log(LogLevel::Info, &format!("Reading URLs from file: {}", urls_path)).await?;
    
    // Read URLs file
    let content = tokio::fs::read_to_string(urls_path).await
        .context("Failed to read URLs file")?;

    // Read processed URLs file
    let processed_content = tokio::fs::read_to_string(processed_path).await.unwrap_or_default();
    let processed_urls: Vec<_> = processed_content.lines().collect();
    
    // Filter out empty lines and already processed URLs
    let urls: Vec<_> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|url| !processed_urls.contains(url))
        .map(|s| s.to_string())
        .collect();
    
    logging::log(LogLevel::Info, &format!("Found {} unprocessed URLs", urls.len())).await?;
    
    Ok(urls)
}

pub async fn mark_url_processed(url: &str, processed_path: &str) -> Result<()> {
    logging::log(LogLevel::Info, &format!("Marking URL as processed: {}", url)).await?;
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(processed_path)
        .await?;
        
    use tokio::io::AsyncWriteExt;
    file.write_all(url.as_bytes()).await?;
    file.write_all(b"\n").await?;
    
    Ok(())
}
