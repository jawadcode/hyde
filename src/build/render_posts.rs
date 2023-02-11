use std::{fs, path::Path};

use anyhow::Context;
use upon::Engine;

use crate::{posts::post::Post, Config};

pub fn render_posts(
    config: &Config,
    posts_dir: impl AsRef<Path>,
    proj_dir: impl AsRef<Path>,
    engine: &Engine,
) -> anyhow::Result<()> {
    let post_template = engine
        .get_template("post")
        .context("Missing 'templates/post.html'")?;
    let static_posts_dir = proj_dir.as_ref().join("static/posts");
    fs::create_dir_all(&static_posts_dir).context("Failed to create 'static/posts' dir")?;
    for source in fs::read_dir(&posts_dir).context("Failed to read 'posts/' dir")? {
        let source = source?;
        let source_path = source.path();
        let source_metadata = source.metadata().with_context(|| {
            format!(
                "Failed to read metadata for post '{}'",
                source.path().display()
            )
        })?;
        let html_path = {
            let mut html_filename = source
                .path()
                .file_stem()
                .with_context(|| {
                    format!(
                        "Failed to get filestem for post '{}'",
                        source.path().display()
                    )
                })?
                .to_os_string();
            html_filename.push(".html");
            static_posts_dir.join(html_filename)
        };
        if html_path.exists() {
            let html_metadata = html_path.metadata().with_context(|| {
                format!(
                    "Failed to get metadata for rendered post '{}'",
                    html_path.display()
                )
            })?;
            if source_metadata.modified()? > html_metadata.modified()? {
                let post = Post::from_path(&source_path)
                    .with_context(|| format!("Failed to read post '{}'", source_path.display()))?;
                post.render(config, html_path, post_template)?;
            }
        } else {
            let post = Post::from_path(&source_path)
                .with_context(|| format!("Failed to read post '{}'", source_path.display()))?;
            post.render(config, html_path, post_template)?;
        }
    }
    Ok(())
}
