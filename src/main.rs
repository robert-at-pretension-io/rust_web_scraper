mod models;
mod tui;
mod web;
mod scraping;
mod db;
mod logging;

use anyhow::Result;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database connection
    let pool = SqlitePool::connect("sqlite:data/data.db").await?;
    let pool = Arc::new(pool);
    
    // Initialize scraping config
    let scraping_config = scraping::ScrapingConfig::new()?;
    
    // Create channels for communication between TUI and web server
    let (tx, _rx) = mpsc::channel::<String>(32);
    
    // Start web server in background task
    tokio::spawn(web::start_server());

    // Initialize and run TUI
    let mut app = tui::App::new(Arc::clone(&pool), scraping_config);
    app.run()?;

    Ok(())
}
