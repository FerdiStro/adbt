use crate::app::Navigation::{CANCEL, COMMAND_ROW, DOWN, NULL, UP};
use ratatui::crossterm::event::KeyCode;
use crate::render_header::CommandLineState;

pub struct App {
    pub navigation: Navigation,
    pub title: String,
    pub command_line_state: CommandLineState
}

pub enum Navigation {
    NULL,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ENTER,
    CANCEL,
    COMMAND_ROW,
}

impl Navigation {
    pub fn on_key_code(&mut self, code: KeyCode) -> Navigation {
        match code {
            KeyCode::Up => UP,
            KeyCode::Esc => COMMAND_ROW,
            KeyCode::Char('q') => CANCEL,
            KeyCode::Char(':') => COMMAND_ROW,
            _ => NULL,
        }
    }
}

impl App {
    pub fn new() -> App {
        Self {
            title: "Adb-Helper".to_string(),
            navigation: Navigation::NULL,
            command_line_state: CommandLineState::default()
        }
    }
}
