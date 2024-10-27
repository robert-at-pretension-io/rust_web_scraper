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

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
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

    // Clean the content string of any control characters before parsing
    let cleaned_content = content.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect::<String>();

    let parsed: serde_json::Value = serde_json::from_str(&cleaned_content)
        .context("Failed to parse OpenAI response as JSON")?;

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
