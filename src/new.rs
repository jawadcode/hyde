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

    #[snafu(display("Failed to create project directory: {source}"))]
    ProjectDir { source: io::Error },

    #[snafu(display("Failed to open config file: {source}"))]
    OpenConfig { source: io::Error },

    #[snafu(display("Failed to extract default theme: {source}"))]
    ExtractTheme { source: io::Error },
}

/// The [`Result`] of trying to create a Hyde project
pub type CreateRes = std::result::Result<(), CreateError>;

static DEFAULT_THEME: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/default_theme");

/// Create a new Hyde project with a name (passed to templates for generating the `<title>`) and optional description, including creating and writing the default config as well as extracting the embedded default theme into the project dir
pub fn new_project(dir: impl AsRef<Path>, name: &str, desc: Option<&str>) -> CreateRes {
    let dir = dir.as_ref().join(name);
    fs::create_dir(&dir).map_err(|source| CreateError::ProjectDir { source })?;
    let mut config = File::options()
        .write(true)
        .create(true)
        .open(dir.join("hyde.toml"))
        .map_err(|source| CreateError::OpenConfig { source })?;
    write_config(&mut config, name, desc).map_err(|source| CreateError::Io { source })?;
    DEFAULT_THEME
        .extract(dir.join("default_theme"))
        .map_err(|source| CreateError::ExtractTheme { source })?;
    println!(
        "\x1b[32;1mSuccess\x1b[0m: Created project '{name}' at '{}'",
        dir.display()
    );
    Ok(())
}

fn write_config(config: &mut File, name: &str, desc: Option<&str>) -> io::Result<()> {
    write!(
        config,
        r#"name = "{name}"
description = "{}"
theme = "default_theme"
"#,
        desc.unwrap_or_default()
    )
}
