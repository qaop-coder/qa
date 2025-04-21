#![allow(dead_code)]

mod buffer;
mod config;
mod editor;
mod render;
mod theme;
mod view;

use clap::Parser;
use color_eyre::Result;
use config::CliConfig;
use editor::EditorState;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
};
use render::{
    command_area::render_command_area, status_bar::render_status_bar, text_area::render_text_area,
};
use tracing::{info, level_filters::LevelFilter, trace};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_max_level(LevelFilter::TRACE)
        // .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    info!("Starting editor...");

    let config = CliConfig::parse();
    trace!("Configuration: {:#?}", config);

    // Initialize the editor state
    let mut editor_state = EditorState::new(&config);

    let mut terminal = ratatui::init();
    loop {
        let _ = terminal.draw(|frame| draw_frame(frame, &editor_state));
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                KeyCode::Esc => break,
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => command_scroll_up(&mut editor_state),
                KeyCode::Char('k') => command_scroll_down(&mut editor_state),
                _ => {}
            },
            Ok(_) => {}
            Err(_) => break,
        }
    }
    ratatui::restore();

    trace!("Editor state: {:#?}", editor_state);
    info!("Exiting editor...");

    Ok(())
}

pub fn draw_frame(frame: &mut Frame, editor_state: &EditorState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_text_area(frame, layout[0], editor_state);
    render_status_bar(frame, layout[1], editor_state);
    render_command_area(frame, layout[2]);
}

fn command_scroll_up(editor_state: &mut EditorState) {
    let view = editor_state.current_view();
    let buffer_id = view.buffer_id;
    let buffer = editor_state.get_buffer(buffer_id);

    let num_lines = buffer.num_lines();

    let view = editor_state.current_view_mut();
    if view.top_line < num_lines - 1 {
        view.top_line += 1;
    }
}

fn command_scroll_down(editor_state: &mut EditorState) {
    let view = editor_state.current_view_mut();

    if view.top_line > 0 {
        view.top_line -= 1;
    }
}
