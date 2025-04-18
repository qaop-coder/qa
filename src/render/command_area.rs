use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
};

use crate::theme::{TEXT_BG, TEXT_FG};

pub fn render_command_area(frame: &mut Frame, area: Rect) {
    let command_area = Block::new()
        .style(Style::default().bg(TEXT_BG).fg(TEXT_FG))
        .borders(Borders::NONE);
    frame.render_widget(command_area, area);
}
