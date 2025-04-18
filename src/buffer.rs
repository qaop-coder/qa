use std::{
    fmt::Debug,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use color_eyre::Result;
use crop::Rope;

/// A buffer in the editor.
pub struct Buffer {
    /// The content of the buffer.
    rope: Rope,

    /// The path to the file.
    file_path: Option<PathBuf>,
}

impl Buffer {
    pub fn empty() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
        }
    }

    pub fn load(path: &Path) -> Result<Buffer> {
        let file = read_to_string(path).map_err(|e| {
            tracing::error!("Failed to read file: {:?}", e);
            e
        })?;

        let rope = Rope::from(file);

        Ok(Buffer {
            rope,
            file_path: Some(path.to_path_buf()),
        })
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self
            .file_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "<No Path>".to_string());

        f.debug_struct("Buffer")
            .field("path", &path)
            .field("rope", &self.rope.byte_len())
            .finish()
    }
}
