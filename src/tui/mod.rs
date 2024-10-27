use std::sync::Arc;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use sqlx::SqlitePool;
use std::io::stdout;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use crate::{db, logging::{self, LogLevel}, scraping}; 
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[derive(Debug)]
pub enum MenuItem {
    CacheUrl,
    CacheFromFile,
    RefreshUrlsList,
    Quit,
}

pub struct App {
    menu_items: Vec<MenuItem>,
    selected_item: usize,
    input_mode: bool,
    input_value: String,
    status_message: String,
    pool: Arc<SqlitePool>,
    scraping_config: scraping::ScrapingConfig,
}

impl App {
    pub fn new(pool: Arc<SqlitePool>, scraping_config: scraping::ScrapingConfig) -> Self {
        Self {
            menu_items: vec![
                MenuItem::CacheUrl,
                MenuItem::CacheFromFile,
                MenuItem::RefreshUrlsList,
                MenuItem::Quit,
            ],
            selected_item: 0,
            input_mode: false,
            input_value: String::new(),
            status_message: String::from("Use ↑↓ to navigate, Enter to select, q to quit"),
            pool,
            scraping_config,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main loop
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    if self.input_mode {
                        match key.code {
                            KeyCode::Enter => {
                                self.handle_input_submit().await?;
                                self.input_mode = false;
                            }
                            KeyCode::Esc => {
                                self.input_mode = false;
                                self.input_value.clear();
                            }
                            KeyCode::Char(c) => {
                                self.input_value.push(c);
                            }
                            KeyCode::Backspace => {
                                self.input_value.pop();
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Up => {
                                self.selected_item = self.selected_item.saturating_sub(1);
                            }
                            KeyCode::Down => {
                                self.selected_item = (self.selected_item + 1).min(self.menu_items.len() - 1);
                            }
                            KeyCode::Enter => {
                                match self.menu_items[self.selected_item] {
                                    MenuItem::CacheUrl => {
                                        self.input_mode = true;
                                        self.status_message = String::from("Enter URL to cache (press Enter to submit, Esc to cancel):");
                                    }
                                    MenuItem::CacheFromFile => {
                                        self.handle_cache_from_file().await?;
                                    }
                                    MenuItem::RefreshUrlsList => {
                                        self.handle_refresh_urls().await?;
                                    }
                                    MenuItem::Quit => break,
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        
        Ok(())
    }

    async fn handle_input_submit(&mut self) -> Result<()> {
        if !self.input_value.is_empty() {
            let url = self.input_value.clone();
            self.input_value.clear();
            
            logging::log(LogLevel::Info, &format!("Processing URL: {}", url)).await?;
            
            match scraping::scrape_url(&url, &self.scraping_config).await {
                Ok(content) => {
                    match db::store_document(&url, &content, &self.pool).await {
                        Ok(id) => {
                            self.status_message = format!("Successfully cached document {}", id);
                            logging::log(LogLevel::Info, &format!("Cached document {} from {}", id, url)).await?;
                        }
                        Err(e) => {
                            self.status_message = "Failed to store document".to_string();
                            logging::log(LogLevel::Error, &format!("Database error for {}: {}", url, e)).await?;
                        }
                    }
                }
                Err(e) => {
                    self.status_message = "Failed to scrape URL".to_string();
                    logging::log(LogLevel::Error, &format!("Scraping error for {}: {}", url, e)).await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_cache_from_file(&mut self) -> Result<()> {
        match tokio::fs::read_to_string("urls.txt").await {
            Ok(content) => {
                let urls: Vec<_> = content.lines().filter(|l| !l.trim().is_empty()).collect();
                logging::log(LogLevel::Info, &format!("Processing {} URLs from file", urls.len())).await?;
                
                for url in urls {
                    match scraping::scrape_url(url, &self.scraping_config).await {
                        Ok(content) => {
                            if let Err(e) = db::store_document(url, &content, &self.pool).await {
                                logging::log(LogLevel::Error, &format!("Failed to store {}: {}", url, e)).await?;
                            }
                        }
                        Err(e) => {
                            logging::log(LogLevel::Error, &format!("Failed to scrape {}: {}", url, e)).await?;
                        }
                    }
                }
                self.status_message = "Finished processing URLs from file".to_string();
            }
            Err(e) => {
                self.status_message = "Failed to read urls.txt".to_string();
                logging::log(LogLevel::Error, &format!("Failed to read urls.txt: {}", e)).await?;
            }
        }
        Ok(())
    }

    async fn handle_refresh_urls(&mut self) -> Result<()> {
        self.status_message = "Refreshed URLs list".to_string();
        logging::log(LogLevel::Info, "Refreshed URLs list").await?;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // Title
        let title = Paragraph::new("Documentation Aggregator")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        // Menu
        let items: Vec<ListItem> = self.menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected_item {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{:?}", item)).style(style)
            })
            .collect();

        let menu = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Menu"));
        frame.render_widget(menu, chunks[1]);

        // Status/Input area
        let status = if self.input_mode {
            format!("{}\n> {}", self.status_message, self.input_value)
        } else {
            self.status_message.clone()
        };
        
        let status = Paragraph::new(status)
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(status, chunks[2]);
    }
}
