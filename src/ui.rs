use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListDirection, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, f: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    //
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Text Entry
    let entry_block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::from_u32(0x00ffac1c)))
        .title("Search for");
    let entry = Paragraph::new(app.query.clone()).block(entry_block);
    f.render_widget(entry, chunks[0]);

    match app.instances.as_ref() {
        Some(Ok(_)) => {
            let items = app.filtered.iter().map(|f| f.display());
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Matching Instances (is running, is running windoze)")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White))
                .direction(ListDirection::TopToBottom);

            f.render_stateful_widget(list, chunks[1], &mut app.list_state);
        }
        Some(Err(e)) => {
            let message = Paragraph::new(format!("Failed to load; AWS API reports \"{e}\"").red());
            f.render_widget(message, chunks[1])
        }
        None => {
            let full = throbber_widgets_tui::Throbber::default()
                .label("Loading...")
                .style(Style::default().fg(Color::Cyan))
                .throbber_style(Style::default().fg(Color::Red))
                .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
                .use_type(throbber_widgets_tui::WhichUse::Spin);
            f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
        }
    }

    // Bottom line
    let status_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Green));
    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(50),
                     Constraint::Percentage(50)
        ])
        .split(status_block.inner(chunks[2]));
    let profile = Paragraph::new(format!("AWS Profile {}", app.profile));
    f.render_widget(profile, status_layout[0]);
    let port = Paragraph::new("Will use port [3389]");
    f.render_widget(port, status_layout[1]);
    f.render_widget(status_block, chunks[2]);
}
