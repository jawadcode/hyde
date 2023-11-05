/* This code is a port, from JS to Rust, of KaTeX's auto-render functionality, which can be found at:
 * https://github.com/KaTeX/KaTeX/tree/3d5de92fb0d0511ac64901bb60b5d46c5677eb28/contrib/auto-render
 *
 * The licence for that code is as follows:

The MIT License (MIT)

Copyright (c) 2013-2020 Khan Academy and other contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

 */

use std::iter::Peekable;

use once_cell::sync::Lazy;
use pulldown_cmark::{Event, Parser};
use regex::Regex;

const LATEX_DELIMS: [LatexDelim; 8] = [
    LatexDelim::new("$$", "$$", true),
    LatexDelim::new("\\(", "\\)", false),
    LatexDelim::new("\\begin{equation}", "\\end{equation}", true),
    LatexDelim::new("\\begin{align}", "\\end{align}", true),
    LatexDelim::new("\\begin{alignat}", "\\end{alignat}", true),
    LatexDelim::new("\\begin{gather}", "\\end{gather}", true),
    LatexDelim::new("\\begin{CD}", "\\end{CD}", true),
    LatexDelim::new("\\[", "\\]", true),
];

const IGNORED_TAGS: [&str; 7] = [
    "script", "noscript", "style", "textarea", "pre", "code", "option",
];

struct LatexDelim {
    left: &'static str,
    right: &'static str,
    display: bool,
}

impl LatexDelim {
    const fn new(left: &'static str, right: &'static str, display: bool) -> Self {
        Self {
            left,
            right,
            display,
        }
    }
}

pub struct Latexifier<'input, 'callback> {
    parser: Peekable<Parser<'input, 'callback>>,
    acc: Vec<Event<'input>>,
}

enum Node<'input> {
    Maths {
        data: &'input str,
        raw_data: &'input str,
        display: bool,
    },
    Text(&'input str),
}

impl<'input, 'callback> Latexifier<'input, 'callback> {
    fn new(parser: Parser<'input, 'callback>) -> Self {
        Self {
            parser: parser.peekable(),
            acc: Vec::new(),
        }
    }
}

impl<'input, 'callback> Iterator for Latexifier<'input, 'callback> {
    type Item = Event<'input>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'input, 'callback> Latexifier<'input, 'callback>
where
    'input: 'callback,
{
    fn split_at_delims(text: &'input str) -> Vec<Node<'input>> {
        static REGEX_LEFT: Lazy<Regex> = Lazy::new(|| {
            // We do a little bit of hard-coding
            Regex::new("(\\$\\$|\\\\\\(|\\\\begin\\{equation\\}|\\\\begin\\{align\\}|\\\\begin\\{alignat\\}|\\\\begin\\{gather\\}|\\\\begin\\{CD\\}|\\\\\\[)").unwrap()
        });
        static AMS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\\begin\{").unwrap());

        let mut data = Vec::new();
        let mut text_view = text;
        let mut index;
        loop {
            index = if let Some(index) = REGEX_LEFT.find(text) {
                index.start()
            } else {
                break;
            };
            if index > 0 {
                data.push(Node::Text(&text_view[..index]));
                text_view = &text_view[index..];
            }

            // Text now starts with delimiter so the find always succeeds
            let delim = LATEX_DELIMS
                .iter()
                .find(|&delim| text_view.starts_with(delim.left))
                .unwrap();
            index = if let Some(index) =
                Self::find_end_of_maths(delim.right, text_view, delim.left.len())
            {
                index
            } else {
                break;
            };
            let raw_data = &text_view[0..(index + delim.right.len())];
            let maths = if AMS_REGEX.is_match(raw_data) {
                raw_data
            } else {
                &text_view[delim.left.len()..index]
            };
            data.push(Node::Maths {
                data: maths,
                raw_data,
                display: delim.display,
            });
            text_view = &text_view[(index + delim.right.len())..];
        }

        data
    }

    fn find_end_of_maths(delim: &'static str, text: &str, start_index: usize) -> Option<usize> {
        let delim_len = delim.len();

        let mut brace_level = 0;
        let mut chars = text.char_indices().peekable();

        while let Some(&(index, ch)) = chars.peek() {
            match ch {
                _ if brace_level <= 0 && &text[index..((index) + delim_len)] == delim => {
                    return Some(index);
                }
                '\\' => {
                    chars.next();
                }
                '{' => brace_level += 1,
                '}' => brace_level -= 1,
                _ => (),
            }
            chars.next();
        }

        None
    }
}

#[test]
fn test() {
    let source = r#"
# Hi there

This is some
text

This is also  
some text

This is some more text
        "#;

    let parser = Parser::new_ext(source, Options::all());
    parser.for_each(|event| println!("{event:?}"));
}
