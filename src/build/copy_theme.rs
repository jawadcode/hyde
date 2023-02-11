use std::{fs::File, path::Path};

use anyhow::Context;
use serde::Serialize;
use upon::Engine;

use crate::{posts::post::RecentPost, Config};

#[derive(Serialize)]
pub struct IndexTemplate<'a> {
    #[serde(flatten)]
    config: &'a Config,
    recent_posts: &'a [RecentPost],
}

pub(super) fn copy_theme(
    config: &Config,
    proj_dir: impl AsRef<Path>,
    engine: &Engine,
) -> anyhow::Result<()> {
    let posts_dir = proj_dir.as_ref().join("posts");
    let mut recent_posts: Vec<RecentPost> = posts_dir
        .read_dir()
        .context("Failed to read 'posts/' dir")?
        .filter_map(Result::ok)
        .map(|entry| RecentPost::from_path(entry.path()))
        .try_collect()?;
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
        .try_collect()?;

    let template = engine
        .get_template("index")
        .context("'templates/index.html' not found")?;
    let mut dest_index_html = File::create(proj_dir.as_ref().join("static/index.html"))
        .context("Failed to create file 'static/index.html'")?;
    template
        .render_to_writer(
            &mut dest_index_html,
            IndexTemplate {
                config,
                recent_posts: &recent_posts,
            },
        )
        .context("Failed to render 'static/index.html'")?;
    Ok(())
}
