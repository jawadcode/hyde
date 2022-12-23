use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Options, Parser};

use self::frontmatter::Frontmatter;

pub mod frontmatter;

#[derive(Debug, Clone)]
pub struct Post<'src> {
    front_matter: Frontmatter<'src>,
    content: String,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    /// The path of the file in which the error occurred
    pub path: PathBuf,
    /// The error itself
    pub info: String,
}

impl<'src> Post<'src> {
    fn from_str(source: &'src str, path: &Path) -> Result<Self, ParseError> {
        let mut sections = source.split("---");
        let front_matter_text = sections.nth(1).ok_or_else(|| ParseError {
            path: path.to_path_buf(),
            info: "Missing frontmatter".to_string(),
        })?;
        let front_matter = Frontmatter::from_str(front_matter_text, path)?;

        let content_markdown = sections.next().ok_or_else(|| ParseError {
            path: path.to_path_buf(),
            info: "Missing frontmatter terminator".to_string(),
        })?;
        let parser = Parser::new_ext(content_markdown, Options::empty());
        let mut content = String::new();
        html::push_html(&mut content, parser);

        Ok(Self {
            front_matter,
            content,
        })
    }
}

#[test]
fn test() {
    let test = r#"---
title: Cooking
datetime: 2022-12-23T02:58:04.390Z
tags:
---
egg"#;
    let test = Post::from_str(test, &Path::new("test"));
    dbg!(test);
}
