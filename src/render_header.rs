use crate::app::{App, Navigation};
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Constraint::Length;
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use regex::Regex;

pub struct CommandLineState {
    pub is_selected: bool,
    pub command_string: String,
    pub execute_command: Command,
}

pub struct Command {
    pub command_matcher: CommandMatcher,
}

impl Command {
    pub fn default() -> Self {
        Self {
            command_matcher: CommandMatcher::NULL,
        }
    }

    pub fn get_command_matcher(&self, command: &str) -> CommandMatcher {
        [
            self.get_command_matcher_helper::<2>(command, CommandMatcher::QUIT),
            // self.get_command_matcher_helper::<2>(command, CommandMatcher::TEST),
        ]
        .into_iter()
        .find_map(|m| m)
        .unwrap_or(CommandMatcher::NULL)
    }

    fn get_command_matcher_helper<const N: usize>(
        &self,
        command: &str,
        check_command: CommandMatcher,
    ) -> Option<CommandMatcher> {
        if self
            .get_command_matcher_strings(&check_command)
            .iter()
            .any(|matcher| Regex::new(matcher).unwrap().is_match(command))
        {
            Some(check_command)
        } else {
            None
        }
    }

    fn get_command_matcher_strings(
        &self,
        _command_matcher: &CommandMatcher,
    ) -> &'static [&'static str] {
        match _command_matcher {
            CommandMatcher::NULL => &[],
            CommandMatcher::QUIT => &["q", "quit"],
            CommandMatcher::TEST => &["t", "test"],
        }
    }
}

pub enum CommandMatcher {
    NULL,
    QUIT,
    TEST,
}

impl CommandLineState {
    pub(crate) fn default() -> CommandLineState {
        CommandLineState {
            is_selected: false,
            command_string: String::new(),
            execute_command: Command::default(),
        }
    }

    fn execute_command(&mut self) -> Navigation {
        self.is_selected = false;
        match Command::get_command_matcher(&self.execute_command, &*self.command_string){
            CommandMatcher::QUIT => {Navigation::CANCEL}
            CommandMatcher::TEST => {Navigation::CANCEL}
            CommandMatcher::NULL => {Navigation::NULL}
        }


    }

    pub fn handle_key(&mut self, key: KeyCode) -> Navigation {
        match key {
            KeyCode::Enter => return self.execute_command(),
            KeyCode::Esc => self.is_selected = false,
            KeyCode::Backspace => {
                self.command_string = {
                    let mut chars = self.command_string.chars();
                    chars.next_back();
                    chars.as_str().to_string()
                }
            }
            KeyCode::Char(..) => {
                self.command_string = self.command_string.clone() + &*key.to_string()
            }
            _ => {}
        }
        Navigation::NULL
    }
}

pub(crate) fn render_header(_frame: &mut Frame, area: Rect, _app: &mut App) {
    let block = Block::bordered().title("Header");
    _frame.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    let [headline_area, commando_line_area] = Layout::default()
        .constraints([Length(5), Length(3)])
        .direction(Direction::Vertical)
        .areas(inner_area);

    render_header_text(_frame, headline_area, _app);
    render_commandline(_frame, commando_line_area, _app);
}

//adb-helper text generated on: https://www.messletters.com/en/big-text/
fn render_header_text(_frame: &mut Frame, headline_area: Rect, _app: &mut App) {
    let [first_row, second_row, third_row, forth_row, fives_row] = Layout::default()
        .constraints([Length(1), Length(1), Length(1), Length(1), Length(1)])
        .direction(Direction::Vertical)
        .areas(headline_area);

    _frame.render_widget(
        Paragraph::new(
            "█████  ██████  ██████        ██   ██ ███████ ██      ██████  ███████ ██████  ",
        )
        .style(Style::default().fg(Color::Cyan))
        .centered(),
        first_row,
    );
    _frame.render_widget(
        Paragraph::new(
            "██   ██ ██   ██ ██   ██       ██   ██ ██      ██      ██   ██ ██      ██   ██ 
",
        )
        .style(Style::default().fg(Color::Cyan))
        .centered(),
        second_row,
    );
    _frame.render_widget(
        Paragraph::new(
            "███████ ██   ██ ██████  █████ ███████ █████   ██      ██████  █████   ██████  
",
        )
        .style(Style::default().fg(Color::Cyan))
        .centered(),
        third_row,
    );
    _frame.render_widget(
        Paragraph::new(
            "██   ██ ██   ██ ██   ██       ██   ██ ██      ██      ██      ██      ██   ██ ",
        )
        .style(Style::default().fg(Color::Cyan))
        .centered(),
        forth_row,
    );
    _frame.render_widget(
        Paragraph::new(
            "██   ██ ██████  ██████        ██   ██ ███████ ███████ ██      ███████ ██   ██",
        )
        .style(Style::default().fg(Color::Cyan))
        .centered(),
        fives_row,
    );
}

fn render_commandline(_frame: &mut Frame, commandline_area: Rect, _app: &mut App) {
    let selected_style = match _app.command_line_state.is_selected {
        true => Style::default().fg(Color::White),
        false => Style::default().fg(Color::Black),
    };

    let commandline_block = Block::bordered().title("cmd").style(selected_style);
    _frame.render_widget(commandline_block.clone(), commandline_area);

    let commandline_inner = commandline_block.inner(commandline_area);
    _frame.render_widget(
        Paragraph::new(": ".to_owned() + &*_app.command_line_state.command_string),
        commandline_inner,
    );
}
