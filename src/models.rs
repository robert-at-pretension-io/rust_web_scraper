use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(try_from = "Vec<u8>")]
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
