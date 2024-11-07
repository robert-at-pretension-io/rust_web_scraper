mod ai;
mod db;
mod logging;
mod scraping;
mod models;
mod search;
mod cli;
mod crawling;
mod utils;

use anyhow::{Result, Context};
use std::path::Path;
use tokio::fs;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Search for content
    Search {
        /// Search query
        #[arg(short, long)]
        query: String,
        
        /// Save results to urls.txt
        #[arg(short, long)]
        save: bool,

        /// Number of URLs to select
        #[arg(short, long, default_value = "10")]
        num_urls: usize,
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
    /// Crawl starting from a URL
    Crawl {
        /// Starting URL to crawl
        #[arg(short, long)]
        url: String,

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
    
    // Check if any command line arguments were provided
    if std::env::args().len() > 1 {
        // Use traditional CLI mode
        let cli = Cli::parse();
    match cli.command {
        Commands::Search { query, save, num_urls } => {
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
                let selected_urls = ai::select_urls(&results, num_urls).await?;
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("urls.txt")
                    .await?;
                
                use tokio::io::AsyncWriteExt;
                let urls_text = selected_urls.join("\n") + "\n";
                file.write_all(urls_text.as_bytes()).await?;
                println!("Appended {} selected URLs to urls.txt", selected_urls.len());
            }
        }
        
        Commands::Scrape { urls_file, processed_file, output_dir } => {
            // Initialize scraping config
            let config = scraping::ScrapingConfig::new()?;
            
            // Create output directory if it doesn't exist
            fs::create_dir_all(&output_dir).await?;
    
            // Scrape URLs and process content
            let urls = scraping::read_unprocessed_urls(&urls_file, &processed_file).await?;
    
            for url in urls {
                match scraping::scrape_url(&url, &config).await {
                    Ok(html) => {
                        match ai::process_html_content(&html, &url).await {
                            Ok(processed) => {
                                // Save markdown file
                                let output_path = Path::new(&output_dir).join(&processed.filename);
                                fs::write(&output_path, &processed.content).await?;
                                
                                // Only mark as processed if everything succeeded
                                scraping::mark_url_processed(&url, &processed_file).await?;
                                
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
        }
        
        Commands::Crawl { url, processed_file, output_dir } => {
            cli::handle_crawl(&url, &processed_file, &output_dir).await?;
        }
    }
        Ok(())
    } else {
        // Use interactive mode
        cli::interactive_mode().await
    }
}
