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
    Documents,
    Search,
    Settings,
    Quit,
}

pub struct App {
    menu_items: Vec<MenuItem>,
    selected_item: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            menu_items: vec![
                MenuItem::Documents,
                MenuItem::Search,
                MenuItem::Settings,
                MenuItem::Quit,
            ],
            selected_item: 0,
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

        let items: Vec<ListItem> = self
            .menu_items
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
                    self.selected_item = self.selected_item.saturating_sub(1);
                }
                KeyCode::Down => {
                    self.selected_item = (self.selected_item + 1).min(self.menu_items.len() - 1);
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
