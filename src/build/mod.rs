mod render_index;
mod render_posts;

use std::{ffi::OsStr, fs, path::Path};

use anyhow::{bail, Context};

use self::{render_index::render_index, render_posts::render_posts};

pub fn build(proj_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let hyde_toml_path = proj_dir.as_ref().join("hyde.toml");
    if !hyde_toml_path.exists()
        || !hyde_toml_path
            .metadata()
            .with_context(|| "Failed to read metadata of 'hyde.toml'")?
            .is_file()
    {
        bail!("Missing 'hyde.toml'");
    }
    let hyde_toml =
        fs::read_to_string(hyde_toml_path).with_context(|| "Failed to read 'hyde.toml'")?;
    let config = toml::from_str(&hyde_toml).with_context(|| "Failed to parse 'hyde.toml'")?;
    let static_dir = proj_dir.as_ref().join("static");
    fs::create_dir_all(&static_dir).with_context(|| "Failed to create 'static/'")?;
    render_index(&config, &proj_dir)?;
    // Remove any files in 'static/' that do not exist in the theme dir, excluding 'posts/' (which will be handled separately)
    clean_dir(&config.theme, &static_dir, &[OsStr::new("posts")])
        .with_context(|| "Failed to remove")?;
    // Copy auxilliary theme entries
    copy_dir(
        &config.theme,
        &static_dir,
        &["index.html", "post.html"].map(OsStr::new),
    )
    .with_context(|| "Failed to copy over theme files")?;
    // Render markdown posts in 'posts/' to 'static/posts/' as html
    render_posts(&config, proj_dir.as_ref().join("posts"))
        .with_context(|| "Failed to render markdown posts")?;
    Ok(())
}

/// Traverse `src`, copying all its files and folders to `dest`, excluding entries in the top-level of the directory with a filename that occurs in `exclude`
/// Note: takes the last modified timestamp of the source and destination entries into account
fn copy_dir(
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    exclude: &[&OsStr],
) -> anyhow::Result<()> {
    fs::create_dir_all(&dest)
        .with_context(|| format!("Failed to create directory '{}'", dest.as_ref().display()))?;
    for entry in fs::read_dir(&src)
        .with_context(|| format!("Failed to read directory '{}'", src.as_ref().display()))?
        .filter(|entry| {
            entry
                .as_ref()
                .map(|entry| !exclude.contains(&entry.file_name().as_os_str()))
                .unwrap_or(true)
        })
    {
        let Ok(entry) = entry else {
            eprintln!("Warning: Failed to access entry in '{}'", src.as_ref().display());
            continue;
        };
        let entry_metadata = entry.metadata().with_context(|| {
            format!("Failed to access metadata of '{}'", entry.path().display())
        })?;
        let dest = dest.as_ref().join(entry.file_name());
        let dest_metadata = dest
            .metadata()
            .with_context(|| format!("Failed to access metadata of '{}'", dest.display()))?;
        if entry_metadata.modified()? > dest_metadata.modified()? {
            if entry_metadata.is_file() {
                fs::copy(entry.path(), dest)?;
            } else if entry_metadata.is_dir() {
                copy_dir(entry.path(), dest, &[])?;
            } else {
                bail!(
                    "Failed to copy entry '{}' as it is not a file or directory",
                    entry.path().display()
                );
            }
        }
    }
    Ok(())
}

/// Traverse `dest` and remove entries that do not exist in `src`, excluding entries in the top-level with a filename that occurs in `excluded`
fn clean_dir(
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    exclude: &[&OsStr],
) -> anyhow::Result<()> {
    if !dest.as_ref().exists() {
        return Ok(());
    }
    for entry in fs::read_dir(&dest)
        .with_context(|| format!("Failed to read directory '{}'", dest.as_ref().display()))?
        .filter(|entry| {
            entry
                .as_ref()
                .map(|entry| !exclude.contains(&entry.file_name().as_os_str()))
                .unwrap_or(true)
        })
    {
        let Ok(entry) = entry else {
            eprintln!("Warning: Failed to access entry in '{}'", src.as_ref().display());
            continue;
        };

        let src_entry = src.as_ref().join(entry.file_name());
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to get file type for '{}'", entry.path().display()))?;
        if !src_entry.exists() {
            if file_type.is_file() {
                fs::remove_file(entry.path()).with_context(|| {
                    format!("Failed to remove file '{}'", entry.path().display())
                })?;
            } else if file_type.is_dir() {
                fs::remove_dir_all(entry.path()).with_context(|| {
                    format!("Failed to remove directory '{}'", entry.path().display())
                })?;
            } else {
                bail!(
                    "Failed to remove entry '{}' as it is not a file or directory",
                    entry.path().display()
                );
            }
        } else if src_entry.exists() && file_type.is_dir() {
            clean_dir(src.as_ref().join(entry.file_name()), entry.path(), &[])?;
        }
    }
    Ok(())
}
