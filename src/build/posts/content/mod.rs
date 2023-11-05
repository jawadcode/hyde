mod highlight;
mod latex;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};

use self::highlight::highlight;

/// Compile the markdown content of a post into HTML
pub(super) fn compile_content(content_markdown: &str) -> String {
    let options = Options::all();
    let mut code_block_lang = None;
    let mut fragment_id = None;
    let parser = Parser::new_ext(content_markdown, options).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) => {
            code_block_lang = Some(lang.clone());
            event
        }
        Event::Text(code) if code_block_lang.is_some() => {
            // We are inside a fenced code block that has a specified source language
            Event::Html(highlight(code_block_lang.clone().unwrap(), code))
        }
        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
            code_block_lang = None;
            event
        }
        Event::Start(Tag::Heading(_, frag_id, _)) => {
            fragment_id = frag_id;
            event
        }
        Event::Text(text) if fragment_id.is_some() => {
            // We are inside a heading which has a fragment identifier
            Event::Html(format_heading(fragment_id.unwrap(), text))
        }
        Event::End(Tag::Heading(..)) => {
            fragment_id = None;
            event
        }
        event => event,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Format a heading which has an associated fragment identifier as a link
fn format_heading<'a>(fragment_id: &'a str, heading: CowStr<'a>) -> CowStr<'a> {
    let mut html = String::new();
    html.push_str(r#"<a href=""#);
    html.push_str(fragment_id);
    html.push_str(r#"">"#);
    html.push_str(heading.as_ref());
    html.push_str(r#"</a>"#);
    CowStr::from(html)
}
