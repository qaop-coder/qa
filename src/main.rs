#![allow(unused)]

mod config;
mod editor;

use clap::Parser;
use color_eyre::Result;
use config::CliConfig;
use editor::EditorState;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventState},
    text::Text,
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
        terminal.draw(|frame| {
            let text = Text::raw("Hello, world!");
            frame.render_widget(text, frame.area());
        });
        if matches!(
            event::read(),
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            }))
        ) {
            break;
        }
    }
    ratatui::restore();

    Ok(())
}
