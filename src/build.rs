use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::{
    posts::{self, post::Post},
    Config,
};

pub struct Theme {
    index: String,
    post: String,
    
}

pub fn build(path: PathBuf) -> anyhow::Result<()> {
    let mut path = path;
    path.push("hyde.toml");
    if !path.exists() {
        return Err(anyhow::Error::msg(
            "Current directory is not a project (missing 'hyde.toml')",
        ));
    }
    let config_str = fs::read_to_string(&path).with_context(|| "Couldn't read 'hyde.toml'")?;
    let config: Config = toml::from_str(&config_str).with_context(|| "Invalid 'hyde.toml'")?;
    let posts = posts::get_posts().with_context(|| "Failed to read posts")?;
    // TODO: read templates from theme directory
    //       copy over any non-template files over to the 'static' dir
    //       render the index.html template to 'static/index.html'
    //       render the post.html template for each 'posts/post_name.md' and write to 'static/post_name.html' (copy the filename, and not the name of the post from its frontmatter)
    //       that's it i think
    for Post {
        path,
        front_matter,
        content,
    } in posts
    {}
    Ok(())
}

fn read_theme(path: PathBuf) -> anyhow::Result<()> {
    fs::copy
    Ok(())
}
