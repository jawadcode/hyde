use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::post::ParseError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub title: String,
    pub datetime: DateTime<FixedOffset>,
    // According to RFC 5646 (i.e. the `lang` attribute of the `html` tag)
    pub language: String,
    pub tags: Vec<String>,
}
/*
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct DT(DateTime<FixedOffset>);

impl FilterArg for DT {
    fn from_value<'a>(v: upon::Value) -> args::Result<Self::Output<'a>> {}
    fn from_value_ref(v: &upon::Value) -> args::Result<Self::Output<'_>> {}
    fn from_cow_mut<'a>(v: &'a mut ValueCow<'a>) -> args::Result<Self::Output<'a>> {}
}*/

impl Frontmatter {
    /// Parse the `front_matter` as YAML
    pub(super) fn from_str(front_matter: &str, path: &Path) -> Result<Self, ParseError> {
        serde_yaml::from_str(front_matter).map_err(|err| ParseError {
            path: path.to_path_buf(),
            info: err.to_string(),
        })
    }
}
