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
    let results = scraping::scrape_urls_from_file("urls.txt", &config).await?;
    
    for (url, html) in results {
        // Process HTML with AI
        let processed = ai::process_html_content(&html, &url).await?;
        
        // Save markdown file
        let output_path = Path::new("output").join(&processed.filename);
        fs::write(&output_path, processed.content).await?;
        
        println!("Processed {} -> {}", url, processed.filename);
    }

    Ok(())
}
