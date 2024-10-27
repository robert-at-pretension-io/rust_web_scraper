mod ai;
mod db;
mod logging;
mod scraping;
mod models;

use anyhow::Result;
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize scraping config
    let config = scraping::ScrapingConfig::new()?;
    
    // Create output directory if it doesn't exist
    fs::create_dir_all("output").await?;
    
    // Scrape URLs and process content
    let urls = scraping::read_unprocessed_urls("urls.txt").await?;
    
    for url in urls {
        match scraping::scrape_url(&url, &config).await {
            Ok(html) => {
                match ai::process_html_content(&html, &url).await {
                    Ok(processed) => {
                        // Save markdown file
                        let output_path = Path::new("output").join(&processed.filename);
                        fs::write(&output_path, &processed.content).await?;
                        
                        // Only mark as processed if everything succeeded
                        scraping::mark_url_processed(&url).await?;
                        
                        println!("Processed {} -> {}", url, processed.filename);
                    },
                    Err(e) => {
                        println!("Failed to process content for {}: {}", url, e);
                        continue;
                    }
                }
            },
            Err(e) => {
                println!("Failed to scrape {}: {}", url, e);
                continue;
            }
        }
    }

    Ok(())
}
