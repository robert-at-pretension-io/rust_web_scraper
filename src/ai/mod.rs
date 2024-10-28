use anyhow::{Result, Context};
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs},
    Client,
};
use serde::{Deserialize, Serialize};
use slug::slugify;
use crate::search::models::SearchResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedContent {
    pub filename: String,
    pub content: String,
    pub source_url: String,
}

async fn get_markdown_content(html: &str) -> Result<String> {
    let client = Client::new();

    let prompt = "You are a helpful assistant that converts HTML content into clean markdown format. \
        The markdown should preserve the important content while removing navigation, ads, \
        and other non-essential elements. Return ONLY the markdown content, nothing else.";

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(html)
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
    crate::logging::log_openai_interaction(html, &content).await?;

    Ok(content.trim().to_string())
}

async fn get_filename(html: &str) -> Result<String> {
    let client = Client::new();

    let prompt = "Based on the HTML content provided, suggest a descriptive filename (without extension) \
        that summarizes what this content is about. Return ONLY the filename, nothing else.";

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(html)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let filename = response.choices.first()
        .context("No response from OpenAI")?
        .message.content.clone()
        .context("No content in response")?;

    // Log the interaction
    crate::logging::log_openai_interaction(html, &filename).await?;

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
        .model("gpt-4")
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
