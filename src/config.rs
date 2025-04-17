use std::path::PathBuf;

use clap::Parser;

/// Command line configuration for the application.
#[derive(Parser, Debug)]
#[clap(author = "QAOP", version, about = "QAOP's personal editor", long_about = None)]
pub struct CliConfig {
    /// The paths to edit.
    pub paths: Vec<PathBuf>,
}
