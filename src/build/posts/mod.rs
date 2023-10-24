mod content;

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::Serialize;
use snafu::ResultExt;
use upon::TemplateRef;

use crate::{frontmatter::Frontmatter, Config};

use self::content::compile_content;

use super::{engine::Engine, read_dir, BuildError, BuildRes, ParseFrontmatterSnafu};

/// All of the required information about a given post
#[derive(Clone, Serialize)]
struct Post {
    /// The path of the source file
    #[serde(skip_serializing)]
    pub path: PathBuf,
    /// The frontmatter for this post, contains metadata
    pub frontmatter: Frontmatter,
    /// The main content of the post, rendered as html
    pub content: String,
}

/// Information to be passed to the `post.html` template for each post
#[derive(Serialize)]
struct PostInfo<'a> {
    #[serde(flatten)]
    post: &'a Post,
    #[serde(flatten)]
    config: &'a Config,
}

impl Post {
    /// Parse a post, which consists of frontmatter and content
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, BuildError> {
        let path = path.as_ref();
        let source = fs::read_to_string(path).map_err(|err| (err, path.to_path_buf()))?;

        let mut sections = source.splitn(3, "---");

        let frontmatter_source = sections
            .nth(1)
            .ok_or_else(|| BuildError::MissingFrontmatter {
                path: path.to_path_buf(),
            })?;
        let frontmatter =
            serde_yaml::from_str(frontmatter_source).with_context(|_| ParseFrontmatterSnafu {
                path: path.to_path_buf(),
            })?;

        let content_markdown = sections
            .next()
            .ok_or_else(|| BuildError::MissingFrontmatter {
                path: path.to_path_buf(),
            })?;
        let content = compile_content(content_markdown);

        Ok(Self {
            path: path.to_path_buf(),
            frontmatter,
            content,
        })
    }

    /// Render a post to its destination using an `upon::TemplateRef`
    pub fn render(
        &self,
        config: &Config,
        post_dest: impl AsRef<Path>,
        template: TemplateRef,
    ) -> BuildRes {
        let post_dest = post_dest.as_ref();
        dbg!(post_dest);
        let writer = File::create(post_dest).map_err(|err| (err, post_dest.to_path_buf()))?;
        template
            .render(PostInfo { post: self, config })
            .to_writer(writer)
            .map_err(|err| BuildError::RenderPost {
                source: Box::new(err),
                path: self.path.to_path_buf(),
            })
    }
}

/// Compile posts in the project's `posts/` directory to HTML, storing results in `static/posts/`
///
/// # Details
///
/// If a corresponding HTML file does not exist for a post, or the post source is newer than the
/// HTML outout, then the post will be compiled, otherwise no action will be taken for that post
///
/// # Panics
///
/// If the `post.html` template has not been loaded into `engine`, a panic will occur
pub fn compile_posts(config: &Config, engine: &Engine, dir: impl AsRef<Path>) -> BuildRes {
    let dir = dir.as_ref();
    let posts_dir = dir.join("posts");
    let static_posts_dir = dir.join("static/posts");
    fs::create_dir_all(&static_posts_dir).map_err(|err| (err, posts_dir.clone()))?;

    let post_template = engine.get_post();
    for post in read_dir(&posts_dir, &[])? {
        let post_path = post.path();
        let post_metadata = post.metadata().map_err(|err| (err, post_path.clone()))?;
        let html_path = {
            let mut html_filename = post_path
                .file_stem()
                .expect("missing filename")
                .to_os_string();
            html_filename.push(".html");
            static_posts_dir.join(html_filename)
        };
        // If the corresponding HTML file exists then re-compile the post if it is newer, otherwise
        // do nothing
        if html_path.exists() {
            let html_metadata = html_path.metadata().expect("Failed to get file metadata");
            if post_metadata.modified().unwrap() > html_metadata.modified().unwrap() {
                let post = Post::from_path(&post_path)?;
                post.render(config, html_path, post_template)?;
            }
        // If it does not exist then just compile the post
        } else {
            let post = Post::from_path(&post_path)?;
            post.render(config, html_path, post_template)?;
        }
    }

    Ok(())
}
