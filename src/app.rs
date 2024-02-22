use crate::aws::{fuzzy_search_instances, Instance};

use color_eyre::Result;
use ratatui::widgets::ListState;
use std::error;
use throbber_widgets_tui::ThrobberState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
    pub query: String,
    pub filtered: Vec<Instance>,
    pub instances: Option<Result<Vec<Instance>>>,
    pub list_state: ListState,
    pub profile: String,
    pub start_session: bool,
    pub port: Option<u16>
}

impl Default for App {
    fn default() -> Self {

        Self {
            running: true,
            query: "".to_string(),
            throbber_state: ThrobberState::default(),
            filtered: vec![],
            instances: None,
            list_state: ListState::default(),
            profile: std::env::var("AWS_PROFILE").unwrap_or("NOT SET".to_string()),
            start_session: false,
            port: None
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.throbber_state.calc_next();
    }

    pub fn load(&mut self, instances: Result<Vec<Instance>>) {
        self.instances = Some(instances);
        self.filter();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.running = false;
        self.start_session = true;
    }

    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
        self.filter();
    }

    pub fn backspace(&mut self) {
        self.query.pop();
        self.filter();
    }

    pub fn scroll_down(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.filtered.len() - 1 {
                *self.list_state.selected_mut() = Some(i + 1);
            }
        }
    }

    pub fn scroll_up(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i > 0 {
                *self.list_state.selected_mut() = Some(i - 1);
            }
        }
    }

    fn filter(&mut self) {
        if let Some(Ok(is)) = self.instances.as_ref() {
            // Inefficient, but i'm getting skill-issued.
            let mut new_filter = vec![];
            for f in fuzzy_search_instances(is, &self.query) {
                new_filter.push(f.clone())
            }
            self.filtered = new_filter;
            if self.filtered.is_empty() && self.list_state.selected().is_none() {
                *self.list_state.selected_mut() = Some(0);
            }
            *self.list_state.selected_mut() = Some(0)
        }
    }
}
