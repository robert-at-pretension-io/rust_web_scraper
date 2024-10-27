use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data")?;
    
    // Database URL
    let db_path = Path::new("data").join("data.db");
    let database_url = format!("sqlite:{}", db_path.display());

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            url TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            vector_embedding BLOB
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS search_queries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS search_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query_id INTEGER NOT NULL,
            document_id INTEGER NOT NULL,
            relevance_score REAL NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (query_id) REFERENCES search_queries(id),
            FOREIGN KEY (document_id) REFERENCES documents(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Close the connection pool
    pool.close().await;

    Ok(())
}
