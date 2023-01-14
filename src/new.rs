use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Context;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PROJ_NAME_REGEX: Regex = Regex::new(r"^[\p{Letter}0-9-_]*$").unwrap();
}

static DEFAULT_THEME: Dir<'_> = include_dir!("./default_theme");

pub fn new(name: &str, desc: Option<&str>, dir: impl AsRef<Path>) -> anyhow::Result<()> {
    if !PROJ_NAME_REGEX.is_match(name) {
        return Err(anyhow::Error::msg(
            "Project name can only contain unicode letters, numbers, '-' or '_'",
        ));
    }
    fs::create_dir(&dir).with_context(|| format!("Couldn't create project folder for '{name}'"))?;
    let desc = desc.unwrap_or_default();
    let mut config_file = File::options()
        .write(true)
        .create(true)
        .open(dir.as_ref().join("hyde.toml"))?;
    write!(
        config_file,
        r#"name = "{name}"
description = "{desc}"
theme = "default_theme"
"#
    )?;
    DEFAULT_THEME.extract(dir.as_ref().join("default_theme"))?;
    fs::create_dir(dir.as_ref().join("posts"))
        .with_context(|| format!("Couldn't create posts directory for '{name}'"))?;
    Ok(())
}
