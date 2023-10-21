use std::{ffi::OsStr, fs};

use crate::Config;
use snafu::ResultExt;

use super::Engine;

use super::super::{read_dir, BuildRes, CompileTemplateSnafu};

impl Engine<'_> {
    /// Load all of the HTML templates from a project's `theme/templates/` directory into the engine,
    /// ensuring the `index.html` and `post.html` templates exist
    pub fn load_templates(&mut self, config: &Config) -> BuildRes {
        let templates_dir = &config.theme.join("templates");
        for template in read_dir(&templates_dir, &["index.html", "post.html"].map(OsStr::new))? {
            let path = template.path();
            let name = path
                .file_stem()
                .expect("File with no filename lmao")
                .to_string_lossy()
                .into_owned();
            let source = fs::read_to_string(&path).map_err(|err| (err, path))?;
            self.engine
                .add_template(name, source)
                .context(CompileTemplateSnafu)?;
        }
        for (name, file_name) in [("index", "index.html"), ("post", "post.html")] {
            let path = templates_dir.join(file_name);
            let source = fs::read_to_string(&path).map_err(|err| (err, path))?;
            self.engine
                .add_template(name, source)
                .context(CompileTemplateSnafu)?;
        }

        Ok(())
    }
}
