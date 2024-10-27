[package]
name = "web_scraper"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite", "chrono"] }
dotenv = "0.15"
async-openai = "0.16"
slug = "0.1"

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SearchQuery {
    pub id: i64,
    pub query: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SearchResult {
    pub id: i64,
    pub query_id: i64,
    pub document_id: i64,
    pub relevance_score: f32,
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub title: String,
    pub content: String,
}
