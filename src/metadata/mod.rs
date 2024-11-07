use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub filename: String,
    pub title: String,
    pub source_url: String,
    pub last_updated: DateTime<Utc>,
    pub purpose: String,
    pub ai_model: String,
    pub word_count: usize,
    pub tags: Vec<String>,
    pub processing_time: f64,  // in seconds
    pub success: bool,
    pub error_message: Option<String>,
    // Add custom metadata fields
    pub custom_metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub project_name: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub documents: Vec<DocumentMetadata>,
}

impl ProjectMetadata {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            documents: Vec::new(),
        }
    }

    pub async fn load(output_dir: &str) -> Result<Self> {
        let path = Path::new(output_dir).join("metadata.json");
        if path.exists() {
            let content = fs::read_to_string(&path).await?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::new("Documentation Project"))
        }
    }

    pub async fn save(&self, output_dir: &str) -> Result<()> {
        let path = Path::new(output_dir).join("metadata.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content).await?;
        Ok(())
    }

    pub fn add_document(&mut self, doc: DocumentMetadata) {
        // Remove existing document with same filename if it exists
        self.documents.retain(|d| d.filename != doc.filename);
        self.documents.push(doc);
        self.last_updated = Utc::now();
    }

    pub fn get_document(&self, filename: &str) -> Option<&DocumentMetadata> {
        self.documents.iter().find(|d| d.filename == filename)
    }
}

pub async fn create_document_metadata(
    filename: String,
    title: String,
    source_url: String,
    purpose: String,
    ai_model: String,
    content: &str,
    processing_time: f64,
    success: bool,
    error_message: Option<String>,
) -> DocumentMetadata {
    // Calculate word count
    let word_count = content.split_whitespace().count();

    // Extract potential tags from content (e.g., from headers)
    let tags = extract_tags_from_content(content);

    DocumentMetadata {
        filename,
        title,
        source_url,
        last_updated: Utc::now(),
        purpose,
        ai_model,
        word_count,
        tags,
        processing_time,
        success,
        error_message,
        custom_metadata: HashMap::new(),
    }
}

fn extract_tags_from_content(content: &str) -> Vec<String> {
    // Extract headers and key terms as tags
    let mut tags = Vec::new();
    
    // Extract markdown headers
    for line in content.lines() {
        if line.starts_with('#') {
            let tag = line.trim_start_matches('#')
                         .trim()
                         .to_lowercase();
            if !tag.is_empty() {
                tags.push(tag);
            }
        }
    }
    
    tags.sort();
    tags.dedup();
    tags
}
