use lazy_static::lazy_static;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use syntect::{
    highlighting::{Theme, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};

/// Parse the main markdown content of a `Post`
pub(super) fn parse_content(content_markdown: &str) -> String {
    let options = Options::all();
    let mut code_block_lang = None;
    let parser = Parser::new_ext(content_markdown, options).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
            code_block_lang = Some(lang.clone());
            event
        }
        Event::Text(code) if code_block_lang.is_some() => {
            Event::Html(highlight_code_block(code_block_lang.clone().unwrap(), code))
        }
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
            code_block_lang = None;
            event
        }
        event => event,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

lazy_static! {
    static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref TS: ThemeSet = ThemeSet::load_defaults();
    static ref THEME: Theme = TS.themes["InspiredGitHub"].clone();
}

fn highlight_code_block<'src>(lang: CowStr<'src>, code: CowStr<'src>) -> CowStr<'src> {
    let syntax = if let Some(lang) = PS.find_syntax_by_extension(&lang) {
        lang
    } else {
        return code;
    };
    let highlighted = if let Ok(html) = highlighted_html_for_string(&code, &PS, syntax, &THEME) {
        html
    } else {
        return code;
    };
    CowStr::from(highlighted)
}

#[test]
fn parse_markdown_post_content() {
    let test = r#"
# This is a heading

This is some text

```rs
fn this(is) {
    some(code);
}
```
"#;
    let html = "<h1>This is a heading</h1>\n<p>This is some text</p>\n<pre><code class=\"language-rs\"><pre style=\"background-color:#ffffff;\">\n<span style=\"font-weight:bold;color:#a71d5d;\">fn </span><span style=\"font-weight:bold;color:#795da3;\">this</span><span style=\"color:#323232;\">(is) {\n</span><span style=\"color:#323232;\">    </span><span style=\"color:#62a35c;\">some</span><span style=\"color:#323232;\">(code);\n</span><span style=\"color:#323232;\">}\n</span></pre>\n</code></pre>\n";
    assert_eq!(parse_content(test), html);
}
