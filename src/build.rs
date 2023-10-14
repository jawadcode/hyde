//! Building a Hyde project

use std::{
    ffi::OsStr,
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use snafu::{ResultExt, Snafu};

use crate::Config;

/// An error that arose while building a Hyde project, this is a very broad categorisation, involving user-input-induced errors and I/O errors
#[derive(Debug, Snafu)]
pub enum BuildError {
    /// The directory is missing the `hyde.toml` config file
    #[snafu(display("The directory is missing the `hyde.toml` config file"))]
    MissingConfig,

    /// Failed to parse the `hyde.toml` config file
    #[snafu(display("Failed to parse the `hyde.toml` config file: {source}"))]
    ParseConfig { source: toml::de::Error },

    /// A miscellaneous I/O error
    #[snafu(display("IO error at '{}': {source}", path.display()))]
    MiscIO { source: io::Error, path: PathBuf },
}

impl From<(io::Error, PathBuf)> for BuildError {
    fn from((source, path): (io::Error, PathBuf)) -> Self {
        BuildError::MiscIO { source, path }
    }
}

/// The [`Result`] of trying to build a Hyde project
pub type BuildRes = std::result::Result<(), BuildError>;

/// Builds the Hyde project in a given directory
///
/// # Summary
///
/// Read from the `hyde.toml` config file, create the `static/` directory for statically generated output, copy over the auxiliary theme files,
/// and compile all of the posts in the `posts/` directory into it, using the `templates/` from the theme specified in the config.
pub fn build_proj(dir: impl AsRef<Path>) -> BuildRes {
    /* Read and parse the `hyde.toml` config */
    let config_path = dir.as_ref().join("hyde.toml");
    if !config_path.exists() {
        return Err(BuildError::MissingConfig);
    }
    let config_source =
        fs::read_to_string(config_path.clone()).context(MiscIOSnafu { path: config_path })?;
    let config: Config = toml::from_str(&config_source).context(ParseConfigSnafu {})?;

    /* Create the `static/` directory for statically generated output if it does not already exist */
    let static_dir = dir.as_ref().join("static");
    fs::create_dir_all(static_dir.clone()).context(MiscIOSnafu {
        path: static_dir.clone(),
    })?;

    /* Remove any extra files in `static/` that do not exist in `config.source` */
    compare_and_clean(
        static_dir,
        config.theme,
        &["posts", "index.html"].map(OsStr::new),
    )?;
    todo!()
}

/// Compares two directories and cleans entries in the former that aren't present in the latter,
/// excluding certain files.
///
/// # Arguments
///
/// * `dir` - The directory being cleaned
/// * `against` - The directory against which comparisons are made
/// * `exclude` - The top-level entries in `dir` that are not to be removed
fn compare_and_clean(
    dir: impl AsRef<Path>,
    against: impl AsRef<Path>,
    exclude: &[&OsStr],
) -> Result<(), (io::Error, PathBuf)> {
    if !against.as_ref().exists() {
        return Ok(());
    }

    let dir = dir.as_ref();
    for entry in fs::read_dir(dir)
        .map_err(|err| (err, dir.to_path_buf()))?
        .filter_map(|entry| {
            entry
                .map(|entry| {
                    if exclude.contains(&entry.path().as_os_str()) {
                        None
                    } else {
                        Some(entry)
                    }
                })
                .unwrap_or(None) // Just discard `Err`oneous entries, they're not worth handling
        })
    {
        let against_path = against.as_ref().join(entry.file_name());
        let entry_path = entry.path();
        let file_type = entry.file_type().map_err(|err| (err, entry_path.clone()))?;

        if !against_path.exists() {
            if file_type.is_file() || file_type.is_symlink() {
                fs::remove_file(&entry_path).map_err(|err| (err, entry_path.to_path_buf()))?;
            } else if file_type.is_dir() {
                fs::remove_dir_all(&entry_path).map_err(|err| (err, entry_path.to_path_buf()))?;
            } else
            /* ✨ a magical fourth thing ✨ */
            {
                return Err((
                    io::Error::new(ErrorKind::Other, "not a file, directory or symlink"),
                    entry_path.to_path_buf(),
                ));
            }
        } else if against_path.exists() && file_type.is_dir() {
            compare_and_clean(dir.join(entry.file_name()), against_path, &[])?;
        }
    }
    todo!()
}
