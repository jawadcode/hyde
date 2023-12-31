use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};

use self::highlight::highlight;

mod highlight;

/// Parse the main markdown content of a `Post`
pub(super) fn parse_content(content_markdown: &str) -> String {
    let options = Options::all();
    let mut code_block_lang = None;
    let mut fragment_id = None;
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
        Event::Start(Tag::Heading(_, frag_id, _)) => {
            fragment_id = frag_id;
            event
        }
        Event::Text(text) if fragment_id.is_some() => {
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

fn format_heading<'a>(fragment_id: &'a str, heading: CowStr<'a>) -> CowStr<'a> {
    let mut html = String::new();
    html.push_str(r#"<a href=""#);
    html.push_str(fragment_id);
    html.push_str(r#"">"#);
    html.push_str(heading.as_ref());
    html.push_str(r#"</a>"#);
    CowStr::from(html)
}

#[test]
fn test() {
    println!(
        "{}",
        parse_content(
            r#"# This is a H1 {#this-is-an-id1}
## This is a H2 {#this-is-an-id2}
### This is a H3 {#this-is-an-id3}
#### This is a H4 {#this-is-an-id4}
##### This is a H5 {#this-is-an-id5}
"#,
        )
    );
}

pub(super) fn summarise_content(content_markdown: &str) -> String {
    let parser = Parser::new_ext(content_markdown, Options::ENABLE_STRIKETHROUGH);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    // For each event we push into the buffer to produce the plain text version.
    for event in parser {
        match event {
            // The start and end events don't contain the text inside the tag. That's handled by the `Event::Text` arm.
            Event::Start(tag) => {
                start_tag(&tag, &mut buffer, &mut tags_stack);
                tags_stack.push(tag);
            }
            Event::End(tag) => {
                tags_stack.pop();
                end_tag(&tag, &mut buffer, &tags_stack);
            }
            Event::Text(content) => {
                if !tags_stack
                    .iter()
                    .any(|tag| matches!(tag, Tag::Strikethrough))
                {
                    buffer.push_str(&content)
                }
            }
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }
    buffer.trim().to_string()
}

fn start_tag(tag: &Tag, buffer: &mut String, tags_stack: &mut [Tag]) {
    match tag {
        Tag::Link(_, _, title) | Tag::Image(_, _, title) => buffer.push_str(title),
        Tag::Item => {
            buffer.push(' ');
            let mut lists_stack = tags_stack
                .iter_mut()
                .filter_map(|tag| match tag {
                    Tag::List(nb) => Some(nb),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let prefix_tabs_count = lists_stack.len() - 1;
            for _ in 0..prefix_tabs_count {
                buffer.push(' ')
            }
            if let Some(Some(nb)) = lists_stack.last_mut() {
                buffer.push_str(&nb.to_string());
                buffer.push_str(". ");
                *nb += 1;
            } else {
                buffer.push_str("• ");
            }
        }
        Tag::Paragraph | Tag::CodeBlock(_) | Tag::Heading(..) => buffer.push(' '),
        _ => (),
    }
}

fn end_tag(tag: &Tag, buffer: &mut String, tags_stack: &[Tag]) {
    match tag {
        Tag::Paragraph | Tag::Heading(..) => buffer.push(' '),
        Tag::CodeBlock(_) => {
            if buffer.ends_with(' ') {
                buffer.push(' ');
            }
        }
        Tag::List(_) => {
            let is_sublist = tags_stack.iter().any(|tag| matches!(tag, Tag::List(_)));
            if !is_sublist {
                buffer.push(' ')
            }
        }
        _ => (),
    }
}
