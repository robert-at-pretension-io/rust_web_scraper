mod ai;
mod db;
mod logging;
mod scraping;
mod models;
mod search;

use anyhow::Result;
use std::path::Path;
use tokio::fs;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Commands {
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,
        
        /// Save results to urls.txt
        #[arg(short, long)]
        save: bool,
    },
    Scrape {
        /// File containing URLs to scrape
        #[arg(short, long, default_value = "urls.txt")]
        urls_file: String,

        /// File to store processed URLs
        #[arg(short, long, default_value = "processed_urls.txt")]
        processed_file: String,

        /// Output directory for markdown files
        #[arg(short, long, default_value = "output")]
        output_dir: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    match Commands::parse() {
        Commands::Search { query, save } => {
            let api_key = std::env::var("SERPAPI_KEY")
                .context("SERPAPI_KEY must be set in environment")?;
            
            let client = search::SerpApiClient::new(api_key);
            let results = client.search(&query).await?;
            
            for result in &results {
                println!("Title: {}", result.title);
                println!("URL: {}", result.link);
                if let Some(snippet) = &result.snippet {
                    println!("Snippet: {}", snippet);
                }
                println!("---");
            }

            if save {
                let urls: String = results.iter()
                    .map(|r| format!("{}\n", r.link))
                    .collect();
                fs::write("urls.txt", urls).await?;
                println!("Saved {} URLs to urls.txt", results.len());
            }
        }
        
        Commands::Scrape { urls_file, processed_file, output_dir } => {
            // Initialize scraping config
            let config = scraping::ScrapingConfig::new()?;
            
            // Create output directory if it doesn't exist
            fs::create_dir_all(&output_dir).await?;
    
    // Scrape URLs and process content
    let urls = scraping::read_unprocessed_urls(&args.urls_file, &args.processed_file).await?;
    
    for url in urls {
        match scraping::scrape_url(&url, &config).await {
            Ok(html) => {
                match ai::process_html_content(&html, &url).await {
                    Ok(processed) => {
                        // Save markdown file
                        let output_path = Path::new(&args.output_dir).join(&processed.filename);
                        fs::write(&output_path, &processed.content).await?;
                        
                        // Only mark as processed if everything succeeded
                        scraping::mark_url_processed(&url, &args.processed_file).await?;
                        
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
