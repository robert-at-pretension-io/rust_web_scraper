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
    // Ask for AI model first
    let ai_model = Text::new("Enter AI model to use:")
        .with_default("gpt-4o-mini")
        .prompt()?;
    
    let purpose = Text::new("What is the purpose of this crawl? (e.g. 'Gather documentation about WebSocket APIs')")
        .prompt()?;
    
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

    // Set the AI model in environment for the crawl process
    std::env::set_var("AI_MODEL", ai_model);

    let config = crate::scraping::ScrapingConfig::new()?;
    crate::crawling::crawl_url(&url, &processed_file, &output_dir, &config, max_depth, &purpose).await?;

    Ok(())
}

pub async fn handle_crawl(url: &str, processed_file: &str, output_dir: &str) -> Result<()> {
    let config = crate::scraping::ScrapingConfig::new()?;
    let max_depth = 2; // Default depth for CLI mode
    let purpose = "General documentation gathering"; // Default purpose for CLI mode
    
    // Use environment variable if set, otherwise use default
    let ai_model = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());
    std::env::set_var("AI_MODEL", ai_model);
    
    crate::crawling::crawl_url(url, processed_file, output_dir, &config, max_depth, purpose).await
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
    let ai_config = crate::ai::AiConfig {
        model: Text::new("Enter AI model to use:")
            .with_default("gpt-4o-mini")
            .prompt()?,
        purpose: Text::new("What is the purpose of this scraping?")
            .with_default("Extract and format documentation content")
            .prompt()?,
    };

    let urls_file = Text::new("Enter the URLs file path:")
        .with_default("urls.txt")
        .prompt()?;

    let processed_file = Text::new("Enter the processed URLs file path:")
        .with_default("processed_urls.txt")
        .prompt()?;

    let output_dir = Text::new("Enter the output directory:")
        .with_default("output")
        .prompt()?;

    // Initialize scraping config and metadata
    let config = crate::scraping::ScrapingConfig::new()?;
    let mut project_metadata = crate::metadata::ProjectMetadata::load(&output_dir).await?;
    
    // Create output directory if it doesn't exist
    tokio::fs::create_dir_all(&output_dir).await?;

    // Scrape URLs and process content
    let urls = crate::scraping::read_unprocessed_urls(&urls_file, &processed_file).await?;

    for url in urls {
        match crate::scraping::scrape_url(&url, &config).await {
            Ok(html) => {
                match crate::ai::process_html_content(&html, &url, &ai_config, &project_metadata).await {
                    Ok(processed) => {
                        let output_path = std::path::Path::new(&output_dir).join(&processed.filename);
                        tokio::fs::write(&output_path, &processed.content).await?;
                        
                        // Create and add metadata
                        let doc_metadata = crate::metadata::create_document_metadata(
                            processed.filename.clone(),
                            processed.title,
                            url.clone(),
                            ai_config.purpose.clone(),
                            ai_config.model.clone(),
                            &processed.content,
                            processed.processing_time,
                            true,
                            None,
                        ).await;
                        
                        project_metadata.add_document(doc_metadata?);
                        project_metadata.save(&output_dir).await?;
                        
                        crate::scraping::mark_url_processed(&url, &processed_file).await?;
                        println!("Processed {} -> {}", url, processed.filename);
                    },
                    Err(e) => {
                        // Track failed processing in metadata
                        let doc_metadata = crate::metadata::create_document_metadata(
                            format!("failed_{}", slug::slugify(&url)),
                            "Processing Failed".to_string(),
                            url.clone(),
                            ai_config.purpose.clone(),
                            ai_config.model.clone(),
                            "",
                            0.0,
                            false,
                            Some(e.to_string()),
                        ).await;
                        
                        project_metadata.add_document(doc_metadata?);
                        project_metadata.save(&output_dir).await?;
                        
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
