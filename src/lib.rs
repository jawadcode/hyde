#![feature(round_char_boundary)]
#![feature(iterator_try_collect)]
#![feature(try_blocks)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod build;
pub mod new;
pub mod posts;
pub mod serve;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub theme: PathBuf,
}
