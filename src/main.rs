use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame};
use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind };
use color_eyre::Result;
use std::{io, vec, fmt};

#[derive(Debug, PartialEq)]
enum UserType {
    Driver, 
    Dispatcher,
}

#[derive(Debug, Default)]
pub struct App {
    user_type: Option<UserType>,
    exit: bool,
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UserType::Driver => "Driver",
            UserType::Dispatcher => "Dispatcher",
        };
        write!(f, "{}", s)
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.user_type = Some(UserType::Driver),
            KeyCode::Char('2') => self.user_type = Some(UserType::Dispatcher),
            _ => {}
        }
    }

    fn get_user_type_str(&self) -> &str {
        match self.user_type {
            Some(UserType::Driver) => " Operating as: Driver ",
            Some(UserType::Dispatcher) => " Operating as: Dispatcher ",
            None => " Welcome, please login as a Driver or Dispatcher "
        }
    }
    
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)    {
    let title = Line::from(" FleetSync Dispatch Hub ".bold());
    let instructions = Line::from(vec![
        " Dispatcher ".into(),
        " <1> ".blue().bold(),
        " Driver ".into(),
        " <2> ".blue().bold(),
        " Quit ".into(),
        " <Q> ".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    
    let user_type_text = Text::from(vec![Line::from(vec![
        self.get_user_type_str().bold().into(),
    ])]);
   
    Paragraph::new(user_type_text)
        .centered()
        .block(block)
        .render(area, buf);
    }
}

fn main() {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_render_app() {
        let app = App::default();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()) )
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('1').into());
        assert_eq!(app.user_type, Some(UserType::Driver));
        
        app.handle_key_event(KeyCode::Char('2').into());
        assert_eq!(app.user_type, Some(UserType::Dispatcher));

        let mut app= App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}