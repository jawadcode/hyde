use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use include_dir::include_dir;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum CreateError {
    #[snafu(display("IO Error: {source}"))]
    Io { source: io::Error },

    #[snafu(display("Failed to create project directory: {source}"))]
    ProjectDir { source: io::Error },

    #[snafu(display("Failed to open config file: {source}"))]
    OpenConfig { source: io::Error },

    #[snafu(display("Failed to extract default theme: {source}"))]
    ExtractTheme { source: io::Error },
}

pub type CreateRes = std::result::Result<(), CreateError>;

static DEFAULT_THEME: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/default_theme");

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
