use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, File},
    path::{Path, PathBuf},
};

use crate::Config;

use super::{
    content::{parse_content, summarise_content},
    frontmatter::Frontmatter,
};

use anyhow::Context;
use serde::Serialize;
use upon::TemplateRef;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Post {
    /// The path of the source file
    #[serde(skip_serializing)]
    pub path: PathBuf,
    /// The frontmatter for this post, contains metadata
    pub front_matter: Frontmatter,
    /// The main content of the post, rendered as html
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    /// The path of the file in which the error occurred
    pub path: PathBuf,
    /// The error itself
    pub info: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nFailed to parse '{}'",
            self.info,
            self.path.display()
        )
    }
}

impl Error for ParseError {}

#[derive(Serialize)]
pub struct PostInfo<'a> {
    #[serde(flatten)]
    post: &'a Post,
    #[serde(flatten)]
    config: &'a Config,
}

impl Post {
    /// Parse a post, made up of the frontmatter and content
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ParseError> {
        let source = fs::read_to_string(&path).map_err(|err| ParseError {
            path: path.as_ref().to_path_buf(),
            info: format!("Failed to read '{}': {err}", path.as_ref().display()),
        })?;
        let mut sections = source.splitn(3, "---");
        let front_matter_text = sections.nth(1).ok_or_else(|| ParseError {
            path: path.as_ref().to_path_buf(),
            info: "Missing frontmatter".to_string(),
        })?;
        let front_matter = Frontmatter::from_str(front_matter_text, &path)?;

        let content_markdown = sections.next().ok_or_else(|| ParseError {
            path: path.as_ref().to_path_buf(),
            info: "Missing frontmatter terminator".to_string(),
        })?;
        let content = parse_content(content_markdown);

        Ok(Self {
            path: path.as_ref().to_path_buf(),
            front_matter,
            content,
        })
    }

    /// Render a post to its destination using an `upon::TemplateRef`
    pub fn render(
        &self,
        config: &Config,
        post_dest: impl AsRef<Path>,
        template: TemplateRef,
    ) -> anyhow::Result<()> {
        let post = self;
        let writer = File::create(&post_dest)
            .with_context(|| format!("Failed to create file '{}'", post_dest.as_ref().display()))?;
        template
            .render_to_writer(writer, PostInfo { post, config })
            .with_context(|| format!("Failed to render post '{}'", post.path.display()))
    }
}

#[derive(Serialize)]
pub struct RecentPost {
    url: String,
    #[serde(flatten)]
    pub frontmatter: Frontmatter,
    summary: String,
}

impl RecentPost {
    /// Returns the post at the path, with an empty summary
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let filename = path
            .as_ref()
            .file_stem()
            .expect("should have a filename")
            .to_string_lossy()
            .to_string()
            + ".html";
        let url = PathBuf::from(".")
            .join("posts")
            .join(filename)
            .to_string_lossy()
            .to_string();
        let source = fs::read_to_string(&path).map_err(|err| ParseError {
            path: path.as_ref().to_path_buf(),
            info: format!("Failed to read '{}': {err}", path.as_ref().display()),
        })?;
        let mut sections = source.splitn(3, "---");
        let front_matter_text = sections.nth(1).ok_or_else(|| ParseError {
            path: path.as_ref().to_path_buf(),
            info: "Missing frontmatter".to_string(),
        })?;
        let frontmatter = Frontmatter::from_str(front_matter_text, &path)?;

        let content_markdown = sections.next().ok_or_else(|| ParseError {
            path: path.as_ref().to_path_buf(),
            info: "Missing frontmatter terminator".to_string(),
        })?;
        let summary = summarise_content(content_markdown);

        Ok(Self {
            url,
            frontmatter,
            summary,
        })
    }
}
