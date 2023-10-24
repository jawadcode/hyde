use std::{fs, path::PathBuf};

use serde::Serialize;
use snafu::ResultExt;

use crate::{
    build::{BuildError, ParseFrontmatterSnafu},
    frontmatter::Frontmatter,
};

use super::summarise::summarise_content;

/// The representation of a recent post, to be passed to the `index.html` template
#[derive(Debug, Serialize)]
pub struct RecentPost {
    #[serde(skip)]
    path: PathBuf,
    url: Option<String>,
    #[serde(flatten)]
    pub frontmatter: Frontmatter,
    md_content: String,
    summary: Option<String>,
}

impl RecentPost {
    /// Returns the post at the path, with an empty summary
    pub fn from_path(path: PathBuf) -> Result<Self, BuildError> {
        let md_content = fs::read_to_string(&path).map_err(|err| (err, path.clone()))?;
        let mut sections = md_content.split("---");
        let frontmatter_source = sections
            .nth(1)
            .ok_or_else(|| BuildError::MissingFrontmatter { path: path.clone() })?;
        let frontmatter = serde_yaml::from_str(frontmatter_source)
            .with_context(|_| ParseFrontmatterSnafu { path: path.clone() })?;

        Ok(Self {
            path,
            url: None,
            frontmatter,
            md_content,
            summary: None,
        })
    }

    /// Fill in the `url` and `summary` fields of the post
    pub fn hydrate(self) -> Result<Self, BuildError> {
        let filename = self
            .path
            .file_stem()
            .expect("missing filename")
            .to_string_lossy()
            .to_string()
            + ".html";
        let url = Some(
            PathBuf::from(".")
                .join("posts")
                .join(filename)
                .to_string_lossy()
                .to_string(),
        );
        let content_markdown =
            self.md_content
                .split("---")
                .nth(2)
                .ok_or_else(|| BuildError::MissingFrontmatter {
                    path: self.path.clone(),
                })?;
        let summary = Some(summarise_content(content_markdown));
        Ok(RecentPost {
            url,
            summary,
            ..self
        })
    }
}
