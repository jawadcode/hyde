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
        .with_context(|| "Failed to read 'posts/' dir")?
        .filter_map(Result::ok)
        .map(|entry| RecentPost::from_path(entry.path()))
        .try_collect()?;
    recent_posts.sort_unstable_by(|post1, post2| {
        post1.frontmatter.datetime.cmp(&post2.frontmatter.datetime)
    });
    // TODO: Modify `RecentPost` and `IndexTemplate` such that we render the summaries for only the 5 most recent posts
    recent_posts = recent_posts.into_iter().rev().take(5).collect();

    let template = engine
        .get_template("index")
        .with_context(|| "'templates/index.html' not found")?;

    let dest_index_html = File::create(proj_dir.as_ref().join("static/index.html"))
        .with_context(|| "Could not create file 'index.html'")?;
    template
        .render_to_writer(
            dest_index_html,
            IndexTemplate {
                config,
                recent_posts: &recent_posts,
            },
        )
        .with_context(|| "Failed to render 'static/index.html'")?;

    Ok(())
}
