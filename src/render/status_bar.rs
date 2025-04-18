use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, Borders},
};

use crate::{
    editor::EditorState,
    theme::{STATUS_BAR_BG, STATUS_BAR_FG},
};

pub fn render_status_bar(frame: &mut Frame, area: Rect, editor_state: &EditorState) {
    let mode_text = editor_state.mode_str();

    let status_bar_area = Block::new()
        .style(Style::default().bg(STATUS_BAR_BG).fg(STATUS_BAR_FG))
        .borders(Borders::NONE);
    frame.render_widget(status_bar_area, area);

    let status_text =
        Text::raw(mode_text).style(Style::default().bg(STATUS_BAR_BG).fg(STATUS_BAR_FG));
    let status_area = Rect::new(area.x + 1, area.y, area.width - 1, 1);
    frame.render_widget(status_text, status_area);
}
