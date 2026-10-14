mod adb_executer;
mod app;
mod commands;
mod render;

use crate::app::{App, Navigation};
use color_eyre::Result;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEventKind};
use render::renderer;
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;
    create_app()
}

fn create_app() -> Result<()> {
    let is_set_up = match adb_executer::check_deps() {
        Some(_) => true,
        _ => false,
    };

    if !is_set_up {
        panic!("Deps not installed and ready yet.")
    }

    let mut app = App::new();
    let mut terminal = ratatui::init();

    adb_executer::check_devices(&mut app);

    loop {
        //render
        terminal.draw(|f| renderer::render(f, &mut app))?;

        //keyBoard
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    //Main focus should always work
                    if !app.command_line_state.is_selected {
                        app.navigation = app.navigation.on_key_code(key.code);
                    } else {
                        app.navigation = app.command_line_state.handle_key(key.code);
                    }

                    app.navigation = app
                        .device_module
                        .handle_key(key.code)
                        .unwrap_or(app.navigation);
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
