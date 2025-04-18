#![allow(dead_code)]

mod buffer;
mod config;
mod editor;
mod view;

use clap::Parser;
use color_eyre::Result;
use config::CliConfig;
use editor::EditorState;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tracing::{info, trace};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    info!("Starting editor...");

    let config = CliConfig::parse();
    trace!("Configuration: {:#?}", config);

    // Initialize the editor state
    let editor_state = EditorState::new(&config);

    let mut terminal = ratatui::init();
    loop {
        let _ = terminal.draw(draw_frame);
        if matches!(
            event::read(),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }))
        ) {
            break;
        }
    }
    ratatui::restore();

    trace!("Editor state: {:#?}", editor_state);
    info!("Exiting editor...");

    Ok(())
}

// const TEXT_BG: Color = Color::Rgb(0x3b, 0x22, 0x4a);
const TEXT_BG: Color = Color::Rgb(0x4a, 0x22, 0x3b);
const TEXT_FG: Color = Color::White;
const STATUS_BAR_BG: Color = Color::Rgb(0x32, 0x17, 0x28);
const STATUS_BAR_FG: Color = Color::White;

pub fn draw_frame(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let text_area = Block::new()
        .style(Style::default().bg(TEXT_BG).fg(TEXT_FG))
        .borders(Borders::NONE);

    let status_bar_area = Block::new()
        .style(Style::default().bg(STATUS_BAR_BG).fg(STATUS_BAR_FG))
        .borders(Borders::NONE);

    let command_area = Block::new()
        .style(Style::default().bg(TEXT_BG).fg(TEXT_FG))
        .borders(Borders::NONE);

    frame.render_widget(text_area, layout[0]);
    frame.render_widget(status_bar_area, layout[1]);
    frame.render_widget(command_area, layout[2]);
}
