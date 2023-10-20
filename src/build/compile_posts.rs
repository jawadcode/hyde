use std::{
    io,
    path::{Path, PathBuf},
};

use snafu::Snafu;

use crate::Config;

#[derive(Snafu, Debug)]
pub enum CompilePostsError {
    /// A miscellaneous I/O error
    #[snafu(display("IO error at '{}': {source}", path.display()))]
    MiscIO { source: io::Error, path: PathBuf },
}

pub type CompilePostsResult = Result<(), CompilePostsError>;

pub fn compile_posts(config: &Config, proj_dir: impl AsRef<Path>) -> CompilePostsResult {
    todo!()
}
