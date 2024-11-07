use anyhow::Result;
use regex::Regex;
use url::Url;
use crate::crawling::url_filter::UrlFilter;

pub mod url_filter;
use crate::scraping::{ScrapingConfig, scrape_url, mark_url_processed};
use crate::ai;
use std::collections::HashSet;
use tokio::fs;
use std::path::Path;
use crate::utils::with_spinner;
use crate::logging::{log, LogLevel};

pub async fn crawl_url(
    start_url: &str, 
    processed_file: &str,
    output_dir: &str,
    config: &ScrapingConfig,
    max_depth: usize,
    purpose: &str,
) -> Result<()> {
    let mut url_filter = UrlFilter::new("disallow_urls.txt").await?;
    log(LogLevel::Info, &format!("Starting crawl from {} with purpose: {}", start_url, purpose)).await?;
    let mut visited = HashSet::new();
    let mut to_visit = vec![(start_url.to_string(), 0)];
    
    // Create output directory if it doesn't exist
    with_spinner("Creating output directory...", async {
        fs::create_dir_all(output_dir).await?;
        log(LogLevel::Info, "Output directory created/verified").await?;
        Ok::<_, anyhow::Error>(())
    }).await?;

    while let Some((url, depth)) = to_visit.pop() {
        if depth > max_depth || visited.contains(&url) || !url_filter.is_allowed(&url) {
            continue;
        }
        
        visited.insert(url.clone());
        
        match scrape_url(&url, config).await {
            Ok(html) => {
                match ai::process_html_content(&html, &url).await {
                    Ok(processed) => {
                        // Save markdown file
                        let output_path = Path::new(output_dir).join(&processed.filename);
                        fs::write(&output_path, &processed.content).await?;
                        mark_url_processed(&url, processed_file).await?;
                        println!("Processed {} -> {}", url, processed.filename);
                    },
                    Err(e) => {
                        log(LogLevel::Error, &format!(
                            "Failed to process content for {}: {}", url, e
                        )).await?;
                        continue;
                    }
                }
            }
            Err(e) => {
                log(LogLevel::Error, &format!(
                    "Failed to scrape {}: {}. Skipping URL.", url, e
                )).await?;
                
                // Mark URL as processed to avoid retrying
                mark_url_processed(&url, processed_file).await?;
                continue;
                    
                    // Extract and queue new URLs if not at max depth
                    if depth < max_depth {
                        let new_urls = extract_urls(&html, &url)?;
                        
                        // Let user select which URLs to crawl
                        let selected_urls = url_filter.select_urls(&new_urls).await?;
                        
                        // Add unselected URLs to disallow list
                        for url in new_urls.iter() {
                            if !selected_urls.contains(&url) {
                                url_filter.add_to_disallow(url, "disallow_urls.txt").await?;
                            }
                        }
                        
                        // Queue selected URLs
                        for url in selected_urls {
                            if !visited.contains(url) {
                                to_visit.push((url.clone(), depth + 1));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to scrape {}: {}", url, e);
            }
        }
    }

    Ok(())
}

fn extract_urls(html: &str, base_url: &str) -> Result<Vec<String>> {
    let base_url = Url::parse(base_url)?;
    let href_regex = Regex::new(r#"href=["']([^"']+)["']"#)?;
    
    let mut urls = Vec::new();
    for cap in href_regex.captures_iter(html) {
        if let Some(href) = cap.get(1) {
            if let Ok(url) = base_url.join(href.as_str()) {
                // Only keep http(s) URLs
                if url.scheme() == "http" || url.scheme() == "https" {
                    urls.push(url.to_string());
                }
            }
        }
    }
    
    Ok(urls)
}
