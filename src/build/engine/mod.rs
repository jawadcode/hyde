mod load_templates;
mod render_index;

use chrono::DateTime;
use upon::{Engine as Enjin, TemplateRef};

/// A wrapper around [`upon::Engine`] that exists purely for convenience methods
pub struct Engine<'en> {
    engine: Enjin<'en>,
}

impl Default for Engine<'_> {
    fn default() -> Self {
        let mut engine = Enjin::new();
        fn truncate(text: String, len: usize) -> String {
            let index = if text.len() < len {
                text.len()
            } else {
                ceil_char_boundary(&text, len)
            };
            text[0..index].to_string() + "..."
        }
        engine.add_filter("truncate", truncate);

        fn fmt_timestamp(timestamp: String, format: String) -> String {
            DateTime::parse_from_rfc3339(&timestamp)
                .map(|dt| dt.format(&format).to_string())
                .unwrap_or_else(|_| "<invalid date>".to_string())
        }
        engine.add_filter("fmt_timestamp", fmt_timestamp);
        Self { engine }
    }
}

impl Engine<'_> {
    pub fn get_post(&self) -> TemplateRef {
        self.engine.get_template("post").unwrap()
    }
}

/* I refuse to use nightly rust */

#[inline]
fn ceil_char_boundary(s: &str, index: usize) -> usize {
    if index > s.len() {
        s.len()
    } else {
        let upper_bound = Ord::min(index + 4, s.len());
        s.as_bytes()[index..upper_bound]
            .iter()
            .position(|b| is_utf8_char_boundary(*b))
            .map_or(upper_bound, |pos| pos + index)
    }
}

#[inline]
const fn is_utf8_char_boundary(ch: u8) -> bool {
    (ch as i8) >= -0x40
}
