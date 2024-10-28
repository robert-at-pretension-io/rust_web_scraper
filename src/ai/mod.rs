use anyhow::{Result, Context};
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs},
    Client,
};
use serde::{Deserialize, Serialize};
use slug::slugify;
use crate::search::models::SearchResult;
use crate::logging::log;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedContent {
    pub filename: String,
    pub content: String,
    pub source_url: String,
}

const MAX_CHUNK_SIZE: usize = 60000; // Leave room for prompt and overhead

async fn get_openai_response(content: &str, system_prompt: &str, model: &str) -> Result<String> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response.choices.first()
        .context("No response from OpenAI")?
        .message.content.clone()
        .context("No content in response")?;

    // Log the interaction
    crate::logging::log_openai_interaction(&content, &content).await?;

    Ok(content)
}

async fn get_markdown_content(html: &str) -> Result<String> {
    // Convert HTML to plain text first to reduce size
    let plain_text = html2text::from_read(html.as_bytes(), 80);
    let mut final_content = String::new();

    // Split content into chunks
    let text_chars: Vec<char> = plain_text.chars().collect();
    let chunks: Vec<String> = text_chars
        .chunks(MAX_CHUNK_SIZE)
        .map(|c| c.iter().collect::<String>())
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let prompt = if chunks.len() > 1 {
            format!(
                "You are processing part {} of {} of a document. \
                Convert this text content into clean markdown format. \
                Preserve the important content while improving readability. \
                Return ONLY the markdown content, nothing else.",
                i + 1,
                chunks.len()
            )
        } else {
            "Convert this text content into clean markdown format. \
            Preserve the important content while improving readability. \
            Return ONLY the markdown content, nothing else.".to_string()
        };

        let content = get_openai_response(chunk, &prompt, "gpt-4o-mini").await?;
        final_content.push_str(&content);
        final_content.push_str("\n\n");
    }

    Ok(final_content.trim().to_string())
}

async fn get_filename(html: &str) -> Result<String> {
    // Convert HTML to plain text first
    let plain_text = html2text::from_read(html.as_bytes(), 80);
    
    let prompt = "Based on the text content provided, suggest a descriptive filename (without extension) \
        that summarizes what this content is about. Return ONLY the filename, nothing else.";

    let filename = get_openai_response(&plain_text, prompt, "gpt-4o").await?;
    Ok(slugify(&filename.trim()))
}

pub async fn select_urls(results: &[SearchResult], num_urls: usize) -> Result<Vec<String>> {
    let client = Client::new();

    let results_json = serde_json::to_string(results)?;
    
    let prompt = format!(
        "You are an expert at analyzing search results to find the most relevant and high-quality content. \
        Select the {} most relevant URLs from these search results. \
        Consider factors like: content relevance, source authority, content freshness, and information depth. \
        Return ONLY a JSON array of the selected URLs, nothing else.", 
        num_urls
    );

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(results_json)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response.choices.first()
        .context("No response from OpenAI")?
        .message.content.clone()
        .context("No content in response")?;

    let selected_urls: Vec<String> = serde_json::from_str(&content)
        .context("Failed to parse selected URLs from AI response")?;

    Ok(selected_urls)
}

pub async fn process_html_content(html: &str, url: &str) -> Result<ProcessedContent> {
    // Get markdown content and filename in parallel
    let (markdown, filename) = tokio::join!(
        get_markdown_content(html),
        get_filename(html)
    );

    let markdown = markdown?;
    let filename = format!("{}.md", filename?);

    Ok(ProcessedContent {
        filename,
        content: markdown,
        source_url: url.to_string(),
    })
}
