use std::str::FromStr;

use lazy_static::lazy_static;
use pulldown_cmark::CowStr;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter, HtmlRenderer};

/* WARNING: It is absolutely imperative that `HIGHLIGHT_NAMES` and `HTML_ATTRS` line up exactly */

// The list of recognised treesitter highlight names, as stolen from some helix theme
const HIGHLIGHT_NAMES: &[&str] = &[
    "error",
    "punctuation.bracket",
    "punctuation.special",
    "comment",
    "constant",
    "constant.builtin",
    "constant.macro",
    "string.regex",
    "string",
    "character",
    "number",
    "boolean",
    "float",
    "annotation",
    "attribute",
    "attribute.builtin",
    "namespace",
    "function.builtin",
    "function",
    "function.macro",
    "parameter",
    "parameter.reference",
    "method",
    "field",
    "property",
    "constructor",
    "conditional",
    "repeat",
    "label",
    "keyword",
    "keyword.function",
    "keyword.operator",
    "operator",
    "exception",
    "type",
    "type.builtin",
    "type.qualifier",
    "storageClass",
    "structure",
    "include",
    "variable",
    "variable.builtin",
    "text",
    "text.underline",
    "tag",
    "tag.delimiter",
    "tag.attribute",
    "text.title",
    "text.literal",
    "text.literal.markdown",
    "text.literal.markdown_inline",
    "text.emphasis",
    "text.strike",
    "text.strong",
    "text.uri",
    "textReference",
    "punctuation.delimiter",
    "stringEscape",
    "text.note",
    "text.warning",
    "text.danger",
    "text.diff.add",
    "text.diff.delete",
];

// The highlight names turned into HTML class attributes
const HTML_ATTRS: &[&str] = &[
    r#"class="error""#,
    r#"class="punctuation-bracket""#,
    r#"class="punctuation-special""#,
    r#"class="comment""#,
    r#"class="constant""#,
    r#"class="constant-builtin""#,
    r#"class="constant-macro""#,
    r#"class="string-regex""#,
    r#"class="string""#,
    r#"class="character""#,
    r#"class="number""#,
    r#"class="boolean""#,
    r#"class="float""#,
    r#"class="annotation""#,
    r#"class="attribute""#,
    r#"class="attribute-builtin""#,
    r#"class="namespace""#,
    r#"class="function-builtin""#,
    r#"class="function""#,
    r#"class="function-macro""#,
    r#"class="parameter""#,
    r#"class="parameter-reference""#,
    r#"class="method""#,
    r#"class="field""#,
    r#"class="property""#,
    r#"class="constructor""#,
    r#"class="conditional""#,
    r#"class="repeat""#,
    r#"class="label""#,
    r#"class="keyword""#,
    r#"class="keyword-function""#,
    r#"class="keyword-operator""#,
    r#"class="operator""#,
    r#"class="exception""#,
    r#"class="type""#,
    r#"class="type-builtin""#,
    r#"class="type-qualifier""#,
    r#"class="storageClass""#,
    r#"class="structure""#,
    r#"class="include""#,
    r#"class="variable""#,
    r#"class="variable-builtin""#,
    r#"class="text""#,
    r#"class="text-underline""#,
    r#"class="tag""#,
    r#"class="tag-delimiter""#,
    r#"class="tag-attribute""#,
    r#"class="text-title""#,
    r#"class="text-literal""#,
    r#"class="text-literal-markdown""#,
    r#"class="text-literal-markdown_inline""#,
    r#"class="text-emphasis""#,
    r#"class="text-strike""#,
    r#"class="text-strong""#,
    r#"class="text-uri""#,
    r#"class="textReference""#,
    r#"class="punctuation-delimiter""#,
    r#"class="stringEscape""#,
    r#"class="text-note""#,
    r#"class="text-warning""#,
    r#"class="text-danger""#,
    r#"class="text-diff-add""#,
    r#"class="text-diff-delete""#,
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
    static ref HASKELL_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            npezza93_tree_sitter_haskell::language(),
            npezza93_tree_sitter_haskell::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
    static ref OCAML_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_ocaml::language_ocaml(),
            tree_sitter_ocaml::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    };
    static ref PYTHON_CONFIG: HighlightConfiguration = {
        let mut config = HighlightConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::HIGHLIGHT_QUERY,
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
}

/// Any language that Hyde recognises inside a fenced code block
enum Language {
    C,
    Cpp,
    Haskell,
    OCaml,
    Python,
    Rust,
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Just going to assume there's no standard for this kind of thing and wing it
        match s {
            "c" => Ok(Self::C),
            "cpp" | "c++" => Ok(Self::Cpp),
            "haskell" | "hs" => Ok(Self::Haskell),
            "ocaml" | "ml" => Ok(Self::OCaml),
            "python" | "py" => Ok(Self::Python),
            "rs" | "rust" => Ok(Self::Rust),
            _ => Err(()),
        }
    }
}

impl Language {
    fn get_config(&self) -> &HighlightConfiguration {
        match self {
            Self::C => &C_CONFIG,
            Self::Cpp => &CPP_CONFIG,
            Self::Haskell => &HASKELL_CONFIG,
            Self::OCaml => &OCAML_CONFIG,
            Self::Python => &PYTHON_CONFIG,
            Self::Rust => &RUST_CONFIG,
        }
    }
}

/// Highlight the contents of a fenced code block of a given source language as HTML
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
