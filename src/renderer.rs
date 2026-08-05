use crate::app::App;
use crate::render_header;
use ratatui::layout::Constraint::Ratio;
use ratatui::layout::{Direction, Layout};
use ratatui::Frame;

pub fn render(_frame: &mut Frame, _app: &mut App) {
    let [header, body] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 4), Ratio(3, 4)])
        .areas(_frame.area());

    render_header::render_header(_frame, header, _app)
}
