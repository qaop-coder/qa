use std::collections::BTreeMap;

use maplit::btreemap;
use tracing::info;

use crate::{
    buffer::Buffer,
    config::CliConfig,
    view::{View, create_first_views},
};

/// Editor's state
#[derive(Debug, Default)]
pub struct EditorState {
    /// Temporary IDs
    next_buffer_id: usize,
    next_view_id: usize,

    /// Buffers currently open in the editor.
    buffers: BTreeMap<usize, Buffer>,
    views: BTreeMap<usize, View>,

    /// The current view in the editor.
    current_view: usize,

    /// Editing state
    mode: EditorMode,
}

impl EditorState {
    pub fn new(config: &CliConfig) -> Self {
        let mut buffer_index = 0;

        // Initialize the editor state
        let buffers = config
            .paths
            .iter()
            .filter_map(|path| {
                let path = path.canonicalize().unwrap_or_else(|_| path.clone());
                info!("Opening file: {:?}", path);

                let current_buffer_index = buffer_index;
                buffer_index += 1;

                let buffer = Buffer::load(&path);
                match buffer {
                    Ok(buffer) => Some((current_buffer_index, buffer)),
                    Err(_) => None,
                }
            })
            .collect::<BTreeMap<_, _>>();

        if buffers.is_empty() {
            // If no buffers were loaded, create a new empty buffer
            let empty_buffer = Buffer::empty();
            let buffers = btreemap!(
                0 => empty_buffer,
            );

            let views = create_first_views(1);

            EditorState {
                next_buffer_id: 1,
                next_view_id: 1,
                buffers,
                views,
                ..Default::default()
            }
        } else {
            let views = create_first_views(buffer_index);
            EditorState {
                next_buffer_id: buffer_index,
                next_view_id: buffer_index,
                buffers,
                views,
                ..Default::default()
            }
        }
    }

    pub fn current_view(&self) -> &View {
        self.views
            .get(&self.current_view)
            .expect("Current view index does not reference an existing view")
    }

    pub fn current_buffer(&self) -> &Buffer {
        let view = self.current_view();
        self.buffers
            .get(&view.buffer_id)
            .expect("Current view does not reference an existing buffer")
    }

    pub fn mode_str(&self) -> &str {
        match self.mode {
            EditorMode::Normal => "NOR",
            EditorMode::Insert => "INS",
            EditorMode::Command => "CMD",
            EditorMode::Select => "SEL",
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum EditorMode {
    /// Normal mode, where the editor is in a state of normal operation.
    #[default]
    Normal,

    /// Insert mode, where the editor is in a state of inserting text.
    Insert,

    /// Command mode, where the editor is in a state of executing commands.
    Command,

    /// Select mode, where the editor is in a state of selecting text.
    Select,
}
