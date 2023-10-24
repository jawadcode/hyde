mod recent_post;
mod summarise;

use crate::{
    build::{engine::render_index::recent_post::RecentPost, read_dir, BuildError, BuildRes},
    Config,
};

use super::Engine;

use std::{fs::File, path::Path};

use serde::Serialize;

#[derive(Serialize)]
pub struct IndexTemplate<'a> {
    #[serde(flatten)]
    config: &'a Config,
    recent_posts: &'a [RecentPost],
}

impl Engine<'_> {
    /// Renders the `index.html` template, including summarised forms of the 5 most recent posts
    ///
    /// # Panics
    ///
    /// If the `index.html` template has not been loaded into the engine beforehand using
    /// [`Engine::load_templates`], a panic will occur
    pub fn render_index(&mut self, config: &Config, dir: impl AsRef<Path>) -> BuildRes {
        let dir = dir.as_ref();
        let posts_dir = dir.join("posts");
        let mut recent_posts = read_dir(&posts_dir, &[])?
            .map(|entry| RecentPost::from_path(entry.path()))
            .collect::<Result<Vec<RecentPost>, BuildError>>()?;

        recent_posts.sort_unstable_by(|post1, post2| {
            post1
                .frontmatter
                .timestamp
                .cmp(&post2.frontmatter.timestamp)
        });
        recent_posts = recent_posts
            .into_iter()
            .rev()
            .take(5)
            .map(RecentPost::hydrate)
            .collect::<Result<_, BuildError>>()?;

        let template = self.engine.get_template("index").unwrap();
        let index_path = dir.join("static/index.html");
        let mut dest_index_html =
            File::create(&index_path).map_err(|err| (err, index_path.clone()))?;

        template
            .render(IndexTemplate {
                config,
                recent_posts: &recent_posts,
            })
            .to_writer(&mut dest_index_html)
            .map_err(|err| BuildError::RenderPost {
                source: Box::new(err),
                path: index_path,
            })?;
        Ok(())
    }
}
