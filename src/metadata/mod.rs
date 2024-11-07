use anyhow::{Result, Context};
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs},
    Client,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub filename: String,
    pub title: String,
    pub description: String,
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

    pub fn get_context_for_processing(&self) -> String {
        let mut context = String::new();
        
        // Add project purpose if available
        if let Some(purpose) = self.custom_metadata.get("purpose") {
            context.push_str(&format!("Project Purpose: {}\n\n", purpose));
        }
        
        // Add summaries of existing documents
        if !self.documents.is_empty() {
            context.push_str("Existing Documentation Context:\n");
            for doc in &self.documents {
                if doc.success {  // Only include successfully processed documents
                    context.push_str(&format!(
                        "- {}: {}\n",
                        doc.title,
                        doc.description
                    ));
                }
            }
        }
        
        context
    }
}

pub async fn generate_description(
    content: &str,
    title: &str,
    purpose: &str,
    ai_model: &str,
) -> Result<String> {
    let client = Client::new();
    
    let prompt = format!(
        "You are a technical documentation expert. Given the following content from '{}', \
        create a clear, concise description (less than 10 sentences) that explains what this document covers. \
        Consider that this documentation is being collected for the purpose of: '{}'. \
        Focus on the key topics and their relevance to this purpose.\n\n\
        Content:\n{}\n\n\
        Return only the description, nothing else.",
        title,
        purpose,
        content
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model(ai_model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a technical documentation expert creating concise document descriptions.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let description = response.choices.first()
        .context("No response from OpenAI")?
        .message.content.clone()
        .context("No content in response")?;

    Ok(description.trim().to_string())
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
) -> Result<DocumentMetadata> {
    let word_count = content.split_whitespace().count();
    let tags = extract_tags_from_content(content);
    
    // Generate description using AI
    let description = if success {
        generate_description(content, &title, &purpose, &ai_model).await?
    } else {
        "Processing failed - no description available".to_string()
    };

    Ok(DocumentMetadata {
        filename,
        title,
        description,
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
    })
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
