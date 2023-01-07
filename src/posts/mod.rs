use std::{
    ffi::OsStr,
    fmt::{self, Display},
    fs, io,
    path::PathBuf,
};

use self::post::{ParseError, Post};

mod content;
pub mod frontmatter;
pub mod post;

#[derive(Debug)]
pub enum GetPostsError {
    ParseError(ParseError),
    ReadDir(io::Error),
    ReadFile(PathBuf, io::Error),
}

impl Display for GetPostsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetPostsError::ParseError(err) => err.fmt(f),
            GetPostsError::ReadDir(err) => write!(f, "Failed to read posts dir\n{err}"),
            GetPostsError::ReadFile(path, err) => {
                write!(f, "Failed to read post '{}\n{err}", path.display())
            }
        }
    }
}

pub fn get_posts() -> Result<Vec<Post>, GetPostsError> {
    let entries = fs::read_dir("./posts").map_err(GetPostsError::ReadDir)?;
    let mut posts = Vec::new();
    for entry in entries.flatten() {
        if let Ok(true) = entry.metadata().map(|meta| meta.is_file()) {
            let path = entry.path();
            if path.extension().and_then(OsStr::to_str) == Some("md") {
                let source = fs::read_to_string(&path)
                    .map_err(|err| GetPostsError::ReadFile(path.to_path_buf(), err))?;
                posts.push(Post::from_str(&source, path).map_err(GetPostsError::ParseError)?);
            }
        }
    }
    Ok(posts)
}
