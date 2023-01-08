use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

use self::highlight::highlight;

mod highlight;

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
            Event::Html(highlight(code_block_lang.clone().unwrap(), code))
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
    let html =  "<h1>This is a heading</h1>\n<p>This is some text</p>\n<pre><code class=\"language-rs\"><span class=\"keyword\">fn</span> <span class=\"function\">this</span>(<span class=\"type\">is</span>) {\n    <span class=\"function\">some</span>(code);\n}\n</code></pre>\n";
    assert_eq!(parse_content(test), html);
}
