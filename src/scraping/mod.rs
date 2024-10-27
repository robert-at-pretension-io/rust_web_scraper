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
    logging::log(LogLevel::Info, &format!("Starting to scrape URL: {}", url))?;
    
    let client = reqwest::Client::new();
    
    logging::log(LogLevel::Info, "Building ScrapingBee API URL")?;
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

    logging::log(LogLevel::Info, "Sending request to ScrapingBee API")?;
    let response = client.get(api_url)
        .send()
        .await
        .context("Failed to send request to ScrapingBee")?;

    if !response.status().is_success() {
        let error_msg = format!(
            "ScrapingBee API error: {} - {}", 
            response.status(),
            response.text().await.unwrap_or_default()
        );
        logging::log(LogLevel::Error, &error_msg)?;
        return Err(anyhow::anyhow!(error_msg));
    }

    logging::log(LogLevel::Info, "Successfully received response from ScrapingBee")?;

    // Parse response
    let scraped = response
        .json::<ScrapingBeeResponse>()
        .await
        .context("Failed to parse ScrapingBee response")?;

    Ok(scraped.body)
}

pub async fn scrape_urls_from_file(path: &str, config: &ScrapingConfig) -> Result<Vec<(String, String)>> {
    logging::log(LogLevel::Info, &format!("Reading URLs from file: {}", path))?;
    
    let content = tokio::fs::read_to_string(path).await
        .context("Failed to read URLs file")?;
    
    let mut results = Vec::new();
    let urls: Vec<_> = content.lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    
    logging::log(LogLevel::Info, &format!("Found {} URLs to process", urls.len()))?;
    
    for (i, url) in urls.iter().enumerate() {
        logging::log(LogLevel::Info, &format!("Processing URL {}/{}: {}", i + 1, urls.len(), url))?;
        
        match scrape_url(url, config).await {
            Ok(content) => {
                logging::log(LogLevel::Info, &format!("Successfully scraped {}", url))?;
                results.push((url.to_string(), content));
            }
            Err(e) => {
                logging::log(LogLevel::Error, &format!("Failed to scrape {}: {}", url, e))?;
                // Continue with next URL instead of failing completely
                continue;
            }
        }
    }
    
    logging::log(LogLevel::Info, &format!("Completed scraping {} URLs", results.len()))?;
    Ok(results)
}
