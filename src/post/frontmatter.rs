use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::ParseError;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Frontmatter<'src> {
    pub title: &'src str,
    pub datetime: DateTime<FixedOffset>,
    pub tags: Vec<&'src str>,
}

impl<'src> Frontmatter<'src> {
    /// Parse the `front_matter` as YAML
    pub fn from_str(front_matter: &'src str, path: &Path) -> Result<Self, ParseError> {
        serde_yaml::from_str(front_matter).map_err(|err| ParseError {
            path: path.to_path_buf(),
            info: err.to_string(),
        })
    }
}
