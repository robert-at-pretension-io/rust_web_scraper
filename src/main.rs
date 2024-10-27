mod models;
mod tui;
mod web;

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // Create channels for communication between TUI and web server
    let (tx, _rx) = mpsc::channel::<String>(32);
    
    // Start web server in background task
    tokio::spawn(web::start_server());

    // Initialize and run TUI
    let mut app = tui::App::new();
    app.run()?;

    Ok(())
}
