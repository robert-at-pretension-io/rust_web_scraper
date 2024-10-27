use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[derive(Debug)]
pub enum MenuItem {
    Documents(Option<DocumentsMenuItem>),
    Search,
    Settings,
    Quit,
}

#[derive(Debug)]
pub enum DocumentsMenuItem {
    CacheUrl,
    CacheFromFile,
    RefreshUrlsList,
    Back,
}

pub struct App {
    menu_items: Vec<MenuItem>,
    selected_item: usize,
    documents_menu_items: Vec<DocumentsMenuItem>,
    selected_documents_item: usize,
    is_in_documents_submenu: bool,
    help_text: String,
    progress: f64,
    progress_message: String,
    pool: Arc<SqlitePool>,
    scraping_config: scraping::ScrapingConfig,
}

impl App {
    pub async fn handle_documents_action(&mut self) -> Result<()> {
        match self.documents_menu_items[self.selected_documents_item] {
            DocumentsMenuItem::CacheUrl => {
                self.progress = 0.0;
                self.progress_message = "Enter URL to cache:".to_string();
                
                // TODO: Implement proper input dialog
                let url = "https://example.com"; // This should come from user input
                
                self.progress = 0.2;
                self.progress_message = format!("Scraping {}...", url);
                logging::log(&self.pool, logging::LogLevel::Info, &format!("Starting to scrape {}", url)).await?;
                
                match scraping::scrape_url(url, &self.scraping_config).await {
                    Ok(content) => {
                        self.progress = 0.6;
                        self.progress_message = "Storing in database...".to_string();
                        
                        match db::store_document(url, &content, &self.pool).await {
                            Ok(id) => {
                                self.progress = 1.0;
                                self.progress_message = "Successfully cached document".to_string();
                                logging::log(&self.pool, logging::LogLevel::Info, 
                                    &format!("Successfully cached document id {} from {}", id, url)).await?;
                            },
                            Err(e) => {
                                self.progress = 0.0;
                                self.progress_message = "Failed to store document".to_string();
                                logging::log(&self.pool, logging::LogLevel::Error, 
                                    &format!("Failed to store document from {}: {}", url, e)).await?;
                            }
                        }
                    },
                    Err(e) => {
                        self.progress = 0.0;
                        self.progress_message = "Failed to scrape URL".to_string();
                        logging::log(&self.pool, logging::LogLevel::Error, 
                            &format!("Failed to scrape {}: {}", url, e)).await?;
                    }
                }
            }
            DocumentsMenuItem::CacheFromFile => {
                // TODO: Implement file caching
            }
            DocumentsMenuItem::RefreshUrlsList => {
                // TODO: Implement refresh
            }
            DocumentsMenuItem::Back => {
                self.is_in_documents_submenu = false;
            }
        }
        Ok(())
    }

    pub fn new(pool: Arc<SqlitePool>, scraping_config: scraping::ScrapingConfig) -> Self {
        Self {
            menu_items: vec![
                MenuItem::Documents(None),
                MenuItem::Search,
                MenuItem::Settings,
                MenuItem::Quit,
            ],
            selected_item: 0,
            documents_menu_items: vec![
                DocumentsMenuItem::CacheUrl,
                DocumentsMenuItem::CacheFromFile,
                DocumentsMenuItem::RefreshUrlsList,
                DocumentsMenuItem::Back,
            ],
            selected_documents_item: 0,
            is_in_documents_submenu: false,
            help_text: String::from("Use ↑↓ to navigate, Enter to select, q to quit"),
        }
    }

    fn get_help_text(&self) -> String {
        match (self.is_in_documents_submenu, &self.menu_items[self.selected_item]) {
            (false, MenuItem::Documents(_)) => 
                "Document management and caching options. Press Enter to view sub-menu.".to_string(),
            (false, MenuItem::Search) => 
                "Search through cached documents.".to_string(),
            (false, MenuItem::Settings) => 
                "Configure application settings.".to_string(),
            (false, MenuItem::Quit) => 
                "Exit the application.".to_string(),
            (true, MenuItem::Documents(_)) => {
                match self.documents_menu_items[self.selected_documents_item] {
                    DocumentsMenuItem::CacheUrl => 
                        "Cache a single URL by entering its address.".to_string(),
                    DocumentsMenuItem::CacheFromFile => 
                        "Cache multiple URLs from urls.txt file.".to_string(),
                    DocumentsMenuItem::RefreshUrlsList => 
                        "Refresh the list of URLs from urls.txt.".to_string(),
                    DocumentsMenuItem::Back => 
                        "Return to main menu.".to_string(),
                }
            }
            _ => String::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        let mut terminal = ratatui::init();

        // Main loop
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if self.handle_input(crossterm::event::read()?)? {
                    break;
                }
            }
        }

        // Restore terminal
        ratatui::restore();
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

        let title = Paragraph::new("Documentation Aggregator")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = if self.is_in_documents_submenu {
            self.documents_menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == self.selected_documents_item {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(format!("{:?}", item)).style(style)
                })
                .collect()
        } else {
            self.menu_items
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
                .collect()
        };

        let menu = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(if self.is_in_documents_submenu {
                    "Documents Menu"
                } else {
                    "Main Menu"
                }),
        );
        frame.render_widget(menu, chunks[1]);

        let status = Paragraph::new("Press q to quit")
            .style(Style::default().fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(status, chunks[2]);
    }

    fn handle_input(&mut self, event: Event) -> Result<bool> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => {
                    if self.is_in_documents_submenu {
                        self.selected_documents_item = self.selected_documents_item.saturating_sub(1);
                    } else {
                        self.selected_item = self.selected_item.saturating_sub(1);
                    }
                }
                KeyCode::Down => {
                    if self.is_in_documents_submenu {
                        self.selected_documents_item = (self.selected_documents_item + 1)
                            .min(self.documents_menu_items.len() - 1);
                    } else {
                        self.selected_item = (self.selected_item + 1).min(self.menu_items.len() - 1);
                    }
                }
                KeyCode::Enter => {
                    if !self.is_in_documents_submenu {
                        if let MenuItem::Documents(_) = &self.menu_items[self.selected_item] {
                            self.is_in_documents_submenu = true;
                            self.selected_documents_item = 0;
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
