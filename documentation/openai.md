

 Docs.rs
 async-openai-0.25.0 
 Platform 
 Feature flags
Rust
 
Find crate
async_openai
0.25.0
All Items
Sections
Creating client
Microsoft Azure Endpoints
Making requests
Examples
Crate Items
Modules
Structs
Crates
async_openai
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
Crate async_openaiCopy item path
Settings
Help

Summary
source
Rust library for OpenAI

Creating client
use async_openai::{Client, config::OpenAIConfig};

// Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
let client = Client::new();

// Above is shortcut for
let config = OpenAIConfig::default();
let client = Client::with_config(config);

// OR use API key from different source and a non default organization
let api_key = "sk-..."; // This secret could be from a file, or environment variable.
let config = OpenAIConfig::new()
    .with_api_key(api_key)
    .with_org_id("the-continental");

let client = Client::with_config(config);

// Use custom reqwest client
let http_client = reqwest::ClientBuilder::new().user_agent("async-openai").build().unwrap();
let client = Client::new().with_http_client(http_client);

Microsoft Azure Endpoints
use async_openai::{Client, config::AzureConfig};

let config = AzureConfig::new()
    .with_api_base("https://my-resource-name.openai.azure.com")
    .with_api_version("2023-03-15-preview")
    .with_deployment_id("deployment-id")
    .with_api_key("...");

let client = Client::with_config(config);

// Note that `async-openai` only implements OpenAI spec
// and doesn't maintain parity with the spec of Azure OpenAI service.
Making requests

 use async_openai::{Client, types::{CreateCompletionRequestArgs}};

 // Create client
 let client = Client::new();

 // Create request using builder pattern
 // Every request struct has companion builder struct with same name + Args suffix
 let request = CreateCompletionRequestArgs::default()
     .model("gpt-3.5-turbo-instruct")
     .prompt("Tell me the recipe of alfredo pasta")
     .max_tokens(40_u32)
     .build()
     .unwrap();

 // Call API
 let response = client
     .completions()      // Get the API "group" (completions, images, etc.) from the client
     .create(request)    // Make the API call in that "group"
     .await
     .unwrap();

 println!("{}", response.choices.first().unwrap().text);
Examples
For full working examples for all supported features see examples directory in the repository.

Modules
config	Client configurations: OpenAIConfig for OpenAI, AzureConfig for Azure OpenAI Service.
error	Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
types	Types used in OpenAI API requests and responses. These types are created from component schemas in the OpenAPI spec
Structs
AssistantFiles	Files attached to an assistant.
Assistants	Build assistants that can call models and use tools to perform tasks.
Audio	Turn audio into text or text into audio. Related guide: Speech to text
Batches	Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
Chat	Given a list of messages comprising a conversation, the model will return a response.
Client	Client is a container for config, backoff and http_client used to make API calls.
Completions	Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position. We recommend most users use our Chat completions API. Learn more
Embeddings	Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
Files	Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
FineTuning	Manage fine-tuning jobs to tailor a model to your specific training data.
Images	Given a prompt and/or an input image, the model will generate a new image.
MessageFiles	Files attached to a message.
Messages	Represents a message within a thread.
Models	List and describe the various models available in the API. You can refer to the Models documentation to understand what models are available and the differences between them.
Moderations	Given some input text, outputs if the model classifies it as potentially harmful across several categories.
Runs	Represents an execution run on a thread.
Steps	Represents a step in execution of a run.
Threads	Create threads that assistants can interact with.
VectorStoreFileBatches	Vector store file batches represent operations to add multiple files to a vector store.
VectorStoreFiles	Vector store files represent files inside a vector store.
VectorStores


use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let schema = json!({
      "type": "object",
      "properties": {
        "steps": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "explanation": { "type": "string" },
              "output": { "type": "string" }
            },
            "required": ["explanation", "output"],
            "additionalProperties": false
          }
        },
        "final_answer": { "type": "string" }
      },
      "required": ["steps", "final_answer"],
      "additionalProperties": false
    });

    let response_format = ResponseFormat::JsonSchema {
        json_schema: ResponseFormatJsonSchema {
            description: None,
            name: "math_reasoning".into(),
            schema: Some(schema),
            strict: Some(true),
        },
    };

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-2024-08-06")
        .messages([
            ChatCompletionRequestSystemMessage::from(
                "You are a helpful math tutor. Guide the user through the solution step by step.",
            )
            .into(),
            ChatCompletionRequestUserMessage::from("how can I solve 8x + 7 = -23").into(),
        ])
        .response_format(response_format)
        .build()?;

    let response = client.chat().create(request).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            print!("{content}")
        }
    }

    Ok(())
}



use std::error::Error;

use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;

    println!("{}", serde_json::to_string(&request).unwrap());

    let response = client.chat().create(request).await?;

    println!("\nResponse:\n");
    for choice in response.choices {
        println!(
            "{}: Role: {}  Content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }

    Ok(())
}


 Docs.rs
 async-openai-0.25.0 
 Platform 
 Feature flags
Rust
 
Find crate
async_openai
0.25.0
All Items
Sections
Creating client
Microsoft Azure Endpoints
Making requests
Examples
Crate Items
Modules
Structs
Crates
async_openai
Type ‘S’ or ‘/’ to search, ‘?’ for more options…
Crate async_openaiCopy item path
Settings
Help

Summary
source
Rust library for OpenAI

Creating client
use async_openai::{Client, config::OpenAIConfig};

// Create a OpenAI client with api key from env var OPENAI_API_KEY and default base url.
let client = Client::new();

// Above is shortcut for
let config = OpenAIConfig::default();
let client = Client::with_config(config);

// OR use API key from different source and a non default organization
let api_key = "sk-..."; // This secret could be from a file, or environment variable.
let config = OpenAIConfig::new()
    .with_api_key(api_key)
    .with_org_id("the-continental");

let client = Client::with_config(config);

// Use custom reqwest client
let http_client = reqwest::ClientBuilder::new().user_agent("async-openai").build().unwrap();
let client = Client::new().with_http_client(http_client);
Microsoft Azure Endpoints
use async_openai::{Client, config::AzureConfig};

let config = AzureConfig::new()
    .with_api_base("https://my-resource-name.openai.azure.com")
    .with_api_version("2023-03-15-preview")
    .with_deployment_id("deployment-id")
    .with_api_key("...");

let client = Client::with_config(config);

// Note that `async-openai` only implements OpenAI spec
// and doesn't maintain parity with the spec of Azure OpenAI service.
Making requests

 use async_openai::{Client, types::{CreateCompletionRequestArgs}};

 // Create client
 let client = Client::new();

 // Create request using builder pattern
 // Every request struct has companion builder struct with same name + Args suffix
 let request = CreateCompletionRequestArgs::default()
     .model("gpt-3.5-turbo-instruct")
     .prompt("Tell me the recipe of alfredo pasta")
     .max_tokens(40_u32)
     .build()
     .unwrap();

 // Call API
 let response = client
     .completions()      // Get the API "group" (completions, images, etc.) from the client
     .create(request)    // Make the API call in that "group"
     .await
     .unwrap();

 println!("{}", response.choices.first().unwrap().text);
Examples
For full working examples for all supported features see examples directory in the repository.

Modules
config	Client configurations: OpenAIConfig for OpenAI, AzureConfig for Azure OpenAI Service.
error	Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
types	Types used in OpenAI API requests and responses. These types are created from component schemas in the OpenAPI spec
Structs
AssistantFiles	Files attached to an assistant.
Assistants	Build assistants that can call models and use tools to perform tasks.
Audio	Turn audio into text or text into audio. Related guide: Speech to text
Batches	Create large batches of API requests for asynchronous processing. The Batch API returns completions within 24 hours for a 50% discount.
Chat	Given a list of messages comprising a conversation, the model will return a response.
Client	Client is a container for config, backoff and http_client used to make API calls.
Completions	Given a prompt, the model will return one or more predicted completions, and can also return the probabilities of alternative tokens at each position. We recommend most users use our Chat completions API. Learn more
Embeddings	Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
Files	Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
FineTuning	Manage fine-tuning jobs to tailor a model to your specific training data.
Images	Given a prompt and/or an input image, the model will generate a new image.
MessageFiles	Files attached to a message.
Messages	Represents a message within a thread.
Models	List and describe the various models available in the API. You can refer to the Models documentation to understand what models are available and the differences between them.
Moderations	Given some input text, outputs if the model classifies it as potentially harmful across several categories.
Runs	Represents an execution run on a thread.
Steps	Represents a step in execution of a run.
Threads	Create threads that assistants can interact with.
VectorStoreFileBatches	Vector store file batches represent operations to add multiple files to a vector store.
VectorStoreFiles	Vector store files represent files inside a vector store.
VectorStores






use std::error::Error;

use async_openai::{types::CreateEmbeddingRequestArgs, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // An embedding is a vector (list) of floating point numbers.
    // The distance between two vectors measures their relatedness.
    // Small distances suggest high relatedness and large distances suggest low relatedness.

    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input([
            "Why do programmers hate nature? It has too many bugs.",
            "Why was the computer cold? It left its Windows open.",
        ])
        .build()?;

    let response = client.embeddings().create(request).await?;

    for data in response.data {
        println!(
            "[{}]: has embedding of length {}",
            data.index,
            data.embedding.len()
        )
    }

    Ok(())
}