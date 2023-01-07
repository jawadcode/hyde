use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use super::post::ParseError;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub title: String,
    pub datetime: DateTime<FixedOffset>,
    pub tags: Vec<String>,
}

impl Frontmatter {
    /// Parse the `front_matter` as YAML
    pub(super) fn from_str(front_matter: &str, path: &Path) -> Result<Self, ParseError> {
        serde_yaml::from_str(front_matter).map_err(|err| ParseError {
            path: path.to_path_buf(),
            info: err.to_string(),
        })
    }
}
