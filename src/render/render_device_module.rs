use crate::app::{App, DebugState, Navigation};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph};
use std::sync::{Arc, Mutex};
use ratatui::crossterm::event::KeyCode;
use ratatui::prelude::{Line, Modifier, Span, Style};
use ratatui::style::Color;

pub struct DeviceModule {
    pub device_id_list: Arc<Mutex<Vec<String>>>,
    pub is_keyboard_listener: bool,
}

impl DeviceModule {

    pub(crate) fn handle_key(&self, p0: KeyCode) -> Option<Navigation> {
        None
    }

    pub fn default() -> DeviceModule {
        Self {
            device_id_list: Arc::new(Mutex::new(Vec::new())),
            is_keyboard_listener: false,
        }
    }
}

pub(crate) fn render_device_module(_frame: &mut Frame, area: Rect, _app: &mut App) {
    let block = Block::bordered().title("Devices:");

    _frame.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    if let Ok(device_list_lock) = _app.device_module.device_id_list.lock() {

        let lines: Vec<Line> = device_list_lock
            .iter()
            .enumerate()
            .map(|(index, device_id)| {
                let device_style = if index % 2 == 0 {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Yellow)
                };

                Line::from(vec![
                    Span::styled(
                        format!("{}. ", index + 1),
                        Style::default().add_modifier(Modifier::BOLD)
                    ),
                    Span::styled(format!("{}", device_id), device_style),
                ])
            })
            .collect();

        _frame.render_widget(Paragraph::new(lines), inner_area);
    }
}
