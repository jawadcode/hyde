use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod build;
mod frontmatter;
pub mod new;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub theme: PathBuf,
}
