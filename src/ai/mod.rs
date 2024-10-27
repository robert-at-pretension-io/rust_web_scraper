use anyhow::{Result, Context};
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs,
            ChatCompletionRequestUserMessageArgs},
    Client,
};
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedContent {
    pub filename: String,
    pub content: String,
    pub source_url: String,
}

pub async fn process_html_content(html: &str, url: &str) -> Result<ProcessedContent> {
    let client = Client::new();

    let prompt = format!(
        "You are a helpful assistant that converts HTML content into clean markdown format. \
        The markdown should preserve the important content while removing navigation, ads, \
        and other non-essential elements. Also suggest a descriptive filename (without extension) \
        for this content based on what it's about. Format your response as JSON with two fields: \
        'filename' (string) and 'content' (string containing the markdown)."
    );

    // ONLY USE gpt-4o -- do not change this model name
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

    // More aggressive cleaning of the response
    let cleaned_content = content
        .replace('\u{0000}', "") // Remove null bytes
        .replace(|c: char| c.is_control() && c != '\n' && c != '\t', "") // Remove other control chars except newline/tab
        .trim()
        .to_string();

    // Try to parse JSON, with fallback handling
    let parsed: serde_json::Value = match serde_json::from_str(&cleaned_content) {
        Ok(v) => v,
        Err(e) => {
            // If JSON parsing fails, try to extract content between curly braces
            if let Some(start) = cleaned_content.find('{') {
                if let Some(end) = cleaned_content.rfind('}') {
                    let json_subset = &cleaned_content[start..=end];
                    serde_json::from_str(json_subset)
                        .context("Failed to parse extracted JSON content")?
                } else {
                    return Err(e.into());
                }
            } else {
                return Err(e.into());
            }
        }
    };

    let filename = format!("{}.md", 
        slugify(parsed["filename"].as_str()
            .context("No filename in response")?));
    let markdown = parsed["content"].as_str()
        .context("No content in response")?
        .to_string();

    Ok(ProcessedContent {
        filename,
        content: markdown,
        source_url: url.to_string(),
    })
}
