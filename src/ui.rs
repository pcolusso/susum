use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
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
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title line
    let title_block = Block::default()
        .borders(Borders::all())
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        "Select EC2 Instance",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    f.render_widget(title, chunks[0]);

    // Text Entry
    let entry_block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .title("Search for");
    let entry = Paragraph::new(app.query.clone()).block(entry_block);
    f.render_widget(entry, chunks[1]);

    let full = throbber_widgets_tui::Throbber::default()
        .label("Running...")
        .style(Style::default().fg(Color::Cyan))
        .throbber_style(Style::default().fg(Color::Red))
        .throbber_set(throbber_widgets_tui::BRAILLE_SIX_DOUBLE)
        .use_type(throbber_widgets_tui::WhichUse::Spin);
    f.render_stateful_widget(full, chunks[2], &mut app.throbber_state);

    // Bottom line
    let status_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Green));
    let status = Paragraph::new(Text::styled(
        "AWS Profile: smallapps-prod | Will use port [3389]",
        Style::default(),
    ))
    .block(status_block);
    f.render_widget(status, chunks[3]);
}
