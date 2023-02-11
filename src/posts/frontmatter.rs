use std::path::Path;

use serde::{Deserialize, Serialize};

use super::post::ParseError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub title: String,
    pub timestamp: String,
    /// According to RFC 5646 (i.e. the `lang` attribute of the `html` tag)
    pub language: String,
    pub tags: Vec<String>,
}

impl Frontmatter {
    /// Parse the `front_matter` as YAML
    pub(super) fn from_str(front_matter: &str, path: impl AsRef<Path>) -> Result<Self, ParseError> {
        serde_yaml::from_str(front_matter).map_err(|err| ParseError {
            path: path.as_ref().to_path_buf(),
            info: err.to_string(),
        })
    }
}
