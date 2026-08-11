use crate::app::Navigation::{CANCEL, COMMAND_ROW, NULL, UP};

use crate::render::render_device_module::DeviceModule;
use crate::render::render_header::CommandLineState;
use ratatui::crossterm::event::KeyCode;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

pub struct App {
    pub navigation: Navigation,
    pub title: String,
    pub command_line_state: CommandLineState,
    pub device_module: DeviceModule,
    pub debug: Arc<Mutex<DebugState>>,
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
            navigation: NULL,
            command_line_state: CommandLineState::default(),
            device_module: DeviceModule::default(),
            debug: Arc::new(Mutex::new(DebugState::default())),
        }
    }
}

#[derive(Debug, Default)]
pub struct DebugState {
    pub is_error: bool,
    pub message: String,
}

impl DebugState {
    pub fn write_error(
        _debug_shared_state: LockResult<MutexGuard<DebugState>>,
        error_message: &str,
    ) {
        if let Ok(mut debug_lock) = _debug_shared_state {
            debug_lock.message = error_message.to_string();
            debug_lock.is_error = true;
        }
    }
}
