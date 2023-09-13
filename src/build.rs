//! Building a Hyde project

use std::path::Path;

use snafu::Snafu;

/// An error that arose while building a Hyde project, this is a very broad categorisation, involving user-input-induced errors and IO errors
#[derive(Debug, Snafu)]
pub enum BuildError {}

/// The [`Result`] of trying to build a Hyde project
pub type BuildRes = std::result::Result<(), BuildError>;

/// Build a Hyde project
pub fn build_proj(dir: impl AsRef<Path>) -> BuildRes {
    let config_path = dir.as_ref().join("hyde.toml");
    todo!()
}
