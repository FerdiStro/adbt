mod app;
mod render_header;
mod render_utils;
mod renderer;

use crate::app::{App, Navigation};
use color_eyre::Result;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;
    create_app()
}

fn create_app() -> Result<()> {
    let mut app = App::new();
    let mut terminal = ratatui::init();

    loop {
        //render
        terminal.draw(|f| renderer::render(f, &mut app))?;

        //keyBoard
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if (!app.command_line_state.is_selected) {
                        app.navigation = app.navigation.on_key_code(key.code);
                    } else {
                        app.navigation =  app.command_line_state.handle_key(key.code);
                    }
                }
            }
        }
        match app.navigation {
            Navigation::CANCEL => break Ok(()),
            Navigation::COMMAND_ROW => {
                app.command_line_state.is_selected = !app.command_line_state.is_selected;
                app.navigation = Navigation::NULL;
            }
            _ => {}
        }
    }
}
