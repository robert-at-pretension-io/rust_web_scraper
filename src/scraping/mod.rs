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

pub async fn scrape_urls_from_file(path: &str, config: &ScrapingConfig) -> Result<Vec<(String, String)>> {
    logging::log(LogLevel::Info, &format!("Reading URLs from file: {}", path)).await?;
    
    // Read URLs file
    let content = tokio::fs::read_to_string(path).await
        .context("Failed to read URLs file")?;

    // Read or create processed URLs file
    let processed_path = "processed_urls.txt";
    let processed_content = tokio::fs::read_to_string(processed_path).await.unwrap_or_default();
    let processed_urls: Vec<_> = processed_content.lines().collect();
    
    let mut results = Vec::new();
    let urls: Vec<_> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|url| !processed_urls.contains(url))
        .collect();
    
    logging::log(LogLevel::Info, &format!("Found {} URLs to process", urls.len())).await?;
    
    for (i, url) in urls.iter().enumerate() {
        logging::log(LogLevel::Info, &format!("Processing URL {}/{}: {}", i + 1, urls.len(), url)).await?;
        
        match scrape_url(url, config).await {
            Ok(content) => {
                logging::log(LogLevel::Info, &format!("Successfully scraped {}", url)).await?;
                results.push((url.to_string(), content));
            }
            Err(e) => {
                logging::log(LogLevel::Error, &format!("Failed to scrape {}: {}", url, e)).await?;
                // Continue with next URL instead of failing completely
                continue;
            }
        }
    }
    
    logging::log(LogLevel::Info, &format!("Completed scraping {} URLs", results.len())).await?;
    
    // Append newly processed URLs to processed_urls.txt
    if !results.is_empty() {
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(processed_path)
            .await?;
            
        for (url, _) in &results {
            use tokio::io::AsyncWriteExt;
            file.write_all(url.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }
    }
    
    Ok(results)
}
