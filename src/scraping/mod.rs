use anyhow::{Result, Context};
use reqwest::Url;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use crate::logging::{self, LogLevel};

#[derive(Clone)]
pub struct ScrapingConfig {
    api_key: String,
    premium_proxy: bool,
    stealth_proxy: bool,
}

impl ScrapingConfig {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let api_key = env::var("SCRAPING_BEE_API_KEY")
            .context("SCRAPING_BEE_API_KEY must be set in environment")?;
        Ok(Self { 
            api_key,
            premium_proxy: true,
            stealth_proxy: false,
        })
    }

    pub fn with_key(api_key: String) -> Self {
        Self { 
            api_key,
            premium_proxy: true,
            stealth_proxy: false,
        }
    }

    pub fn with_stealth_mode(mut self) -> Self {
        self.stealth_proxy = true;
        self.premium_proxy = false;
        self
    }
}

#[derive(Deserialize)]
struct ScrapingBeeResponse {
    body: String,
}

pub async fn scrape_url(url: &str, config: &ScrapingConfig) -> Result<String> {
    // Create inner function that can be boxed
    async fn scrape_url_inner(url: &str, config: &ScrapingConfig) -> Result<String> {
        logging::log(LogLevel::Info, &format!("Starting to scrape URL: {}", url)).await?;
        
        let client = reqwest::Client::new();
        
        let mut params = vec![
            ("api_key", config.api_key.clone()),
            ("url", url.to_string()),
            ("render_js", "true".to_string()),
            ("block_ads", "true".to_string()),
        ];

        if config.premium_proxy {
            params.push(("premium_proxy", "true".to_string()));
        }

        if config.stealth_proxy {
            params.push(("stealth_proxy", "true".to_string()));
        }

        logging::log(LogLevel::Info, "Building ScrapingBee API URL").await?;
        let api_url = Url::parse_with_params(
            "https://app.scrapingbee.com/api/v1/", 
            &params
        ).context("Failed to build API URL")?;

        let mut attempts = 0;
        let max_attempts = 3;

        while attempts < max_attempts {
            logging::log(LogLevel::Info, &format!("Attempt {} of {}", attempts + 1, max_attempts)).await?;
            
            match client.get(api_url.clone()).send().await {
                Ok(response) => {
                    let status = response.status();
                    let text = response.text().await.context("Failed to get response text")?;

                    if status.is_success() {
                        match serde_json::from_str::<ScrapingBeeResponse>(&text) {
                            Ok(scraped) => return Ok(scraped.body),
                            Err(_) => {
                                if text.trim().starts_with("<") {
                                    return Ok(text);
                                }
                            }
                        }
                    }

                    logging::log(LogLevel::Warning, &format!(
                        "ScrapingBee API error: {} - {}", 
                        status, text
                    )).await?;

                    if (status == 403 || text.contains("captcha")) && !config.stealth_proxy {
                        logging::log(LogLevel::Info, "Switching to stealth mode").await?;
                        let mut stealth_config = config.clone();
                        stealth_config.stealth_proxy = true;
                        stealth_config.premium_proxy = false;
                        // Box the recursive call
                        return Box::pin(scrape_url_inner(url, &stealth_config)).await;
                    }
                }
                Err(e) => {
                    logging::log(LogLevel::Error, &format!("Request failed: {}", e)).await?;
                }
            }

            attempts += 1;
            if attempts < max_attempts {
                let delay = std::time::Duration::from_secs(2u64.pow(attempts as u32));
                tokio::time::sleep(delay).await;
            }
        }

        Err(anyhow::anyhow!("Failed to scrape URL after {} attempts", max_attempts))
    }

    // Call the inner function
    scrape_url_inner(url, config).await
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
