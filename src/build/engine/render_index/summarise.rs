use pulldown_cmark::{Event, Options, Parser, Tag};

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
