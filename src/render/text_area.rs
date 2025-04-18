use ratatui::{Frame, layout::Rect, style::Style, text::Text};

use crate::{
    editor::EditorState,
    theme::{TEXT_BG, TEXT_FG, TILDE_FG},
};

pub fn render_text_area(frame: &mut Frame, area: Rect, editor_state: &EditorState) {
    let view = editor_state.current_view();
    let buffer = editor_state.current_buffer();
    let num_lines_to_render = area.height as usize;
    let screen_width = area.width as usize;

    let last_line = buffer.num_lines().min(view.top_line + num_lines_to_render);

    // Render all the lines that contain text.
    for line in view.top_line..last_line {
        let line_text = buffer
            .get_line(line)
            .chars()
            .take(screen_width)
            .collect::<String>();
        let line_area = Rect::new(area.x, area.y + line as u16, area.width, 1);
        let text = Text::raw(line_text).style(Style::default().bg(TEXT_BG).fg(TEXT_FG));
        frame.render_widget(text, line_area);
    }

    let lines_rendered = last_line - view.top_line;

    // Fill the remaining lines with a tilde character.
    for line in lines_rendered..num_lines_to_render {
        let line_area = Rect::new(
            area.x,
            area.y + (view.top_line + line) as u16,
            area.width,
            1,
        );
        let text = Text::raw("~").style(Style::default().bg(TEXT_BG).fg(TILDE_FG));
        frame.render_widget(text, line_area);
    }
}
