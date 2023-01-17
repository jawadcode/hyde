use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Context;
use include_dir::{include_dir, Dir};

static DEFAULT_THEME: Dir<'_> = include_dir!("./default_theme");

pub fn new(name: &str, desc: Option<&str>, dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let proj_dir = dir.as_ref().join(name);
    fs::create_dir(&proj_dir)
        .with_context(|| format!("Failed to create project folder for '{name}'"))?;
    let desc = desc.unwrap_or_default();
    let mut config_file = File::options()
        .write(true)
        .create(true)
        .open(proj_dir.join("hyde.toml"))?;
    write!(
        config_file,
        r#"name = "{name}"
description = "{desc}"
theme = "default_theme"
"#
    )?;
    DEFAULT_THEME.extract(proj_dir.join("default_theme"))?;
    fs::create_dir(proj_dir.join("posts"))
        .with_context(|| format!("Failed to create 'posts/' directory for '{name}'"))?;

    println!(
        "\x1b[32;1mSuccess\x1b[0m: Created project '{name}' at '{}'",
        proj_dir.display()
    );
    Ok(())
}
