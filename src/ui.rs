use std::vec;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Welcome to FleetSync",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let current_navigation_text = vec![
        // first half of text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Main Menu", Style::default().fg(Color::Green)),
            CurrentScreen::Dispatch => {
                Span::styled("Current Mode: Dispatcher", Style::default().fg(Color::Blue))
            }
            CurrentScreen::Driver => {
                Span::styled("Current Mode: Driver", Style::default().fg(Color::Blue))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::Blue)),
        }
        .to_owned(),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit/ (1) login as driver / (2) login as dispatcher",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(y)/(n) are you sure you want to quit?",
                Style::default().fg(Color::Red),
            ),
            _ => Span::styled("(m) back to main menu", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
