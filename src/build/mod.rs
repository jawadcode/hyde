mod copy_theme_files;
mod render_index;

use std::{fs, path::Path};

use anyhow::{anyhow, bail, Context};

use self::copy_theme_files::copy_theme_files;
use self::render_index::render_index;

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
    fs::create_dir_all(proj_dir.as_ref().join("static"))
        .with_context(|| "Failed to create 'static/'")?;
    render_index(&config, &proj_dir)?;
    copy_theme_files(&proj_dir)?;
    Ok(())
}

fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>, exclude: &[&str]) -> anyhow::Result<()> {
    fs::create_dir_all(&dest)
        .with_context(|| format!("Failed to create directory '{}'", dest.as_ref().display()))?;
    for entry in fs::read_dir(&src)
        .with_context(|| format!("Failed to read directory '{}'", src.as_ref().display()))?
        .filter(|entry| {
            entry
                .as_ref()
                .map(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .map(|name| !exclude.contains(&name.as_str()))
                        .unwrap_or(true)
                })
                .unwrap_or(true)
        })
    {
        let entry = entry?;
        let Ok(entry_metadata) = entry.metadata() else {
            eprintln!("Warning: Failed to access '{}''s metadata", entry.path().display());
            continue;
        };
        if entry_metadata.is_file() {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else if entry_metadata.is_dir() {
            copy_dir(entry.path(), dest.as_ref().join(entry.file_name()), &[])?;
        } else {
            return Err(anyhow!(
                "'{}' is not a file or directory",
                entry.path().display()
            ));
        }
    }
    Ok(())
}
