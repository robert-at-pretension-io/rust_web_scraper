use anyhow::Result;
use sqlx::SqlitePool;
use chrono::Utc;

pub async fn store_document(url: &str, content: &str, pool: &SqlitePool) -> Result<i64> {
    let now = Utc::now();
    
    let result = sqlx::query!(
        r#"
        INSERT INTO documents (title, content, url, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        "", // title - placeholder for now
        content,
        url,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
use anyhow::Result;
use sqlx::SqlitePool;
use chrono::Utc;

pub async fn store_document(url: &str, content: &str, pool: &SqlitePool) -> Result<i64> {
    let now = Utc::now();
    
    let result = sqlx::query!(
        r#"
        INSERT INTO documents (title, content, url, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        "", // title - placeholder for now
        content,
        url,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(result.id)
}
