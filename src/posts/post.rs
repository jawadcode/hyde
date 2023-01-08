use std::{
    fmt::{self, Display},
    path::PathBuf,
};

use super::{content::parse_content, frontmatter::Frontmatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Post {
    /// The path of the source file
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

impl Post {
    /// Parse a post, made up of the frontmatter and content
    pub fn from_str(source: &str, path: PathBuf) -> Result<Self, ParseError> {
        let mut sections = source.split("---");
        let front_matter_text = sections.nth(1).ok_or_else(|| ParseError {
            path: path.clone(),
            info: "Missing frontmatter".to_string(),
        })?;
        let front_matter = Frontmatter::from_str(front_matter_text, &path)?;

        let content_markdown = sections.next().ok_or_else(|| ParseError {
            path: path.clone(),
            info: "Missing frontmatter terminator".to_string(),
        })?;
        let content = parse_content(content_markdown);
        Ok(Self {
            path,
            front_matter,
            content,
        })
    }
}

#[test]
fn parse_markdown_post() {
    use chrono::DateTime;
    let test = r#"---
title: My Favourite Recipe
datetime: 2022-12-23T02:58:04.390Z
language: en-GB
tags:
---
**egg**"#;
    let path = PathBuf::from("test");
    let test = Post::from_str(test, path.clone());
    assert_eq!(
        test,
        Ok(Post {
            path,
            front_matter: Frontmatter {
                title: "My Favourite Recipe".to_string(),
                datetime: DateTime::parse_from_rfc3339("2022-12-23T02:58:04.390Z").unwrap(),
                language: "en-GB".to_string(),
                tags: Vec::new()
            },
            content: "<p><strong>egg</strong></p>\n".to_string()
        })
    );
}
