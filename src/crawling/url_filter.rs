use std::collections::HashSet;
use std::path::Path;
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use anyhow::Result;
use inquire::MultiSelect;

pub struct UrlFilter {
    disallowed_urls: HashSet<String>,
}

impl UrlFilter {
    pub async fn new(disallow_file: &str) -> Result<Self> {
        let disallowed_urls = if Path::new(disallow_file).exists() {
            let file = fs::File::open(disallow_file).await?;
            let reader = BufReader::new(file);
            let mut urls = HashSet::new();
            let mut lines = reader.lines();
            
            while let Some(line) = lines.next_line().await? {
                urls.insert(line);
            }
            urls
        } else {
            HashSet::new()
        };

        Ok(Self { disallowed_urls })
    }

    pub fn is_allowed(&self, url: &str) -> bool {
        !self.disallowed_urls.contains(url)
    }

    pub async fn add_to_disallow(&mut self, url: &str, disallow_file: &str) -> Result<()> {
        self.disallowed_urls.insert(url.to_string());
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(disallow_file)
            .await?;
            
        file.write_all(format!("{}\n", url).as_bytes()).await?;
        Ok(())
    }

    pub async fn select_urls<'a>(&self, urls: &'a [String]) -> Result<Vec<&'a String>> {
        if urls.is_empty() {
            return Ok(vec![]);
        }

        let allowed_urls: Vec<_> = urls.iter()
            .filter(|url| self.is_allowed(url))
            .collect();

        if allowed_urls.is_empty() {
            return Ok(vec![]);
        }

        let selected = MultiSelect::new(
            "Select URLs to crawl:",
            allowed_urls.clone()
        ).prompt()?;

        Ok(selected)
    }
}
