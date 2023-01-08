use std::str::FromStr;

use lazy_static::lazy_static;
use pulldown_cmark::CowStr;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.numeric",
    "constant.builtin",
    "constant.character.escape",
    "constructor",
    "function",
    "function.builtin",
    "function.macro",
    "keyword",
    "keyword.control",
    "keyword.control.import",
    "keyword.directive",
    "label",
    "namespace",
    "operator",
    "keyword.operator",
    "special",
    "string",
    "type",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.other.member",
    "markup.heading",
    "markup.raw.inline",
    "markup.bold",
    "markup.italic",
    "markup.list",
    "markup.quote",
    "markup.link.url",
    "markup.link.text",
];

const HTML_ATTRS: &[&str] = &[
    r#"class="attribute""#,
    r#"class="comment""#,
    r#"class="constant""#,
    r#"class="constant-numeric""#,
    r#"class="constant-builtin""#,
    r#"class="constant-character-escape""#,
    r#"class="constructor""#,
    r#"class="function""#,
    r#"class="function-builtin""#,
    r#"class="function-macro""#,
    r#"class="keyword""#,
    r#"class="keyword-control""#,
    r#"class="keyword-control-import""#,
    r#"class="keyword-directive""#,
    r#"class="label""#,
    r#"class="namespace""#,
    r#"class="operator""#,
    r#"class="keyword-operator""#,
    r#"class="special""#,
    r#"class="string""#,
    r#"class="type""#,
    r#"class="variable""#,
    r#"class="variable-builtin""#,
    r#"class="variable-parameter""#,
    r#"class="variable-other-member""#,
    r#"class="markup-heading""#,
    r#"class="markup-raw-inline""#,
    r#"class="markup-bold""#,
    r#"class="markup-italic""#,
    r#"class="markup-list""#,
    r#"class="markup-quote""#,
    r#"class="markup-link-url""#,
    r#"class="markup-link-text""#,
];

lazy_static! {
    static ref C_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
    static ref CPP_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_cpp::language(),
            tree_sitter_cpp::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
    static ref RUST_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
    static ref TOML_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_toml::language(),
            tree_sitter_toml::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
}

enum Language {
    C,
    Cpp,
    Rust,
    Toml,
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // just going to assume there's no standard for this kind of thing and wing it
        match s {
            "c" => Ok(Self::C),
            "cpp" | "c++" => Ok(Self::Cpp),
            "rs" | "rust" => Ok(Self::Rust),
            "toml" => Ok(Self::Toml),
            _ => Err(()),
        }
    }
}

impl Language {
    fn get_config(&self) -> &HighlightConfiguration {
        match self {
            Language::C => &C_CONFIG,
            Language::Cpp => &CPP_CONFIG,
            Language::Rust => &RUST_CONFIG,
            Language::Toml => &TOML_CONFIG,
        }
    }
}

pub fn highlight<'src>(lang: CowStr<'src>, code: CowStr<'src>) -> CowStr<'src> {
    let lang: Language = if let Ok(lang) = lang.parse() {
        lang
    } else {
        return code;
    };
    let config = lang.get_config();
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code.as_bytes(), None, |_| None)
        .unwrap();
    let mut renderer = HtmlRenderer::new();
    renderer
        .render(highlights, code.as_bytes(), &|highlight| {
            HTML_ATTRS[highlight.0].as_bytes()
        })
        .unwrap();
    CowStr::from(String::from_utf8(renderer.html).unwrap())
}
