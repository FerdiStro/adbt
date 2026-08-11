

use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::layout::Constraint::Ratio;

pub fn center_vertically(area: Rect, height: u16, padding: u32) -> Rect {
    let centered_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area)[0];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Ratio(padding, 8),
            Ratio(8 - padding * 2, 8),
            Ratio(padding, 8),
        ])
        .areas::<3>(centered_vertical)[1]
}