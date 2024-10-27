use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

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
        // TUI main loop implementation
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // Draw TUI components
    }

    fn handle_input(&mut self, event: Event) -> Result<bool> {
        // Handle user input
        Ok(false)
    }
}
