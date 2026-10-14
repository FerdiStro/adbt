use crate::app::App;
use crate::render::render_device_module::render_device_module;
use crate::render::render_header::render_header;
use ratatui::Frame;
use ratatui::layout::Constraint::{Fill, Length, Ratio};
use ratatui::layout::{Direction, Layout};

pub fn render(_frame: &mut Frame, _app: &mut App) {
    let [header, body] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(12), Fill(1)])
        .areas(_frame.area());

    render_header(_frame, header, _app);

    let [top_body, bottom_body] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Ratio(1, 2), Ratio(1, 2)])
        .areas(body);

    let [device_area, x, y] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Ratio(1, 6), Ratio(1, 3), Ratio(1, 3)])
        .areas(top_body);

    render_device_module(_frame, device_area, _app);
}
