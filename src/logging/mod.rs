use std::fs::{self, OpenOptions};
use std::io::Write;
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

pub async fn log(level: LogLevel, message: &str) -> Result<()> {
    // Ensure logs directory exists
    fs::create_dir_all("logs")?;
    
    let now = Utc::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S%.3f");
    let level_str = level.to_string();
    
    // Write to log file
    let log_file = format!("logs/app.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    writeln!(file, "[{}] {} - {}", timestamp, level_str, message)?;
    
    Ok(())
}
