use std::fs::{self, OpenOptions};
use std::io::Write;
use chrono::Utc;
use anyhow::Result;
use serde::Serialize;

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

#[derive(Serialize)]
struct OpenAILog {
    timestamp: String,
    prompt: String,
    response: String,
}

pub async fn log_openai_interaction(prompt: &str, response: &str) -> Result<()> {
    // Ensure logs directory exists
    fs::create_dir_all("logs")?;
    
    let now = Utc::now();
    let log = OpenAILog {
        timestamp: now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        prompt: prompt.to_string(),
        response: response.to_string(),
    };

    // Write to OpenAI log file
    let log_file = format!("logs/openai.log");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    let log_line = serde_json::to_string(&log)?;
    writeln!(file, "{}", log_line)?;
    
    Ok(())
}
