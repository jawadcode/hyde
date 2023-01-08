use std::{env, fs, path::PathBuf, process};

use hyde::posts::{self, frontmatter::Frontmatter, post::Post};

const STYLES: &str = include_str!("styles.css");
const INTER_FONT: &[u8] = include_bytes!("fonts/Inter.ttf");
const FIRA_CODE_FONT: &[u8] = include_bytes!("fonts/Fira-Code.ttf");

fn main() {
    let posts = match posts::get_posts() {
        Ok(posts) => posts,
        Err(err) => {
            eprintln!("\x1b[31;1mError\x1b[0m: {err}");
            process::exit(1);
        }
    };
    dbg!(&posts);
    let mut static_path = {
        let mut cwd = env::current_dir().unwrap();
        cwd.push("static");
        cwd
    };
    if static_path.exists() {
        fs::remove_dir_all(&static_path).unwrap();
    }
    fs::create_dir(&static_path).unwrap();
    write_to_dir(&mut static_path, "styles.css", STYLES);
    static_path.push("fonts");
    fs::create_dir(&static_path).unwrap();
    write_to_dir(&mut static_path, "Inter-Regular.ttf", INTER_FONT);
    write_to_dir(&mut static_path, "Fira-Code.ttf", FIRA_CODE_FONT);
    static_path.pop();
    for Post {
        path,
        front_matter,
        content,
    } in posts
    {
        let file_stem = path.file_stem().unwrap().to_owned();
        static_path.push(file_stem.into_string().unwrap() + ".html");
        let html = gen_html(&front_matter, &content);
        fs::write(&static_path, html).unwrap();
        static_path.pop();
    }
}

fn write_to_dir(path: &mut PathBuf, filename: &'static str, content: impl AsRef<[u8]>) {
    path.push(filename);
    fs::write(&path, content).unwrap();
    path.pop();
}

fn gen_html(front_matter: &Frontmatter, content: &str) -> String {
    let mut buf = r#"<!DOCTYPE html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>"#.to_string();
    buf.push_str(&front_matter.title);
    buf.push_str(r#"</title><link rel="stylesheet" href="styles.css"></head><body lang=""#);
    buf.push_str(&front_matter.language);
    buf.push_str(r#""><h1>"#);
    buf.push_str(&front_matter.title);
    buf.push_str("</h1><h5>");
    buf.push_str(&front_matter.datetime.format("%d/%m/%y").to_string());
    buf.push_str("</h5>");
    buf.push_str(content);
    buf.push_str("</body></html>");
    buf
}

#[allow(unused)]
fn gen_navbar(buffer: &mut str, posts: &[Post]) {}
