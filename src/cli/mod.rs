use anyhow::Result;
use inquire::{Select, Text, Confirm};
use crate::search::SerpApiClient;

pub async fn interactive_mode() -> Result<()> {
    // Main menu
    let options = vec!["Search", "Scrape", "Crawl", "Exit"];
    let choice = Select::new("What would you like to do?", options).prompt()?;

    match choice {
        "Search" => handle_search().await?,
        "Scrape" => handle_scrape().await?,
        "Crawl" => handle_crawl_interactive().await?,
        "Exit" => return Ok(()),
        _ => unreachable!(),
    }

    Ok(())
}

async fn handle_crawl_interactive() -> Result<()> {
    let url = Text::new("Enter the starting URL:").prompt()?;
    
    let processed_file = Text::new("Enter the processed URLs file path:")
        .with_default("processed_urls.txt")
        .prompt()?;

    let output_dir = Text::new("Enter the output directory:")
        .with_default("output")
        .prompt()?;

    let max_depth = Text::new("Enter maximum crawl depth:")
        .with_default("2")
        .prompt()?
        .parse::<usize>()?;

    let config = crate::scraping::ScrapingConfig::new()?;
    crate::crawling::crawl_url(&url, &processed_file, &output_dir, &config, max_depth).await?;

    Ok(())
}

pub async fn handle_crawl(url: &str, processed_file: &str, output_dir: &str) -> Result<()> {
    let config = crate::scraping::ScrapingConfig::new()?;
    let max_depth = 2; // Default depth for CLI mode
    crate::crawling::crawl_url(url, processed_file, output_dir, &config, max_depth).await
}

async fn handle_search() -> Result<()> {
    let query = Text::new("Enter your search query:").prompt()?;
    
    let api_key = std::env::var("SERPAPI_KEY")?;
    let client = SerpApiClient::new(api_key);
    let results = client.search(&query).await?;

    // Display results
    for result in &results {
        println!("Title: {}", result.title);
        println!("URL: {}", result.link);
        if let Some(snippet) = &result.snippet {
            println!("Snippet: {}", snippet);
        }
        println!("---");
    }

    let save = Confirm::new("Would you like to save the URLs?")
        .with_default(false)
        .prompt()?;

    if save {
        let num_urls = Text::new("How many URLs would you like to select?")
            .with_default("10")
            .prompt()?
            .parse::<usize>()?;

        let selected_urls = crate::ai::select_urls(&results, num_urls).await?;
        
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("urls.txt")
            .await?;
        
        let urls_text = selected_urls.join("\n") + "\n";
        file.write_all(urls_text.as_bytes()).await?;
        println!("Appended {} selected URLs to urls.txt", selected_urls.len());
    }

    Ok(())
}

async fn handle_scrape() -> Result<()> {
    let urls_file = Text::new("Enter the URLs file path:")
        .with_default("urls.txt")
        .prompt()?;

    let processed_file = Text::new("Enter the processed URLs file path:")
        .with_default("processed_urls.txt")
        .prompt()?;

    let output_dir = Text::new("Enter the output directory:")
        .with_default("output")
        .prompt()?;

    // Initialize scraping config
    let config = crate::scraping::ScrapingConfig::new()?;
    
    // Create output directory if it doesn't exist
    tokio::fs::create_dir_all(&output_dir).await?;

    // Scrape URLs and process content
    let urls = crate::scraping::read_unprocessed_urls(&urls_file, &processed_file).await?;

    for url in urls {
        match crate::scraping::scrape_url(&url, &config).await {
            Ok(html) => {
                match crate::ai::process_html_content(&html, &url).await {
                    Ok(processed) => {
                        let output_path = std::path::Path::new(&output_dir).join(&processed.filename);
                        tokio::fs::write(&output_path, &processed.content).await?;
                        crate::scraping::mark_url_processed(&url, &processed_file).await?;
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
