use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Frontmatter {
    /// The full title of the post
    pub title: String,
    /// Format: RFC 3339 (parsed by [`chrono::DateTime::parse_from_rfc3339`])
    pub timestamp: DateTime<FixedOffset>,
    /// Format: RFC 5464 (i.e. the `lang` attribute of the `html` tag)
    pub language: String,
    /// A list of topics that the post is related to
    pub tags: Vec<String>,
}
