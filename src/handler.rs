use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char(ch) => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                match ch {
                    'c' | 'C' => app.quit(),
                    _ => {}
                }
            }
            app.push_char(ch)
        }
        KeyCode::Up => app.scroll_up(),
        KeyCode::Down => app.scroll_down(),
        KeyCode::Backspace => app.backspace(),
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
