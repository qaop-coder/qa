use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use tracing::{info, trace};
use tracing_subscriber::EnvFilter;

/// Command line configuration for the application.
#[derive(Parser, Debug)]
#[clap(author = "QAOP", version, about = "QAOP's personal editor", long_about = None)]
pub struct CliConfig {
    /// The paths to edit.
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = CliConfig::parse();

    info!("Starting editor...");
    trace!("Configuration: {:#?}", config);
    Ok(())
}
