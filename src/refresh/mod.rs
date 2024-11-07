use anyhow::Result;
use chrono::{DateTime, Utc};
use inquire::{MultiSelect, Select, Text, Confirm};
use std::path::Path;
use crate::metadata::{ProjectMetadata, DocumentMetadata};
use crate::ai::AiConfig;
use crate::scraping::ScrapingConfig;

pub struct RefreshableDocument {
    pub filename: String,
    pub title: String,
    pub source_url: String,
    pub last_updated: DateTime<Utc>,
    pub purpose: String,
    pub ai_model: String,
}

impl std::fmt::Display for RefreshableDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): Last updated {}",
            self.title,
            self.filename,
            self.last_updated.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

pub async fn handle_refresh(output_dir: &str) -> Result<()> {
    // Load project metadata
    let project_metadata = ProjectMetadata::load(output_dir).await?;
    
    // Sort documents by last_updated
    let mut docs: Vec<RefreshableDocument> = project_metadata.documents
        .iter()
        .map(|doc| RefreshableDocument {
            filename: doc.filename.clone(),
            title: doc.title.clone(),
            source_url: doc.source_url.clone(),
            last_updated: doc.last_updated,
            purpose: doc.purpose.clone(),
            ai_model: doc.ai_model.clone(),
        })
        .collect();
    
    docs.sort_by(|a, b| a.last_updated.cmp(&b.last_updated));

    // Let user select documents to refresh
    let selected_docs = MultiSelect::new(
        "Select documents to refresh:",
        docs
    ).prompt()?;

    if selected_docs.is_empty() {
        println!("No documents selected for refresh.");
        return Ok(());
    }

    // For each selected document, ask about purpose and model
    let mut new_project_metadata = project_metadata.clone();
    let scraping_config = ScrapingConfig::new()?;

    for doc in selected_docs {
        println!("\nRefreshing: {}", doc.title);
        
        // Ask whether to keep existing purpose
        let keep_purpose = Confirm::new(&format!(
            "Keep existing purpose? ({})",
            doc.purpose
        )).with_default(true).prompt()?;

        let purpose = if keep_purpose {
            doc.purpose.clone()
        } else {
            Text::new("Enter new purpose:")
                .with_default(&doc.purpose)
                .prompt()?
        };

        // Ask whether to keep existing model
        let keep_model = Confirm::new(&format!(
            "Keep existing AI model? ({})",
            doc.ai_model
        )).with_default(true).prompt()?;

        let model = if keep_model {
            doc.ai_model.clone()
        } else {
            Text::new("Enter new AI model:")
                .with_default(&doc.ai_model)
                .prompt()?
        };

        let ai_config = AiConfig {
            model: model.clone(),
            purpose: purpose.clone(),
        };

        // Scrape and process the document
        match crate::scraping::scrape_url(&doc.source_url, &scraping_config).await {
            Ok(html) => {
                match crate::ai::process_html_content(
                    &html,
                    &doc.source_url,
                    &ai_config,
                    &new_project_metadata
                ).await {
                    Ok(processed) => {
                        // Save new markdown file
                        let output_path = Path::new(output_dir).join(&processed.filename);
                        tokio::fs::write(&output_path, &processed.content).await?;
                        
                        // Create and add metadata
                        let doc_metadata = crate::metadata::create_document_metadata(
                            processed.filename.clone(),
                            processed.title,
                            doc.source_url.clone(),
                            purpose,
                            model,
                            &processed.content,
                            processed.processing_time,
                            true,
                            None,
                        ).await?;
                        
                        new_project_metadata.add_document(doc_metadata);
                        println!("Successfully refreshed {}", doc.filename);
                    },
                    Err(e) => {
                        println!("Failed to process content for {}: {}", doc.source_url, e);
                        continue;
                    }
                }
            },
            Err(e) => {
                println!("Failed to scrape {}: {}", doc.source_url, e);
                continue;
            }
        }
    }

    // Save updated metadata
    new_project_metadata.save(output_dir).await?;
    println!("\nRefresh complete. Updated metadata saved.");

    Ok(())
}
