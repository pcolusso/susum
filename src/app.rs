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
    pub index: usize,
    pub filtered: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            query: "".to_string(),
            throbber_state: ThrobberState::default(),
            index: 0,
            filtered: vec![
                "cool app 1".to_string(),
                "cool app 2".to_string(),
                "cool app 3".to_string(),
            ],
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

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn push_char(&mut self, ch: char) {
        self.query.push(ch);
    }

    pub fn backspace(&mut self) {
        self.query.pop();
    }

    pub fn scroll_down(&mut self) {
        if self.index < self.filtered.len() {
            self.index += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}
