//! Creating a new project

use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use include_dir::include_dir;
use snafu::Snafu;

/// An error that occurred during the creation of a Hyde project, there is not much that can go wrong other than IO errors, so this just serves as a classification for them
#[derive(Debug, Snafu)]
pub enum CreateError {
    /// A miscellaneous IO error, e.g. from trying to write to the config
    #[snafu(display("IO Error: {source}"))]
    Io { source: io::Error },

    /// Failed to create project directory
    #[snafu(display("Failed to create project directory: {source}"))]
    ProjectDir { source: io::Error },

    /// Failed to open config file
    #[snafu(display("Failed to open config file: {source}"))]
    OpenConfig { source: io::Error },
    
    /// Failed to extract default theme
    #[snafu(display("Failed to extract default theme: {source}"))]
    ExtractTheme { source: io::Error },
}

/// The [`Result`] of trying to create a Hyde project
pub type CreateRes = std::result::Result<(), CreateError>;

static DEFAULT_THEME: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/default_theme");

/// Creates a new Hyde project
///
/// # Arguments
///
/// * `dir` - The directory in which the project's directory should be created
/// * `name` - The name of the Hyde project and of the directory the project will be stored in
/// * `display_name` - The display name for the site, i.e. the one that will be passed to the index template
/// * `desc` - An optional description of the site (or some witty tagline)
///
/// # Summary
///
/// Includes creating and writing to the config (stored in `hyde.toml`), as well as extracting the embedded default theme into the project dir
pub fn new_project(
    dir: impl AsRef<Path>,
    name: &str,
    display_name: &str,
    desc: Option<&str>,
) -> CreateRes {
    let dir = dir.as_ref().join(name);
    fs::create_dir(&dir).map_err(|source| CreateError::ProjectDir { source })?;

    let mut config = File::options()
        .write(true)
        
        .open(dir.join("hyde.toml"))
        .map_err(|source| CreateError::OpenConfig { source })?;
    write_config(&mut config, name, display_name, desc)
        .map_err(|source| CreateError::Io { source })?;

    DEFAULT_THEME
        .extract(dir.join("default_theme"))
        .map_err(|source| CreateError::ExtractTheme { source })?;

    println!(
        "\x1b[32;1mSuccess\x1b[0m: Created project '{name}' at '{}'",
        dir.display()
    );
    Ok(())
}

fn write_config(
    config: &mut File,
    name: &str,
    display_name: &str,
    desc: Option<&str>,
) -> io::Result<()> {
    write!(
        config,
        r#"name = "{name}"
display_name = "{display_name}"
description = "{}"
theme = "default_theme"
"#,
        desc.unwrap_or_default()
    )
}
