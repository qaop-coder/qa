use std::collections::BTreeMap;

use maplit::btreemap;
use tracing::info;

use crate::{
    buffer::Buffer,
    config::CliConfig,
    view::{View, create_first_views},
};

/// Editor's state
#[derive(Debug)]
pub struct EditorState {
    /// Temporary IDs
    next_buffer_id: usize,
    next_view_id: usize,

    /// Buffers currently open in the editor.
    buffers: BTreeMap<usize, Buffer>,
    views: BTreeMap<usize, View>,

    /// The current view in the editor.
    current_view: usize,
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
                current_view: 0,
            }
        } else {
            let views = create_first_views(buffer_index);
            EditorState {
                next_buffer_id: buffer_index,
                next_view_id: 0,

                buffers,
                views,
                current_view: 0,
            }
        }
    }
}
