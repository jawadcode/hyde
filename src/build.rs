use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use serde::Serialize;
use upon::Engine;

use crate::{
    posts::{self, post::Post},
    Config,
};

pub struct Theme {
    index: String,
    post: String,
}

#[derive(Serialize)]
pub struct PostInfo<'a> {
    #[serde(flatten)]
    post: &'a Post,
    config: &'a Config,
}

pub fn build(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let config_path = path.as_ref().join("hyde.toml");
    if !config_path.exists() {
        return Err(anyhow::Error::msg(
            "Current directory is not a project (missing 'hyde.toml')",
        ));
    }
    let config_str =
        fs::read_to_string(&config_path).with_context(|| "Failed to read 'hyde.toml'")?;
    let config: Config = toml::from_str(&config_str).with_context(|| "Invalid 'hyde.toml'")?;

    let static_dir = path.as_ref().join("static");
    let Theme { index, post } = read_theme(&static_dir, &config.theme)?;
    let mut engine = Engine::new();

    engine.add_template("index", index).with_context(|| {
        format!(
            "Failed to parse template '{}'",
            config.theme.join("index.html").display()
        )
    })?;
    {
        let index_template = engine.get_template("index").unwrap();
        let index = index_template.render(&config).with_context(|| {
            format!(
                "Failed to render '{}' template",
                config.theme.join("index.html").display(),
            )
        })?;
        let index_path = static_dir.join("index.html");
        fs::write(&index_path, index).with_context(|| {
            format!(
                "Failed to write rendered template to '{}'",
                index_path.display()
            )
        })?;
    }
    engine.add_template("post", post).with_context(|| {
        format!(
            "Failed to parse template '{}'",
            config.theme.join("post.html").display()
        )
    })?;

    let posts_dir = static_dir.join("posts");
    if posts_dir.exists() {
        fs::remove_dir_all(&posts_dir)
            .with_context(|| "Failed to delete 'static/posts' directory`")?;
    }
    fs::create_dir(&posts_dir).with_context(|| "Failed to create 'static/posts' directory")?;
    let post_template = engine.get_template("post").unwrap();
    for post in posts::get_posts().with_context(|| "Failed to read posts")? {
        let post_name = post
            .path
            .file_stem()
            .with_context(|| format!("Failed to extract file stem from '{}'", post.path.display()))?
            .to_os_string()
            .into_string()
            .unwrap()
            + ".html";
        let rendered_post = post_template
            .render(PostInfo {
                post: &post,
                config: &config,
            })
            .with_context(|| format!("Failed to render post '{}'", post.path.display()))?;
        fs::write(posts_dir.join(post_name), rendered_post)
            .with_context(|| format!("Failed to write rendered post '{}'", post.path.display()))?;
    }
    Ok(())
}

/// Reads the theme directory, copying non-template files over to 'static' and returning the template files ('index.html' and 'post.html')
fn read_theme(path: impl AsRef<Path>, theme_dir: impl AsRef<Path>) -> anyhow::Result<Theme> {
    fs::create_dir_all(&path).with_context(|| "Failed to create 'static' directory")?;
    for entry in theme_dir
        .as_ref()
        .read_dir()
        .with_context(|| "Failed to read theme directory")?
        .filter(|entry| {
            entry
                .as_ref()
                .map(|entry| {
                    let filename = entry.file_name();
                    filename == "index.html" || filename == "post.html"
                })
                .unwrap_or(false)
        })
    {
        let entry = entry?;
        let Ok(entry_metadata) = entry.metadata() else {
            eprintln!("Warning: Failed to access the metadata of '{}'", entry.path().display());
            continue;
        };
        if entry_metadata.is_file() {
            fs::copy(entry.path(), path.as_ref().join(entry.file_name()))?;
        } else if entry_metadata.is_dir() {
            copy_dir(entry.path(), path.as_ref().join(entry.file_name()))?;
        } else {
            return Err(anyhow!(
                "'{}' is not a file or directory",
                entry.path().display()
            ));
        }
    }

    let index = fs::read_to_string(path.as_ref().join("index.html"))
        .with_context(|| format!("File '{}' not found", path.as_ref().display()))?;
    let post = fs::read_to_string(path.as_ref().join("post.html"))
        .with_context(|| format!("File '{}' not found", path.as_ref().display()))?;
    Ok(Theme { index, post })
}

fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> anyhow::Result<()> {
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let Ok(entry_metadata) = entry.metadata() else {
            eprintln!("Warning: '{}''s metadata could not be accessed", entry.path().display());
            continue;
        };
        if entry_metadata.is_file() {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else if entry_metadata.is_dir() {
            copy_dir(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            return Err(anyhow!(
                "'{}' is not a file or directory",
                entry.path().display()
            ));
        }
    }
    Ok(())
}
