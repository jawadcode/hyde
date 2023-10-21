use std::{
    io,
    path::{Path, PathBuf},
};

use crate::Config;

pub fn compile_posts(
    config: &Config,
    proj_dir: impl AsRef<Path>,
) -> Result<(), (io::Error, PathBuf)> {
    todo!()
}
