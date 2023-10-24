---
title: Hello World
timestamp: 2023-10-24T02:44:07.740916Z
language: en-GB
tags:
---

# A Relevant Table

| Hello | World |
|:-----:|:-----:|
| 12323 | 43242 |
| 57439 | 57438 |

# Some Code

```rs
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
```
