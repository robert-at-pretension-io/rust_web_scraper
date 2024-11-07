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

async fn validate_json_array(content: &str) -> Result<Vec<String>> {
    // Try direct parsing first
    if let Ok(urls) = serde_json::from_str::<Vec<String>>(content) {
        return Ok(urls);
    }

    // If that fails, try to extract JSON array from the text
    if let Some(start) = content.find('[') {
        if let Some(end) = content.rfind(']') {
            let json_str = &content[start..=end];
            if let Ok(urls) = serde_json::from_str::<Vec<String>>(json_str) {
                return Ok(urls);
            }
        }
    }

    Err(anyhow::anyhow!("Could not extract valid JSON array from response"))
}

async fn coerce_to_json_array(content: &str) -> Result<String> {
    let prompt = format!(
        "Convert the following text into a valid JSON array of URLs. \
        Extract any URLs present and return them in a proper JSON array format. \
        Return ONLY the JSON array, nothing else: {}", 
        content
    );

    get_openai_response(content, &prompt, "gpt-4o-mini").await
}

pub async fn evaluate_urls_for_crawl(urls: &[String], base_url: &str, purpose: &str) -> Result<Vec<String>> {
    let urls_json = serde_json::to_string(urls)?;
    let system_prompt = format!(
        "You are a web crawler assistant helping to evaluate which URLs to crawl next. \
        Base URL: {}\n\
        Crawl Purpose: {}\n\
        Evaluate the URLs and select only those that seem relevant to the stated purpose. \
        Consider factors like:\n\
        - URL path relevance to the purpose\n\
        - Whether the URL likely contains documentation/relevant content\n\
        - If the URL stays within the relevant section of the site\n\
        Return ONLY a JSON array of selected URLs, nothing else.", 
        base_url, purpose
    );

    // First attempt
    let mut response = get_openai_response(&urls_json, &system_prompt, "gpt-4o-mini").await?;
    
    // Try up to 3 times to get a valid JSON array
    for _ in 0..2 {
        match validate_json_array(&response).await {
            Ok(urls) => return Ok(urls),
            Err(_) => {
                // Try to coerce the response into a JSON array
                response = coerce_to_json_array(&response).await?;
            }
        }
    }

    // Final attempt to validate
    validate_json_array(&response).await
}

pub async fn select_urls(results: &[SearchResult], num_urls: usize) -> Result<Vec<String>> {
    let results_json = serde_json::to_string(results)?;
    
    let prompt = format!(
        "You are an expert at analyzing search results to find the most relevant and high-quality content. \
        Select the {} most relevant URLs from these search results. \
        Consider factors like: content relevance, source authority, content freshness, and information depth. \
        Return ONLY a JSON array of the selected URLs, nothing else.", 
        num_urls
    );

    // First attempt
    let mut response = get_openai_response(&results_json, &prompt, "gpt-4o-mini").await?;
    
    // Try up to 3 times to get a valid JSON array
    for _ in 0..2 {
        match validate_json_array(&response).await {
            Ok(urls) => return Ok(urls),
            Err(_) => {
                // Try to coerce the response into a JSON array
                response = coerce_to_json_array(&response).await?;
            }
        }
    }

    // Final attempt to validate
    validate_json_array(&response).await
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
