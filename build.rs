use std::error::Error;
use std::process::Command;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Create data directory if it doesn't exist
    fs::create_dir_all("data")?;
    
    // Create schema.sql file
    let schema_sql = r#"
CREATE TABLE IF NOT EXISTS application_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    vector_embedding BLOB
);

CREATE TABLE IF NOT EXISTS search_queries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS search_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query_id INTEGER NOT NULL,
    document_id INTEGER NOT NULL,
    relevance_score REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (query_id) REFERENCES search_queries(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);
"#;

    fs::write("schema.sql", schema_sql)?;

    // Initialize/update database using sqlite3 CLI
    let db_path = "data/data.db";
    Command::new("sqlite3")
        .arg(db_path)
        .arg(".read schema.sql")
        .status()?;

    // Clean up schema file
    fs::remove_file("schema.sql")?;

    Ok(())
}
