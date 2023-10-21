//! Building a Hyde project

mod engine;
mod index;
mod posts;

use std::{
    ffi::OsStr,
    fs::{self, DirEntry, Metadata},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use snafu::{ResultExt, Snafu};

use crate::{
    build::{engine::Engine, posts::compile_posts},
    Config,
};

/// An error that arose while building a Hyde project, this is a very broad categorisation,
/// involving user-input-induced errors and I/O errors
#[derive(Debug, Snafu)]
pub enum BuildError {
    /// The directory is missing the `hyde.toml` config file
    #[snafu(display("The directory is missing the `hyde.toml` config file"))]
    MissingConfig,

    /// Failed to parse the `hyde.toml` config file
    #[snafu(display("Failed to parse the `hyde.toml` config file: {source}"))]
    ParseConfig { source: toml::de::Error },

    /// Failed to compile a template in the project's theme directory
    #[snafu(display("Failed to compile a template: {source}"))]
    CompileTemplate { source: upon::Error },

    /// Missing the index template in the project's theme
    #[snafu(display("Couldn't find 'templates/index.html' in the theme directory: '{}'", path.display()))]
    IndexTemplate { path: PathBuf },

    /// Missing the post template in the project's theme
    #[snafu(display("Couldn't find 'templates/post.html' in the theme directory: '{}'", path.display()))]
    PostTemplate { path: PathBuf },

    /// Failed to compile the markdown posts in the `posts/` directory
    #[snafu(display("Failed to compile posts: '{}': {source}", path.display()))]
    CompilePosts { source: io::Error, path: PathBuf },

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
pub type BuildRes = Result<(), BuildError>;

/// Builds the Hyde project in a given directory
///
/// # Summary
///
/// Read from the `hyde.toml` config file, create the `static/` directory for statically generated output,
/// copy over the auxiliary theme files, and compile all of the posts in the `posts/` directory into it,
/// using the `templates/` from the theme specified in the config.
pub fn build_proj(dir: impl AsRef<Path>) -> BuildRes {
    let dir = dir.as_ref();
    /* Read and parse the `hyde.toml` config */
    let config_path = dir.join("hyde.toml");
    if !config_path.exists() {
        return Err(BuildError::MissingConfig);
    }
    let config_source =
        fs::read_to_string(config_path.clone()).context(MiscIOSnafu { path: config_path })?;
    let config: Config = toml::from_str(&config_source).context(ParseConfigSnafu)?;

    /* Create the `static/` directory for statically generated output if it does not already exist */
    let static_dir = dir.join("static");
    fs::create_dir_all(static_dir.clone()).context(MiscIOSnafu {
        path: static_dir.clone(),
    })?;

    /* Initialise the template engine and render the index page */
    let mut engine = Engine::default();
    engine.load_templates(&config)?;
    engine.render_index(&config, dir)?;

    /* Remove any extra files in `static/` that do not exist in the project's theme dir */
    compare_and_clean(
        &static_dir,
        &config.theme,
        &["posts", "index.html"].map(OsStr::new),
    )?;

    /* Copy all entries other than `templates/` from the project's theme directory into `static/` */
    copy_entries(&config.theme, &static_dir, &[OsStr::new("templates")])?;

    /* Compile all posts in `posts/` into `static/` */
    compile_posts(&config, dir)
        .map_err(|(source, path)| BuildError::CompilePosts { source, path })?;

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
    // Iterate over the entries in `dir` filtering out those present in `exclude`
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

/// Copies all entries from one directory to another, excluding certain entries.
///
/// # Note
///
/// Entries are only copied from `source` to `dest` if the entry in `dest` either does not exist or
/// is older than the entry in `source`
///
/// # Errors
///
/// Other than the usual points of failure, it is assumed that `source` and `dest` are accessible,
/// if not, an `Err` is returned.
fn copy_entries(
    source: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    exclude: &[&OsStr],
) -> Result<(), (io::Error, PathBuf)> {
    let (source, dest) = (source.as_ref(), dest.as_ref());
    fs::create_dir_all(dest).map_err(|err| (err, dest.to_path_buf()))?;
    for entry in read_dir(source, exclude)? {
        let entry_path = entry.path();
        let entry_metadata = entry.metadata().map_err(|err| (err, entry_path))?;
        let entry_dest = dest.join(entry.file_name());
        // If the corresponding entry in `dest` already exists, we only overwrite it if it is older
        if entry_dest.exists() {
            let dest_metadata = dest.metadata().map_err(|err| (err, entry_dest.clone()))?;
            // Idk about the error case, if your platform doesn't have a last write timestamp then
            // it's pretty much a skill issue.
            if entry_metadata.modified().unwrap() > dest_metadata.modified().unwrap() {
                copy_entry(entry, entry_metadata, entry_dest)?;
            }
        }
        // It doesn't exist so we can just copy to it
        else {
            copy_entry(entry, entry_metadata, dest)?;
        }
    }
    todo!()
}

// cheers Matt
fn read_dir<'a>(
    dir: &Path,
    exclude: &'a [&OsStr],
) -> Result<impl Iterator<Item = DirEntry> + 'a, (io::Error, PathBuf)> {
    let dir_entries = fs::read_dir(dir).map_err(|err| (err, dir.to_path_buf()))?;
    let entry_filter = |entry: &DirEntry| !exclude.contains(&entry.path().as_os_str());
    Ok(dir_entries.filter_map(move |entry| entry.ok().filter(entry_filter)))
}

fn copy_entry(
    entry: DirEntry,
    entry_metadata: Metadata,
    dest: impl AsRef<Path>,
) -> Result<(), (io::Error, PathBuf)> {
    let entry_path = entry.path();
    if entry_metadata.is_file() || entry_metadata.is_symlink() {
        fs::copy(&entry_path, dest).map_err(|err| (err, entry_path))?;
        Ok(())
    } else if entry_metadata.is_dir() {
        copy_entries(entry_path, dest, &[])
    } else {
        return Err((
            io::Error::new(ErrorKind::Other, "not a file, directory or symlink"),
            entry_path.to_path_buf(),
        ));
    }
}
