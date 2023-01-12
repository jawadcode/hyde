use std::path::PathBuf;

use serde::Deserialize;

pub mod build;
pub mod new;
pub mod posts;
pub mod serve;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub theme: PathBuf,
}
