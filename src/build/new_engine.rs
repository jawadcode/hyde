use std::fs;

use anyhow::Context;
use upon::Engine;

use crate::Config;

/// Construct a new `upon::Engine`, adding all the templates from '$theme_dir/templates/'
pub fn new_engine(config: &Config) -> anyhow::Result<Engine> {
    let mut engine = Engine::new();
    engine.add_filter("trunc", |text: String, len: usize| {
        let index = text.ceil_char_boundary(len);
        text[0..index].to_string() + "..."
    });
    for template in config
        .theme
        .join("templates")
        .read_dir()
        .with_context(|| "Failed to read 'templates/'")?
        .filter_map(Result::ok)
    {
        let path = template.path();
        let name = path
            .file_stem()
            .with_context(|| format!("Failed to get filestem for template '{}'", path.display()))?
            .to_string_lossy()
            .into_owned();
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read template '{}'", path.display()))?;
        engine
            .add_template(name, content)
            .with_context(|| format!("Failed to add '{}' template", path.display()))?;
    }
    Ok(engine)
}
