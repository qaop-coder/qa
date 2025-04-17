use std::{fs::read_to_string, path::PathBuf};

use crop::Rope;
use tracing::trace;

use crate::config::CliConfig;

/// Editor's state
#[derive(Debug)]
pub struct EditorState {
    /// Buffers currently open in the editor.
    buffers: Vec<Buffer>,
}

/// A buffer in the editor.
#[derive(Debug)]
pub struct Buffer {
    /// The content of the buffer.
    rope: Rope,

    /// The path to the file.
    file_path: Option<PathBuf>,
}

impl EditorState {
    pub fn new(config: &CliConfig) -> Self {
        // Initialize the editor state
        let buffers = config
            .paths
            .iter()
            .map(|path| {
                let path = path.canonicalize().unwrap_or_else(|_| path.clone());
                trace!("Opening file: {:?}", path);

                let file = read_to_string(path.clone()).unwrap_or_else(|_| {
                    trace!("Failed to read file: {:?}", path);
                    String::new()
                });

                let rope = Rope::from(file);

                Buffer {
                    rope,
                    file_path: Some(path.clone()),
                }
            })
            .collect::<Vec<_>>();

        EditorState { buffers }
    }
}
