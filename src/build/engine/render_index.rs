use std::path::Path;

use crate::{build::BuildRes, Config};

use super::Engine;

impl Engine<'_> {
    pub fn render_index(&mut self, config: &Config, dir: impl AsRef<Path>) -> BuildRes {
        let posts_dir = dir.as_ref().join("posts");
        // TODO: Define a `RecentPost` type that can be hydrated with summarised content when
        // needed
        todo!()
    }
}
