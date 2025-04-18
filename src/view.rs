use std::collections::BTreeMap;

#[derive(Debug)]
pub struct View {
    pub buffer_id: usize,
    pub top_line: usize,
}

impl View {
    pub fn new(buffer_id: usize) -> Self {
        View {
            buffer_id,
            top_line: 0,
        }
    }
}

/// Given a collection of buffers with ids 0..max_id, create views for each buffer.
///
/// This is used purely for initialisation of the editor.
pub(crate) fn create_first_views(max_id: usize) -> BTreeMap<usize, View> {
    let mut views = BTreeMap::new();
    for i in 0..max_id {
        views.insert(i, View::new(i));
    }
    views
}
