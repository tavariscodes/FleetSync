// Contains all of our application's state
// we will use application state to
// track the screen the user is seeing

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{Terminal, prelude::Backend};
use std::{default, fmt, io};

use crate::ui::ui;

#[derive(Debug, PartialEq)]
pub enum UserType {
    Driver,
    Dispatcher,
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
#[derive(Default, Debug)]
pub enum CurrentScreen {
    #[default]
    Main,
    Driver,
    Dispatch,
    Exiting,
}


#[derive(Debug, Default)]
pub struct App {
    pub user_type: Option<UserType>,
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            user_type: None,
            current_screen: CurrentScreen::Main,
            exit: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self));
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    pub fn handle_exit_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') => {
                self.current_screen = CurrentScreen::Main;
                self.exit = true;
            }
            KeyCode::Char('n') => {
                self.current_screen = CurrentScreen::Main;
                self.user_type = None
            }
            _ => {}
        }
    }

    pub fn handle_driver_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('m') => {
                self.current_screen = CurrentScreen::Main;
                self.user_type = None;
            }
            _ => {}
        }
    }

    pub fn handle_dispatcher_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('m') => {
                self.current_screen = CurrentScreen::Main;
                self.user_type = None;
            }
            _ => {}
        }
    }

    pub fn handle_main_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.current_screen = CurrentScreen::Exiting;
            }
            KeyCode::Char('1') => {
                self.user_type = Some(UserType::Driver);
                self.current_screen = CurrentScreen::Driver;
            }
            KeyCode::Char('2') => {
                self.user_type = Some(UserType::Dispatcher);
                self.current_screen = CurrentScreen::Dispatch;
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Main => {
                self.handle_main_screen_key_event(key_event);
            }
            CurrentScreen::Driver => {
                self.handle_driver_screen_key_event(key_event);
            }
            CurrentScreen::Dispatch => {
                self.handle_dispatcher_screen_key_event(key_event);
            }
            CurrentScreen::Exiting => {
                self.handle_exit_screen_key_event(key_event);
            }
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_render_app() {
        let mut app = App::default();
    
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal.draw(|frame| ui(frame, &app));
        
        assert_snapshot!(terminal.backend())
    }
}