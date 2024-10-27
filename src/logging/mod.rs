use sqlx::SqlitePool;
use chrono::Utc;
use anyhow::Result;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING", 
            LogLevel::Error => "ERROR",
        }.to_string()
    }
}

pub async fn log(pool: &SqlitePool, level: LogLevel, message: &str) -> Result<()> {
    let level_str = level.to_string();
    let now = Utc::now();
    sqlx::query!(
        r#"
        INSERT INTO application_logs (level, message, created_at)
        VALUES ($1, $2, $3)
        "#,
        level_str,
        message,
        now,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_recent_logs(pool: &SqlitePool, limit: i64) -> Result<Vec<(String, String, String)>> {
    let logs = sqlx::query!(
        r#"
        SELECT level, message, created_at 
        FROM application_logs
        ORDER BY created_at DESC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(logs.into_iter()
        .map(|log| (log.level, log.message, log.created_at.to_string()))
        .collect())
}
